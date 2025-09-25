#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Tuple {
    pub key: u64,
    pub payload: u64
}

impl Tuple {
    pub fn new(key: u64, payload: u64) -> Tuple {
        Tuple {key, payload}
    }
}

fn table_eq(left: &[Tuple], right: &[Tuple]) -> bool {
    let mut leftm = HashMap::with_capacity(left.len());
    for t in left {
        *leftm.entry(t).or_insert(0) += 1;
    }

    let mut rightm = HashMap::with_capacity(right.len());
    for t in right {
        *rightm.entry(t).or_insert(0) += 1;
    }

    leftm == rightm
}

#[cfg(test)]
mod test {
    use rand::{rngs::StdRng, SeedableRng};

    use crate::infrastructure;

    use super::*;

    #[test]
    fn table_eq_test1() {
        let mut rng = StdRng::seed_from_u64(101);
        let ltk = infrastructure::gen_keys(1000, &mut rng);
        let ltv = infrastructure::gen_keys(1000, &mut rng);
        let lt = infrastructure::gen_table(&ltk, &ltv);
        
        assert!(table_eq(&lt, &lt));
    }

    #[test]
    fn table_eq_test2() {
        let lt = vec![
            Tuple::new(5, 10),
            Tuple::new(6, 8),
            Tuple::new(2, 3)
        ];
        let rt = vec![
            Tuple::new(2, 3),
            Tuple::new(5, 10),
            Tuple::new(6, 8),
            Tuple::new(2, 3)
        ];
        assert!(!table_eq(&lt, &rt));
    }

    #[test]
    fn table_eq_test3() {
        let lt = vec![
            Tuple::new(5, 10),
            Tuple::new(2, 3),
            Tuple::new(6, 8),
            Tuple::new(2, 3),
            Tuple::new(5, 10)
        ];
        let rt = vec![
            Tuple::new(2, 3),
            Tuple::new(5, 10),
            Tuple::new(5, 10),
            Tuple::new(6, 8),
            Tuple::new(2, 3)
        ];
        assert!(table_eq(&lt, &rt));
    }
}