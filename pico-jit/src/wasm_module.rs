use crate::compiler::{compile_wasm, WasmContext};
use crate::memory::WasmMemory;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
use core::fmt::{Display, Formatter};
use pico_emit::{as_fn, JitFn};
use wasmparser_nostd::Type::Func;
use wasmparser_nostd::{DataKind, ExternalKind, FuncType, Parser, Payload, TypeRef};

type ExternalFn<'a> = Box<dyn FnMut(&mut WasmMemory) + 'a>;

pub enum WasmFunction<'a> {
    Jit {
        name: Option<String>,
        ty: FuncType,
        wasm_offset: usize,
        function: Option<JitFn>,
    },
    External {
        module: String,
        name: String,
        ty: FuncType,
        function: Option<ExternalFn<'a>>,
    },
}

// pub struct CompilationReporter<'a> {
//     pub current_time: Box<dyn Fn() -> Instant + 'a>,
//     pub report_time: Box<dyn FnMut(Instant, Instant) + 'a>,
// }

pub struct WasmModule<'a> {
    pub memory: WasmMemory,
    pub functions: Vec<WasmFunction<'a>>,
    // reporter: Option<CompilationReporter<'a>>,
    wasm_data: &'a [u8],
}

#[derive(Debug)]
pub enum WasmError {
    InvalidVersion(u16),
    UnsupportedImport(TypeRef),
    UnsupportedSection(String),
    TooManyMemoryDefinitions,
    MemorySizeTooLarge(u32),
    FunctionNotFound(String),
    ParseError(wasmparser_nostd::BinaryReaderError),
    TooManyLocals(u32),
    UnsupportedOp(String),
}

impl From<wasmparser_nostd::BinaryReaderError> for WasmError {
    fn from(e: wasmparser_nostd::BinaryReaderError) -> Self {
        WasmError::ParseError(e)
    }
}

impl Display for WasmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            WasmError::InvalidVersion(version) => write!(f, "Invalid version: {}", version),
            WasmError::UnsupportedImport(ty) => write!(f, "Unsupported import type: {:?}", ty),
            WasmError::UnsupportedSection(section) => {
                write!(f, "Unsupported section: {}", section)
            }
            WasmError::TooManyMemoryDefinitions => {
                write!(f, "pico-jit only supports one memory definition per module")
            }
            WasmError::MemorySizeTooLarge(size) => write!(f, "Memory size too large: {}", size),
            WasmError::UnsupportedOp(op) => write!(f, "Unsupported op: {}", op),
            WasmError::FunctionNotFound(name) => write!(f, "Function not found: {}", name),
            WasmError::ParseError(e) => write!(f, "Parse error: {}", e),
            WasmError::TooManyLocals(count) => write!(f, "Too many locals: {}", count),
        }
    }
}

impl core::error::Error for WasmError {}

pub type Result<T> = core::result::Result<T, WasmError>;

impl<'a> WasmModule<'a> {
    pub fn from_wasm(wasm_data: &'a [u8]) -> Result<Self> {
        let parser = Parser::new(0);
        let mut memory = Vec::with_capacity(1);
        let mut globals = Vec::with_capacity(1);
        let mut functions = Vec::new();
        let mut types = Vec::new();
        let mut body_index = 0;
        for section in parser.parse_all(wasm_data) {
            match section? {
                Payload::Version { num, .. } => {
                    if num != 1 {
                        return Err(WasmError::InvalidVersion(num));
                    }
                }
                Payload::TypeSection(reader) => {
                    for ty in reader {
                        let Func(func_type) = ty?;
                        types.push(func_type);
                    }
                }
                Payload::ImportSection(reader) => {
                    for import in reader {
                        let import = import?;
                        match import.ty {
                            TypeRef::Func(ty) => {
                                functions.push(WasmFunction::External {
                                    module: import.module.to_string(),
                                    name: import.name.to_string(),
                                    ty: types[ty as usize].clone(),
                                    function: None,
                                });
                                body_index += 1;
                            }
                            _ => return Err(WasmError::UnsupportedImport(import.ty)),
                        }
                    }
                }
                Payload::FunctionSection(reader) => {
                    for index in reader {
                        functions.push(WasmFunction::Jit {
                            name: None,
                            ty: types[index? as usize].clone(),
                            wasm_offset: 0,
                            function: None,
                        });
                    }
                }
                Payload::TableSection(_) => {
                    //unimplemented!("Table section");
                }
                Payload::MemorySection(reader) => {
                    let mut reader_iter = reader.into_iter();
                    if reader_iter.len() != 1 {
                        return Err(WasmError::TooManyMemoryDefinitions);
                    }

                    let mem = reader_iter.next().unwrap()?;
                    if mem.initial > 2 {
                        return Err(WasmError::MemorySizeTooLarge(mem.initial as u32));
                    }

                    memory = vec![0; mem.initial as usize * 65536]; // 65536 bytes per page
                }
                Payload::GlobalSection(reader) => {
                    for global in reader {
                        // TODO: Care about the type of the global (when 64-bit stuff is added)
                        let op_reader = global?.init_expr.get_operators_reader();
                        match op_reader.into_iter().next().unwrap()? {
                            wasmparser_nostd::Operator::I32Const { value } => {
                                globals.push(value as u32);
                            }
                            wasmparser_nostd::Operator::F32Const { value } => {
                                globals.push(value.bits());
                            }
                            op => {
                                return Err(WasmError::UnsupportedOp(format!(
                                    "{:?} in global section",
                                    op
                                )))
                            }
                        }
                    }
                }
                Payload::ExportSection(reader) => {
                    for export in reader {
                        let export = export?;
                        if export.kind != ExternalKind::Func {
                            continue;
                        }

                        let Some(WasmFunction::Jit { ref mut name, .. }) =
                            functions.get_mut(export.index as usize)
                        else {
                            unreachable!();
                        };

                        *name = Some(export.name.to_string());
                    }
                }
                Payload::DataSection(reader) => {
                    for data in reader {
                        let data = data?;

                        let DataKind::Active { offset_expr, .. } = data.kind else {
                            todo!("DataKind::Passive not supported yet")
                        };

                        match offset_expr
                            .get_operators_reader()
                            .into_iter()
                            .next()
                            .unwrap()?
                        {
                            wasmparser_nostd::Operator::I32Const { value } => {
                                let offset = value as usize;
                                let data = data.data;
                                memory[offset..offset + data.len()].copy_from_slice(data);
                            }
                            op => {
                                return Err(WasmError::UnsupportedOp(format!(
                                    "{:?} as data offset",
                                    op
                                )))
                            }
                        };
                    }
                }
                Payload::CodeSectionEntry(body) => {
                    let reader = body.get_binary_reader();
                    let Some(WasmFunction::Jit {
                        ref mut wasm_offset,
                        ..
                    }) = functions.get_mut(body_index)
                    else {
                        unreachable!();
                    };

                    *wasm_offset = reader.original_position();
                    body_index += 1;
                }
                Payload::StartSection { .. }
                | Payload::End(_)
                | Payload::CustomSection(_)
                | Payload::DataCountSection { .. }
                | Payload::CodeSectionStart { .. } => {}
                section => {
                    return Err(WasmError::UnsupportedSection(format!("{:?}", section)));
                }
            }
        }

        Ok(WasmModule {
            memory: WasmMemory::new(
                globals.into_boxed_slice(),
                memory.into_boxed_slice(),
                vec![0].into_boxed_slice(),
                1024,
            ),
            functions,
            wasm_data,
        })
    }

