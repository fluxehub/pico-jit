use crate::instructions::ToInstEncoding;
use crate::registers::types::{LowRegister, StackPointer};
use crate::Emitter;
use ux2::{u3, u7};

pub struct Sub2(pub LowRegister, pub LowRegister);
pub struct Sub2Imm(pub LowRegister, pub u3);

pub trait Subs<Dest, Src> {
    fn subs(&mut self, dest: Dest, src: Src);
}

impl Subs<LowRegister, Sub2Imm> for Emitter {
    fn subs(&mut self, dest: LowRegister, src: Sub2Imm) {
        self.buffer.push(
            0b000_11_1_1 << 9
                | src.1.to_instruction_encoded() << 6
                | src.0.to_instruction_encoded() << 3
                | dest.to_instruction_encoded(),
        );
    }
}

impl Subs<LowRegister, u8> for Emitter {
    fn subs(&mut self, dest: LowRegister, src: u8) {
        self.buffer.push(
            0b001_11 << 11 | dest.to_instruction_encoded() << 8 | src.to_instruction_encoded(),
        );
    }
}

impl Subs<LowRegister, Sub2> for Emitter {
    fn subs(&mut self, dest: LowRegister, src: Sub2) {
        self.buffer.push(
            0b000_11_0_1 << 9
                | src.1.to_instruction_encoded() << 6
                | src.0.to_instruction_encoded() << 3
                | dest.to_instruction_encoded(),
        );
    }
}

impl Subs<LowRegister, LowRegister> for Emitter {
    fn subs(&mut self, dest: LowRegister, src: LowRegister) {
        self.buffer.push(
            0b000_11_0_1 << 9
                | src.to_instruction_encoded() << 6
                | dest.to_instruction_encoded() << 3
                | dest.to_instruction_encoded(),
        );
    }
}

pub trait Sub<Dest, Src> {
    fn sub(&mut self, dest: Dest, src: Src);
}

impl Sub<StackPointer, u7> for Emitter {
    fn sub(&mut self, _dest: StackPointer, src: u7) {
        self.buffer
            .push(0b1011_0000_1 << 7 | src.to_instruction_encoded());
    }
}
