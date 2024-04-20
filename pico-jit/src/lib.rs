#![no_std]
#![feature(error_in_core)]

extern crate alloc;

mod aliases;
pub mod compiler;
mod generation;
pub mod memory;
pub mod wasm_module;
