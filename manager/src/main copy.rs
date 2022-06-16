#![allow(dead_code, unused_imports)]
use rand::Rng;
use std::cell::{Ref, RefCell};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::vec;
use std::{error, fmt, io};

#[derive(Debug, Default, Clone, Copy)]
struct FreeLocations {
    size: u64,
    offset: u64,
}
static MAX_SPACE_BYTES: u64 = 8589934592; // 8GB
static MAX_PAGE_BYTE: u64 = 65536;
static RESERVED_SPACE: u64 = 320 * MAX_PAGE_BYTE; // 20.97 Mb reserved
static MAX_PAGES: u64 = MAX_SPACE_BYTES / MAX_PAGE_BYTE;
#[derive(Debug)]
pub enum StableMemoryError {
    OutOfMemory,

    InvalidKey,
    /// Attempted to read more stable memory than had been allocated.
    OutOfBounds,
}

impl fmt::Display for StableMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OutOfMemory => f.write_str("Out of memory"),
            Self::InvalidKey => f.write_str("Key Does Not Exist"),
            Self::OutOfBounds => f.write_str("Read exceeds allocated memory"),
        }
    }
}
impl error::Error for StableMemoryError {}

#[derive(PartialEq, Debug, Clone)]
pub struct Bucket {
    available_space: Vec<MemLocation>,
    index: HashMap<String, Vec<(u64, u64)>>, //(key,(offset size))
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct MemLocation {
    offset: u64,
    size: u64,
    is_located: bool,
}

impl Default for MemLocation {
    fn default() -> Self {
        MemLocation {
            offset: RESERVED_SPACE,
            size: MAX_SPACE_BYTES - RESERVED_SPACE,
            is_located: false,
        }
    }
}

impl Default for Bucket {
    fn default() -> Self {
        Bucket {
            available_space: vec![MemLocation::default()],
            index: HashMap::new(),
        }
    }
}

thread_local!(
    static BUCKET: std::cell::RefCell<Bucket> = std::cell::RefCell::new(Bucket::default());
);

impl Bucket {
    // fn _get_field(&mut self, elem_size: u64) -> Result<(u64, u64), Error> {
    //     match self._inspect_size(elem_size.clone()) {
    //         Err(err) => Err(err),
    //         Ok(..) => {
    //             let field = (self.offset.clone(), elem_size.clone());
    //             self._grow_stable_memory_page(elem_size.clone());
    //             self.offset += elem_size;
    //             Ok(field)
    //         }
    //     }
    // }

    fn _find_location(&self, elem_size: u64) -> Result<MemLocation, StableMemoryError> {
        let location = match self
            .available_space
            .binary_search_by(|mem_loc| mem_loc.size.cmp(&elem_size))
        {
            Ok(pos) => {
                dbg!(" Found : {:?}", pos);
                pos
            }
            Err(pos) => {
                dbg!(" Not Found : {:?}", pos);
                pos
            }
        };
        if location as u64 >= elem_size {
            // not enough space
            Err(StableMemoryError::OutOfMemory)
        } else {
            // get the smaller available_space
            Ok(self.available_space[location])
        }
    }

    pub fn put(key: String, value: Vec<u8>) -> Result<(), StableMemoryError> {
        BUCKET.with(|bucket| {
            let mut bucket = bucket.borrow_mut();
            match bucket._find_location(value.len() as u64) {
                Ok(field) => {}
                Err(_) => {}
            };
        });
        Ok(())
    }
    //    // check total_size
    //    fn _inspect_size(&self, total_size: u64) -> Result<(), Error> {
    //     if total_size <= self._get_available_memory_size()
    //         Ok(())
    //     } else {
    //         Err(Error::InsufficientMemory)
    //     }

    //    // return available memory size can be allocated
    //    fn _get_available_memory_size(&self) -> u64 {
    //     unsafe {
    //         THRESHOLD - self.offset
    //     }

    // ==================================================================================================
    // core api
    // ==================================================================================================

    // pub fn put(key: String, value: Vec<u8>) -> Result<(), Error> {
    //     BUCKET.with(|bucket| {
    //         let mut bucket = bucket.borrow_mut();
    //         match bucket._get_field(value.len() as u64) {
    //             Ok(field) => {
    //                 match bucket.assets.get_mut(&key) {
    //                     None => {
    //                         bucket.assets.insert(key, vec![field.clone()]);
    //                     }
    //                     Some(pre_field) => {
    //                         pre_field.push(field.clone());
    //                     }
    //                 }
    //                 bucket._storage_data(field.0, value);

    //                 // todo check 索引大小，否则assert!
    //                 bucket._check_self_bytes_len();
    //                 Ok(())
    //             }
    //             Err(err) => {
    //                 return Err(err);
    //             }
    //         }
    //     })
    // }
}
fn main() {
    let b = Bucket::default();
    // println!("Memory Size: {:?}", b.stable64_size());
    // println!("Grow memory , Result: {:?}", b.stable64_grow(1));
    // println!("Memory Size After Grow: {:?}", b.stable64_size());
    // let mut data = [0].repeat(20 as usize);
    // b.stable64_read(0, &mut data);
    // println!("Read From Memory: {:?}", data);
    // println!("");

    // let mut write = [255].repeat(8 as usize);
    // println!("Write to Memory: {:?}", write);
    // b.stable64_write(0, &mut write);
    // b.stable64_read(0, &mut data);
    // println!("Read From Memory: {:?}", data);

    // let mut write = [1].repeat(8 as usize);
    // println!("Write to Memory: {:?}", write);
    // b.stable64_write(8, &mut write);

    // b.stable64_read(0, &mut data);
    // println!("Read From Memory: {:?}", data);
}
