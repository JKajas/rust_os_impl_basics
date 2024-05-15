use crate::bsp::common::{MIMODerefWrapper, Permission, Register, RegisterInterface};
use crate::registers;
use crate::synchronization::NullLock;

use super::InitDriverTrait;

registers!(
    (
        REGISTER_NAME(GPFSEL0),
        OFFSET(0x00),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPFSEL1),
        OFFSET(0x04),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPFSEL2),
        OFFSET(0x08),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPFSEL3),
        OFFSET(0x0c),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPFSEL4),
        OFFSET(0x10),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPFSEL5),
        OFFSET(0x14),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPSET0),
        OFFSET(0x1c),
        PERM(Permission::WriteOnly)
    ),
    (
        REGISTER_NAME(GPSET1),
        OFFSET(0x20),
        PERM(Permission::WriteOnly)
    ),
    (
        REGISTER_NAME(GPCLR0),
        OFFSET(0x28),
        PERM(Permission::WriteOnly)
    ),
    (
        REGISTER_NAME(GPCLR1),
        OFFSET(0x2c),
        PERM(Permission::WriteOnly)
    ),
    (
        REGISTER_NAME(GPLEV0),
        OFFSET(0x34),
        PERM(Permission::ReadOnly)
    ),
    (
        REGISTER_NAME(GPLEV1),
        OFFSET(0x38),
        PERM(Permission::ReadOnly)
    ),
    (
        REGISTER_NAME(GPEDS0),
        OFFSET(0x40),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPEDS1),
        OFFSET(0x44),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPREN0),
        OFFSET(0x4c),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPREN1),
        OFFSET(0x50),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPFEN0),
        OFFSET(0x58),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPFEN1),
        OFFSET(0x5c),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPHEN0),
        OFFSET(0x64),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPHEN1),
        OFFSET(0x68),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPLEN0),
        OFFSET(0x70),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPLEN1),
        OFFSET(0x74),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPAREN0),
        OFFSET(0x7c),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPAREN1),
        OFFSET(0x80),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPAFEN0),
        OFFSET(0x88),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPAFEN1),
        OFFSET(0x8c),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPIO_PUP_PDN_CNTRL_REG0),
        OFFSET(0xe4),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPIO_PUP_PDN_CNTRL_REG1),
        OFFSET(0xe8),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPIO_PUP_PDN_CNTRL_REG2),
        OFFSET(0xec),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(GPIO_PUP_PDN_CNTRL_REG3),
        OFFSET(0xf0),
        PERM(Permission::ReadWrite)
    )
);

#[repr(u8)]
enum GPIOFunction {
    Input = 0,
    Output = 1,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}
enum GPIOLevel {
    High,
    Low,
}
impl RegisterInterface for Registers {}

type RegisterMapped = MIMODerefWrapper<Registers>;
pub struct GPIOInner {
    pin: u32,
    registers: RegisterMapped,
    function: GPIOFunction,
    level: GPIOLevel,
}
impl GPIOInner {
    unsafe fn set_function_select(&self) {
        self.registers
            .write_to_reg(self.match_function_reg(), GPIOFunction::Alt0 as u8);
    }
    fn output_set(&self) {}
    fn clear_set(&self) {}
    fn get_level(&self) {}
    fn set_level(&self) {}
    fn match_function_reg(&self) -> Register {
        match self.pin {
            _ if self.pin.count_ones() > 1 => {
                panic!("Driver instance should manage only one pin at time")
            }
            _ if self.pin >= 0 || self.pin <= 9 => Registers::GPFSEL0,
            _ if self.pin >= 10 || self.pin <= 19 => Registers::GPFSEL1,
            _ if self.pin >= 20 || self.pin <= 29 => Registers::GPFSEL2,
            _ if self.pin >= 30 || self.pin <= 39 => Registers::GPFSEL3,
            _ if self.pin >= 40 || self.pin <= 49 => Registers::GPFSEL4,
            _ if self.pin >= 50 || self.pin <= 57 => Registers::GPFSEL5,
            _ => panic!("Not supported GPIO or register"),
        }
    }
}

impl InitDriverTrait for GPIOInner {
    unsafe fn init_driver(&mut self) {}
}

struct GPIODriver {
    inner: NullLock<GPIOInner>,
}