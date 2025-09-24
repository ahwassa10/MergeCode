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

pub fn gen_table(key_set: &[u64], payload_set: &[u64]) -> Vec<Tuple> {
    assert!(key_set.len() == payload_set.len(),
        "key and payload must be the same length");

    let table2 = key_set.iter()
        .zip(payload_set)
        .map(|kp| (Tuple {key: *kp.0, payload: *kp.1}))
        .collect::<Vec<Tuple>>();

    table2
}

pub fn gen_tables<R: Rng>(n: usize, p: f64, rng: &mut R) -> (Vec<Tuple>, Vec<Tuple>) {
    let dimension_keys = gen_keys(n, rng);
    let dimension_payloads = gen_keys(n, rng);
    let dimension_table = gen_table(&dimension_keys, &dimension_payloads);

    let mut fact_keys = gen_fact_keys(&dimension_keys, p, rng);
    fact_keys.shuffle(rng);
    let fact_payloads = gen_keys(fact_keys.len(), rng);
    let fact_table = gen_table(&fact_keys, &fact_payloads);

    (fact_table, dimension_table)
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
}