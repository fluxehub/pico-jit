use alloc::collections::BTreeMap;
use pico_emit::{emitter::Label, Emitter};

pub mod bulk_memory;
pub mod control_flow;
pub mod conversion;
pub mod f32_ops;
pub mod globals;
pub mod i32_ops;
pub mod locals;
pub mod memory;

pub(crate) fn get_data_label(
    func: &mut Emitter,
    data_map: &mut BTreeMap<u32, Label>,
    value: u32,
) -> Label {
    *data_map.entry(value).or_insert_with(|| func.data(value))
}

// extern_func!(emitter, data_map, extern_func, (ARGS) -> Return reg)
#[macro_export]
macro_rules! extern_func {
    ($func:ident, $data_map:ident, $extern_func:ident, (A, B) -> $reg:ident) => {{
        use super::get_data_label;
        use crate::aliases::*;
        use pico_emit::{instructions::*, register_list};

        let extern_label = get_data_label($func, $data_map, $extern_func as u32);

        $func.pop(register_list!(B, C));
        $func.movs(A, C);
        $func.ldr(C, extern_label);
        $func.blx(C);
        $func.push(register_list!($reg));
    }};

    ($func:ident, $data_map:ident, $extern_func:ident, (A) -> $reg:ident) => {{
        use super::get_data_label;
        use crate::aliases::*;
        use pico_emit::{instructions::*, register_list};

        let extern_label = get_data_label($func, $data_map, $extern_func as u32);

        $func.pop(register_list!(A));
        $func.ldr(C, extern_label);
        $func.blx(C);
        $func.push(register_list!($reg));
    }};
}
