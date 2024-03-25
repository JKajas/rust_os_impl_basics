#![no_main]
#![no_std]
mod bsp;
mod console;
mod cpu;
mod panic_wait;
mod print;
mod synchronization;

pub fn kernel_init() -> ! {
    println!("Hello WORLD!");
    panic!("Stopping here");
}
