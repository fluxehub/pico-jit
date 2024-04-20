use crate::registers::traits::Register;
use crate::registers::types::{LowRegister, RegisterList};
use ux2::{u11, u3, u5, u7};

pub mod add;
pub mod asr;
pub mod cmp;
pub mod ldr;
pub mod lsl;
pub mod lsr;
pub mod misc;
pub mod mov;
pub mod pop;
pub mod push;
pub mod str;
pub mod sub;

pub use add::*;
pub use asr::*;
pub use cmp::*;
pub use ldr::*;
pub use lsl::*;
pub use lsr::*;
pub use misc::*;
pub use mov::*;
pub use pop::*;
pub use push::*;
pub use str::*;
pub use sub::*;

pub struct ImmShift(pub LowRegister, pub u5);
pub struct ImmOffset(pub LowRegister, pub u5);
pub struct RegOffset(pub LowRegister, pub LowRegister);
pub struct SPWithOffset(pub u8);

pub(crate) trait ToInstEncoding {
    fn to_instruction_encoded(self) -> u16;
}

impl<R: Register> ToInstEncoding for R {
    fn to_instruction_encoded(self) -> u16 {
        self.to_reg_number()
    }
}

impl<const LR: bool, const PC: bool> ToInstEncoding for RegisterList<LR, PC> {
    fn to_instruction_encoded(self) -> u16 {
        self.0
    }
}

macro_rules! impl_num_to_instruction_encoded {
    ($($t:ty),*) => {
        $(
            impl ToInstEncoding for $t {
                fn to_instruction_encoded(self) -> u16 {
                    u16::from(self)
                }
            }
        )*
    };
}

impl_num_to_instruction_encoded!(u3, u5, u7, u8, u11, u16);

impl ToInstEncoding for SPWithOffset {
    fn to_instruction_encoded(self) -> u16 {
        self.0 as u16
    }
}

impl ToInstEncoding for ImmShift {
    fn to_instruction_encoded(self) -> u16 {
        self.1.to_instruction_encoded() << 3 | self.0.to_instruction_encoded()
    }
}

impl ToInstEncoding for ImmOffset {
    fn to_instruction_encoded(self) -> u16 {
        self.1.to_instruction_encoded() << 3 | self.0.to_instruction_encoded()
    }
}

impl ToInstEncoding for RegOffset {
    fn to_instruction_encoded(self) -> u16 {
        self.1.to_instruction_encoded() << 3 | self.0.to_instruction_encoded()
    }
}

#[macro_export]
macro_rules! impl_opcode {
    ($trait_name:ident, $method:ident, ($opcode_pos:expr, $opcode:expr), $( ($pos:expr, $arg:ident : $arg_type:ty) ),*) => {
        impl $trait_name<$($arg_type),*> for $crate::Emitter {
            fn $method(&mut self, $( $arg : $arg_type ),*) {
                use $crate::instructions::ToInstEncoding;

                let mut opcode = $opcode << $opcode_pos;
                $(
                    opcode |= $arg.to_instruction_encoded() << $pos;
                )*
                self.buffer.push(opcode);
            }
        }
    };
}
