#![allow(dead_code, unused_imports, unused_variables, unused_assignments)]
use bincode::ErrorKind;
use ciborium::{de, ser};

use serde::{Deserialize, Serialize};
use std::default;
#[derive(Serialize, Deserialize, Debug)]
struct User {
    email: String,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User1 {
    email: String,
    id: String,
    #[serde(default)]
    name: String,
}
impl Default for User1 {
    fn default() -> Self {
        Self {
            email: Default::default(),
            id: Default::default(),
            name: Default::default(),
        }
    }
}

fn path() -> Option<String> {
    Some("/".to_string())
}
fn main() {
    let u = User {
        email: "aaaaaaaaaaaaaaaaaasddddqe13454tfgdertyuj".to_string(),
        id: "aaaaaaaaaaaaaaaaaasddddqe13454tfgdertyuj".to_string(),
    };
    let mut u_encoded = Vec::new();
    // u.serialize(&mut Serializer::new(&mut u_encoded)).unwrap();
    u_encoded = rmp_serde::to_vec(&u).unwrap();
    println!("User Encoded: {:?}", u_encoded);
    println!("User Encoded LEN: {:?}", u_encoded.len());
    let d: User1 = rmp_serde::from_slice(&u_encoded).unwrap();
    println!("User DEcoded: {:?}", d);
    // let decoded: Result<User1, serde_cbor::Error> = serde_cbor::from_slice(&u_encoded);
    // match decoded {
    //     Ok(decoded) => {
    //         println!("User DEcoded: {:?}", decoded);
    //     }
    //     Err(err) => println!("Error: {}", err),
    // }
}
