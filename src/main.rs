use core::arch::x86_64::__cpuid;
use core::arch::x86_64::_rdtsc;
use std::time::Duration;
use winapi::um::processthreadsapi::{GetCurrentThread, SetThreadPriority};
use winapi::um::winbase::THREAD_PRIORITY_TIME_CRITICAL;

const PRINT_VALUES: bool = false;
const ITERATION_COUNT: u64 = 20_000;
const SLEEP_DURATION: u64 = 10;

#[inline(always)]
fn rdtsc_rdtsc() {
    println!("[rdtsc - rdtsc]");

    let mut total = 0;
    for _ in 0..ITERATION_COUNT {
        let start = unsafe { _rdtsc() };
        let stop = unsafe { _rdtsc() };

        total += stop - start;

        if PRINT_VALUES {
            println!("{} - {} = {}", start, stop, stop - start);
        }
    }
    println!("Average: {}", total / ITERATION_COUNT);
}

#[inline(always)]
fn rdtsc_cpuid_rdtsc() {
    println!("[rdtsc - cpuid - rdtsc]");

    let mut total = 0;
    for _ in 0..ITERATION_COUNT {
        let start = unsafe { _rdtsc() };
        let _ = unsafe { __cpuid(0) };
        let stop = unsafe { _rdtsc() };

        total += stop - start;

        if PRINT_VALUES {
            println!("{} - {} = {}", start, stop, stop - start);
        }
    }
    println!("Average: {}", total / ITERATION_COUNT);
}

/// See: https://secret.club/2020/01/12/battleye-hypervisor-detection.html
#[inline(always)]
fn rdtsc_cpuid_rdtsc_calibration() {
    println!("[rdtsc - cpuid - rdtsc] (with calibration)");

    let start = unsafe { _rdtsc() };
    std::thread::sleep(std::time::Duration::from_secs(1));
    let timestamp_calibration = unsafe { _rdtsc() } - start;

    let mut total = 0;
    for _ in 0..0x6694 {
        let start = unsafe { _rdtsc() };
        let _ = unsafe { __cpuid(0) };
        let stop = unsafe { _rdtsc() };

        total += stop - start;

        if PRINT_VALUES {
            println!("{} - {} = {}", start, stop, stop - start);
        }
    }
    let result = 10000000 * total / timestamp_calibration / 0x65;
    println!("Average: {}", result);
}

fn main() {
    let old_priority = unsafe { SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_TIME_CRITICAL as _) };

    //
    //

    rdtsc_rdtsc();
    rdtsc_cpuid_rdtsc();
    rdtsc_cpuid_rdtsc_calibration();

    //
    //
    unsafe { SetThreadPriority(GetCurrentThread(), old_priority) };
}
