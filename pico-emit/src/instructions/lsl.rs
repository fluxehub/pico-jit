use crate::impl_opcode;
use crate::instructions::ImmShift;
use crate::registers::types::LowRegister;

pub trait Lsl<Dest, Src> {
    fn lsl(&mut self, dest: Dest, src: Src);
}

impl_opcode!(Lsl, lsl, (6, 0b010000_0010), (0, dest: LowRegister), (3, shift: LowRegister));
impl_opcode!(Lsl, lsl, (11, 0b000_00), (0, dest: LowRegister), (3, src: ImmShift));
