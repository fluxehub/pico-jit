use crate::instructions::{SPWithOffset, ToInstEncoding};
use crate::registers::traits::Register;
use crate::registers::types::{LowRegister, StackPointer};
use crate::{impl_opcode, Emitter};
use ux2::{u3, u7};

pub struct Add2(pub LowRegister, pub LowRegister);
pub struct Add2Imm(pub LowRegister, pub u3);

pub trait Adds<Dest, Src> {
    fn adds(&mut self, dest: Dest, src: Src);
}

impl Adds<LowRegister, Add2Imm> for Emitter {
    fn adds(&mut self, dest: LowRegister, src: Add2Imm) {
        self.buffer.push(
            0b000_11_1_0 << 9
                | src.1.to_instruction_encoded() << 6
                | src.0.to_instruction_encoded() << 3
                | dest.to_instruction_encoded(),
        );
    }
}

impl Adds<LowRegister, u8> for Emitter {
    fn adds(&mut self, dest: LowRegister, src: u8) {
        self.buffer.push(
            0b001_10 << 11 | dest.to_instruction_encoded() << 8 | src.to_instruction_encoded(),
        );
    }
}

impl Adds<LowRegister, Add2> for Emitter {
    fn adds(&mut self, dest: LowRegister, src: Add2) {
        self.buffer.push(
            0b000_11_0_0 << 9
                | src.1.to_instruction_encoded() << 6
                | src.0.to_instruction_encoded() << 3
                | dest.to_instruction_encoded(),
        );
    }
}

impl Adds<LowRegister, LowRegister> for Emitter {
    fn adds(&mut self, dest: LowRegister, src: LowRegister) {
        self.buffer.push(
            0b000_11_0_0 << 9
                | src.to_instruction_encoded() << 6
                | dest.to_instruction_encoded() << 3
                | dest.to_instruction_encoded(),
        );
    }
}

pub trait Add<Dest, Src> {
    fn add(&mut self, dest: Dest, src: Src);
}

impl_opcode!(Add, add, (11, 0b1010_1), (8, dest: LowRegister), (0, src: SPWithOffset));

impl<Dest: Register, Src: Register> Add<Dest, Src> for Emitter {
    fn add(&mut self, dest: Dest, src: Src) {
        let dn = dest.to_instruction_encoded() & 0b1000;
        self.buffer.push(
            0b010001_00 << 8
                | dn << 4
                | src.to_instruction_encoded() << 3
                | dest.to_instruction_encoded() & 0b111,
        );
    }
}

impl Add<StackPointer, u7> for Emitter {
    fn add(&mut self, _dest: StackPointer, src: u7) {
        self.buffer
            .push(0b1011_0000_0 << 7 | src.to_instruction_encoded());
    }
}
