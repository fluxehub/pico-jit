use crate::buffer::{JitBuffer, Offset};
use crate::instructions::{Condition, LabelInstruction, ToInstEncoding};
use crate::JitFn;
use alloc::vec;
use alloc::vec::Vec;
use ux2::i11;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Label {
    Unresolved(usize),
    Resolved(usize, Offset),
}

pub struct Emitter {
    pub(crate) buffer: JitBuffer,
    label_counter: usize,
    pub(crate) unfilled_instructions: Vec<(Offset, LabelInstruction)>,
}

impl Emitter {
    pub fn new() -> Self {
        Self {
            buffer: JitBuffer::new(),
            label_counter: 0,
            unfilled_instructions: Vec::new(),
        }
    }

    pub fn data(&mut self, data: u32) -> Label {
        let offset = self.buffer.push_data(data);
        self.label_counter += 1;
        Label::Resolved(self.label_counter, offset)
    }

    pub fn section_instruction_count(&self) -> usize {
        self.buffer.current_code_section_size
    }

    fn resolve_label(&mut self, label: &mut Label, offset: Offset) {
        let label_id = match *label {
            Label::Unresolved(label_id) => label_id,
            Label::Resolved(_, offset) => panic!("Label already resolved at: {:?}", offset),
        };

        // Update the label
        *label = Label::Resolved(label_id, offset);

        // Update all usages of the label in unfilled instructions
        for (_, instruction) in &mut self.unfilled_instructions {
            match instruction {
                LabelInstruction::ADR(l, _) => {
                    if *l == Label::Unresolved(label_id) {
                        *l = *label;
                    }
                }
                LabelInstruction::B(l, _) => {
                    if *l == Label::Unresolved(label_id) {
                        *l = *label;
                    }
                }
                LabelInstruction::Branch(l, _) => {
                    if *l == Label::Unresolved(label_id) {
                        *l = *label;
                    }
                }
                LabelInstruction::LDR(l, _) => {
                    if *l == Label::Unresolved(label_id) {
                        *l = *label;
                    }
                }
            }
        }
    }

