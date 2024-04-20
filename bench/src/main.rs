//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

mod runner;

extern crate alloc;

use alloc::{string::String, vec::Vec};
use bsp::{
    entry,
    hal::{
        uart::{UartConfig, UartPeripheral},
        Clock, Sio,
    },
    Pins,
};

use defmt_rtt as _;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{clocks::init_clocks_and_plls, pac, watchdog::Watchdog};

use rp_pico::hal;

use embedded_alloc::Heap;

use pico_jit::wasm_module::{Call, WasmModule};

use crate::runner::run_test;

use core::fmt::Write;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024 * 200;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    // info!("Program start");

    // Define the pico's singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Define the pico's clocks, needed for the delay
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let pins = (pins.gpio0.into_function(), pins.gpio1.into_function());
    let mut uart = UartPeripheral::new(pac.UART0, pins, &mut pac.RESETS)
        .enable(UartConfig::default(), clocks.peripheral_clock.freq())
        .unwrap();

    let mut buffer = Vec::new();
    let mut char = [0u8; 1];

    uart.write_str("\r\n(pico-jit) Enter benchmark size (. to enter): ")
        .unwrap();

    loop {
        uart.read_full_blocking(&mut char).unwrap();
        // If char is ., break
        if char[0] == 46 {
            break;
        }

        buffer.push(char[0]);
        uart.write_full_blocking(&char);
    }

    let size = String::from_utf8(buffer).unwrap().parse::<u32>().unwrap();
    write!(uart, "\r\nRunning with size {}...\r\n", size).unwrap();

    // let timer = Rc::new(timer);

    // let compilation_time = Rc::new(RefCell::new(0u64));

    // let reporter = CompilationReporter {
    //     current_time: Box::new(|| timer.get_counter()),
    //     report_time: Box::new(|start, end| {
    //         *compilation_time.borrow_mut() +=
    //             end.checked_duration_since(start).unwrap().to_micros();
    //     }),
    // };

    {
        let wasm = include_bytes!("../benchmarks/wasm/prime.wasm");
        let mut module = WasmModule::from_wasm(wasm).unwrap();

        for t in 1..=2 {
            let mut result = 0;
            let duration = run_test(&timer, || {
                result = module.call("prime", &[0, size]).unwrap();
            });

            write!(
                uart,
                "{}: prime({}): {} us\t{}\r\n",
                t, size, duration, result
            )
            .unwrap();
        }
    }

    // {
    //     let wasm = include_bytes!("../benchmarks/wasm/murmur3.wasm");
    //     let mut module = WasmModule::from_wasm(wasm).unwrap();

    //     for t in 1..=2 {
    //         for i in 0..=size {
    //             module.memory.write_memory(i, i as u8);
    //         }

    //         let mut ptr = 0i32;
    //         let duration = run_test(&timer, || {
    //             ptr = module.call("hash", &[0, size]).unwrap();
    //         });

    //         let mut hash = String::new();
    //         for i in 0..16 {
    //             hash.push_str(&format!(
    //                 "{:02x}",
    //                 module.memory.read_memory(ptr as u32 + i)
    //             ));
    //         }

    //         write!(
    //             uart,
    //             "{}: murmurhash3({}): {} us\t{}\r\n",
    //             t,
    //             size,
    //             hash.as_str(),
    //             duration
    //         )
    //         .unwrap();
    //     }
    // }

    // {
    //     let wasm = include_bytes!("../benchmarks/wasm/mandelbrot.wasm");
    //     let mut module = WasmModule::from_wasm(wasm).unwrap();

    //     for t in 1..=2 {
    //         let mut checksum = 0;
    //         let duration = run_test(&timer, || {
    //             checksum = module.call("mandelbrot", &[size]).unwrap();
    //         });

    //         write!(
    //             uart,
    //             "{}: mandelbrot({}): {} us\t{}\r\n",
    //             t, size, duration, checksum as u32
    //         )
    //         .unwrap();
    //     }
    // }

    // {
    //     let wasm = include_bytes!("../benchmarks/wasm/nbody.wasm");
    //     let mut module = WasmModule::from_wasm(wasm).unwrap();

    //     for t in 1..=2 {
    //         let mut out_ptr = 0;
    //         let duration = run_test(&timer, || {
    //             out_ptr = module.call("nbody", &[size]).unwrap();
    //         });

    //         let energy_start = f32::from_bits(module.memory.read_memory32(out_ptr as u32));
    //         let energy_end = f32::from_bits(module.memory.read_memory32((out_ptr + 4) as u32));

    //         write!(
    //             uart,
    //             "{}: nbody({}): {} us\t{} {}\r\n",
    //             t, size, duration, energy_start, energy_end
    //         )
    //         .unwrap();
    //     }
    // }

    // {
    //     let wasm = include_bytes!("../benchmarks/wasm/quicksort.wasm");
    //     let mut module = WasmModule::from_wasm(wasm).unwrap();

    //     let size = input;

    //     for t in 1..=2 {
    //         let mut out_ptr = 0;
    //         let duration = run_test(&timer, || {
    //             out_ptr = module.call("sort", &[size]).unwrap();
    //         });

    //         let mut sorted = String::new();

    //         for i in 0..size {
    //             sorted.push_str(
    //                 format!(
    //                     "{} ",
    //                     module.memory.read_memory32((i << 2) + out_ptr as u32)
    //                 )
    //                 .as_str(),
    //             );
    //         }

    //         write!(
    //             uart,
    //             "{}: quicksort({}): {} us\t{}\r\n",
    //             t, size, duration, sorted
    //         )
    //         .unwrap();
    //     }
    // }

    loop {
        let aircr_reg = 0xE000ED0C as *mut u32;
        unsafe {
            *aircr_reg = 0x05FA0004; // Resets the cihp
        }
    }
}

// End of file
