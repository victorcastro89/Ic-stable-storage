use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, CandidType)]
pub struct User {
    pub email: String,
    pub id: u64,
}
#[derive(Serialize, Deserialize, Debug, CandidType)]
pub struct Users {
    pub all: Vec<u64>,
}
