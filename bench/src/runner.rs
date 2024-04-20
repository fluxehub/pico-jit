use rp_pico::hal::Timer;

pub fn run_test(timer: &Timer, mut test: impl FnMut()) -> u64 {
    let start = timer.get_counter();
    test();
    let end = timer.get_counter();
    end.checked_duration_since(start).unwrap().to_micros()
}
