use crate::println;
use core::ptr::read_volatile;
// ARM GIC-400 disctibutor offset starts with 0x1000
// ARM GIC-400 Shered Peripheral Interrupt Status Register starts with 0xD04
// PACTL_CS register at  0x7E20 4E00 -> 0xFE20_4E00
// VC interuption IDs 96-159 its 0x1D10 to 0x1D18
// UART interuption ID among VC ids 57 needed OR
//  PACTL_CS (at address 0x7E20 4E00) registers

#[no_mangle]
unsafe fn irq_handler() -> () {
    let id = read_volatile(0xff840000 as *const i32);
    println!("{:x?}", id);
}
