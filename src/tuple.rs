#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tuple {
    pub key: u64,
    pub payload: u64
}

impl Tuple {
    pub fn new(key: u64, payload: u64) -> Tuple {
        Tuple {key, payload}
    }
}