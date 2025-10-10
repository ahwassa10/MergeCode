use std::{ptr, thread};

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

struct Cursor(*mut Tuple);
unsafe impl Send for Cursor {}

pub fn scatter(table: &Vec<Tuple>, chunk_count: usize, prefix_sums: &Vec<Vec<u64>>) -> Vec<Vec<Tuple>> {
    assert!(chunk_count >= 2);

    let chunk_size = table.len().div_ceil(chunk_count);
    let bits_prefix = chunk_count.ilog2();
    let num_bins   = 2usize.pow(bits_prefix);

    assert!(bits_prefix > 0);
    assert!(chunk_count == num_bins);

    // The last prefix sum is equivalent to the final sizes of the chunks.
    let final_chunk_sizes : Vec<usize> = (0..chunk_count)
        .map(|b| prefix_sums[chunk_count][b] as usize)
        .collect();

    let mut final_chunks : Vec<Vec<Tuple>> = final_chunk_sizes.iter()
        .map(|n| vec![Tuple::default(); *n])
        .collect();

    thread::scope(|s| {
        for (chunk_index, chunk) in table.chunks(chunk_size).enumerate() {
            let mut starts: Vec<Cursor> = Vec::with_capacity(num_bins);
            let mut ends: Vec<Cursor> = Vec::with_capacity(num_bins);

            for out_chunk in 0..chunk_count {
                let start = prefix_sums[chunk_index][out_chunk] as usize;
                let end   = prefix_sums[chunk_index + 1][out_chunk] as usize;

                let base_ptr: *mut Tuple = final_chunks[out_chunk].as_mut_ptr();

                unsafe {
                    starts.push(Cursor(base_ptr.add(start)));
                    ends.push(Cursor(base_ptr.add(end)));
                }
            }

            s.spawn(move || {
                let mut curs = starts;

                for t in chunk {
                    let key = t.key;
                    let bin_index = (key >> (64 - bits_prefix)) as usize;

                    unsafe {
                        ptr::write(curs[bin_index].0, *t);
                        curs[bin_index].0 = curs[bin_index].0.add(1);
                    }
                }
            });
        }
    });

    final_chunks
}