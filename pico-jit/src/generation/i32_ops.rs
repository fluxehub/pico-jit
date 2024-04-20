#![allow(clippy::fn_to_numeric_cast_with_truncation)]
use crate::{aliases::*, extern_func};
use alloc::collections::BTreeMap;
use pico_emit::{emitter::Label, instructions::*, register_list, registers::*, Emitter};
use ux2::{u3, u5};

use super::get_data_label;

extern "C" {
    fn __aeabi_idiv(n: i32, d: i32) -> i32;
    fn __aeabi_uidiv(n: u32, d: u32) -> u32;
    fn __aeabi_idivmod(n: i32, d: i32); // (r0: quotient, r1: remainder)
    fn __aeabi_uidivmod(n: u32, d: u32); // (r0: quotient, r1: remainder)
}

pub(crate) fn i32_const(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>, value: i32) {
    if let Ok(value) = u8::try_from(value) {
        func.movs(A, value);
    } else {
        let loc = get_data_label(func, data_map, value as u32);

        func.ldr(A, loc);
    }

    func.push(register_list!(A));
}

pub(crate) fn i32_eqz(func: &mut Emitter) {
    func.ldr(A, sp);
    func.rsb(B, A);
    func.adc(A, B);
    func.str(A, sp);
}

pub(crate) fn i32_eq(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.subs(A, B);
    func.rsb(B, A);
    func.adc(A, B);
    func.push(register_list!(A));
}

pub(crate) fn i32_ne(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.subs(A, B);
    func.subs(B, Sub2Imm(A, u3::new(1)));
    func.sbc(A, B);
    func.push(register_list!(A));
}

pub(crate) fn i32_lt_s(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.movs(C, 1);
    func.cmp(B, A);
    let mut lt_label = func.create_label();
    func.b_if(Condition::LT, lt_label);
    func.movs(C, 0);
    func.label(&mut lt_label);
    func.movs(A, C);
    func.push(register_list!(A));
}

pub(crate) fn i32_lt_u(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.cmp(B, A);
    func.sbc(A, A);
    func.rsb(A, A);
    func.push(register_list!(A));
}

pub(crate) fn i32_gt_s(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.movs(C, 1);
    func.cmp(A, B);
    let mut lt_label = func.create_label();
    func.b_if(Condition::LT, lt_label);
    func.movs(C, 0);
    func.label(&mut lt_label);
    func.movs(A, C);
    func.push(register_list!(A));
}

pub(crate) fn i32_gt_u(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.cmp(A, B);
    func.sbc(A, A);
    func.rsb(A, A);
    func.push(register_list!(A));
}

pub(crate) fn i32_le_s(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.movs(D, A);
    func.lsr(C, ImmShift(B, u5::new(31)));
    func.asr(A, ImmShift(A, u5::new(31)));
    func.cmp(D, B);
    func.adc(A, C);
    func.push(register_list!(A));
}

pub(crate) fn i32_le_u(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.movs(C, A);
    func.movs(A, 0);
    func.cmp(C, B);
    func.adc(A, A);
    func.push(register_list!(A));
}

pub(crate) fn i32_ge_s(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.movs(D, A);
    func.asr(C, ImmShift(B, u5::new(31)));
    func.lsr(A, ImmShift(A, u5::new(31)));
    func.cmp(B, D);
    func.adc(A, C);
    func.push(register_list!(A));
}

pub(crate) fn i32_ge_u(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.movs(C, A);
    func.movs(A, 0);
    func.cmp(B, C);
    func.adc(A, A);
    func.push(register_list!(A));
}

pub(crate) fn i32_add(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.adds(A, B);
    func.push(register_list!(A));
}

pub(crate) fn i32_sub(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.subs(A, Sub2(B, A));
    func.push(register_list!(A));
}

pub(crate) fn i32_mul(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.mul(A, B);
    func.push(register_list!(A));
}

pub(crate) fn i32_div_s(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    extern_func!(func, data_map, __aeabi_idiv, (A, B) -> A);
}

pub(crate) fn i32_div_u(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    extern_func!(func, data_map, __aeabi_uidiv, (A, B) -> A);
}

pub(crate) fn i32_rem_s(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    extern_func!(func, data_map, __aeabi_idivmod, (A, B) -> B);
}

pub(crate) fn i32_rem_u(func: &mut Emitter, data_map: &mut BTreeMap<u32, Label>) {
    extern_func!(func, data_map, __aeabi_uidivmod, (A, B) -> B);
}

pub(crate) fn i32_and(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.and(A, B);
    func.push(register_list!(A));
}

pub(crate) fn i32_or(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.or(A, B);
    func.push(register_list!(A));
}

pub(crate) fn i32_xor(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.eor(A, B);
    func.push(register_list!(A));
}

pub(crate) fn i32_shl(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.lsl(B, A);
    func.push(register_list!(B));
}

pub(crate) fn i32_shr_s(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.asr(B, A);
    func.push(register_list!(B));
}

pub(crate) fn i32_shr_u(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.lsr(B, A);
    func.push(register_list!(B));
}

pub(crate) fn i32_rotl(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    // Thank you Cambridge for including a rotate right but not a rotate left :)
    // TODO: Can I just rotate right by 32 - A?
    func.movs(C, 32);
    func.subs(C, A);
    func.movs(D, B);
    func.asr(B, C);
    func.movs(C, 1);
    func.rsb(C, C);
    func.lsl(C, A);
    func.lsl(D, A);
    func.bic(B, C);
    func.or(B, D);
    func.push(register_list!(B));
}

pub(crate) fn i32_rotr(func: &mut Emitter) {
    func.pop(register_list!(A, B));
    func.ror(B, A);
    func.push(register_list!(B));
}
