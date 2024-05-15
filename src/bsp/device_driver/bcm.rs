pub mod bcm2711_gpio;
pub mod bcm2711_irq;
pub mod bcm2711_uart;

use crate::bsp::console::register_console;
pub use bcm2711_uart::*;

use crate::synchronization::interface::Mutex;
static mut UART_MANAGER: DriverManager<Uart> = DriverManager(None);
pub unsafe fn init_drivers() {
    let uart_manager = uart_manager();
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
    //crate::println!("Console registered successfully!\n");
    //crate::println!("Starting driver initialization...\n");
    uart_manager.init_drivers();
    crate::println!("Drivers initialized successfully!\n");
}

pub trait InitDriverTrait {
    unsafe fn init_driver(&mut self) {}
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
