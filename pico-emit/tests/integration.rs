mod helpers;

#[cfg(test)]
mod tests {
    use crate::*;
    use anyhow::Result;
    use helpers::extensions::*;
    use helpers::random_data::*;
    use helpers::runner::run_tests;
    use pico_emit::instructions::*;
    use pico_emit::register_list;
    use pico_emit::registers::traits::Register;
    use pico_emit::registers::types::{LowRegister, RegisterType};
    use pico_emit::registers::*;
    use probe_rs::MemoryInterface;
    use ux2::{u3, u5, u7};

    #[test]
    fn adc() -> Result<()> {
        run_tests(|emitter, core| {
            let (src, dest, src_val, dest_val, carry) =
                random_data!(LowRegister, LowRegister, u32, u32, bool);

            core.write_reg(src, src_val)?;
            core.write_reg(dest, dest_val)?;
            core.write_carry(carry)?;

            emitter.adc(dest, src);

            let expected = if carry {
                dest_val.wrapping_add(src_val).wrapping_add(1)
            } else {
                dest_val.wrapping_add(src_val)
            };

            deferred_assert_reg!(dest, expected)
        })
    }

    #[test]
    fn adds_imm3() -> Result<()> {
        run_tests(|emitter, core| {
            let (src, dest, val, imm) = random_data!(LowRegister, LowRegister, u32, u3);

            core.write_reg(src, val)?;
            emitter.adds(dest, Add2Imm(src, imm));

            deferred_assert_reg!(dest, val.wrapping_add(imm.into()))
        })
    }

    #[test]
    fn adds_imm8() -> Result<()> {
        run_tests(|emitter, core| {
            let (reg, val, imm) = random_data!(LowRegister, u32, u8);

            core.write_reg(reg, val)?;
            emitter.adds(reg, imm);

            deferred_assert_reg!(reg, val.wrapping_add(imm.into()))
        })
    }

    #[test]
    fn adds_reg() -> Result<()> {
        run_tests(|emitter, core| {
            let (src, dest, src_val, dest_val) = random_data!(LowRegister, LowRegister, u32, u32);

            core.write_reg(src, src_val)?;
            core.write_reg(dest, dest_val)?;
            emitter.adds(dest, src);

            deferred_assert_reg!(dest, src_val.wrapping_add(dest_val))
        })
    }

    #[test]
    fn adds_2_reg() -> Result<()> {
        run_tests(|emitter, core| {
            let (a, b, dest, a_val, b_val) =
                random_data!(LowRegister, LowRegister, LowRegister, u32, u32);

            core.write_reg(a, a_val)?;
            core.write_reg(b, b_val)?;
            emitter.adds(dest, Add2(a, b));

            deferred_assert_reg!(dest, a_val.wrapping_add(b_val))
        })
    }

    #[test]
    fn add_reg() -> Result<()> {
        run_tests(|emitter, core| {
            let (src, dest, src_val, dest_val) = random_data!(RegisterType, RegisterType, u32, u32);

            core.write_reg(src, src_val)?;
            core.write_reg(dest, dest_val)?;
            emitter.add(dest, src);

            deferred_assert_reg!(dest, src_val.wrapping_add(dest_val))
        })
    }

    #[test]
    fn add_reg_sp_imm() -> Result<()> {
        run_tests(|emitter, core| {
            let (dest, sp_val, imm) = random_data!(LowRegister, u32, u8);
            let sp_val = (sp_val & !0b11) << 2; // Align to 4 bytes

            core.write_reg(sp, sp_val)?;
            emitter.add(dest, SPWithOffset(imm));

            deferred_assert_reg!(dest, sp_val.wrapping_add((imm as u32) << 2)) // the instruction shifts the offset by 2
        })
    }

