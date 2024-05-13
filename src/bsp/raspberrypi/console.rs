use crate::bsp::device_driver::bcm::Uart;
use crate::{console, synchronization, synchronization::NullLock};
use core::fmt;

struct QEMUOutputInner {
    chars_written: usize,
}

struct QEMUOutput {
    inner: NullLock<QEMUOutputInner>,
}
static QEMU_OUTPUT: QEMUOutput = QEMUOutput::new();
static mut UART_CONSOLE: Option<&mut Uart> = None;

impl QEMUOutputInner {
    const fn new() -> Self {
        Self { chars_written: 0 }
    }
    fn write_char(&mut self, c: char) {
        unsafe { core::ptr::write_volatile(0x0_FE20_1000 as *mut u8, c as u8) };
        self.chars_written += 1
    }
}

impl fmt::Write for QEMUOutputInner {
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

impl QEMUOutput {
    const fn new() -> Self {
        QEMUOutput {
            inner: NullLock::new(QEMUOutputInner::new()),
        }
    }
}
pub fn console() -> &'static dyn console::interface::All {
    unsafe {
        if let Some(uart) = &UART_CONSOLE {
            return uart;
        }
        panic!("No initialized uart console!")
    }
}
use synchronization::interface::Mutex;

impl console::interface::Write for QEMUOutput {
    fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
        self.inner.lock(|inner| fmt::Write::write_fmt(inner, args))
    }
}
impl console::interface::Statistics for QEMUOutput {
    fn chars_written(&self) -> usize {
        self.inner.lock(|inner| inner.chars_written)
    }
}
impl console::interface::All for QEMUOutput {}

//_____________________________________________________________
//
//  OUTSIDE CONSOLE
//_________________________________________
//
pub fn register_console(driver: &'static mut Uart) {
    unsafe {
        UART_CONSOLE = Some(driver);
    }
}
