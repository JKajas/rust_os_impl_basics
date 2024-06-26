use core::ops::{Drop, RangeBounds};

use crate::bsp::common::{MIMODerefWrapper, Permission, Register, RegisterInterface};
use crate::registers;
use crate::synchronization::interface::Mutex;
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
#[derive(Clone, Copy)]
pub enum GPIOFunction {
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
enum EventState {
    NoEvent,
    EventOccured,
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
    const unsafe fn new(pin: u32, function: GPIOFunction) -> GPIOInner {
        Self {
            pin,
            registers: RegisterMapped::new(0x0_FE20_000),
            function,
            level: GPIOLevel::Low,
        }
    }

    unsafe fn set_function_select(&self) {
        let offset = self.pin * 3;
        let state = self
            .registers
            .read_reg::<u32>(self.match_function_reg())
            .unwrap();
        self.registers.write_to_reg(
            self.match_function_reg(),
            state | ((self.function as u32) << offset),
        );
    }
    unsafe fn set_output(&self) {
        if self.pin < 32 {
            self.registers
                .write_to_reg(self.match_output_reg(), 1 << self.pin);
        } else {
            self.registers
                .write_to_reg(self.match_output_reg(), 1 << (self.pin - 32));
        }
    }
    unsafe fn clear_output(&self) {
        if self.pin < 32 {
            self.registers
                .write_to_reg(self.match_clear_reg(), 1 << self.pin);
        } else {
            self.registers
                .write_to_reg(self.match_clear_reg(), 1 << (self.pin - 32));
        }
    }
    unsafe fn get_level(&mut self) {
        let state = self
            .registers
            .read_reg::<u32>(self.match_level_reg())
            .unwrap();
        if self.pin < 32 && (state & (1 << self.pin) == 1 << self.pin) {
            self.level = GPIOLevel::High;
            return;
        }
        if self.pin < 32 {
            self.level = GPIOLevel::Low;
            return;
        }
        if self.pin > 32 && (state & (1 << self.pin) == 1 << (self.pin - 32)) {
            self.level = GPIOLevel::High;
            return;
        }
        if self.pin > 32 {
            self.level = GPIOLevel::Low;
            return;
        }
        panic!("No pin detected")
    }
    unsafe fn check_if_event_occured(&self) -> EventState {
        let state = self
            .registers
            .read_reg::<u32>(self.match_event_detect_register())
            .unwrap();
        if self.pin < 32 && (state & (1 << self.pin) == 1 << self.pin) {
            return EventState::EventOccured;
        }
        if self.pin < 32 {
            return EventState::NoEvent;
        }
        if self.pin > 32 && (state == 1 << (self.pin - 32)) {
            return EventState::EventOccured;
        }
        if self.pin > 32 {
            return EventState::NoEvent;
        }
        panic!("No pin detected")
    }

    fn match_function_reg(&self) -> Register {
        match self.pin {
            _ if self.pin >= 0 || self.pin <= 9 => Registers::GPFSEL0,
            _ if self.pin >= 10 || self.pin <= 19 => Registers::GPFSEL1,
            _ if self.pin >= 20 || self.pin <= 29 => Registers::GPFSEL2,
            _ if self.pin >= 30 || self.pin <= 39 => Registers::GPFSEL3,
            _ if self.pin >= 40 || self.pin <= 49 => Registers::GPFSEL4,
            _ if self.pin >= 50 || self.pin <= 57 => Registers::GPFSEL5,
            _ => panic!("Not supported GPIO or register"),
        }
    }
    fn match_output_reg(&self) -> Register {
        let first_segment = 0..31;
        let second_segment = 31..57;
        match self.pin {
            _ if first_segment.contains(&self.pin) => Registers::GPSET0,
            _ if second_segment.contains(&self.pin) => Registers::GPSET1,
            _ => panic!("Not supported GPIO or register"),
        }
    }
    fn match_clear_reg(&self) -> Register {
        let first_segment = 0..31;
        let second_segment = 31..57;
        match self.pin {
            _ if first_segment.contains(&self.pin) => Registers::GPCLR0,
            _ if second_segment.contains(&self.pin) => Registers::GPCLR1,
            _ => panic!("Not supported GPIO or register"),
        }
    }
    fn match_level_reg(&self) -> Register {
        let first_segment = 0..31;
        let second_segment = 31..57;
        match self.pin {
            _ if first_segment.contains(&self.pin) => Registers::GPLEV0,
            _ if second_segment.contains(&self.pin) => Registers::GPLEV1,
            _ => panic!("Not supported GPIO or register"),
        }
    }
    fn match_event_detect_register(&self) -> Register {
        let first_segment = 0..31;
        let second_segment = 31..57;
        match self.pin {
            _ if first_segment.contains(&self.pin) => Registers::GPEDS0,
            _ if second_segment.contains(&self.pin) => Registers::GPEDS1,
            _ => panic!("Not supported GPIO or register"),
        }
    }
}

impl InitDriverTrait for GPIOInner {
    unsafe fn init_driver(&mut self) {
        self.set_output();
        self.set_function_select();
        self.get_level();
        crate::println!("GPIO {} initialized!", self.pin);
    }
    unsafe fn clear_driver(&mut self) {
        self.clear_output();
        crate::println!("GPIO {} cleared!", self.pin)
    }
}

pub struct GPIODriver {
    inner: NullLock<GPIOInner>,
}
impl GPIODriver {
    pub const unsafe fn new(pin: u32, function: GPIOFunction) -> GPIODriver {
        if pin > 57 {
            panic!("No supported pin")
        }
        Self {
            inner: NullLock::new(GPIOInner::new(pin, function)),
        }
    }
    pub unsafe fn init(&self) {
        self.inner.lock(|driver| driver.init_driver());
    }
}
impl Drop for GPIODriver {
    fn drop(&mut self) {
        unsafe { self.inner.lock(|driver| driver.clear_driver()) }
    }
}
