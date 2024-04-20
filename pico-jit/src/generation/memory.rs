use alloc::collections::BTreeMap;
use pico_emit::{
    emitter::Label, instructions::*, register_list, registers::types::LowRegister, Emitter,
};
use ux2::u5;
use wasmparser_nostd::MemArg;

use crate::aliases::*;

use super::get_data_label;

fn load32_unaligned(memory: *const u8, src: usize) -> u32 {
    // if src > 65536 * 2 {
    //     panic!(
    //         "Attempted to read offset {} which is larger than 2 WASM pages",
    //         src
    //     );
    // }

    if src % 4 == 0 {
        // This is a performance optimization to avoid overhead of unaligned load
        unsafe {
            return *(memory.add(src) as *const u32); // SAFETY: src is aligned
        }
    }

    unsafe {
        u32::from_le_bytes([
            *memory.add(src),
            *memory.add(src + 1),
            *memory.add(src + 2),
            *memory.add(src + 3),
        ])
    }
}

fn store32_unaligned(memory: *mut u8, dest: usize, value: u32) {
    // if dest > 65536 * 2 {
    //     panic!(
    //         "Attempted to write to offset {} which is larger than 2 WASM pages",
    //         dest
    //     );
    // }

    if dest % 4 == 0 {
        unsafe {
            *(memory.add(dest) as *mut u32) = value;
        }
        return;
    }

    let bytes = value.to_le_bytes();

    unsafe {
        *memory.add(dest) = bytes[0];
        *memory.add(dest + 1) = bytes[1];
        *memory.add(dest + 2) = bytes[2];
        *memory.add(dest + 3) = bytes[3];
    }
}

fn load_memory_offset(
    func: &mut Emitter,
    memarg: &MemArg,
    data_map: &mut BTreeMap<u32, Label>,
    reg: LowRegister,
) {
    if memarg.offset == 0 {
        return;
    }

    match u8::try_from(memarg.offset) {
        Ok(offset) => {
            func.adds(reg, offset);
        }
        Err(_) => {
            let offset = get_data_label(func, data_map, memarg.offset as u32);
            func.ldr(C, offset);
            func.add(reg, C);
        }
    }
}

pub fn x32_load(func: &mut Emitter, memarg: &MemArg, data_map: &mut BTreeMap<u32, Label>) {
    func.pop(register_list!(B));
    load_memory_offset(func, memarg, data_map, B);
    func.mov(A, MEMORY);
    let load_func = get_data_label(func, data_map, load32_unaligned as u32);
    func.ldr(C, load_func);
    func.blx(C);
    func.push(register_list!(A));
}

pub fn x32_store(func: &mut Emitter, memarg: &MemArg, data_map: &mut BTreeMap<u32, Label>) {
    func.pop(register_list!(A, B));
    load_memory_offset(func, memarg, data_map, B);
    func.mov(C, A);
    func.mov(A, MEMORY);
    let store_func = get_data_label(func, data_map, store32_unaligned as u32);
    func.ldr(D, store_func);
    func.blx(D);
}

pub fn i32_load8_u(func: &mut Emitter, memarg: &MemArg, data_map: &mut BTreeMap<u32, Label>) {
    func.pop(register_list!(A));
    load_memory_offset(func, memarg, data_map, B);
    func.ldrb(A, RegOffset(MEMORY, B));
    func.push(register_list!(A));
}

pub fn i32_load16_u(func: &mut Emitter, memarg: &MemArg, data_map: &mut BTreeMap<u32, Label>) {
    func.pop(register_list!(A));
    load_memory_offset(func, memarg, data_map, A);
    func.movs(C, 1);
    func.tst(C, A);
    let mut r#else = func.create_label();
    let mut end = func.create_label();
    func.bic(A, C);
    func.ldrh(A, RegOffset(MEMORY, A));
    func.b(end);
    func.label(&mut r#else);
    func.adds(C, Add2(MEMORY, A));
    func.ldrb(C, ImmOffset(C, u5::new(1)));
    func.ldrb(A, RegOffset(MEMORY, A));
    func.lsl(C, ImmShift(C, u5::new(8)));
    func.or(A, C);
    func.label(&mut end);
    func.push(register_list!(A));
}

pub fn i32_store8(func: &mut Emitter, memarg: &MemArg, data_map: &mut BTreeMap<u32, Label>) {
    func.pop(register_list!(A, B));
    load_memory_offset(func, memarg, data_map, B);
    func.strb(A, RegOffset(MEMORY, B));
}

pub fn i32_store16(func: &mut Emitter, memarg: &MemArg, data_map: &mut BTreeMap<u32, Label>) {
    func.pop(register_list!(A, B));
    load_memory_offset(func, memarg, data_map, B);
    func.movs(C, 1);
    func.tst(C, B);
    let mut r#else = func.create_label();
    let mut end = func.create_label();
    func.b_if(Condition::NE, r#else);
    func.bic(B, C);
    func.strh(A, RegOffset(MEMORY, B));
    func.b(end);
    func.label(&mut r#else);
    func.strb(A, RegOffset(MEMORY, B));
    func.adds(B, 1);
    func.lsr(A, ImmShift(A, u5::new(8)));
    func.strb(A, RegOffset(MEMORY, B));
    func.label(&mut end);
}
