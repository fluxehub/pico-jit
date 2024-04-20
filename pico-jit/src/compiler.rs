use crate::generation::{
    bulk_memory::*, control_flow::*, conversion::*, f32_ops::*, get_data_label, globals::*,
    i32_ops::*, locals::*, memory::*,
};
use crate::wasm_module::{Result, WasmError, WasmModule};
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;
use core::iter::repeat;
use pico_emit::emitter::Label;
use pico_emit::instructions::*;
use pico_emit::registers::*;
use pico_emit::{register_list, Emitter, JitFn};
use ux2::{u5, u7};
use wasmparser_nostd::{FuncType, FunctionBody, Operator, ValType, WasmFuncType};

use crate::aliases::*;

pub(crate) enum Scope {
    Block(Label),
    Loop(Label),
    If { else_label: Label, end_label: Label },
}

#[derive(Debug)]
pub(crate) struct WasmContext<'a> {
    pub(crate) module_ptr: *const WasmModule<'a>,
    pub(crate) call_func: fn(&mut WasmModule<'a>, u32, *const u32) -> *const u32,
    pub(crate) memory_ptr: *mut u8,
    pub(crate) global_ptr: *mut u32,
    pub(crate) memory_size: u32,
}

fn read_local_types(ty: &FuncType, body: &FunctionBody) -> Result<Box<[ValType]>> {
    let mut params = ty.params().to_vec();
    let local_reader = body.get_locals_reader()?;
    for local in local_reader {
        let (count, ty) = local?;
        if ty != ValType::I32 && ty != ValType::F32 {
            unimplemented!("Unsupported local type: {:?}", ty)
        }

        params.extend(repeat(ty).take(count as usize));
    }

    Ok(params.into_boxed_slice())
}

// pub(crate) fn check_stack_underflow(stack: *const u32, locals: *const u32) {
//     if stack > locals {
//         panic!(
//             "Stack underflow: stack pointer is at {:p} but locals pointer is at {:p}",
//             stack, locals
//         );
//     }
// }

fn wasm_panic(code: u32) {
    match code {
        0 => panic!("Unreachable executed"),
        1 => panic!("memory.grow called with non-zero size (grow is not supported)"),
        _ => panic!("Unknown panic code: {}", code),
    }
}

