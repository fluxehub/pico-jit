use crate::impl_opcode;
use crate::instructions::ImmShift;
use crate::registers::types::LowRegister;

pub trait Lsr<Dest, Src> {
    fn lsr(&mut self, dest: Dest, src: Src);
}

impl_opcode!(Lsr, lsr, (6, 0b010000_0011), (0, dest: LowRegister), (3, shift: LowRegister));
impl_opcode!(Lsr, lsr, (11, 0b000_01), (0, dest: LowRegister), (3, src: ImmShift));
