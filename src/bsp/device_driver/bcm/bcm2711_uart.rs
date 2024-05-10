use crate::{bsp::common::MIMODerefWrapper, synchronization::NullLock};

use core::{
    arch::asm,
    fmt::{self, Write},
    ptr::{read_volatile, write_volatile},
};
use fdt::Fdt;

static mut UART_CLOCK: u32 = 48_000_000;

// Required ftd/dtb file with overlay assigning clock rate for uart clock
pub unsafe fn read_uart_clock() -> &'static u32 {
    let dtb_pointer: *const u8 = 0x0 as *const u8;
    asm!("ldr x0, adr_dtb
            str x0, [{}]", in(reg) &dtb_pointer);
    let dtb: Fdt = Fdt::from_ptr(dtb_pointer).expect("No dtb file");
    if let Some(clk_rate_prop) = dtb
        .find_phandle(0x43)
        .unwrap_or_else(|| panic!("No UART config"))
        .property("assigned-clocks-rates")
    {
        UART_CLOCK = clk_rate_prop
            .as_usize()
            .expect("Error while parsing clock value!") as u32;
        &UART_CLOCK
    } else {
        crate::println!("Clock value is not appeared in dtb file. Default value loaded");
        &UART_CLOCK
    }
}

enum Permission {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}
pub enum ParityBit {
    None,
    Odd,
    Even,
}
pub enum WordLength {
    Bit8,
    Bit7,
    Bit6,
    Bit5,
}

pub enum StopBits {
    One,
    Two,
}
struct Register {
    offset: u8,
    permission: Permission,
}

macro_rules! registers {
    ($((REGISTER_NAME($register_name:ident), OFFSET($register_offset:expr), PERM($permission:expr))),+) => {
        #[allow(non_snake_case)]
        struct Registers{}
        impl Registers{

            $(const $register_name: Register = Register{offset: $register_offset, permission: $permission};)+
        }
    }
}
registers!(
    (REGISTER_NAME(DR), OFFSET(0x00), PERM(Permission::ReadWrite)), // Data register
    (
        REGISTER_NAME(RSRECR),
        OFFSET(0x04),
        PERM(Permission::ReadWrite)
    ),
    (REGISTER_NAME(FR), OFFSET(0x18), PERM(Permission::ReadWrite)), // Flag register
    (
        REGISTER_NAME(ILPR),
        OFFSET(0x20),
        PERM(Permission::ReadWrite)
    ), // Not in use
    (
        REGISTER_NAME(IBRD),
        OFFSET(0x24),
        PERM(Permission::ReadWrite)
    ), // Integer Baud Rate divisor
    (
        REGISTER_NAME(FBRD),
        OFFSET(0x28),
        PERM(Permission::ReadWrite)
    ), // Fractional Baud Rate divisor
    (
        REGISTER_NAME(LCRH),
        OFFSET(0x2c),
        PERM(Permission::ReadWrite)
    ), // Line Control register
    (REGISTER_NAME(CR), OFFSET(0x30), PERM(Permission::ReadWrite)), // Control register
    (
        REGISTER_NAME(IFLS),
        OFFSET(0x34),
        PERM(Permission::ReadWrite)
    ), // Interrup FIFO Level Select Register
    (
        REGISTER_NAME(IMSC),
        OFFSET(0x38),
        PERM(Permission::ReadWrite)
    ), // Interrup Mask Set Clear Register
    (
        REGISTER_NAME(RIS),
        OFFSET(0x3c),
        PERM(Permission::ReadWrite)
    ), // Raw Interrupt Status Register
    (
        REGISTER_NAME(MIS),
        OFFSET(0x40),
        PERM(Permission::ReadWrite)
    ), // Masked Interrupt Status Register
    (
        REGISTER_NAME(ICR),
        OFFSET(0x44),
        PERM(Permission::ReadWrite)
    ), // Interrupt Clear Register
    (
        REGISTER_NAME(DMACR),
        OFFSET(0x48),
        PERM(Permission::ReadWrite)
    ), // DMA Control Register
    (
        REGISTER_NAME(ITCR),
        OFFSET(0x80),
        PERM(Permission::ReadWrite)
    ), // Test Control Register
    (
        REGISTER_NAME(ITIP),
        OFFSET(0x84),
        PERM(Permission::ReadWrite)
    ), // Integration test input reg
    (
        REGISTER_NAME(ITOP),
        OFFSET(0x88),
        PERM(Permission::ReadWrite)
    ), // Integration test output reg
    (
        REGISTER_NAME(TDR),
        OFFSET(0x8c),
        PERM(Permission::ReadWrite)
    )  // Test Data reg
);

