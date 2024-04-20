use crate::instructions::{ImmOffset, RegOffset, SPWithOffset, ToInstEncoding};
use crate::registers::types::{LowRegister, StackPointer};
use crate::{impl_opcode, Emitter};

pub trait Str<Src, Base> {
    fn str(&mut self, src: Src, base: Base);
}

impl_opcode!(Str, str, (11, 0b011_0_0), (0, src: LowRegister), (3, base: ImmOffset));
impl_opcode!(Str, str, (6, 0b011_0_0_00000), (0, src: LowRegister), (3, base: LowRegister));
impl_opcode!(Str, str, (11, 0b1001_0), (8, src: LowRegister), (0, base: SPWithOffset));
impl_opcode!(Str, str, (9, 0b0101_000), (0, src: LowRegister), (3, base: RegOffset));

impl Str<LowRegister, StackPointer> for Emitter {
    fn str(&mut self, src: LowRegister, _base: StackPointer) {
        self.buffer
            .push(0b1001_0 << 11 | src.to_instruction_encoded() << 8);
    }
}

pub trait Strb<Src, Base> {
    fn strb(&mut self, src: Src, base: Base);
}

impl_opcode!(Strb, strb, (11, 0b011_1_0), (0, src: LowRegister), (3, base: ImmOffset));
impl_opcode!(Strb, strb, (6, 0b011_1_0_00000), (0, src: LowRegister), (3, base: LowRegister));
impl_opcode!(Strb, strb, (9, 0b0101_010), (0, src: LowRegister), (3, base: RegOffset));

pub trait Strh<Src, Base> {
    fn strh(&mut self, src: Src, base: Base);
}

impl_opcode!(Strh, strh, (11, 0b1000_0), (0, src: LowRegister), (3, base: ImmOffset));
impl_opcode!(Strh, strh, (6, 0b1000_0_00000), (0, src: LowRegister), (3, base: LowRegister));
impl_opcode!(Strh, strh, (9, 0b0101_001), (0, src: LowRegister), (3, base: RegOffset));
