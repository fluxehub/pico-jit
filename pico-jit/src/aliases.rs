use pico_emit::registers::{
    types::{HighRegister, LowRegister},
    *,
};

pub const A: LowRegister = r0;
pub const B: LowRegister = r1;
pub const C: LowRegister = r2;
pub const D: LowRegister = r3;
pub const MEMORY: LowRegister = r5;
pub const GLOBALS: LowRegister = r6;
pub const LOCALS: LowRegister = r7;
pub const ARCH_SP: HighRegister = r10;
pub const MODULE: HighRegister = r11;
