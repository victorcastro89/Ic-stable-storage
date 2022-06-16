#![allow(dead_code, unused_imports, unused_variables)]
use ic_cdk::api;
use ic_cdk::api::call;
use ic_cdk::api::stable;
use ic_cdk::export::candid::{candid_method, CandidType, Decode, Encode, Nat};
use ic_cdk::export::Principal;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};
use serde::{Deserialize, Serialize};
use stable_structures::{Memory, StableBTreeMap, StableStorage};
use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::rc::Rc;
use std::{mem, vec};
use utils::time;
type Filename = String;
type ChunkType = Vec<u8>;
type ChunkId = String;
type AssetId = String;
use std::path::Path;
use types::TimestampMillis;
#[derive(Serialize, Deserialize, Debug, CandidType, Clone)]
struct Chunk {
    filename: Filename,
    chunk_index: u64,
    chunk: ChunkType,
}

#[derive(Serialize, Deserialize, Debug, CandidType, Clone)]
struct AssetEncoding {
    modified: TimestampMillis,
    content_chunks: Vec<ChunkType>,
    total_length: u64,
    certified: bool,
}

#[derive(Serialize, Deserialize, Debug, CandidType, Clone)]
struct Asset {
    encoding: AssetEncoding,
    content_type: String,
}
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
    static CHUNK_MEM: RefCell<HashMap<ChunkId, ChunkType>> = RefCell::new(HashMap::new());
    static ASSET_MEM: RefCell<HashMap<AssetId, Asset>> = RefCell::new(HashMap::new());
);

#[init]
#[candid_method(init)]
fn init() {
    //Initialize Btree Map in the stable store limiting Key in 80 Bytes and Value in 2Kbytes
    StableBTreeMap::new(StableStorage::default(), 1, 100);
}

fn check_supported_file_extentions(filename: &str) -> bool {
    let supported_extensions = vec!["jpg", "jpeg", "png", "gif"];
    let file_extension = Path::new(filename).extension().unwrap().to_str().unwrap();
    supported_extensions.contains(&file_extension)
}
#[update(name = "create_chunk")]
#[candid_method(update, rename = "create_chunk")]
fn create_chunk(
    Chunk {
        filename,
        chunk_index,
        chunk,
    }: Chunk,
) -> ChunkId {
    if check_supported_file_extentions(&filename) != true {
        panic!("Unsupported file extension");
    }
    let chunk_id = format!("{}-{}-{}", filename, chunk_index, time::now_millis());
    CHUNK_MEM.with(|m| {
        m.borrow_mut().insert(chunk_id.clone(), chunk.clone());
    });
    chunk_id
}

#[update(name = "commit_batch")]
#[candid_method(update, rename = "commit_batch")]
fn commit_batch(file_name: String, chunk_ids: Vec<String>, content_type: String) -> String {
    let mut content: Vec<Vec<u8>> = vec![];
    chunk_ids.iter().for_each(|chunk_id| {
        CHUNK_MEM.with(|m| {
            if !m.borrow().contains_key(chunk_id) {
                panic!("Chunk not found");
            }
            content.push(m.borrow().get(chunk_id).unwrap().clone());
        });
    });
    let size = content.len();
    let asset_id = format!("{}-{}-{}", file_name, size, time::now_millis());
    ASSET_MEM.with(|m| {
        m.borrow_mut().insert(
            asset_id.clone(),
            Asset {
                encoding: AssetEncoding {
                    modified: time::now_millis(),
                    content_chunks: content.clone(),
                    total_length: size as u64,
                    certified: false,
                },
                content_type,
            },
        );
    });
    ic_cdk::println!("Asset content {:?}", content);
    file_name
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
