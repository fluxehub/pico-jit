use crate::emitter::Label;
use crate::instructions::{ImmOffset, LabelInstruction, RegOffset, SPWithOffset, ToInstEncoding};
use crate::registers::types::{LowRegister, StackPointer};
use crate::{impl_opcode, Emitter};

pub trait Ldr<Dest, Base> {
    fn ldr(&mut self, dest: Dest, base: Base);
}

impl_opcode!(Ldr, ldr, (11, 0b011_0_1), (0, dest: LowRegister), (3, base: ImmOffset));
impl_opcode!(Ldr, ldr, (6, 0b011_0_1_00000), (0, dest: LowRegister), (3, base: LowRegister));
impl_opcode!(Ldr, ldr, (11, 0b1001_1), (8, dest: LowRegister), (0, base: SPWithOffset));
impl_opcode!(Ldr, ldr, (9, 0b0101_100), (0, dest: LowRegister), (3, base: RegOffset));

impl Ldr<LowRegister, StackPointer> for Emitter {
    fn ldr(&mut self, dest: LowRegister, _base: StackPointer) {
        self.buffer
            .push(0b1001_1 << 11 | dest.to_instruction_encoded() << 8);
    }
}

impl Ldr<LowRegister, Label> for Emitter {
    fn ldr(&mut self, dest: LowRegister, label: Label) {
        let offset = self.buffer.push_empty();
        self.unfilled_instructions
            .push((offset, LabelInstruction::LDR(label, dest)));
    }
}

pub trait Ldrb<Dest, Base> {
    fn ldrb(&mut self, dest: Dest, base: Base);
}

impl_opcode!(Ldrb, ldrb, (11, 0b011_1_1), (0, dest: LowRegister), (3, base: ImmOffset));
impl_opcode!(Ldrb, ldrb, (6, 0b011_1_1_00000), (0, dest: LowRegister), (3, base: LowRegister));
impl_opcode!(Ldrb, ldrb, (9, 0b0101_110), (0, dest: LowRegister), (3, base: RegOffset));

pub trait Ldrh<Dest, Base> {
    fn ldrh(&mut self, dest: Dest, base: Base);
}

impl_opcode!(Ldrh, ldrh, (11, 0b1000_1), (0, dest: LowRegister), (3, base: ImmOffset));
impl_opcode!(Ldrh, ldrh, (6, 0b1000_1_00000), (0, dest: LowRegister), (3, base: LowRegister));
impl_opcode!(Ldrh, ldrh, (9, 0b0101_101), (0, dest: LowRegister), (3, base: RegOffset));

pub trait Ldrsb<Dest, Base> {
    fn ldrsb(&mut self, dest: Dest, base: Base);
}

pub trait Ldrsh<Dest, Base> {
    fn ldrsh(&mut self, dest: Dest, base: Base);
}

impl_opcode!(Ldrsb, ldrsb, (9, 0b0101_011), (0, dest: LowRegister), (3, base: RegOffset));
impl_opcode!(Ldrsh, ldrsh, (9, 0b0101_111), (0, dest: LowRegister), (3, base: RegOffset));