    #[test]
    fn add_sp_imm() -> Result<()> {
        run_tests(|emitter, core| {
            let (sp_val, imm) = random_data!(u32, u7);
            let sp_val = (sp_val & !0b11) << 2; // Align to 4 bytes

            core.write_reg(sp, sp_val)?;
            emitter.add(sp, imm);

            deferred_assert_reg!(sp, sp_val.wrapping_add(u32::from(imm) << 2)) // the instruction shifts the offset by 2
        })
    }

    // TODO: ADR test

    #[test]
    fn and() -> Result<()> {
        run_tests(|emitter, core| {
            let (src, dest, src_val, dest_val) = random_data!(LowRegister, LowRegister, u32, u32);
            core.write_reg(src, src_val)?;
            core.write_reg(dest, dest_val)?;
            emitter.and(dest, src);

            deferred_assert_reg!(dest, src_val & dest_val)
        })
    }

    #[test]
    fn asr_imm() -> Result<()> {
        run_tests(|emitter, core| {
            let (src, dest, val, imm) = random_data!(LowRegister, LowRegister, u32, u5);
            core.write_reg(src, val)?;

            emitter.asr(dest, ImmShift(src, imm));

            // The core will give 0 or -1 when shifting by 32 (which is done when imm is 0)
            // but rust's shift operator will give the original value
            let expected = if imm == u5::new(0) {
                if (val as i32) < 0 {
                    -1
                } else {
                    0
                }
            } else {
                (val as i32).wrapping_shr(imm.into())
            };

            deferred_assert_reg!(dest, expected)
        })
    }

    #[test]
    fn asr_reg() -> Result<()> {
        run_tests(|emitter, core| {
            let (shift, dest, shift_val, dest_val) =
                random_data!(LowRegister, LowRegister, u32, u32);
            core.write_reg(shift, shift_val)?;
            core.write_reg(dest, dest_val)?;

            emitter.asr(dest, shift);

            let shift_val = shift_val & 0xFF; // The core will only use the bottom 8 bits of the shift value

            // Same "rust vs core shift" logic as in asr_imm
            let expected = if shift_val >= 32 {
                if (dest_val as i32) < 0 {
                    -1
                } else {
                    0
                }
            } else {
                (dest_val as i32).wrapping_shr(shift_val)
            };

            deferred_assert_reg!(dest, expected)
        })
    }

    #[test]
    fn b() -> Result<()> {
        run_tests(|emitter, _| {
            let mut label = emitter.create_label();
            emitter.movs(r0, 0);
            emitter.b(label);
            emitter.bkpt();
            emitter.label(&mut label);
            emitter.movs(r0, 1);

            deferred_assert_reg!(r0, 1)
        })
    }

    // TODO: Conditional B test

    #[test]
    fn bic() -> Result<()> {
        run_tests(|emitter, core| {
            let (src, dest, src_val, dest_val) = random_data!(LowRegister, LowRegister, u32, u32);
            core.write_reg(src, src_val)?;
            core.write_reg(dest, dest_val)?;
            emitter.bic(dest, src);

            deferred_assert_reg!(dest, dest_val & !src_val)
        })
    }

    // bkpt isn't tested as the whole testing framework relies on it working, so it's essentially tested implicitly

    // TODO: BLX test
    // TODO: BX test
    // TODO: CMN and CMP test

    #[test]
    fn eor() -> Result<()> {
        run_tests(|emitter, core| {
            let (src, dest, src_val, dest_val) = random_data!(LowRegister, LowRegister, u32, u32);
            core.write_reg(src, src_val)?;
            core.write_reg(dest, dest_val)?;
            emitter.eor(dest, src);

            deferred_assert_reg!(dest, dest_val ^ src_val)
        })
    }

    #[test]
    fn movs_imm8() -> Result<()> {
        run_tests(|emitter, _| {
            let (reg, val) = random_data!(LowRegister, u8);
            emitter.movs(reg, val);

            deferred_assert_reg!(reg, val)
        })
    }

