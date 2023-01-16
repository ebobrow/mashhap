use std::{
    alloc::{self, Layout},
    cell::RefCell,
    env::current_exe,
    ptr::{self, NonNull},
    rc::Rc,
};

use crate::hash::HashFn;

// TODO: tweak
const MAX_LOAD: f64 = 1.0;

pub struct MashHap<T> {
    entries: NonNull<Entry<T>>,
    count: usize,
    capacity: usize,
    hash: HashFn,

    // TODO: best way to store this?
    /// Number of times we had to move past an occupied space
    overlaps: Rc<RefCell<usize>>,
}

type Entry<T> = Option<Data<T>>;

#[derive(Debug, Clone)]
struct Data<T> {
    name: String,
    value: T,
    next: Option<Box<Data<T>>>,
}

impl<T> MashHap<T>
where
    T: Clone,
{
    pub fn new(hash: HashFn) -> Self {
        MashHap::with_capacity(8, hash)
    }

    pub fn with_capacity(capacity: usize, hash: HashFn) -> Self {
        let layout = Layout::array::<Entry<T>>(capacity).unwrap();
        let ptr = unsafe { alloc::alloc(layout) };
        let entries = match NonNull::new(ptr as *mut Entry<T>) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        };

        for i in 0..capacity {
            unsafe { ptr::write(entries.as_ptr().add(i), None) };
        }

        Self {
            entries,
            count: 0,
            capacity,
            hash,
            overlaps: Default::default(),
        }
    }

    /// Sets key, overwriting previous entry if there was one. Returns true if previous entry
    /// did not exist.
    pub fn set(&mut self, key: impl ToString, value: T) -> bool {
        let key = key.to_string();
        // TODO: do we resize here or wait until we see if we're replacing a tombstone, in which
        // case we don't need to resize but the insert takes longer because it has to find the
        // index twice?
        if (self.count + 1) as f64 > self.capacity as f64 * MAX_LOAD {
            self.resize();
        }
        let index = (self.hash)(&key) as usize % self.capacity;
        if let Some(entry) = &self.slice()[index] {
            // TODO: not clone
            let mut new_entry = entry.clone();
            if new_entry.name == key {
                new_entry.value = value;
                self.write(index, Some(new_entry));
                return false;
            }
            if new_entry.next.is_none() {
                new_entry.next = Some(Box::new(Data {
                    name: key,
                    value,
                    next: None,
                }));
                self.write(index, Some(new_entry));
                return true;
            }
            let mut ptr = &mut new_entry;
            while let Some(next) = &mut ptr.next {
                if next.name == key {
                    next.value = value;
                    self.write(index, Some(new_entry));
                    return false;
                } else {
                    if next.next.is_none() {
                        next.next = Some(Box::new(Data {
                            name: key,
                            value,
                            next: None,
                        }));
                        self.write(index, Some(new_entry));
                        return true;
                    }
                    ptr = next;
                }
            }
        }
        self.write(
            index,
            Some(Data {
                name: key,
                value,
                next: None,
            }),
        );
        true
    }

    pub fn get(&mut self, key: impl ToString) -> Option<&T> {
        if self.capacity == 0 {
            return None;
        }
        let key = key.to_string();
        let index = (self.hash)(&key) as usize % self.capacity;
        if let Some(entry) = &self.slice()[index] {
            if entry.name == key {
                return Some(&entry.value);
            }
            let mut ptr = entry;
            while let Some(next) = &ptr.next {
                if next.name == key {
                    return Some(&next.value);
                } else {
                    ptr = &*next;
                }
            }
        }
        None
    }

    /// Deletes entry at `key`. Returns true if entry existed.
    pub fn delete(&mut self, key: impl ToString) -> bool {
        if self.capacity == 0 {
            return false;
        }
        let key = key.to_string();
        let index = (self.hash)(&key) as usize % self.capacity;
        if let Some(entry) = &self.slice()[index] {
            // TODO: not clone
            let mut new_entry = entry.clone();
            if new_entry.name == key {
                self.write(index, new_entry.next.map(|n| *n));
                return true;
            }
            let mut ptr = entry;
            let mut new_ptr = &mut new_entry;
            while let Some(next) = &ptr.next {
                if next.name == key {
                    new_ptr.next = next.next.clone();
                    self.write(index, Some(new_entry));
                    return true;
                } else {
                    ptr = &*next;
                }
            }
        }
        false
    }

    pub fn overlaps(&self) -> usize {
        self.overlaps.borrow().clone()
    }

    fn read(&self, n: usize) -> Entry<T> {
        if self.capacity == 0 || n >= self.capacity {
            None
        } else {
            unsafe { ptr::read(self.entries.as_ptr().add(n)) }
        }
    }

    fn write(&self, n: usize, entry: Entry<T>) {
        if self.capacity == 0 {
            panic!("attempt to write to empty array")
        } else if n >= self.capacity {
            panic!("index out of bounds")
        } else {
            unsafe { ptr::write(self.entries.as_ptr().add(n), entry) }
        }
    }

    fn slice(&self) -> &[Entry<T>] {
        unsafe { std::slice::from_raw_parts(self.entries.as_ptr(), self.capacity) }
    }

    fn inc_overlaps(&self) {
        let mut overlaps = self.overlaps.borrow_mut();
        *overlaps += 1;
    }

    fn resize(&mut self) {
        let (new_cap, new_layout) = if self.capacity == 0 {
            (1, Layout::array::<Entry<T>>(1).unwrap())
        } else {
            let new_cap = 2 * self.capacity;
            let new_layout = Layout::array::<Entry<T>>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        let mut items_to_transfer = Vec::new();
        for i in 0..self.capacity {
            if let Some(entry) = &self.read(i) {
                items_to_transfer.push((entry.name.clone(), entry.value.clone()));
                while let Some(entry) = &entry.next {
                    items_to_transfer.push((entry.name.clone(), entry.value.clone()));
                }
            }
        }

        let new_ptr = if self.capacity == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<Entry<T>>(self.capacity).unwrap();
            let old_ptr = self.entries.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        let new_entries = match NonNull::new(new_ptr as *mut Entry<T>) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };

        for i in 0..new_cap {
            unsafe { ptr::write(new_entries.as_ptr().add(i), None) };
        }

        self.count = items_to_transfer.len();
        self.entries = new_entries;
        self.capacity = new_cap;
        for (k, v) in items_to_transfer {
            self.set(k, v);
        }
    }
}

