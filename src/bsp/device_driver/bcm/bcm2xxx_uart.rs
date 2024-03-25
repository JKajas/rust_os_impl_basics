/*
use crate::{bsp::common::MIMODerefWrapper, synchronization::NullLock};
// data segement for UART_1 0x4_7e00_0000
//
enum Permission {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

struct Register {
    offset: u8,
    permission: Permission,
}
impl Register {
    unsafe fn write(&self) -> () {
        use core::ptr::write_volatile;
        let data: [u8; 10] = [0u8; 10];
        let register_address = self as *const _ + self.offset;
        write_volatile(register_address, data);
    }
}
impl<const ADDR: usize> Register {
    fn write() -> () {}
}
macro_rules! registers {
    ($((REGISTER_NAME($register_name:ident), OFFSET($register_offset:expr), PERM($permission:expr))),+) => {
        #[allow(non_snake_case)]
        struct Registers{

            $($register_name: Register),+
        }
        impl Registers{
            fn new() -> Self {
                Self {
                    $(
                        $register_name: Register{offset: $register_offset, permission: $permission},
                    )+
                }
            }
        }
    }
}
registers!(
    (REGISTER_NAME(DR), OFFSET(0x00), PERM(Permission::ReadOnly)), // Data register
    (
        REGISTER_NAME(RSRECR),
        OFFSET(0x04),
        PERM(Permission::ReadOnly)
    ),
    (REGISTER_NAME(FR), OFFSET(0x18), PERM(Permission::ReadOnly)), // Flag register
    (
        REGISTER_NAME(ILPR),
        OFFSET(0x20),
        PERM(Permission::ReadOnly)
    ), // Not in use
    (
        REGISTER_NAME(IBRD),
        OFFSET(0x24),
        PERM(Permission::ReadOnly)
    ), // Integer Baud Rate divisor
    (
        REGISTER_NAME(FBRD),
        OFFSET(0x28),
        PERM(Permission::ReadOnly)
    ), // Fractional Baud Rate divisor
    (
        REGISTER_NAME(LCRH),
        OFFSET(0x2c),
        PERM(Permission::ReadOnly)
    ), // Line Control register
    (REGISTER_NAME(CR), OFFSET(0x30), PERM(Permission::ReadOnly)), // Control register
    (
        REGISTER_NAME(IFLS),
        OFFSET(0x34),
        PERM(Permission::ReadOnly)
    ), // Interrup FIFO Level Select Register
    (
        REGISTER_NAME(IMSC),
        OFFSET(0x38),
        PERM(Permission::ReadOnly)
    ), // Interrup Mask Set Clear Register
    (REGISTER_NAME(RIS), OFFSET(0x3c), PERM(Permission::ReadOnly)), // Raw Interrupt Status Register
    (REGISTER_NAME(MIS), OFFSET(0x40), PERM(Permission::ReadOnly)), // Masked Interrupt Status Register
    (REGISTER_NAME(ICR), OFFSET(0x44), PERM(Permission::ReadOnly)), // Interrupt Clear Register
    (
        REGISTER_NAME(DMACR),
        OFFSET(0x48),
        PERM(Permission::ReadOnly)
    ), // DMA Control Register
    (
        REGISTER_NAME(ITCR),
        OFFSET(0x80),
        PERM(Permission::ReadOnly)
    ), // Test Control Register
    (
        REGISTER_NAME(ITIP),
        OFFSET(0x84),
        PERM(Permission::ReadOnly)
    ), // Integration test input reg
    (
        REGISTER_NAME(ITOP),
        OFFSET(0x88),
        PERM(Permission::ReadOnly)
    ), // Integration test output reg
    (REGISTER_NAME(TDR), OFFSET(0x8c), PERM(Permission::ReadOnly))  // Test Data reg
);
type RegisterMapped = MIMODerefWrapper<Registers>;
struct UartInner {
    chars_written: usize,
    chars_read: usize,
    registers: RegisterMapped,
}
struct Uart {
    inner: NullLock<UartInner>,
}

impl UartInner {
    unsafe fn new(start_addr: usize) -> Self {
        Self {
            chars_read: 0,
            chars_written: 0,
            registers: RegisterMapped::new(start_addr),
        }
    }
    pub fn init(&mut self) {
        self.registers.TDR
    }
}
*/
