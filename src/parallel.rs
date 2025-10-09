use std::thread;

use crate::tuples::Tuple;

pub fn sort_runs_parallel(table: &mut Vec<Tuple>, chunk_count: usize) {
    assert!(chunk_count > 0);

    let chunk_size = table.len().div_ceil(chunk_count);

    thread::scope(|s| {
        let mut handles = Vec::new();
        for chunk in table.chunks_mut(chunk_size) {
            handles.push(s.spawn(move || chunk.sort_by_key(|t| t.key)));
        }
        for h in handles {
            h.join().unwrap();
        }
    });
}