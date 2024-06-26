pub mod bcm2711_gpio;
pub mod bcm2711_i2c;
pub mod bcm2711_irq;
pub mod bcm2711_uart;

use crate::bsp::{
    bcm::bcm2711_gpio::{GPIODriver, GPIOFunction, PullResistor},
    console::register_console,
};
pub use bcm2711_i2c::*;
pub use bcm2711_uart::*;

use crate::synchronization::interface::Mutex;
static mut UART_MANAGER: DriverManager<Uart> = DriverManager(None);
static mut I2C_MANAGER: DriverManager<I2C> = DriverManager(None);
pub unsafe fn init_drivers() {
    let mut GPIO14: GPIODriver =
        unsafe { GPIODriver::new(14, GPIOFunction::Alt0, PullResistor::Up) };
    let mut GPIO15: GPIODriver =
        unsafe { GPIODriver::new(15, GPIOFunction::Alt0, PullResistor::Up) };
    GPIO14.init();
    GPIO15.init();
    let uart_manager = uart_manager();
    // UART SECTION
    static mut UART: Uart = unsafe {
        Uart::new(
            0x0_FE20_1000,
            ParityBit::None,
            WordLength::Bit8,
            StopBits::One,
            9600,
        )
    };
    uart_manager.register_driver(&mut UART);
    register_console(&mut UART);
    uart_manager.init_drivers();
    // GPIO SECTION
    let mut GPIO2: GPIODriver = unsafe { GPIODriver::new(2, GPIOFunction::Alt0, PullResistor::Up) };
    GPIO2.init();
    let mut GPIO3: GPIODriver = unsafe { GPIODriver::new(3, GPIOFunction::Alt0, PullResistor::Up) };
    GPIO3.init();
    // I2C Section
    let i2c_manager = i2c_manager();
    static mut I2C: I2C = I2C::new(0x0_FE80_4000, 100_000, 3);
    i2c_manager.register_driver(&mut I2C);
    i2c_manager.init_drivers();
    crate::println!("Drivers initialized successfully!\n");
}

pub trait InitDriverTrait {
    unsafe fn init_driver(&mut self);
    unsafe fn clear_driver(&mut self);
}
pub trait MutexControll
where
    <<Self as MutexControll>::M as Mutex>::Data: InitDriverTrait,
{
    type M: Mutex;
    unsafe fn get_inner(&mut self) -> &Self::M;
}
struct DriverManager<T>(pub Option<&'static mut T>)
where
    T: MutexControll + 'static,
    <<T as MutexControll>::M as Mutex>::Data: InitDriverTrait;

impl<T> DriverManager<T>
where
    T: MutexControll + 'static,
    <<T as MutexControll>::M as Mutex>::Data: InitDriverTrait,
{
    fn register_driver(&mut self, driver: &'static mut T) {
        self.0 = Some(driver);
    }

    unsafe fn init_drivers(&mut self) {
        if let Some(mutex) = &mut self.0 {
            mutex.get_inner().lock(|inner| inner.init_driver())
        };
    }
}

pub fn uart_manager() -> &'static mut DriverManager<Uart> {
    unsafe { &mut UART_MANAGER }
}
pub fn i2c_manager() -> &'static mut DriverManager<I2C> {
    unsafe { &mut I2C_MANAGER }
}
