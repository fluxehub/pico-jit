use crate::helpers::extensions::*;
use anyhow::Result;
use lazy_static::lazy_static;
use pico_emit::buffer::JitBuffer;
use pico_emit::instructions::Bkpt;
use pico_emit::{registers, Emitter};
use probe_rs::{Core, MemoryInterface, Permissions, Probe, Session};
use std::sync::Mutex;

const ITERATIONS_PER_TEST: usize = 30;

lazy_static! {
    static ref SESSION: Mutex<Session> = {
        let probe = Probe::list_all()[0].open().unwrap();
        Mutex::new(probe.attach("rp2040", Permissions::default()).unwrap())
    };
}

pub fn run_tests<F: Fn(&mut Core)>(
    test: impl Fn(&mut Emitter, &mut Core) -> Result<F>,
) -> Result<()> {
    for _ in 0..ITERATIONS_PER_TEST {
        let mut emitter = Emitter::new();

        // Get the session lock
        let mut session = SESSION.lock().unwrap_or_else(|e| e.into_inner());

        // Get the core and wait for it to hit the breakpoint
        let mut core = session.core(0).unwrap();
        core.reset()?;
        core.wait_for_core_halted(std::time::Duration::from_secs(1))?;

        // Run the test setup and add a breakpoint to the end
        let verify = test(&mut emitter, &mut core)?;
        emitter.bkpt();

        // Build the function, write it to the chip, and run it, setting the PC to the start of the function
        let func = emitter.build();

        // Cast buffer to a slice of u8s and write it to the chip
        core.write_8(0x2000_2000, bytemuck::cast_slice(&func.data))?;
        core.write_reg(registers::pc, 0x2000_2000)?;
        core.run()?;

        // Wait for the chip to hit the breakpoint
        core.wait_for_core_halted(std::time::Duration::from_secs(1))?;

        // Use the returned function to verify the test
        verify(&mut core);
    }

    Ok(())
}
