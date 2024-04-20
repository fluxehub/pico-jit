use crate::impl_opcode;
use crate::instructions::ImmShift;
use crate::registers::types::LowRegister;

pub trait Asr<Dest, Src> {
    fn asr(&mut self, dest: Dest, src: Src);
}

impl_opcode!(Asr, asr, (6, 0b010000_0100), (0, dest: LowRegister), (3, shift: LowRegister));
impl_opcode!(Asr, asr, (11, 0b000_10), (0, dest: LowRegister), (3, src: ImmShift));
