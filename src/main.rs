#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod panic_wait;
mod print;
mod synchronization;

use bsp::bcm::init_drivers;

pub fn kernel_init() -> ! {
    unsafe { init_drivers() }
    panic!("STOP");
}
