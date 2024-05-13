use crate::{cpu, println};
use core::panic::PanicInfo;

fn panic_prevent_reenter() {
    use core::sync::atomic::{AtomicBool, Ordering};
    #[cfg(not(target_arch = "aarch64"))]
    compile_error!("Add the target arch to above check if the following code is safe to use");
    static PANIC_IN_PROGRES: AtomicBool = AtomicBool::new(false);
    if !PANIC_IN_PROGRES.load(Ordering::Relaxed) {
        PANIC_IN_PROGRES.store(true, Ordering::Relaxed);
        return;
    }
    cpu::wait_forever();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic_prevent_reenter();
    let (location, line, column) = match info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        _ => ("???", 0, 0),
    };
    println!(
        "Kernel panic!\n\n Panic location:\n      Info: {:?} File {}, line {}, column {}\n\n",
        info.payload().downcast_ref::<&str>(),
        location,
        line,
        column
    );
    cpu::wait_forever()
}
