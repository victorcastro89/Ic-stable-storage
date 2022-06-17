#![allow(dead_code, unused_imports)]
use bincode;
use candid::Deserialize;
use serde::Serialize;

pub fn ser_key(key: &str) -> Vec<u8> {
    bincode::serialize(&key).unwrap()
}

pub fn de_key(encoded: &Vec<u8>) -> String {
    bincode::deserialize(&encoded).unwrap()
}

pub fn ser_value<T>(val: &T) -> Result<Vec<u8>, rmp_serde::encode::Error>
where
    T: Serialize + ?Sized,
{
    rmp_serde::to_vec(&val)
}

pub fn de_value<'a, T>(input: &'a [u8]) -> Result<T, rmp_serde::decode::Error>
where
    T: Deserialize<'a>,
{
    rmp_serde::from_slice(&input)
}
