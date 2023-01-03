use std::{
    alloc::{self, Layout},
    ptr::{self, NonNull},
};

pub struct Hec<T> {
    ptr: NonNull<T>,
    count: usize,
    capacity: usize,
}

impl<T> Hec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            count: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.count + 1 > self.capacity {
            self.grow();
        }
        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.count), item);
        }
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.count))) }
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.capacity == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 2 * self.capacity;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        let new_ptr = if self.capacity == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.capacity = new_cap;
    }
}

impl<T> Drop for Hec<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            while let Some(_) = self.pop() {}
            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}
