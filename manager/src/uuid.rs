use ic_cdk::export::Principal;
use uuid::{Builder, Uuid};
pub async fn gen_uuid() -> Result<String, String> {
    let b = gen_rand().await;
    match b {
        Ok(b) => return Ok(Builder::from_random_bytes(b).into_uuid().to_string()),
        Err(err) => return Err(err),
    }
}

fn vector_as_u8_16_array(vector: Vec<u8>) -> [u8; 16] {
    let mut arr = [0u8; 16];
    for (place, element) in arr.iter_mut().zip(vector.iter()) {
        *place = *element;
    }
    arr
}

async fn gen_rand() -> Result<[u8; 16], String> {
    let c = ic_cdk::api::call::call(Principal::management_canister(), "raw_rand", ()).await;
    let (bytes,): (Option<Vec<u8>>,) = match c {
        Ok(c) => c,
        Err(err) => (None,),
    };

    match bytes {
        Some(bytes) => return Ok(vector_as_u8_16_array(bytes)),
        None => return Err("Error Generation UUID".to_string()),
    }
}
