#![allow(dead_code)]

use rand::{rngs::StdRng, SeedableRng};

use merge::{infrastructure::gen_tables};


fn main() {
    let mut rng = StdRng::seed_from_u64(42);
    let (ft, dt) = gen_tables(20, 0.5, &mut rng);

    println!("ft len: {}, dt len: {}", ft.len(), dt.len());
    println!("ft: {:?}", ft);
    println!("dt: {:?}", dt);
}