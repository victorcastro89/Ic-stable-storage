#![allow(dead_code, unused_imports, unused_variables)]

mod serializekeys;
mod types;
mod uuid;
use base64::{decode, encode};
use ic_cdk::api;

use ic_cdk::api::call;
use ic_cdk::api::stable;
use ic_cdk::export::candid::{candid_method, CandidType, Decode, Deserialize, Encode, Nat};
use ic_cdk::export::Principal;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};

use serializekeys::{de_data, de_key, ser_data, ser_key};

use stable_structures::{Memory, StableBTreeMap, StableStorage};
use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::rc::Rc;
use std::{mem, vec};
use types::User;

struct Stable {
    btree: StableBTreeMap<StableStorage>,
}
impl Default for Stable {
    fn default() -> Self {
        Self {
            btree: StableBTreeMap::load(StableStorage::default()),
        }
    }
}

thread_local!(
    static STABLE_MEM: RefCell<Stable> = RefCell::new(Stable::default());
);

#[init]
#[candid_method(init)]
fn init() {
    //Initialize Btree Map in the stable store limiting Key in 80 Bytes and Value in 2Kbytes
    StableBTreeMap::new(StableStorage::default(), 1, 100);
}

fn insert_multiple(from: u64, to: u64) {
    for i in from..to {
        let value = types::User {
            email: format!("victor{}@hotmail.com", i.to_string()),
            id: i,
        };

        STABLE_MEM.with(|m| {
            m.borrow_mut()
                .btree
                .insert(ser_key(&value.id.to_string()), ser_data(&value).unwrap())
                .unwrap()
        });
    }
}
#[update(name = "insert_multiple")]
#[candid_method(update, rename = "insert_multiple")]
fn insert_multiple_data(from: u64, to: u64) {
    insert_multiple(from, to)
}

#[update(name = "insert_string")]
#[candid_method(update, rename = "insert_string")]
async fn insert_string(key: String, value: String) {
    STABLE_MEM.with(|m| {
        m.borrow_mut()
            .btree
            .insert(ser_key(&key), ser_key(&value))
            .unwrap()
    });
}

#[update(name = "insert_vec")]
#[candid_method(update, rename = "insert_vec")]
async fn insert_vec(key: Vec<u8>, value: Vec<u8>) {
    STABLE_MEM.with(|m| m.borrow_mut().btree.insert(key, value).unwrap());
    // let got = STABLE_MEM.with(|m| m.borrow_mut().btree.get(&vec![key]).unwrap());
}

#[update(name = "remove_vec")]
#[candid_method(update, rename = "remove_vec")]
async fn remove_vec(key: Vec<u8>) {
    STABLE_MEM.with(|m| m.borrow_mut().btree.remove(&key).unwrap());
    // let got = STABLE_MEM.with(|m| m.borrow_mut().btree.get(&vec![key]).unwrap());
}

#[query(name = "get_by_key")]
#[candid_method(query, rename = "get_by_key")]
async fn get_by_key(key: String) -> String {
    let got = STABLE_MEM.with(|m| m.borrow_mut().btree.get(&ser_key(&key)).unwrap());
    let val = de_key(&got);

    val
}

#[query(name = "get_user_by_id")]
#[candid_method(query, rename = "get_user_by_id")]
fn get_usr_by_id(key: u64) -> User {
    let got = STABLE_MEM.with(|m| {
        m.borrow_mut()
            .btree
            .get(&ser_key(&key.to_string()))
            .unwrap()
    });
    let val: User = de_data(&got).unwrap();
    ic_cdk::println!(" Data: {:?}", val);
    val
}

#[query(name = "read")]
#[candid_method(query, rename = "read")]
fn read(position: u64, size: u64) -> Vec<u8> {
    let mut buf = [0].repeat(size as usize);
    stable::stable64_read(position, &mut buf);
    return buf;
}

#[query(name = "stablesize")]
#[candid_method(query, rename = "stablesize")]
fn stable_size() -> u64 {
    stable::stable64_size()
}
#[update(name = "stablegrow")]
#[candid_method(update, rename = "stablegrow")]
fn stable_grow(pages: u64) -> u64 {
    stable::stable64_grow(pages).unwrap()
}

#[pre_upgrade]
fn pre_upgrade() {
    // let _nftinfo = NFTINFO.with(|nftinfo| nftinfo.borrow().clone());

    // // let bytes = bincode::serialize::<NftInfo>(&_nftinfo).unwrap();

    // let bytes = Encode!(&_nftinfo).unwrap();
    // Bucket::pre_upgrade(bytes);
}

#[post_upgrade]
fn post_upgrade() {
    // ic_cdk::println!("Post upgrade");
    // let m = StableStorage::default();
    // let size = stable::stable64_size();
    // // let bytes = Bucket::post_upgrade();
    // if size > 0 {
    //     StableBTreeMap::load(m);
    // }
    // STABLE_MEM.with(|m| m.borrow_mut().btree.load(vec![key], vec![value]).unwrap());
    // NFTINFO.with(|nftinfo| {
    //     // *nftinfo.borrow_mut() = bincode::deserialize(&bytes).unwrap()
    //     *nftinfo.borrow_mut() = Decode!(&bytes, NftInfo).unwrap();
    // });
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {
    // let mem = Rc::new(RefCell::new(Vec::new()));
    // let mut btree = StableBTreeMap::new(mem, 3, 4);
    // let string = "foo".to_string().into_bytes();
    // let mut bar = "bar".to_string().into_bytes();
    // let string1 = "foo".to_string().into_bytes();
    // btree.insert(string, bar);
    // ic_cdk::println!(
    //     " data: {:?}",
    //     String::from_utf8(btree.get(&string1).unwrap()).unwrap()
    // );
}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    ic_cdk::export::candid::export_service!();
    std::print!("{}", __export_service());
}
