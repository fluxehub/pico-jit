#![allow(dead_code)]
#![allow(non_upper_case_globals)]

pub mod traits {
    pub trait Register: Copy {
        fn to_reg_number(self) -> u16;

        fn is_low_register(self) -> bool {
            self.to_reg_number() < 8
        }
    }

    pub trait GeneralPurposeRegister: Register {}
}

pub mod types {
    use crate::registers::traits::*;

    #[derive(Copy, Clone, Debug)]
    pub struct LowRegister(pub(super) u16);

    #[derive(Copy, Clone, Debug)]
    pub struct HighRegister(pub(super) u16);

    #[derive(Copy, Clone, Debug)]
    pub struct StackPointer(pub(super) u16);

    #[derive(Copy, Clone, Debug)]
    pub struct ProgramCounter(pub(super) u16);

    pub struct RegisterList<const LR: bool, const PC: bool>(pub u16);

    #[derive(Copy, Clone)]
    pub enum RegisterType {
        Low(LowRegister),
        High(HighRegister),
        StackPointer(StackPointer),
        ProgramCounter(ProgramCounter),
    }

    impl Register for LowRegister {
        fn to_reg_number(self) -> u16 {
            self.0
        }
    }

    impl Register for HighRegister {
        fn to_reg_number(self) -> u16 {
            self.0
        }
    }

    impl Register for StackPointer {
        fn to_reg_number(self) -> u16 {
            self.0
        }
    }

    impl Register for ProgramCounter {
        fn to_reg_number(self) -> u16 {
            self.0
        }
    }

    impl GeneralPurposeRegister for LowRegister {}
    impl GeneralPurposeRegister for HighRegister {}

    impl Register for RegisterType {
        fn to_reg_number(self) -> u16 {
            match self {
                RegisterType::Low(r) => r.to_reg_number(),
                RegisterType::High(r) => r.to_reg_number(),
                RegisterType::StackPointer(r) => r.to_reg_number(),
                RegisterType::ProgramCounter(r) => r.to_reg_number(),
            }
        }
    }
}

use types::*;

pub const r0: LowRegister = LowRegister(0);
pub const r1: LowRegister = LowRegister(1);
pub const r2: LowRegister = LowRegister(2);
pub const r3: LowRegister = LowRegister(3);
pub const r4: LowRegister = LowRegister(4);
pub const r5: LowRegister = LowRegister(5);
pub const r6: LowRegister = LowRegister(6);
pub const r7: LowRegister = LowRegister(7);
pub const r8: HighRegister = HighRegister(8);
pub const r9: HighRegister = HighRegister(9);
pub const r10: HighRegister = HighRegister(10);
pub const r11: HighRegister = HighRegister(11);
pub const r12: HighRegister = HighRegister(12);
pub const sp: StackPointer = StackPointer(13);
pub const lr: HighRegister = HighRegister(14);
pub const pc: ProgramCounter = ProgramCounter(15);

#[macro_export]
macro_rules! register_list {
    (lr) => {
        {
            use pico_emit::registers::types::RegisterList;

            RegisterList::<true, false>(0)
        }
    };
    (pc) => {
        {
            use pico_emit::registers::types::RegisterList;

            RegisterList::<false, true>(0)
        }
    };
    (lr, $($reg:ident),*) => {
        {
            use pico_emit::registers::traits::Register;
            use pico_emit::registers::types::{LowRegister, RegisterList};
            use pico_emit::registers::*;

            RegisterList::<true, false>($(
                1 << ($reg as LowRegister).to_reg_number()
            )|*)
        }
    };
    (pc, $($reg:ident),*) => {
        {
            use pico_emit::registers::traits::Register;
            use pico_emit::registers::types::{LowRegister, RegisterList};
            use pico_emit::registers::*;

            RegisterList::<false, true>($(
                1 << ($reg as LowRegister).to_reg_number()
            )|*)
        }
    };
    ($($reg:ident),*) => {
        {
            use pico_emit::registers::traits::Register;
            use pico_emit::registers::types::{LowRegister, RegisterList};
            use pico_emit::registers::*;

            RegisterList::<false, false>($(
                1 << ($reg as LowRegister).to_reg_number()
            )|*)
        }
    };
}