impl Registers {
    unsafe fn write_to_reg<T>(&self, register: Register, data: T) -> Result<(), ()> {
        let instruction = {
            let register_address: isize =
                self as *const Registers as isize + register.offset as isize;
            write_volatile(register_address as *mut T, data);
            Ok(())
        };
        match register.permission {
            Permission::ReadOnly => panic!("Register is write only"),
            Permission::WriteOnly => instruction,
            Permission::ReadWrite => instruction,
        }
    }

    unsafe fn read_reg<T>(&self, register: Register) -> Result<T, ()> {
        let instruction = {
            let register_address: isize =
                self as *const Registers as isize + register.offset as isize;
            let result = read_volatile(register_address as *mut T);
            Ok(result)
        };
        match register.permission {
            Permission::ReadOnly => instruction,
            Permission::WriteOnly => panic!("Register is read only"),
            Permission::ReadWrite => instruction,
        }
    }
}
type RegisterMapped = MIMODerefWrapper<Registers>;
pub struct UartInner {
    chars_written: usize,
    chars_read: usize,
    registers: RegisterMapped,
    parity: ParityBit,
    word_length: WordLength,
    stop_bit: StopBits,
    baud_rate: u32,
}
pub struct Uart {
    pub inner: NullLock<UartInner>,
}

impl MutexControll for Uart {
    type M = NullLock<UartInner>;
    unsafe fn get_inner(&mut self) -> &Self::M {
        &self.inner
    }
}
impl Write for UartInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            if c == 0x0a as char {
                self.write_char(0x0a as char);
                self.write_char(0x0d as char);
            }
            self.write_char(c)
        }
        Ok(())
    }
}
use crate::synchronization::interface::Mutex;

use super::{InitDriverTrait, MutexControll};
impl crate::console::interface::Write for &mut Uart {
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        self.inner.lock(|inner| fmt::Write::write_fmt(inner, args))
    }
}
impl crate::console::interface::Statistics for &mut Uart {
    fn chars_written(&self) -> usize {
        self.inner.lock(|inner| inner.chars_written)
    }
}

impl crate::console::interface::All for &mut Uart {}

impl UartInner {
    const unsafe fn new(
        start_addr: usize,
        parity: ParityBit,
        word_length: WordLength,
        stop_bit: StopBits,
        baud_rate: u32,
    ) -> Self {
        Self {
            chars_read: 0,
            chars_written: 0,
            registers: RegisterMapped::new(start_addr),
            parity,
            word_length,
            stop_bit,
            baud_rate,
        }
    }
    pub unsafe fn set_parity(&mut self, parity: Option<ParityBit>) {
        // If settings are made by hand
        if let Some(parity_present) = parity {
            self.parity = parity_present;
        }
        let state = self.registers.read_reg::<u32>(Registers::LCRH).unwrap();
        let cleared_state = state & !(0b11) << 1;
        let _ = match self.parity {
            ParityBit::None => self
                .registers
                .write_to_reg(Registers::LCRH, cleared_state | 0b0 << 1),
            ParityBit::Even => self
                .registers
                .write_to_reg(Registers::LCRH, cleared_state | 0b11 << 1),
            ParityBit::Odd => self
                .registers
                .write_to_reg(Registers::LCRH, cleared_state | 0b01 << 1),
        };
    }
    pub unsafe fn set_length(&mut self, length: Option<WordLength>) {
        if let Some(length_present) = length {
            self.word_length = length_present;
        }
        let state = self.registers.read_reg::<u32>(Registers::LCRH).unwrap();
        let cleared_state = state & !(0b11) << 5;
        let _ = match self.word_length {
            WordLength::Bit5 => self
                .registers
                .write_to_reg(Registers::LCRH, cleared_state | 0b00 << 5),
            WordLength::Bit6 => self
                .registers
                .write_to_reg(Registers::LCRH, cleared_state | 0b01 << 5),
            WordLength::Bit7 => self
                .registers
                .write_to_reg(Registers::LCRH, cleared_state | 0b10 << 5),
            WordLength::Bit8 => self
                .registers
                .write_to_reg(Registers::LCRH, cleared_state | 0b11 << 5),
        };
    }

