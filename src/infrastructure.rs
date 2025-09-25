use std::collections::HashMap;

use rand::{seq::SliceRandom, Rng};

use crate::tuple::Tuple;

pub fn gen_fact_keys<R: Rng>(key_set: &[u64], p: f64, rng: &mut R) -> Vec<u64> {
    assert!(p >= 0.0 && p < 1.0,
        "p must be a probability between 0 and 1");

    let mut output = Vec::new();
    for x in key_set {
        while rng.random_bool(p) {
            output.push(*x);
        }
    }
    output
}

pub fn gen_keys<R: Rng>(n: usize, rng: &mut R) -> Vec<u64> {
    let mut output = Vec::new();
    for _ in 0..n {
        output.push(rng.random());
    }
    output
}

pub fn zip_table(key_set: &[u64], payload_set: &[u64]) -> Vec<Tuple> {
    assert!(key_set.len() == payload_set.len(),
        "key and payload must be the same length");

    let table2 = key_set.iter()
        .zip(payload_set)
        .map(|kp| (Tuple {key: *kp.0, payload: *kp.1}))
        .collect::<Vec<Tuple>>();

    table2
}

pub fn gen_table<R: Rng>(n: usize, rng: &mut R) -> Vec<Tuple> {
    let keys = gen_keys(n, rng);
    let payloads = gen_keys(n, rng);
    zip_table(&keys, &payloads)
}

pub fn gen_tables<R: Rng>(n: usize, p: f64, rng: &mut R) -> (Vec<Tuple>, Vec<Tuple>) {
    let dimension_keys = gen_keys(n, rng);
    let dimension_payloads = gen_keys(n, rng);
    let dimension_table = zip_table(&dimension_keys, &dimension_payloads);

    let mut fact_keys = gen_fact_keys(&dimension_keys, p, rng);
    fact_keys.shuffle(rng);
    let fact_payloads = gen_keys(fact_keys.len(), rng);
    let fact_table = zip_table(&fact_keys, &fact_payloads);

    (fact_table, dimension_table)
}

pub fn table_eq(left: &[Tuple], right: &[Tuple]) -> bool {
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
mod tests {
    use std::collections::HashSet;

    use rand::{rngs::StdRng, SeedableRng};

    use super::*;

    #[test]
    fn foreign_key_check() {
        let mut rng = StdRng::seed_from_u64(101);
        let (ft, dt) = gen_tables(10000, 0.8, &mut rng);

        let keys = dt.iter().map(|tuple| tuple.key).collect::<HashSet<u64>>();
        
        for tuple in ft {
            let key = tuple.key;
            assert!(keys.contains(&key),
            "Referential integrity violation. Foreign key {key} does not exist in dimension table");
        }
    }

    #[test]
    fn table_eq_test1() {
        let mut rng = StdRng::seed_from_u64(101);
        let t = gen_table(1000, &mut rng);
        
        assert!(table_eq(&t, &t));
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