impl<T> Drop for MashHap<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            for i in 0..self.capacity {
                unsafe {
                    ptr::read(self.entries.as_ptr().add(i));
                }
            }
            let layout = Layout::array::<Entry<T>>(self.capacity).unwrap();
            unsafe {
                alloc::dealloc(self.entries.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hash::fnv_1a;

    use super::*;

    #[test]
    fn basics() {
        let mut map = MashHap::new(fnv_1a);
        assert_eq!(map.get("nonexistant"), None);
        assert_eq!(map.set("A", 10), true);
        assert_eq!(map.set("B", 24), true);
        assert_eq!(map.get("A"), Some(&10));
        assert_eq!(map.set("A", 11), false);
        assert_eq!(map.get("A"), Some(&11));
        assert_eq!(map.get("B"), Some(&24));
        assert_eq!(map.delete("A"), true);
        assert_eq!(map.delete("A"), false);
    }

    // #[test]
    // fn resizing() {
    //     let mut map = MashHap::with_capacity(2, fnv_1a);
    //     assert_eq!(map.capacity, 2);
    //     map.set("A", 1);
    //     assert_eq!(map.capacity, 1);
    //     map.set("B", 1);
    //     assert_eq!(map.capacity, 1);
    //     map.set("C", 1);
    //     assert_eq!(map.capacity, 4);
    //     map.set("D", 1);
    //     assert_eq!(map.capacity, 8);
    //     map.set("E", 1);
    //     assert_eq!(map.capacity, 8);
    //     map.set("F", 1);
    //     assert_eq!(map.capacity, 8);
    //     map.set("G", 1);
    //     assert_eq!(map.capacity, 16);
    // }
}