    pub unsafe fn set_stop_bits(&mut self, stop_bit: Option<StopBits>) {
        if let Some(stop_bit_present) = stop_bit {
            self.stop_bit = stop_bit_present;
        }
        let state = self.registers.read_reg::<u32>(Registers::LCRH).unwrap();
        let cleared_state = state & !(1 << 3);
        let _ = match self.stop_bit {
            StopBits::One => self
                .registers
                .write_to_reg(Registers::LCRH, cleared_state | 0b0 << 3),
            StopBits::Two => self
                .registers
                .write_to_reg(Registers::LCRH, cleared_state | 0b1 << 3),
        };
    }
    pub unsafe fn enable_fifo(&self) {
        let state = self.registers.read_reg::<u32>(Registers::LCRH).unwrap();
        let cleared_state = state & !(1 << 4);
        self.registers
            .write_to_reg(Registers::LCRH, cleared_state | 0b1 << 4);
    }
    pub unsafe fn disable_fifo(&self) {
        let state = self.registers.read_reg::<u32>(Registers::LCRH).unwrap();
        let cleared_state = state & !(1 << 4);
        self.registers.write_to_reg(Registers::LCRH, cleared_state);
    }
    pub unsafe fn flush(&self) {
        let flag_register_state = self.registers.read_reg::<u32>(Registers::FR).unwrap();
        if flag_register_state & (1 << 7) == flag_register_state & !(1 << 7) {
            //Default FIFO interrupts processor while exceeds 1/2 capacity
            // Transmit FIFO is not empty
            self.disable_fifo();
            self.enable_fifo();
        } else if (flag_register_state & 1 << 4) == flag_register_state & !(1 << 4) {
            // Receive FIFO is not empty
            for i in 0..16 + 1 {
                //Default FIFO interrupts processor while exceeds 1/2 capacity
                self.registers.read_reg::<u32>(Registers::DR);
            }
        }
    }
    pub unsafe fn calculate_baud_rate(&self, baud_rate: u32) -> (u16, u8) {
        let permissible_error_value: f32 = 1.0 / 64.0 * 100.0;

        let baud_rate_divider_base: f32 = UART_CLOCK as f32 / (16.0 * baud_rate as f32);
        let integer_part: u16 = baud_rate_divider_base as u16;

        let fractional: f32 = baud_rate_divider_base - integer_part as f32;
        let fractional_part: u8 = ((fractional * 64.0) + 0.5) as u8;

        let generated_baud_rate: f32 =
            UART_CLOCK as f32 / (16.0 * (integer_part as f32 + (fractional_part as f32 / 64.0)));
        let error = (generated_baud_rate - baud_rate as f32) / baud_rate as f32 * 100.0;
        if error > permissible_error_value {
            panic!("Baud rate error too high!");
        }
        (integer_part, fractional_part)
    }
    pub unsafe fn set_baud_rate(&self, i_part: u16, f_part: u8) {
        if f_part > !(0 << 6) {
            panic!("Fractional part should be max 6 bits");
        }
        self.registers.write_to_reg(Registers::IBRD, i_part);
        self.registers.write_to_reg(Registers::FBRD, f_part);
    }
    pub unsafe fn read_data(&self, data: &mut [u8]) {}
    pub unsafe fn write_data(&mut self, data: &[u8]) {
        self.chars_written = 0;
        for byte in data {
            self.registers.write_to_reg(Registers::DR, byte);
            self.chars_written += 1
        }
    }
    pub fn write_char(&mut self, c: char) {
        unsafe { self.registers.write_to_reg(Registers::DR, c).unwrap() }
        self.chars_written += 1;
    }
}

