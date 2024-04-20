use crate::registers::traits::Register;
use crate::registers::types::LowRegister;
use crate::{impl_opcode, Emitter};

pub trait Cmp<A, B> {
    fn cmp(&mut self, a: A, b: B);
}

impl_opcode!(Cmp, cmp, (0, 0b001_01 << 11), (8, a: LowRegister), (0, b: u8));

impl<A: Register, B: Register> Cmp<A, B> for Emitter {
    fn cmp(&mut self, a: A, b: B) {
        if a.is_low_register() && b.is_low_register() {
            self.buffer
                .push(0b010000_1010 << 6 | b.to_reg_number() << 3 | a.to_reg_number());
        } else {
            // MSB of a
            let dn = a.to_reg_number() & 0b1000;
            self.buffer.push(
                0b010001_01 << 8 | dn << 4 | b.to_reg_number() << 3 | (a.to_reg_number() & 0b111),
            )
        }
    }
}
