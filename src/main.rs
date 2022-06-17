mod serializekeys;
mod types;
mod uuid;
use ic_cdk::api::stable;
use ic_cdk::export::candid::candid_method;
use ic_cdk_macros::{init, query, update};
use serializekeys::{de_key, de_value, ser_key, ser_value};
use stable_structures::{StableBTreeMap, StableStorage};
use std::cell::RefCell;
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
    StableBTreeMap::new(StableStorage::default(), 80, 200);
}

#[update(name = "insert_string")]
#[candid_method(update, rename = "insert_string")]
async fn insert_string(key: String, value: String) -> String {
    STABLE_MEM.with(|m| {
        m.borrow_mut()
            .btree
            .insert(ser_key(&key), ser_value(&value).unwrap())
            .unwrap()
    });
    format!("Inserted: {} , {}", key, value)
}

#[query(name = "get_string_by_key")]
#[candid_method(query, rename = "get_string_by_key")]
async fn get_string_by_key(key: String) -> String {
    let got = STABLE_MEM.with(|m| m.borrow_mut().btree.get(&ser_key(&key)).unwrap());
    let val = de_value(&got);
    let res = match val {
        Ok(v) => v,
        Err(_) => "Not Found".to_string(),
    };
    res
}

#[query(name = "get_user_by_id")]
#[candid_method(query, rename = "get_user_by_id")]
fn get_usr_by_id(key: u64) -> Result<User, String> {
    let got = STABLE_MEM.with(|m| {
        m.borrow_mut()
            .btree
            .get(&ser_key(&key.to_string()))
            .unwrap()
    });
    let val: Result<User, rmp_serde::decode::Error> = de_value(&got);
    match val {
        Ok(v) => Ok(v),
        Err(_) => Err("Not Found".to_string()),
    }
}

#[update(name = "insert_multiple_users")]
#[candid_method(update, rename = "insert_multiple_users")]
fn insert_multiple_users(from: u64, to: u64) {
    insert_multiple(from, to)
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
                .insert(ser_key(&value.id.to_string()), ser_value(&value).unwrap())
                .unwrap()
        });
    }
}
#[query(name = "read_raw_memory")]
#[candid_method(query, rename = "read_raw_memory")]
fn read_raw_memory(position: u64, size: u64) -> Vec<u8> {
    let mut buf = [0].repeat(size as usize);
    stable::stable64_read(position, &mut buf);
    return buf;
}

#[query(name = "stablesize")]
#[candid_method(query, rename = "stablesize")]
fn stable_size() -> u64 {
    stable::stable64_size()
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    ic_cdk::export::candid::export_service!();
    std::print!("{}", __export_service());
}
