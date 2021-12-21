use core::arch::x86_64::__cpuid;
use core::arch::x86_64::_rdtsc;
use std::time::Duration;

fn main() {
    let print_values: bool = true;
    const ITERATION_COUNT: u64 = 100;
    const SLEEP_DURATION: u64 = 10;

    println!("[rdtsc - rdtsc]");

    let mut total = 0;
    for _ in 0..ITERATION_COUNT {
        let start = unsafe { _rdtsc() };
        let stop = unsafe { _rdtsc() };

        total += stop - start;

        if print_values {
            println!("{} - {} = {}", start, stop, stop - start);
        }

        std::thread::sleep(Duration::from_millis(SLEEP_DURATION));
    }
    println!("Average: {}", total / ITERATION_COUNT);

    println!("[rdtsc - cpuid - rdtsc]");

    total = 0;
    for _ in 0..ITERATION_COUNT {
        let start = unsafe { _rdtsc() };
        let _ = unsafe { __cpuid(0) };
        let stop = unsafe { _rdtsc() };

        total += stop - start;

        if print_values {
            println!("{} - {} = {}", start, stop, stop - start);
        }

        std::thread::sleep(Duration::from_millis(SLEEP_DURATION));
    }
    println!("Average: {}", total / ITERATION_COUNT);
}
