#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Chapter 2");

    // Register addresses for ROW1 and COL1
    // See section 14 of https://infocenter.nordicsemi.com/pdf/nRF51_RM_v3.0.1.pdf
    const GPIO0_PINCFGP0_04_COL1_ADDR: *mut u32 = 0x5000_0710 as *mut u32;
    const GPIO0_PINCFGP0_13_ROW1_ADDR: *mut u32 = 0x5000_0734 as *mut u32;
    // Register address for GPIO0 OUT
    const GPIO0_OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;

    // Only need to set bit 0 to make the pin an output
    const DIR_OUTPUT_POS: u32 = 0;
    const PINCF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;

    unsafe {
        // Make both pins output
        write_volatile(GPIO0_PINCFGP0_13_ROW1_ADDR, PINCF_DRIVE_LED);
        write_volatile(GPIO0_PINCFGP0_04_COL1_ADDR, PINCF_DRIVE_LED);
    }

    const GPIO0_OUT_ROW1_POS: u32 = 13;
    let mut is_on = false;
    loop {
        unsafe {
            write_volatile(GPIO0_OUT_ADDR, (is_on as u32) << GPIO0_OUT_ROW1_POS);
        }

        for _ in 0..50_000 {
            cortex_m::asm::nop();
        }

        rprintln!("LED is {}", if is_on { "on" } else { "off" });
        is_on = !is_on;
    }
}