    pub fn fill_label_instructions(&mut self, allow_unresolved_branches: bool) {
        fn get_offset(label: Label) -> Offset {
            match label {
                Label::Unresolved(id) => panic!("Label {} not resolved", id),
                Label::Resolved(_, offset) => offset,
            }
        }

        let mut new_unfilled_instructions = vec![];

        for (position, instruction) in self.unfilled_instructions.iter() {
            match instruction {
                LabelInstruction::B(label, condition) => {
                    let target = match label {
                        Label::Unresolved(id) => {
                            if allow_unresolved_branches {
                                new_unfilled_instructions
                                    .push((*position, LabelInstruction::B(*label, *condition)));
                                continue;
                            } else {
                                panic!("Label {} not resolved", id);
                            }
                        }
                        Label::Resolved(_, offset) => offset,
                    };

                    if target.is_data() {
                        panic!("Cannot branch to a data address");
                    }

                    let branch_offset = self.buffer.resolve_offset(*position, *target) >> 1;

                    if *condition == Condition::AL {
                        let Ok(branch_offset) = i11::try_from(branch_offset) else {
                            panic!("An offset of {} is too large for B", branch_offset << 1);
                        };

                        self.buffer.fill_instruction(
                            *position,
                            0b11100 << 11 | (i16::from(branch_offset) as u16 & 0x7FF),
                        );
                    } else {
                        let Ok(branch_offset) = i8::try_from(branch_offset) else {
                            panic!("An offset of {} is too large for B_if", branch_offset << 1);
                        };

                        self.buffer.fill_instruction(
                            *position,
                            0b1101 << 12 | (*condition as u16) << 8 | (branch_offset as u16 & 0xFF),
                        );
                    }
                }
                LabelInstruction::Branch(label, condition) => {
                    // Code duplication!
                    let target = match label {
                        Label::Unresolved(id) => {
                            if allow_unresolved_branches {
                                new_unfilled_instructions.push((
                                    *position,
                                    LabelInstruction::Branch(*label, *condition),
                                ));
                                continue;
                            } else {
                                panic!("Label {} not resolved", id);
                            }
                        }
                        Label::Resolved(_, offset) => offset,
                    };

                    if target.is_data() {
                        panic!("Cannot branch to a data address");
                    }

                    let branch_offset = self.buffer.resolve_offset(*position, *target) >> 1;

                    if *condition == Condition::AL {
                        match i11::try_from(branch_offset) {
                            Ok(branch_offset) => {
                                self.buffer.fill_instruction(
                                    *position,
                                    0b11100 << 11 | (i16::from(branch_offset) as u16 & 0x7FF),
                                );
                            }
                            Err(_) => {
                                // Transform B into BL
                                let s = (branch_offset >> 31) & 1; // Sign bit
                                let j1 = !(s ^ (1 & branch_offset >> 21));
                                let j2 = !(s ^ (1 & branch_offset >> 20));
                                let imm10 = (branch_offset >> 11) & 0x3FF;
                                let imm11 = branch_offset & 0x7FF;
                                self.buffer.fill_instruction(
                                    *position,
                                    0b11110 << 11 | (s as u16) << 10 | imm10 as u16,
                                );
                                self.buffer.fill_instruction(
                                    position.add(1),
                                    0b1101 << 12
                                        | (j1 as u16) << 13
                                        | (j2 as u16) << 11
                                        | imm11 as u16,
                                );
                            }
                        }
                    } else {
                        match i8::try_from(branch_offset) {
                            Ok(branch_offset) => {
                                self.buffer.fill_instruction(
                                    *position,
                                    0b1101 << 12
                                        | (*condition as u16) << 8
                                        | (branch_offset as u16 & 0xFF),
                                );
                            }
                            Err(_) => {
                                // Effectively encoding the following
                                // b<inverse cond> else
                                // bl <if>
                                // else:

                                // Emit the b<inverse cond> instruction
                                self.buffer.fill_instruction(
                                    *position,
                                    0xd001 | (condition.invert() as u16) << 8, // 1 is the correct offset for the else label
                                );

                                // Emit the bl <if> instruction
                                let position = position.add(1);
                                let branch_offset =
                                    self.buffer.resolve_offset(position, *target) >> 1;

                                let s = (branch_offset >> 31) & 1; // Sign bit
                                let j1 = !(s ^ (1 & branch_offset >> 21));
                                let j2 = !(s ^ (1 & branch_offset >> 20));
                                let imm10 = (branch_offset >> 11) & 0x3FF;
                                let imm11 = branch_offset & 0x7FF;
                                self.buffer.fill_instruction(
                                    position,
                                    0b11110 << 11 | (s as u16) << 10 | imm10 as u16,
                                );
                                self.buffer.fill_instruction(
                                    position.add(1),
                                    0b1101 << 12
                                        | (j1 as u16) << 13
                                        | (j2 as u16) << 11
                                        | imm11 as u16,
                                );
                            }
                        }
                    }
                }
                LabelInstruction::LDR(label, dest) => {
                    let target = get_offset(*label);
                    if target.is_instruction() {
                        panic!("Cannot load an instruction as data");
                    }

                    let offset =
                        self.buffer.resolve_offset(position.align_word(), target) as usize >> 2; // Divide by 4 to get the word offset

                    let offset = u8::try_from(offset).expect("LDR offset too large");
                    self.buffer.fill_instruction(
                        *position,
                        0b01001 << 11 | dest.to_instruction_encoded() << 8 | (offset as u16),
                    );
                }
                _ => unimplemented!(),
            }
        }

        self.unfilled_instructions = new_unfilled_instructions;
    }

    pub fn create_label(&mut self) -> Label {
        let label = Label::Unresolved(self.label_counter);
        self.label_counter += 1;
        label
    }

    pub fn label(&mut self, label: &mut Label) {
        self.resolve_label(label, self.buffer.current_offset());
    }

    // Allows for early writing of the data section in cases where the data section needs to be
    // interleaved with the instructions (i.e. when the code section is too large)
    pub fn emit_data_section(&mut self) {
        // Fill in LDR instructions (+ resolved branches)
        self.fill_label_instructions(true);
        self.buffer.write_data_section();
    }

    /// Builds the function and returns it
    pub fn build(mut self) -> JitFn {
        // Fill in the label instructions
        self.fill_label_instructions(false);

        // Finish the buffer
        JitFn {
            data: self.buffer.finish(),
        }
    }
}
