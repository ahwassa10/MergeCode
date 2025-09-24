use rand::Rng;

use crate::tuple::Tuple;

pub fn gen_dimension_keys<R: Rng>(key_set: &[u64], p: f64, rng: &mut R) -> Vec<u64> {
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

pub fn gen_fact_keys<R: Rng>(n: usize, rng: &mut R) -> Vec<u64> {
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
    let fact_keys = gen_fact_keys(n, rng);
    let fact_payloads = gen_fact_keys(n, rng);
    let dimension_keys = gen_dimension_keys(&fact_keys, p, rng);
    let dimension_payloads = gen_fact_keys(dimension_keys.len(), rng);
    
    let fact_table = gen_table(&fact_keys, &fact_payloads);
    let dimension_table = gen_table(&dimension_keys, &dimension_payloads);

    (fact_table, dimension_table)
}