impl InitDriverTrait for UartInner {
    unsafe fn init_driver(&mut self) {
        // Read uart clock from ftd/dtb file
        read_uart_clock();

        //enable UART
        self.registers.write_to_reg(Registers::CR, 0b11 << 8 | 0b1);
        // Set baud rate
        let (integer_part, fractional_part) = self.calculate_baud_rate(self.baud_rate);
        self.set_baud_rate(integer_part, fractional_part);

        // Flush FIFO
        self.flush();
        //Set parity bit
        self.set_parity(None);

        //Set Word Length
        self.set_length(None);
        // Set Stop bit
        self.set_stop_bits(None);
    }
}
impl Uart {
    pub const unsafe fn new(
        start_addr: usize,
        parity: ParityBit,
        word_length: WordLength,
        stop_bit: StopBits,
        baud_rate: u32,
    ) -> Self {
        Self {
            inner: NullLock::new(UartInner::new(
                start_addr,
                parity,
                word_length,
                stop_bit,
                baud_rate,
            )),
        }
    }
}

#[allow(non_camel_case_types)]
trait IRQ_getter {
    unsafe fn get_irq_code(&self) -> u16;
}
#[repr(u16)]
enum IRQs {
    OVERRUN = 1 << 10,
    BREAK = 1 << 9,
    PARITY = 1 << 8,
    FRAMING = 1 << 7,
    TIMEOUT = 1 << 6,
    TX = 1 << 5,
    RX = 1 << 4,
    DSR = 1 << 3,
    DCD = 1 << 2,
    CTS = 1 << 1,
    RI = 1,
}
#[allow(non_camel_case_types)]
#[repr(u16)]
enum FIFO_IRQs {
    RX_1_8 = 0b000111,
    RX_1_4 = 0b001 << 3,
    RX_1_2 = 0b010 << 3,
    RX_3_4 = 0b011 << 3,
    RX_7_8 = 0b100 << 3,
    TX_1_8 = 0b000,
    TX_1_4 = 0b001,
    TX_1_2 = 0b010,
    TX_3_4 = 0b011,
    TX_7_8 = 0b100,
}
#[allow(non_camel_case_types)]
trait UART_IRQ_handler {
    const IRQ_ID: i8 = 57;
    const PACTL_CS: *const usize = 0xFE20_4E00 as *const usize;
    fn get_irq_interface_id(&self) -> usize;
    unsafe fn get_irq_code(&self) -> u16;
    unsafe fn get_fifo_irq_code(&self) -> u16;
    unsafe fn handle_irq(&self) -> () {
        let interruption = self.get_irq_code();
        match interruption {
            irq if interruption == IRQs::OVERRUN as u16 => {}
            irq if interruption == IRQs::BREAK as u16 => {}
            irq if interruption == IRQs::PARITY as u16 => {}
            irq if interruption == IRQs::FRAMING as u16 => {}
            irq if interruption == IRQs::TIMEOUT as u16 => {}
            irq if interruption == IRQs::TX as u16 => {}
            irq if interruption == IRQs::RX as u16 => {}
            irq if interruption == IRQs::DSR as u16 => {}
            irq if interruption == IRQs::DCD as u16 => {}
            irq if interruption == IRQs::CTS as u16 => {}
            irq if interruption == IRQs::RI as u16 => {}
            0 => {}
            _ => panic!("No handler"),
        }
    }
}
