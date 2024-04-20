#![allow(clippy::fn_to_numeric_cast_with_truncation)]
use alloc::collections::BTreeMap;
use pico_emit::{emitter::Label, instructions::*, register_list, Emitter};

use crate::aliases::*;

use super::get_data_label;

extern "C" {
    fn __aeabi_memcpy(dst: *mut u8, src: *const u8, n: usize);
    fn __aeabi_memset(dst: *mut u8, n: usize, value: u8);
}

pub(crate) fn memory_copy(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    func.pop(register_list!(B, C, D));
    // B now has n, C has src, D has dst (annoyingly)
    // memcpy takes A dst, B src, C n
    func.adds(A, Add2(MEMORY, D));
    func.adds(D, Add2(MEMORY, C));
    func.movs(C, B);
    func.movs(B, D);

    let memcpy = get_data_label(func, data_map, __aeabi_memcpy as u32);
    func.ldr(D, memcpy);
    func.blx(D);
}

pub fn memory_fill(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    func.pop(register_list!(B, C, D));
    func.movs(D, A);

    let memset = get_data_label(func, data_map, __aeabi_memset as u32);
    func.ldr(D, memset);
    func.blx(D);
}
