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