    fn compile_and_execute(&mut self, function_index: u32, sp: *const u32) -> *const u32 {
        self.memory.set_stack_ptr(sp);

        let context = WasmContext {
            module_ptr: self as *const WasmModule,
            call_func: Self::compile_and_execute,
            memory_ptr: self.memory.get_memory_ptr(),
            global_ptr: self.memory.get_globals_ptr(),
            memory_size: self.memory.get_memory_size(),
        };

        let Some(func) = self.functions.get_mut(function_index as usize) else {
            unreachable!("Function not found at index {}", function_index);
        };

        match func {
            WasmFunction::External { function, .. } => {
                let function = function.as_mut().unwrap();
                function(&mut self.memory);
                self.memory.get_stack_ptr()
            }
            WasmFunction::Jit {
                function: jit_fn,
                wasm_offset,
                ty,
                ..
            } => {
                if jit_fn.is_none() {
                    // let start_time = self.reporter.as_ref().map(|r| (*r.current_time)());
                    let parser = Parser::new(0);

                    // TODO: This is a horrible, no good hack
                    for payload in parser.parse_all(self.wasm_data) {
                        if let Payload::CodeSectionEntry(body) = payload.unwrap() {
                            if body.get_binary_reader().original_position() != *wasm_offset {
                                continue;
                            }

                            let compiled = compile_wasm(&context, ty, body).unwrap();

                            jit_fn.replace(compiled);
                        }
                    }
                    // let end_time = self.reporter.as_ref().map(|r| (*r.current_time)());
                    // if let Some(ref mut reporter) = self.reporter {
                    //     (*reporter.report_time)(start_time.unwrap(), end_time.unwrap());
                    // }
                }

                let function = jit_fn.as_ref().unwrap();
                let function = as_fn!(function, (*const u32) -> *const u32);

                function(sp)
            }
        }
    }

    fn internal_call(&mut self, name: &str, args: &[u32]) -> Result<()> {
        // Find the function index by name
        let index = self
            .functions
            .iter()
            .enumerate()
            .find_map(|(i, f)| match f {
                WasmFunction::Jit { name: Some(n), .. } if n == name => Some(i as u32),
                _ => None,
            })
            .ok_or_else(|| WasmError::FunctionNotFound(name.to_string()))?;

        // Push args on to stack
        for arg in args {
            self.memory.push_stack(*arg);
        }

        // Call the function
        let sp = self.compile_and_execute(index, self.memory.get_stack_ptr());
        self.memory.set_stack_ptr(sp);
        Ok(())
    }

    pub fn add_external_function(&mut self, module: &str, name: &str, function: ExternalFn<'a>) {
        let index = self
            .functions
            .iter()
            .enumerate()
            .find_map(|(i, f)| match f {
                WasmFunction::External {
                    module: m, name: n, ..
                } if m == module && n == name => Some(i),
                _ => None,
            })
            .unwrap();

        if let WasmFunction::External { function: f, .. } = &mut self.functions[index] {
            f.replace(function);
        }
    }
}

pub trait Call<RetType> {
    fn call(&mut self, name: &str, args: &[u32]) -> Result<RetType>;
}

impl Call<()> for WasmModule<'_> {
    fn call(&mut self, name: &str, args: &[u32]) -> Result<()> {
        self.internal_call(name, args)
    }
}

impl Call<i32> for WasmModule<'_> {
    fn call(&mut self, name: &str, args: &[u32]) -> Result<i32> {
        self.internal_call(name, args)?;

        Ok(self.memory.pop_stack() as i32)
    }
}

impl Call<f32> for WasmModule<'_> {
    fn call(&mut self, name: &str, args: &[u32]) -> Result<f32> {
        self.internal_call(name, args)?;

        Ok(f32::from_bits(self.memory.pop_stack()))
    }
}
