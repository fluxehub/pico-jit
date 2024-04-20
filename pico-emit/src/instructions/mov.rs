use crate::registers::traits::Register;
use crate::registers::types::LowRegister;
use crate::{impl_opcode, Emitter};

pub trait Movs<Dest, Src> {
    fn movs(&mut self, dest: Dest, src: Src);
}

impl_opcode!(Movs, movs, (0, 0), (0, dest: LowRegister), (3, src: LowRegister));
impl_opcode!(Movs, movs, (11, 0b001_00), (8, dest: LowRegister), (0, src: u8));

pub trait Mov<Dest, Src> {
    fn mov(&mut self, dest: Dest, src: Src);
}

impl<Dest: Register, Src: Register> Mov<Dest, Src> for Emitter {
    fn mov(&mut self, dest: Dest, src: Src) {
        // MSB of dest
        let dn = dest.to_reg_number() & 0b1000;
        self.buffer.push(
            0b010001_10 << 8 | dn << 4 | src.to_reg_number() << 3 | (dest.to_reg_number() & 0b111),
        );
    }
}
