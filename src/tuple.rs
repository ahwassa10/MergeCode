#![allow(dead_code)]

#[derive(Debug)]
pub struct Tuple {
    pub key: u64,
    pub payload: u64
}

impl Tuple {
    fn new(key: u64, payload: u64) -> Tuple {
        Tuple {key, payload}
    }
}

