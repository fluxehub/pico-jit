use crate::wasm_module::Result;
use crate::{aliases::*, wasm_module::WasmError};
use pico_emit::{instructions::*, register_list, registers::*, Emitter};
use ux2::u5;
use wasmparser_nostd::ValType;

pub enum LocalOffset {
    Immediate(u5),
    Register(u8),
}

fn get_local_offset(local_index: u32, locals: &[ValType]) -> Result<LocalOffset> {
    // TODO: Support up to 255 locals thru movs and reg offset
    if let Ok(offset) = u5::try_from(locals.len() as u8 - local_index as u8 - 1) {
        return Ok(LocalOffset::Immediate(offset));
    } else {
        let offset = u8::try_from(locals.len() as u8 - local_index as u8 - 1)
            .map_err(|_| WasmError::TooManyLocals(locals.len() as u32))?;

        Ok(LocalOffset::Register(offset))
    }
}

pub(crate) fn local_get(func: &mut Emitter, locals: &[ValType], index: u32) -> Result<()> {
    match get_local_offset(index, locals)? {
        LocalOffset::Immediate(offset) => func.ldr(A, ImmOffset(LOCALS, offset)),
        LocalOffset::Register(offset) => {
            func.movs(B, offset);
            func.lsl(B, ImmShift(B, u5::new(2)));
            func.ldr(A, RegOffset(LOCALS, B));
        }
    }

    func.push(register_list!(A));
    Ok(())
}

pub(crate) fn local_set(func: &mut Emitter, locals: &[ValType], index: u32) -> Result<()> {
    func.pop(register_list!(A));
    match get_local_offset(index, locals)? {
        LocalOffset::Immediate(offset) => func.str(A, ImmOffset(LOCALS, offset)),
        LocalOffset::Register(offset) => {
            func.movs(B, offset);
            func.lsl(B, ImmShift(B, u5::new(2)));
            func.str(A, RegOffset(LOCALS, B));
        }
    }
    Ok(())
}

pub(crate) fn local_tee(func: &mut Emitter, locals: &[ValType], index: u32) -> Result<()> {
    func.ldr(A, sp);
    match get_local_offset(index, locals)? {
        LocalOffset::Immediate(offset) => func.str(A, ImmOffset(LOCALS, offset)),
        LocalOffset::Register(offset) => {
            func.movs(B, offset);
            func.lsl(B, ImmShift(B, u5::new(2)));
            func.str(A, RegOffset(LOCALS, B));
        }
    }
    Ok(())
}
