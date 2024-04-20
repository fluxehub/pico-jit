use alloc::boxed::Box;
use alloc::vec;

#[derive(Debug)]
pub struct WasmMemory {
    globals: Box<[u32]>,
    memory: Box<[u8]>,
    table: Box<[u32]>,
    stack: Box<[u32]>,
    stack_index: u32,
    memory_pages: u32,
}

impl WasmMemory {
    pub fn new(globals: Box<[u32]>, memory: Box<[u8]>, table: Box<[u32]>, stack_size: u32) -> Self {
        WasmMemory {
            globals,
            memory_pages: memory.len() as u32 / 65536,
            memory,
            table,
            stack: vec![0; stack_size as usize].into_boxed_slice(),
            stack_index: stack_size, // Full descending stack
        }
    }

    pub fn get_globals_ptr(&mut self) -> *mut u32 {
        self.globals.as_mut_ptr()
    }

    pub fn get_memory_ptr(&mut self) -> *mut u8 {
        self.memory.as_mut_ptr()
    }

    pub fn get_table_ptr(&self) -> *const u32 {
        self.table.as_ptr()
    }

    pub fn get_stack_ptr(&self) -> *const u32 {
        unsafe { self.stack.as_ptr().add(self.stack_index as usize) }
    }

    pub fn set_stack_ptr(&mut self, ptr: *const u32) {
        self.stack_index = (ptr as u32 - self.stack.as_ptr() as u32) / 4;
        assert!(
            self.stack_index <= self.stack.len() as u32,
            "Tried to set stack index to {} but stack size is {}",
            self.stack_index,
            self.stack.len()
        );
    }

    pub fn push_stack(&mut self, value: u32) {
        self.stack_index -= 1;
        self.stack[self.stack_index as usize] = value;
    }

    pub fn pop_stack(&mut self) -> u32 {
        let value = self.stack[self.stack_index as usize];
        self.stack_index += 1;
        value
    }

    pub fn get_global(&self, index: u32) -> u32 {
        self.globals[index as usize]
    }

    pub fn write_memory(&mut self, index: u32, value: u8) {
        self.memory[index as usize] = value;
    }

    pub fn read_memory(&self, index: u32) -> u8 {
        self.memory[index as usize]
    }

    pub fn read_memory32(&self, index: u32) -> u32 {
        let index = index as usize;
        (self.memory[index] as u32)
            | ((self.memory[index + 1] as u32) << 8)
            | ((self.memory[index + 2] as u32) << 16)
            | ((self.memory[index + 3] as u32) << 24)
    }

    pub fn get_memory_size(&self) -> u32 {
        self.memory_pages
    }
}
