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

pub fn chunk_histograms(table: &Vec<Tuple>, chunk_count: usize) -> Vec<Vec<u64>> {
    assert!(chunk_count >= 2);

    let chunk_size = table.len().div_ceil(chunk_count);
    let bits_prefix = chunk_count.ilog2();
    let num_bins   = 2usize.pow(bits_prefix);

    assert!(bits_prefix > 0);
    assert!(chunk_count == num_bins);

    thread::scope(|s| {
        let mut handles = Vec::new();
        for (chunk_index, chunk) in table.chunks(chunk_size).enumerate() {
            handles.push(s.spawn(move || {
                let mut histogram: Vec<u64> = vec![0; num_bins];

                for t in chunk {
                    let key = t.key;
                    let histogram_index = (key >> (64 - bits_prefix)) as usize;
                    histogram[histogram_index] += 1;
                }

                (chunk_index, histogram)
            }));
        }

        let mut histograms: Vec<Vec<u64>> = vec![Vec::new(); chunk_count];
        for h in handles {
            let (chunk_index, histogram) = h.join().unwrap();
            histograms[chunk_index] = histogram;
        }
        histograms
    })
}