use core::ptr::{read_volatile, write, write_volatile};
use core::{marker::PhantomData, ops};
pub struct MIMODerefWrapper<T> {
    start_addr: usize,
    phantom_data: PhantomData<fn() -> T>,
}

impl<T> MIMODerefWrapper<T> {
    pub const unsafe fn new(start_addr: usize) -> Self {
        Self {
            start_addr,
            phantom_data: PhantomData,
        }
    }
}
impl<T> ops::Deref for MIMODerefWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.start_addr as *const Self::Target) }
    }
}
impl<T> ops::DerefMut for MIMODerefWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.start_addr as *mut Self::Target) }
    }
}

pub enum Permission {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}
pub struct Register {
    pub offset: u8,
    pub permission: Permission,
}
pub trait RegisterInterface {
    unsafe fn write_to_reg<T>(&self, register: Register, data: T) -> Result<(), ()> {
        let instruction = {
            let register_address: usize =
                self as *const Self as *const usize as usize + register.offset as usize;
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
            let register_address: usize =
                self as *const Self as *const usize as usize + register.offset as usize;
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

#[macro_export]
macro_rules! registers {
    ($((REGISTER_NAME($register_name:ident), OFFSET($register_offset:expr), PERM($permission:expr))),+) => {
        #[allow(non_snake_case)]
        struct Registers{}
        impl Registers{
            $(const $register_name: Register = Register{offset: $register_offset, permission: $permission};)+
        }
    }
}
