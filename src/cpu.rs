#[path = "_arch/aarch64/cpu.rs"]
pub mod arch_cpu;

pub mod boot;

pub use arch_cpu::wait_forever;
