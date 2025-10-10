#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub struct Tuple {
    pub key: u64,
    pub payload: u64
}

impl Tuple {
    pub fn new(key: u64, payload: u64) -> Tuple {
        Tuple {key, payload}
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Joined {
    pub key: u64,
    pub left_payload: u64,
    pub right_payload: u64
}

impl Joined {
    pub fn new(key: u64, left_payload: u64, right_payload: u64) -> Joined {
        Joined {key, left_payload, right_payload}
    }
}