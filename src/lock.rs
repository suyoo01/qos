use core::cell::Cell;

pub struct NullLock <T>{
    data: Cell<T>
}

unsafe impl<T> core::marker::Sync for NullLock<T> {}

impl<T> NullLock<T> {
    pub fn new(data: T) -> Self{
        NullLock { data:Cell::new(data) }
    }
    pub fn lock(&self) -> &mut T {
        unsafe {
            &mut *self.data.as_ptr() 
        }
    }
}