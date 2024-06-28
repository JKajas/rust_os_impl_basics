use crate::{
    bsp::common::{MIMODerefWrapper, Permission, Register, RegisterInterface},
    registers,
    synchronization::{interface::Mutex, NullLock},
};

use super::{InitDriverTrait, MutexControll};

const CORE_CLK: u32 = 150_000_000;

registers!(
    (REGISTER_NAME(C), OFFSET(0x00), PERM(Permission::ReadWrite)),
    (REGISTER_NAME(S), OFFSET(0x04), PERM(Permission::ReadWrite)),
    (
        REGISTER_NAME(DLEN),
        OFFSET(0x08),
        PERM(Permission::ReadWrite)
    ),
    (REGISTER_NAME(A), OFFSET(0x0c), PERM(Permission::ReadWrite)),
    (
        REGISTER_NAME(FIFO),
        OFFSET(0x10),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(DIV),
        OFFSET(0x14),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(DEL),
        OFFSET(0x18),
        PERM(Permission::ReadWrite)
    ),
    (
        REGISTER_NAME(CLKT),
        OFFSET(0x1c),
        PERM(Permission::ReadWrite)
    )
);
impl RegisterInterface for Registers {}
type RegisterMapped = MIMODerefWrapper<Registers>;

enum OperType {
    Read,
    Write,
}

pub struct I2CInner {
    registers: RegisterMapped,
    chars_written: usize,
    chars_read: usize,
    clock_rate: u32,
    timeout: u16,
    data_length: usize,
}

enum TransferType {
    Read,
    Write,
}
impl I2CInner {
    pub const fn new(start_addr: usize, clock_rate: u32, timeout: u16) -> Self {
        Self {
            registers: unsafe { RegisterMapped::new(start_addr) },
            chars_read: 0,
            chars_written: 0,
            timeout,
            clock_rate,
            data_length: 0,
        }
    }
    unsafe fn start_transfer(&self) {
        let state = self.registers.read_reg::<u32>(Registers::C).unwrap();
        if let 0 = state & (1 << 15) {
            panic!("I2C is not enabled to start new transaction")
        }
        self.registers
            .write_to_reg::<u32>(Registers::C, state | (1 << 7))
            .unwrap()
    }
    unsafe fn clear_fifo(&self) {
        let state = self.registers.read_reg::<u32>(Registers::C).unwrap();
        self.registers
            .write_to_reg::<u32>(Registers::C, state | (0b11 << 4))
            .unwrap();
    }
    unsafe fn read_fifo<const DATA_LENGTH: usize>(&self) -> [u8; DATA_LENGTH] {
        let mut data: [u8; DATA_LENGTH] = [0; DATA_LENGTH];
        for i in 0..self.data_length {
            data[i as usize] = self.registers.read_reg::<u8>(Registers::FIFO).unwrap();
        }
        data
    }
    unsafe fn set_transfer_type(&self, transfer_type: TransferType) {
        let state = self.registers.read_reg::<u32>(Registers::C).unwrap();
        match transfer_type {
            TransferType::Read => self
                .registers
                .write_to_reg::<u32>(Registers::C, state | 1)
                .unwrap(),
            TransferType::Write => self
                .registers
                .write_to_reg::<u32>(Registers::C, (state | 1) ^ 1)
                .unwrap(),
        }
    }
    unsafe fn set_timeout(&self) {
        self.registers
            .write_to_reg(Registers::CLKT, self.timeout as u32);
    }
    unsafe fn set_data_length(&self) {
        self.registers
            .write_to_reg(Registers::DLEN, self.data_length)
            .unwrap();
    }

    unsafe fn read_slave<const DATA_LENGTH: usize>(&mut self, slave_addr: u8) {
        if let 1 = slave_addr & 1 << 7 {
            panic!("I2C bus supports only 7 bits address")
        }
        self.data_length = DATA_LENGTH;
        self.set_data_length();
        self.clear_fifo();
        self.registers
            .write_to_reg(Registers::A, slave_addr as u32)
            .unwrap();
        self.set_transfer_type(TransferType::Read);
        self.start_transfer();
        self.read_fifo::<DATA_LENGTH>();
    }
    unsafe fn clear_status(&self) {
        self.registers.write_to_reg(Registers::S, 1 << 1).unwrap();
        self.registers.write_to_reg(Registers::S, 1 << 8).unwrap();
        self.registers.write_to_reg(Registers::S, 1 << 9).unwrap()
    }

    unsafe fn write_slave(&mut self, slave_addr: u8, data: &str) {
        if let 128 = slave_addr & 1 << 7 {
            panic!("I2C bus supports only 7 bits address")
        }
        self.set_transfer_type(TransferType::Write);
        self.registers
            .write_to_reg(Registers::A, slave_addr as u32)
            .unwrap();
        self.data_length = data.len();
        self.set_data_length();
        self.clear_fifo();
        for c in data.chars() {
            self.registers.write_to_reg(Registers::FIFO, c).unwrap()
        }
        self.clear_status();
        self.start_transfer();
    }

    unsafe fn set_clock_rate(&self) {
        let divisor = CORE_CLK / self.clock_rate;
        self.registers
            .write_to_reg(Registers::DIV, divisor)
            .unwrap();
    }
}
impl InitDriverTrait for I2CInner {
    unsafe fn init_driver(&mut self) {
        self.registers.write_to_reg(Registers::C, 1 << 15).unwrap();
        self.set_clock_rate();
        self.set_timeout();
        self.write_slave(0x7f, "I2C Init Done")
    }
    unsafe fn clear_driver(&mut self) {}
}
pub struct I2C {
    pub inner: NullLock<I2CInner>,
}

impl I2C {
    pub const fn new(start_addr: usize, clock_rate: u32, timeout: u16) -> Self {
        Self {
            inner: NullLock::new(I2CInner::new(start_addr, clock_rate, timeout)),
        }
    }
    pub unsafe fn init_driver(&self) {
        self.inner.lock(|i| i.init_driver())
    }
    pub unsafe fn write(&self, slave_addr: u8, data: &str) {
        self.inner.lock(|i| i.write_slave(slave_addr, data))
    }
}
impl MutexControll for I2C {
    type M = NullLock<I2CInner>;
    unsafe fn get_inner(&mut self) -> &Self::M {
        &self.inner
    }
}
