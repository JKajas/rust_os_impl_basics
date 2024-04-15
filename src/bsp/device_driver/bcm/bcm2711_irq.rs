use core::ptr::read_volatile;
// ARM GIC-400 disctibutor offset starts with 0x1000
// ARM GIC-400 Shered Peripheral Interrupt Status Register starts with 0xD04
// PACTL_CS register at  0x7E20 4E00 -> 0xFE20_4E00
// VC interuption IDs 96-159 its 0x1D10 to 0x1D18
// UART interuption ID among VC ids 57 needed OR
//  PACTL_CS (at address 0x7E20 4E00) registers

const PACTL_CS: *const u32 = 0xfe20_4e00 as *const u32;
const AUX_IRQ: *const u32 = 0xfe21_500 as *const u32;
const MIMOIRQ: [*const u32; 5] = [
    0xff841D08 as *const u32,
    0xff841D10 as *const u32,
    0xff841D18 as *const u32,
    0xff841D20 as *const u32,
    0xff841D28 as *const u32,
]; // All segments where IRQ occures

#[no_mangle]
async unsafe fn irq_handler() -> () {
    let ids: [u32; 5] = core::array::from_fn(|i| read_volatile(MIMOIRQ[i]));
    if (ids[0] & !(0 << 15)).count_ones() > 0 {}
    if ids[0] & 1 << 16 == 1 << 16 {}
    if ids[0] & 1 << 17 == 1 << 17 {}
    if ids[0] & 1 << 18 == 1 << 18 {}
    if ids[0] & 1 << 19 == 1 << 19 {}
    if ids[0] & 1 << 20 == 1 << 20 {}
    if ids[0] & 1 << 21 == 1 << 21 {}
    if ids[1].count_ones() > 0 {}
    if ids[2].count_ones() > 0 {}
    if ids[3].count_ones() > 0 {
        VC_IRQ::call_driver_handler(ids[3])
    }
    if ids[4].count_ones() > 0 {}
}
#[allow(non_camel_case_types)]
struct VC_IRQ {}
#[repr(u32)]
#[allow(non_camel_case_types)]
enum UART_interfaces {
    UART5 = 1 << 16,
    UART4 = 1 << 17,
    UART3 = 1 << 18,
    UART2 = 1 << 19,
    UART0 = 1 << 20,
}
#[repr(u32)]
#[allow(non_camel_case_types)]
enum I2C_interfaces {
    I2C0 = 1 << 8,
    I2C1 = 1 << 9,
    I2C2 = 1 << 10,
    I2C3 = 1 << 11,
    I2C4 = 1 << 12,
    I2C5 = 1 << 13,
    I2C6 = 1 << 14,
    I2C7 = 1 << 15,
}
#[repr(u32)]
#[allow(non_camel_case_types)]
enum SPI_interfaces {
    SPI0 = 1 << 0,
    SPI1 = 1 << 1,
    SPI2 = 1 << 2,
    SPI3 = 1 << 3,
    SPI4 = 1 << 4,
    SPI5 = 1 << 5,
    SPI6 = 1 << 6,
}
impl Handler for VC_IRQ {
    unsafe fn call_driver_handler(id: u32) {
        match id {
            29 => Self::call_aux_handler(id),
            53 => Self::call_i2c_handler(id),
            54 => Self::call_spi_handler(id),
            57 => Self::call_uart_handler(id), // &'static UART_handler.handle()
            _ => {}
        }
    }
}
impl VC_IRQ {
    unsafe fn call_aux_handler(id: u32) {
        let pactl_cs_register = read_volatile(PACTL_CS);
        match pactl_cs_register {
            0 => {}
            _ => {}
        }
    }

    unsafe fn call_uart_handler(id: u32) {
        // -> &'static UART_handler
        let pactl_cs_register = read_volatile(PACTL_CS);
        match pactl_cs_register {
            id if pactl_cs_register == UART_interfaces::UART5 as u32 => {}
            id if pactl_cs_register == UART_interfaces::UART4 as u32 => {}
            id if pactl_cs_register == UART_interfaces::UART3 as u32 => {}
            id if pactl_cs_register == UART_interfaces::UART2 as u32 => {}
            id if pactl_cs_register == UART_interfaces::UART0 as u32 => {}
            _ => {}
        }
    }
    unsafe fn call_spi_handler(id: u32) {
        let pactl_cs_register = read_volatile(PACTL_CS);
        match pactl_cs_register {
            id if pactl_cs_register == SPI_interfaces::SPI0 as u32 => {}
            id if pactl_cs_register == SPI_interfaces::SPI1 as u32 => {}
            id if pactl_cs_register == SPI_interfaces::SPI2 as u32 => {}
            id if pactl_cs_register == SPI_interfaces::SPI3 as u32 => {}
            id if pactl_cs_register == SPI_interfaces::SPI4 as u32 => {}
            id if pactl_cs_register == SPI_interfaces::SPI5 as u32 => {}
            id if pactl_cs_register == SPI_interfaces::SPI6 as u32 => {}
            _ => {}
        }
    }
    unsafe fn call_i2c_handler(id: u32) {
        let pactl_cs_register = read_volatile(PACTL_CS);
        match pactl_cs_register {
            id if pactl_cs_register == I2C_interfaces::I2C0 as u32 => {}
            id if pactl_cs_register == I2C_interfaces::I2C1 as u32 => {}
            id if pactl_cs_register == I2C_interfaces::I2C2 as u32 => {}
            id if pactl_cs_register == I2C_interfaces::I2C3 as u32 => {}
            id if pactl_cs_register == I2C_interfaces::I2C4 as u32 => {}
            id if pactl_cs_register == I2C_interfaces::I2C5 as u32 => {}
            id if pactl_cs_register == I2C_interfaces::I2C6 as u32 => {}
            id if pactl_cs_register == I2C_interfaces::I2C7 as u32 => {}
            _ => {}
        }
    }
}

trait Handler {
    unsafe fn call_driver_handler(id: u32);
}
