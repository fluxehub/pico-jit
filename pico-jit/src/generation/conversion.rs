#![allow(clippy::fn_to_numeric_cast_with_truncation)]
use crate::{aliases::A, extern_func};
use alloc::collections::BTreeMap;
use pico_emit::{emitter::Label, instructions::*, register_list, Emitter};

extern "C" {
    fn __aeabi_f2iz(value: f32) -> i32;
    fn __aeabi_i2f(value: i32) -> f32;
    fn __aeabi_ui2f(value: u32) -> f32;
}

pub fn i32_trunc_f32_s(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    // TODO: THIS GIVES THE INCORRECT VALUE FOR NEGATIVE NUMBERS (-1.5 SHOULD BE -1)
    extern_func!(func, data_map, __aeabi_f2iz, (A) -> A);
}

pub fn i32_extend8_s(func: &mut Emitter) {
    func.pop(register_list!(A));
    func.sxtb(A, A);
    func.push(register_list!(A));
}

pub fn i32_extend16_s(func: &mut Emitter) {
    func.pop(register_list!(A));
    func.sxth(A, A);
    func.push(register_list!(A));
}

pub fn f32_convert_i32_s(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    extern_func!(func, data_map, __aeabi_i2f, (A) -> A);
}

pub fn f32_convert_i32_u(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    extern_func!(func, data_map, __aeabi_ui2f, (A) -> A);
}