pub(crate) fn compile_wasm(
    context: &WasmContext,
    ty: &FuncType,
    body: FunctionBody,
) -> Result<JitFn> {
    let locals = read_local_types(ty, &body).unwrap();
    let param_count = ty.params().len();

    let mut func = Emitter::new();
    func.mov(r1, ARCH_SP); // We need to save the high registers
    func.mov(r2, MODULE);
    func.push(register_list!(lr, r1, r2, MEMORY, GLOBALS, LOCALS)); // Save the link register and locals register

    // We need to zero non-param locals
    // TODO: Find most efficient way to do this
    func.movs(r1, 0); // Zero register
    for _ in param_count..locals.len() {
        func.subs(r0, 4);
        func.str(r1, r0);
    }

    let module_ptr = func.data(context.module_ptr as u32);
    let memory_ptr = func.data(context.memory_ptr as u32);
    let globals_ptr = func.data(context.global_ptr as u32);

    #[allow(clippy::fn_to_numeric_cast_with_truncation)] // ARMv6-M is a 32-bit architecture
    let call_func = func.data(context.call_func as u32);

    func.ldr(r1, module_ptr); // Load module ptr
    func.mov(MODULE, r1); // Move module ptr into high register
    func.ldr(MEMORY, memory_ptr); // Load memory ptr
    func.ldr(GLOBALS, globals_ptr); // Load global ptr
    func.mov(ARCH_SP, sp); // Save the original stack pointer
    func.mov(sp, r0); // Move the WASM stack pointer into sp
    func.mov(LOCALS, r0); // Move the start of the locals into r0

    let mut scope_stack: Vec<Scope> = vec![];
    let mut emitted_data: BTreeMap<u32, Label> = BTreeMap::new();

    let mut func_end = func.create_label();
    scope_stack.push(Scope::Block(func_end)); // Function end counts as a block that we can break out of

    // func.bkpt();

    for op in body.get_operators_reader()? {
        // func.mov(A, sp);
        // func.mov(B, LOCALS);
        // let underflow_check =
        //     get_data_label(&mut func, &mut emitted_data, check_stack_underflow as u32);
        // func.ldr(C, underflow_check);
        // func.blx(C);

        if func.section_instruction_count() > 256 {
            let mut next_code_section = func.create_label();
            func.branch(next_code_section);
            func.emit_data_section();
            func.label(&mut next_code_section);
        }

        match op? {
            // Control flow operators
            Operator::Unreachable => {
                // Doing this here for simplicity
                let panic = get_data_label(&mut func, &mut emitted_data, wasm_panic as u32);
                func.movs(A, 0);
                func.ldr(B, panic);
                func.blx(B);
            }
            Operator::Block { .. } => block(&mut func, &mut scope_stack),
            Operator::Loop { .. } => r#loop(&mut func, &mut scope_stack),
            Operator::Nop => (), // Do nothing
            Operator::If { .. } => r#if(&mut func, &mut scope_stack),
            Operator::Else => r#else(&mut func, &mut scope_stack),
            Operator::End => end(&mut func, &mut scope_stack),
            Operator::Br { relative_depth } => br(&mut func, &scope_stack, relative_depth),
            Operator::BrIf { relative_depth } => br_if(&mut func, &scope_stack, relative_depth),
            Operator::BrTable { targets } => br_table(
                &mut func,
                &scope_stack,
                &targets
                    .targets()
                    .collect::<core::result::Result<Vec<_>, _>>()?,
                targets.default(),
            ),
            Operator::Return => r#return(&mut func, &scope_stack),
            Operator::Call { function_index } => call(&mut func, &call_func, function_index),
            Operator::Drop => func.add(sp, u7::new(4)), // So simple that splitting it out into a function is overkill
            Operator::Select => select(&mut func),

            // Local variable operators
            Operator::LocalGet { local_index } => local_get(&mut func, &locals, local_index)?,
            Operator::LocalSet { local_index } => local_set(&mut func, &locals, local_index)?,
            Operator::LocalTee { local_index } => local_tee(&mut func, &locals, local_index)?,

            // Global variable operators
            Operator::GlobalGet { global_index } => global_get(&mut func, global_index),
            Operator::GlobalSet { global_index } => global_set(&mut func, global_index),

            // Memory operators
            Operator::I32Load { memarg } => x32_load(&mut func, &memarg, &mut emitted_data),
            Operator::F32Load { memarg } => x32_load(&mut func, &memarg, &mut emitted_data),
            // Operator::I32Load8S { memarg } => todo!("I32Load8S"),
            Operator::I32Load8U { memarg } => i32_load8_u(&mut func, &memarg, &mut emitted_data),
            // Operator::I32Load16S { memarg } => todo!("I32Load16S"),
            Operator::I32Load16U { memarg } => i32_load16_u(&mut func, &memarg, &mut emitted_data),
            Operator::I32Store { memarg } => x32_store(&mut func, &memarg, &mut emitted_data),
            Operator::F32Store { memarg } => x32_store(&mut func, &memarg, &mut emitted_data),
            Operator::I32Store8 { memarg } => i32_store8(&mut func, &memarg, &mut emitted_data),
            Operator::I32Store16 { memarg } => i32_store16(&mut func, &memarg, &mut emitted_data),

            // I32 operators
            Operator::I32Const { value } => i32_const(&mut func, &mut emitted_data, value),
            Operator::I32Eqz => i32_eqz(&mut func),
            Operator::I32Eq => i32_eq(&mut func),
            Operator::I32Ne => i32_ne(&mut func),
            Operator::I32LtS => i32_lt_s(&mut func),
            Operator::I32LtU => i32_lt_u(&mut func),
            Operator::I32GtS => i32_gt_s(&mut func),
            Operator::I32GtU => i32_gt_u(&mut func),
            Operator::I32LeS => i32_le_s(&mut func),
            Operator::I32LeU => i32_le_u(&mut func),
            Operator::I32GeS => i32_ge_s(&mut func),
            Operator::I32GeU => i32_ge_u(&mut func),
            Operator::I32Clz => todo!("I32Clz"),
            Operator::I32Ctz => todo!("I32Ctz"),
            Operator::I32Popcnt => todo!("I32Popcnt"),
            Operator::I32Add => i32_add(&mut func),
            Operator::I32Sub => i32_sub(&mut func),
            Operator::I32Mul => i32_mul(&mut func),
            Operator::I32DivS => i32_div_s(&mut func, &mut emitted_data),
            Operator::I32DivU => i32_div_u(&mut func, &mut emitted_data),
            Operator::I32RemS => i32_rem_s(&mut func, &mut emitted_data),
            Operator::I32RemU => i32_rem_u(&mut func, &mut emitted_data),
            Operator::I32And => i32_and(&mut func),
            Operator::I32Or => i32_or(&mut func),
            Operator::I32Xor => i32_xor(&mut func),
            Operator::I32Shl => i32_shl(&mut func),
            Operator::I32ShrS => i32_shr_s(&mut func),
            Operator::I32ShrU => i32_shr_u(&mut func),
            Operator::I32Rotl => i32_rotl(&mut func),
            Operator::I32Rotr => i32_rotr(&mut func),

            // F32 operators
            Operator::F32Const { value } => f32_const(&mut func, &mut emitted_data, value),
            Operator::F32Eq => f32_eq(&mut func, &mut emitted_data),
            Operator::F32Ne => f32_ne(&mut func, &mut emitted_data),
            Operator::F32Lt => f32_lt(&mut func, &mut emitted_data),
            Operator::F32Gt => f32_gt(&mut func, &mut emitted_data),
            Operator::F32Le => f32_le(&mut func, &mut emitted_data),
            Operator::F32Ge => f32_ge(&mut func, &mut emitted_data),
            Operator::F32Abs => todo!("F32Abs"),
            Operator::F32Neg => todo!("F32Neg"),
            Operator::F32Ceil => todo!("F32Ceil"),
            Operator::F32Floor => todo!("F32Floor"),
            Operator::F32Trunc => todo!("F32Trunc"),
            Operator::F32Nearest => todo!("F32Nearest"),
            Operator::F32Sqrt => f32_sqrt(&mut func, &mut emitted_data),
            Operator::F32Add => f32_add(&mut func, &mut emitted_data),
            Operator::F32Sub => f32_sub(&mut func, &mut emitted_data),
            Operator::F32Mul => f32_mul(&mut func, &mut emitted_data),
            Operator::F32Div => f32_div(&mut func, &mut emitted_data),
            Operator::F32Min => f32_min(&mut func, &mut emitted_data),
            Operator::F32Max => f32_max(&mut func, &mut emitted_data),
            Operator::F32Copysign => todo!("F32Copysign"),

            // Conversion operators
            Operator::I32WrapI64 => todo!("I32WrapI64"),
            Operator::I32TruncF32S => i32_trunc_f32_s(&mut func, &mut emitted_data),
            Operator::I32TruncF32U => todo!("I32TruncF32U"),
            Operator::I32TruncF64S => todo!("I32TruncF64S"),
            Operator::I32TruncF64U => todo!("I32TruncF64U"),
            Operator::I32Extend8S => i32_extend8_s(&mut func),
            Operator::I32Extend16S => i32_extend16_s(&mut func),
            Operator::F32ConvertI32S => f32_convert_i32_s(&mut func, &mut emitted_data),
            Operator::F32ConvertI32U => f32_convert_i32_u(&mut func, &mut emitted_data),
            Operator::I32ReinterpretF32 => todo!("I32ReinterpretF32"),

            // Bulk memory operators
            Operator::MemoryCopy { .. } => memory_copy(&mut func, &mut emitted_data),
            Operator::MemoryGrow { .. } => {
                // Grow is not supported, but emit it anyway so that we can use programs containing it
                // as long as they don't actually call it with a non-zero argument.
                func.pop(register_list!(A));
                func.cmp(A, 0);
                let mut zero = func.create_label();
                func.b_if(Condition::EQ, zero);
                let panic = get_data_label(&mut func, &mut emitted_data, wasm_panic as u32);
                func.movs(A, 1);
                func.ldr(B, panic);
                func.blx(B);
                func.label(&mut zero);
                func.movs(A, 1);
                func.push(register_list!(A));
            }
            Operator::MemorySize { .. } => {
                func.movs(A, context.memory_size as u8);
                func.push(register_list!(A));
            }
            Operator::MemoryFill { .. } => memory_fill(&mut func, &mut emitted_data),
            op => return Err(WasmError::UnsupportedOp(format!("{:?}", op))),
        }
    }

    func.label(&mut func_end); // Label for the end of the function
    func.ldr(r1, sp); // Load the top value of the stack into r1
    match ty.len_outputs() {
        0 => {
            func.adds(LOCALS, locals.len() as u8 * 4); // Move the locals pointer back to the top of the stack
        }
        1 => {
            if locals.len() == 0 {
                func.subs(LOCALS, 4); // Create space for the return value
            } else {
                if let Ok(offset) = u8::try_from((locals.len() - 1) * 4) {
                    func.adds(LOCALS, offset)
                } else {
                    func.movs(r0, (locals.len() - 1) as u8);
                    func.lsl(r0, ImmShift(r0, u5::new(2)));
                    func.adds(LOCALS, r0);
                }
                // func.adds(LOCALS, (locals.len() - 1) as u8 * 4); // Move the locals pointer to the first local
            }
            func.str(r1, LOCALS); // Overwrite the first local with the top value of the stack to return
        }
        len => {
            panic!("Unsupported number of return values: {}", len) // Since we ensure WASM v1, this should (theoretically) never happen
        }
    }

    func.mov(sp, ARCH_SP); // Restore sp
    func.mov(r0, LOCALS); // Move the locals pointer into r0 for return

    func.pop(register_list!(r1, r2, MEMORY, GLOBALS, LOCALS)); // Restore the link register and locals register

    func.mov(ARCH_SP, r1); // CORRECTLY restores the high registers
    func.mov(MODULE, r2);
    func.pop(register_list!(pc)); // Return
    Ok(func.build())
}
