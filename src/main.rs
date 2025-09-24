use rand::{rngs::StdRng, SeedableRng};

use crate::{infrastructure::gen_tables};

mod merge;
mod tuple;
mod infrastructure;
mod join;

fn main() {
    let mut rng = StdRng::seed_from_u64(42);
    let (ft, dt) = gen_tables(20, 0.5, &mut rng);

    println!("ft len: {}, dt len: {}", ft.len(), dt.len());
    println!("ft: {:?}", ft);
    println!("dt: {:?}", dt);
}