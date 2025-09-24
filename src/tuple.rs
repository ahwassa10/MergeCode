#![allow(dead_code)]

use std::cmp::Ordering;

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

impl Ord for Tuple {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for Tuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Tuple {}