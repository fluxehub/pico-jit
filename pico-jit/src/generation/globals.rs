use pico_emit::{instructions::*, register_list, Emitter};
use ux2::u5;

use crate::aliases::*;

pub fn global_get(func: &mut Emitter, index: u32) {
    func.ldr(A, ImmOffset(GLOBALS, u5::new(index as u8)));
    func.push(register_list!(A));
}

pub fn global_set(func: &mut Emitter, index: u32) {
    func.pop(register_list!(A));
    func.str(A, ImmOffset(GLOBALS, u5::new(index as u8)));
}
