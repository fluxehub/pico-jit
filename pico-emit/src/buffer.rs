use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct JitBuffer {
    buffer: Vec<u16>, // Emitted code buffer
    data: Vec<u16>, // Used for data (not code) that the function will use. Copied into the code buffer when the function is built
    pub current_code_section_size: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Offset {
    Instruction(usize),
    Data(usize),
}

impl Offset {
    pub(crate) fn is_instruction(&self) -> bool {
        matches!(self, Offset::Instruction(_))
    }

    pub(crate) fn is_data(&self) -> bool {
        matches!(self, Offset::Data(_))
    }

    pub(crate) fn align_word(&self) -> Self {
        match self {
            Offset::Instruction(offset) => {
                if *offset % 2 == 0 {
                    *self
                } else {
                    Offset::Instruction(*offset - 1)
                }
            }
            Offset::Data(_) => *self, // Data is always aligned
        }
    }

    pub(crate) fn add(&self, offset: usize) -> Self {
        match self {
            Offset::Instruction(offset1) => Offset::Instruction(offset1 + offset),
            Offset::Data(offset1) => Offset::Data(offset1 + offset),
        }
    }
}

impl JitBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(1),
            data: Vec::new(),
            current_code_section_size: 0,
        }
    }

    pub(crate) fn push(&mut self, instruction: u16) {
        self.buffer.push(instruction);
        self.current_code_section_size += 1;
    }

    pub(crate) fn push_empty(&mut self) -> Offset {
        let offset = self.buffer.len();
        self.buffer.push(0);
        Offset::Instruction(offset)
    }

    pub(crate) fn push_data(&mut self, data: u32) -> Offset {
        let offset = self.data.len();
        self.data.push((data & 0xFFFF) as u16);
        self.data.push((data >> 16) as u16);

        Offset::Data(offset)
    }

    pub(crate) fn fill_instruction(&mut self, offset: Offset, instruction: u16) {
        match offset {
            Offset::Instruction(offset) => {
                self.buffer[offset] = instruction;
            }
            _ => panic!("Cannot set instruction at a data offset"),
        }
    }

    fn get_data_start(&self) -> usize {
        if self.buffer.len() % 2 == 0 {
            self.buffer.len() - 1
        } else {
            self.buffer.len() // Data is aligned to 32 bits
        }
    }

    pub(crate) fn current_offset(&self) -> Offset {
        Offset::Instruction(self.buffer.len() - 2)
    }

    // Helper function to get the absolute memory offset of a label
    fn resolve_offset_address(&self, offset: Offset) -> usize {
        match offset {
            Offset::Instruction(offset) => offset << 1, // Shift left 1 to convert 16 bit offset to byte offset
            Offset::Data(offset) => (self.get_data_start() + offset) << 1, // Data is stored after the code
        }
    }

    /// Returns the memory offset between a source and target
    ///
    /// This should only be called after all code has been emitted, otherwise data offsets will be incorrect
    pub(crate) fn resolve_offset(&self, source: Offset, target: Offset) -> isize {
        let source = self.resolve_offset_address(source);
        let target = self.resolve_offset_address(target);

        target as isize - source as isize
    }

    // Writes the current data section to the buffer
    // Does not clear the data section
    pub(crate) fn write_data_section(&mut self) {
        if self.buffer.len() % 2 == 1 {
            self.buffer.push(0); // Pad with a 0 if the function end is not word aligned
        }

        self.buffer.extend_from_slice(&self.data); // Append the data to the end of the code buffer
        self.current_code_section_size = 0; // Reset the code section size
    }

    pub(crate) fn finish(mut self) -> Box<[u16]> {
        self.write_data_section();
        self.buffer.shrink_to_fit(); // Shrink the buffer to the minimum size needed
        let buffer_ptr = self.buffer.as_ptr();
        let slice = self.buffer.into_boxed_slice();
        let slice_ptr = slice.as_ptr();
        assert_eq!(slice_ptr, buffer_ptr); // Ensure the buffer is not moved
        slice
    }

    pub fn copy_to_slice(&self, slice: &mut [u16]) {
        slice.copy_from_slice(&self.buffer);
    }
}
