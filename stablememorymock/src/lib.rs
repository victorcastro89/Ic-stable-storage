#![allow(dead_code, unused_imports)]
use std::cell::{Ref, RefCell};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::vec;
use std::{error, fmt, io};
static MAX_SPACE_BYTES: u64 = 8589934592; // 8GB
static MAX_PAGE_BYTE: u64 = 65536;
static RESERVED_SPACE: u64 = 320 * MAX_PAGE_BYTE; // 20.97 Mb reserved
static MAX_PAGES: u64 = MAX_SPACE_BYTES / MAX_PAGE_BYTE;

pub trait Memory {
    fn stable64_size(&self) -> u64; //WasmPages
    fn stable64_grow(&mut self, pages: u64) -> Result<u64, SmError>; //WasmPages
    fn stable64_read(&self, offset: u64, dst: &mut [u8]);
    fn stable64_write(&self, offset: u64, src: &[u8]);
}
#[derive(Debug)]
pub enum SmError {
    OutOfMemory,

    InvalidKey,
    /// Attempted to read more stable memory than had been allocated.
    OutOfBounds,
}

impl fmt::Display for SmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OutOfMemory => f.write_str("Out of memory"),
            Self::InvalidKey => f.write_str("Key Does Not Exist"),
            Self::OutOfBounds => f.write_str("Read exceeds allocated memory"),
        }
    }
}
impl error::Error for SmError {}

impl Memory for Storage {
    fn stable64_size(&self) -> u64 {
        MEMORY.with(|mem| {
            let mem = mem.borrow_mut();
            mem.len() as u64 / MAX_PAGE_BYTE as u64
        })
    }

    fn stable64_grow(&mut self, pages: u64) -> Result<u64, SmError> {
        MEMORY.with(|mem| {
            let mut mem = mem.borrow_mut();
            let current_pages = mem.len() as u64 / MAX_PAGE_BYTE;
            if current_pages + pages > MAX_PAGES {
                return Err(SmError::OutOfMemory);
            }

            *mem = [0].repeat(MAX_PAGE_BYTE as usize);
            self.allocated = self.allocated + MAX_PAGE_BYTE;
            Ok(current_pages)
        })
    }
    fn stable64_read(&self, offset: u64, buf: &mut [u8]) {
        let data = MEMORY.with(|mem| {
            let mem = mem.borrow();
            let buf_len = buf.len() as u64;
            let mem_len = mem.len() as u64;

            if offset + buf_len > mem_len {
                panic!(" Memory Reading error: {}", SmError::OutOfBounds);
            }

            mem[offset as usize..buf_len as usize].to_owned()
        });

        let mut i = 0;
        for val in data.iter() {
            buf[i] = *val;
            i = i + 1;
        }
    }
    fn stable64_write(&self, offset: u64, buf: &[u8]) {
        MEMORY.with(|mem| {
            let mut mem = mem.borrow_mut();
            let buf_len = buf.len() as u64;

            if offset + buf_len > MAX_SPACE_BYTES {
                panic!(" Memory Writing error: {}", SmError::OutOfMemory);
            }

            if offset + buf_len > self.allocated {
                panic!(" Memory Write error:: {}", SmError::OutOfBounds);
            }
            let mut i = 0;
            for val in buf.iter() {
                mem[offset as usize + i] = *val;
                i = i + 1;
            }
        });
    }
}
#[derive(Default)]
struct Storage {
    allocated: u64,
}
thread_local!(
    static MEMORY: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::new());
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grow_memory() {
        let mut s = Storage::default();

        assert_eq!(s.stable64_size(), 0);
        assert_eq!(s.stable64_grow(1).unwrap(), 0);
        assert_eq!(s.stable64_size(), 1);
    }
    #[test]
    #[should_panic]
    fn grow_memory_max_than_capacity() {
        let mut s = Storage::default();
        assert_eq!(
            s.stable64_grow((MAX_SPACE_BYTES / MAX_PAGE_BYTE) + 1)
                .unwrap(),
            0
        );
    }
    #[test]
    #[should_panic]
    fn write_memory_not_alocated() {
        let s = Storage::default();
        let mut write = [5].repeat(255 as usize);
        s.stable64_write(0, &mut write);
    }

    #[test]
    #[should_panic]
    fn write_memory_big_size() {
        let mut s = Storage::default();
        s.stable64_grow(1).unwrap();
        let mut write = [5].repeat(65537 as usize);
        s.stable64_write(0, &mut write);
    }

    #[test]
    fn write_memory_and_read() {
        let mut s = Storage::default();
        s.stable64_grow(1).unwrap();
        let mut write = [5].repeat(255 as usize);
        s.stable64_write(0, &mut write);

        let mut data = [0].repeat(255 as usize);
        s.stable64_read(0, &mut data);

        assert_eq!(write, data);
    }
}
