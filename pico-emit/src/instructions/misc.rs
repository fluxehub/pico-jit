use crate::emitter::Label;
use crate::instructions::ToInstEncoding;
use crate::registers::traits::GeneralPurposeRegister;
use crate::registers::types::{LowRegister, RegisterList};
use crate::Emitter;

#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Condition {
    EQ = 0b0000,
    NE = 0b0001,
    /// Also known as HS
    CS = 0b0010,
    /// Also known as LO
    CC = 0b0011,
    MI = 0b0100,
    PL = 0b0101,
    VS = 0b0110,
    VC = 0b0111,
    HI = 0b1000,
    LS = 0b1001,
    GE = 0b1010,
    LT = 0b1011,
    GT = 0b1100,
    LE = 0b1101,
    AL = 0b1110,
}

impl Condition {
    pub fn invert(self) -> Self {
        match self {
            Self::EQ => Self::NE,
            Self::NE => Self::EQ,
            Self::CS => Self::CC,
            Self::CC => Self::CS,
            Self::MI => Self::PL,
            Self::PL => Self::MI,
            Self::VS => Self::VC,
            Self::VC => Self::VS,
            Self::HI => Self::LS,
            Self::LS => Self::HI,
            Self::GE => Self::LT,
            Self::LT => Self::GE,
            Self::GT => Self::LE,
            Self::LE => Self::GT,
            Self::AL => Self::AL, // AL is never encoded in this implementation so it doesn't matter
        }
    }
}

#[derive(Debug)]
pub enum LabelInstruction {
    ADR(Label, LowRegister),
    B(Label, Condition),
    Branch(Label, Condition),
    LDR(Label, LowRegister),
}

macro_rules! define_opcode {
    ($trait_name:ident, $name:ident, $opcode:expr) => {
        pub trait $trait_name {
            fn $name(&mut self);
        }

        impl $trait_name for Emitter {
            fn $name(&mut self) {
                self.buffer.push($opcode);
            }
        }
    };

    ($trait_name:ident, $name:ident, ($opcode_pos:expr, $opcode:expr), $( ($pos:expr, $arg:ident : $arg_type:ty) ),*) => {
        pub trait $trait_name {
            fn $name(&mut self, $( $arg : $arg_type ),*);
        }

        impl $trait_name for Emitter {
            fn $name(&mut self, $( $arg : $arg_type ),*) {
                let mut opcode = $opcode << $opcode_pos;
                $(
                    opcode |= $arg.to_instruction_encoded() << $pos;
                )*
                self.buffer.push(opcode);
            }
        }
    };
}

define_opcode!(Adc, adc, (6, 0b010000_0101), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(And, and, (6, 0b010000_0000), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Bic, bic, (6, 0b010000_1110), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Bkpt, bkpt, 0b1011_1110_00000000);
define_opcode!(Blx, blx, (7, 0b010001_11_1), (3, reg: impl GeneralPurposeRegister));
define_opcode!(Bx, bx, (7, 0b010001_11_0), (3, reg: impl GeneralPurposeRegister));
define_opcode!(Cmn, cmn, (6, 0b010000_1011), (3, a: LowRegister), (0, b: LowRegister));
define_opcode!(Eor, eor, (6, 0b010000_0001), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Ldm, ldm, (11, 0b1100_1), (8, base: LowRegister), (0, list: RegisterList<false, false>));
define_opcode!(Mul, mul, (6, 0b010000_1101), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Mvn, mvn, (6, 0b010000_1111), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Nop, nop, 0b1011_1111_0000_0000);
define_opcode!(Or, or, (6, 0b010000_1100), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Rev, rev, (6, 0b1011_1010_00), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Rev16, rev16, (6, 0b1011_1010_01), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Revsh, revsh, (6, 0b1011_1010_11), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Ror, ror, (6, 0b010000_0111), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Rsb, rsb, (6, 0b010000_1001), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Sbc, sbc, (6, 0b010000_0110), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Stm, stm, (11, 0b1100_0), (8, base: LowRegister), (0, list: RegisterList<false, false>));
define_opcode!(Sxtb, sxtb, (6, 0b1011_0010_01), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Sxth, sxth, (6, 0b1011_0010_00), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Tst, tst, (6, 0b010000_1000), (3, a: LowRegister), (0, b: LowRegister));
define_opcode!(Uxtb, uxtb, (6, 0b1011_0010_11), (0, dest: LowRegister), (3, src: LowRegister));
define_opcode!(Uxth, uxth, (6, 0b1011_0010_10), (0, dest: LowRegister), (3, src: LowRegister));

pub trait Adr {
    fn adr(&mut self, dest: LowRegister, label: Label);
}

impl Adr for Emitter {
    fn adr(&mut self, dest: LowRegister, label: Label) {
        let offset = self.buffer.push_empty();
        self.unfilled_instructions
            .push((offset, LabelInstruction::ADR(label, dest)));
    }
}

pub trait B {
    fn b(&mut self, label: Label);
    fn b_if(&mut self, condition: Condition, label: Label);
}

impl B for Emitter {
    fn b(&mut self, label: Label) {
        self.b_if(Condition::AL, label);
    }

    fn b_if(&mut self, condition: Condition, label: Label) {
        let offset = self.buffer.push_empty();
        self.unfilled_instructions
            .push((offset, LabelInstruction::B(label, condition)));
    }
}

pub trait Branch {
    fn branch(&mut self, label: Label);
    fn branch_if(&mut self, condition: Condition, label: Label);
}

impl Branch for Emitter {
    fn branch(&mut self, label: Label) {
        let offset = self.buffer.push_empty();
        self.nop(); // Reserve space in case branch becomes a BL
        self.unfilled_instructions
            .push((offset, LabelInstruction::Branch(label, Condition::AL)));
    }

    fn branch_if(&mut self, condition: Condition, label: Label) {
        let offset = self.buffer.push_empty();
        self.nop(); // Reserve space in case branch becomes a BL
        self.nop();
        self.unfilled_instructions
            .push((offset, LabelInstruction::Branch(label, condition)));
    }
}
