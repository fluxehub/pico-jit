#![no_std]
#![allow(clippy::unusual_byte_groupings)]
extern crate alloc;

pub mod buffer;
pub mod emitter;
pub mod instructions;
pub mod registers;

use alloc::boxed::Box;

// Re-export types
pub use emitter::Emitter;
pub use ux2;

#[derive(Debug)]
pub struct JitFn {
    /// Do not use this field directly, instead use the `as_fn!` macro to convert JitFn to a function pointer
    pub data: Box<[u16]>,
}

#[macro_export]
macro_rules! as_fn {
    ($func:expr, ($($args:ty), *) -> $ret:ty) => {
        unsafe {
            let ptr = ($func.data.as_ptr() as *const u8).add(1);
            let func: extern "C" fn($($args), *) -> $ret = core::mem::transmute(ptr);
            func
        }
    }
}
