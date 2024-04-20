#![allow(clippy::fn_to_numeric_cast_with_truncation)]
use crate::{aliases::*, extern_func};
use alloc::collections::BTreeMap;
use pico_emit::{
    emitter::Label,
    instructions::*,
    register_list,
    registers::{r4, r5},
    Emitter,
};
use ux2::u3;
use wasmparser_nostd::Ieee32;

use super::get_data_label;

extern "C" {
    fn __aeabi_fcmpeq(a: f32, b: f32) -> u32;
    fn __aeabi_fcmplt(a: f32, b: f32) -> u32;
    fn __aeabi_fcmpgt(a: f32, b: f32) -> u32;
    fn __aeabi_fcmple(a: f32, b: f32) -> u32;
    fn __aeabi_fcmpge(a: f32, b: f32) -> u32;
    fn __aeabi_fadd(a: f32, b: f32) -> f32;
    fn __aeabi_fsub(a: f32, b: f32) -> f32;
    fn __aeabi_fmul(a: f32, b: f32) -> f32;
    fn __aeabi_fdiv(n: f32, d: f32) -> f32;
}

pub(crate) fn f32_const(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>, value: Ieee32) {
    let data = get_data_label(func, data_map, value.bits());

    func.ldr(A, data);
    func.push(register_list!(A));
}

fn f32_compare(
    func: &mut Emitter,
    data_map: &mut BTreeMap<u32, Label>,
    compare_func: unsafe extern "C" fn(f32, f32) -> u32,
) {
    func.pop(register_list!(A, B));
    let cmp_func = get_data_label(func, data_map, compare_func as u32);
    func.ldr(C, cmp_func);
    func.blx(C);
    func.subs(C, Sub2Imm(A, u3::new(1)));
    func.sbc(A, C);
    func.push(register_list!(A));
}

pub(crate) fn f32_eq(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    f32_compare(func, data_map, __aeabi_fcmpeq);
}

pub(crate) fn f32_ne(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    // Subs is different so we can't use the helper function
    func.pop(register_list!(A, B));
    let cmp_func = get_data_label(func, data_map, __aeabi_fcmpeq as u32);
    func.ldr(C, cmp_func);
    func.blx(C);
    func.rsb(C, A);
    func.adc(A, C);
    func.push(register_list!(A));
}

pub(crate) fn f32_lt(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    // We do the opposite of the comparison, because data from the stack is loaded in reverse order
    f32_compare(func, data_map, __aeabi_fcmpgt);
}

pub(crate) fn f32_gt(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    f32_compare(func, data_map, __aeabi_fcmplt);
}

pub(crate) fn f32_le(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    f32_compare(func, data_map, __aeabi_fcmpge);
}

pub(crate) fn f32_ge(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    f32_compare(func, data_map, __aeabi_fcmple);
}

pub(crate) fn f32_sqrt(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    use rp_pico::hal::rom_data::float_funcs::fsqrt;
    extern_func!(func, data_map, fsqrt, (A) -> A)
}

pub(crate) fn f32_add(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    extern_func!(func, data_map, __aeabi_fadd, (A, B) -> A)
}

pub(crate) fn f32_sub(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    extern_func!(func, data_map, __aeabi_fsub, (A, B) -> A)
}

pub(crate) fn f32_mul(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    extern_func!(func, data_map, __aeabi_fmul, (A, B) -> A)
}

pub(crate) fn f32_div(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    extern_func!(func, data_map, __aeabi_fdiv, (A, B) -> A)
}

pub(crate) fn f32_min(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    func.pop(register_list!(A, B));
    func.push(register_list!(r4, r5)); // We need to save these registers under the ABI
    func.movs(r5, A);
    func.movs(r4, B);
    let lt = get_data_label(func, data_map, __aeabi_fcmplt as u32);
    func.ldr(C, lt);
    func.blx(C);
    func.cmp(A, 0);
    let mut r#else = func.create_label();
    func.b_if(Condition::EQ, r#else);
    func.movs(r4, r5);
    func.label(&mut r#else);
    func.movs(A, r4);
    func.pop(register_list!(r4, r5)); // Restore the registers
    func.push(register_list!(A));
}

pub(crate) fn f32_max(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    func.pop(register_list!(A, B));
    func.push(register_list!(r4, r5)); // We need to save these registers under the ABI
    func.movs(r5, A);
    func.movs(r4, B);
    let gt = get_data_label(func, data_map, __aeabi_fcmpgt as u32);
    func.ldr(C, gt);
    func.blx(C);
    func.cmp(A, 0);
    let mut r#else = func.create_label();
    func.b_if(Condition::EQ, r#else);
    func.movs(r4, r5);
    func.label(&mut r#else);
    func.movs(A, r4);
    func.pop(register_list!(r4, r5)); // Restore the registers
    func.push(register_list!(A));
}
