#![allow(dead_code, unused_imports, unused_variables, unused_assignments)]
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::vec;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    email: String,
    id: String,
    #[serde(default)]
    name: String,
    address: String,
}
impl Default for User {
    fn default() -> Self {
        Self {
            email: Default::default(),
            id: Default::default(),
            name: Default::default(),
            address: Default::default(),
        }
    }
}

fn mk_user(num: u64) -> Vec<User> {
    let mut v = Vec::new();
    for i in 0..num {
        v.push(User {
            email: "Victorcastro89@gmail.com".to_string(),
            id: "22f283e2-4a60-4293-93a1-85a595778e3c".to_string(),
            name: "My not Very big name".to_string(),
            address: "My super address, I' live in the clouds BR-SC:1535  apto:102".to_string(),
        });
    }
    v
}

fn json_ser<T>(data: &T) -> String
where
    T: ?Sized + Serialize,
{
    serde_json::to_string(&data).unwrap()
}

fn json_de(data: &str) -> Vec<User> {
    let user: Vec<User> = serde_json::from_str(data).unwrap();
    user
}

fn bincode_ser(key: &Vec<User>) -> Vec<u8> {
    bincode::serialize(&key).unwrap()
}

fn bincode_de(encoded: &Vec<u8>) -> Vec<User> {
    bincode::deserialize(&encoded).unwrap()
}

fn rmp_se(u: &Vec<User>) -> Vec<u8> {
    let mut u_encoded = Vec::new();
    u_encoded = rmp_serde::to_vec(&u).unwrap();
    u_encoded
}

fn rmp_de(u: &Vec<u8>) -> Vec<User> {
    let user: Vec<User> = rmp_serde::from_slice(&u).unwrap();
    user
}

fn bench_fibs(c: &mut Criterion) {
    let user = mk_user(1000000);
    let json_ser_ = json_ser(&user);
    let bincode_ser_ = bincode_ser(&user);
    let msg_ser_ = rmp_se(&user);

    assert_eq!(user, json_de(&json_ser_));
    assert_eq!(user, bincode_de(&bincode_ser_));
    assert_eq!(user, rmp_de(&msg_ser_));
    // println!("Deserialize {:?}", rmp_de(&msg_ser_));
    // println!("Deserialize {:?}", bincode_de(&bincode_ser_));
    // println!("Deserialize {:?}", json_de(&json_ser_));
    let mut group = c.benchmark_group("Serialize");
    for i in [20u64, 21u64].iter() {
        group.bench_function(BenchmarkId::new("Json SE", i), |b| {
            b.iter(|| json_ser(&user))
        });

        group.bench_function(BenchmarkId::new("BincoSE", i), |b| {
            b.iter(|| bincode_ser(&user))
        });

        group.bench_function(BenchmarkId::new("MessagePack SE", i), |b| {
            b.iter(|| rmp_se(&user))
        });
    }
    group.finish();

    let mut group = c.benchmark_group("Deserialize");
    for i in [20u64, 21u64].iter() {
        group.bench_function(BenchmarkId::new("JsonDe", i), |b| {
            b.iter(|| json_de(&json_ser_))
        });

        group.bench_function(BenchmarkId::new("BincodeDE", i), |b| {
            b.iter(|| bincode_de(&bincode_ser_))
        });

        group.bench_function(BenchmarkId::new("MessagePackDE", i), |b| {
            b.iter(|| rmp_de(&msg_ser_))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