    #[test]
    fn ldm() -> Result<()> {
        run_tests(|emitter, core| {
            let (a, b, c) = random_data!(u32, u32, u32);
            core.write_reg(r0, 0x2000_4000)?; // random address
            core.write_mem_32bit(0x2000_4000, &a.to_le_bytes())?;
            core.write_mem_32bit(0x2000_4004, &b.to_le_bytes())?;
            core.write_mem_32bit(0x2000_4008, &c.to_le_bytes())?;

            emitter.ldm(r0, register_list!(r1, r2, r3));

            Ok(move |core: &mut probe_rs::Core| {
                assert_eq!(
                    core.read_core_reg::<u32>(r0.to_reg_number()).unwrap(),
                    0x2000_400C
                );
                assert_eq!(core.read_core_reg::<u32>(r1.to_reg_number()).unwrap(), a);
                assert_eq!(core.read_core_reg::<u32>(r2.to_reg_number()).unwrap(), b);
                assert_eq!(core.read_core_reg::<u32>(r3.to_reg_number()).unwrap(), c);
            })
        })
    }

    #[test]
    fn ldr_label() -> Result<()> {
        run_tests(|emitter, _| {
            let (reg, val) = random_data!(LowRegister, u32);

            let label = emitter.data(val);

            emitter.nop();
            emitter.ldr(reg, label);
            emitter.nop();

            deferred_assert_reg!(reg, val)
        })
    }

    #[test]
    fn mov_reg() -> Result<()> {
        run_tests(|emitter, core| {
            let (src, dest, val) = random_data!(RegisterType, RegisterType, u32);
            core.write_reg(src, val)?;
            emitter.mov(dest, src);

            deferred_assert_reg!(dest, val)
        })
    }

    #[test]
    fn movs_reg() -> Result<()> {
        run_tests(|emitter, core| {
            let (src, dest, val) = random_data!(LowRegister, LowRegister, u32);
            core.write_reg(src, val)?;
            emitter.movs(dest, src);

            deferred_assert_reg!(dest, val)
        })
    }

    #[test]
    fn or() -> Result<()> {
        run_tests(|emitter, core| {
            let (src, dest, src_val, dest_val) = random_data!(LowRegister, LowRegister, u32, u32);
            core.write_reg(src, src_val)?;
            core.write_reg(dest, dest_val)?;
            emitter.or(dest, src);

            deferred_assert_reg!(dest, src_val | dest_val)
        })
    }

    #[test]
    fn fib_test() -> Result<()> {
        fn fib(n: u32) -> u32 {
            if n <= 1 {
                return n;
            }

            let mut f0: u32 = 0;
            let mut f1: u32 = 1;
            for _ in 2..=n {
                let f2 = f0.wrapping_add(f1);
                f0 = f1;
                f1 = f2;
            }
            f1
        }

        run_tests(|emitter, _| {
            let i = random_data!(u8);

            let mut loop_label = emitter.create_label();
            let mut exit_label = emitter.create_label();

            emitter.movs(r0, 0);
            emitter.movs(r1, 1);
            emitter.movs(r3, r0);

            emitter.movs(r2, i);
            emitter.b_if(Condition::EQ, exit_label); // if i == 0, return 0
            emitter.movs(r3, r1);
            emitter.subs(r2, Sub2Imm(r2, u3::new(1))); // i -= 1
            emitter.b_if(Condition::EQ, exit_label); // if now i == 0, return 1

            emitter.label(&mut loop_label);
            emitter.adds(r3, Add2(r0, r1));
            emitter.movs(r0, r1);
            emitter.movs(r1, r3);
            emitter.subs(r2, Sub2Imm(r2, u3::new(1)));
            emitter.b_if(Condition::NE, loop_label);

            emitter.label(&mut exit_label);
            emitter.movs(r0, r3);

            deferred_assert_reg!(r0, fib(i as u32))
        })
    }
}
