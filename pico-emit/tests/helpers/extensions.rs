use anyhow::Result;
use pico_emit::registers::traits::Register;
use probe_rs::{Core, RegisterValue};

pub trait CoreExt {
    fn write_reg(&mut self, reg: impl Register, value: u32) -> Result<()>;
    fn write_carry(&mut self, flag: bool) -> Result<()>;
}

impl<'a> CoreExt for Core<'a> {
    fn write_reg(&mut self, reg: impl Register, value: u32) -> Result<()> {
        self.write_core_reg(reg.to_reg_number(), RegisterValue::U32(value))?;
        Ok(())
    }

    fn write_carry(&mut self, flag: bool) -> Result<()> {
        let xpsr_value: u32 = self.read_core_reg::<u32>(16)?;
        let xpsr_value = if flag {
            xpsr_value | (0b1 << 29)
        } else {
            xpsr_value & !(0b1 << 29)
        };
        self.write_core_reg(16, RegisterValue::U32(xpsr_value))?;
        Ok(())
    }
}

#[macro_export]
macro_rules! deferred_assert_reg (
    ($reg:expr, $val:expr) => {
        {
            use probe_rs::Core;
            use pico_emit::registers::traits::Register;
            Ok(move |core: &mut Core| assert_eq!(core.read_core_reg::<u32>($reg.to_reg_number()).unwrap(), $val as u32))
        }
    };
);
