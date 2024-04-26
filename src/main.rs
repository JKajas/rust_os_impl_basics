#![no_main]
#![no_std]

use crate::synchronization::interface::Mutex;
mod bsp;
mod console;
mod cpu;
mod panic_wait;
mod print;
mod synchronization;

pub fn kernel_init() -> () {
    use bsp::bcm::bcm2711_uart::Uart;
    let uart = unsafe { Uart::new(0x0_FE20_1000) };
    unsafe {
        uart.inner.lock(|inner| {
            inner.init(
                bsp::bcm::ParityBit::None,
                bsp::bcm::WordLength::Bit8,
                bsp::bcm::StopBits::One,
                115200,
            )
        });
        bsp::bcm::read_uart_clock();
    };
    panic!("Stopping here");
}
