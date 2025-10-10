#![allow(dead_code)]

use std::{cmp::Ordering, thread};

use crate::{histograms, parallel, tuples::{Joined, Tuple}};

fn nested_loop_join(left: &Vec<Tuple>, right: &Vec<Tuple>) -> Vec<Joined> {
    let mut output = Vec::new();

    for lt in left {
        for rt in right {
            if lt.key == rt.key {
                output.push(Joined::new(lt.key, lt.payload, rt.payload));
            }
        }
    }

    output
}

fn merge_join_sorted(left: &[Tuple], right: &[Tuple], output: &mut Vec<Joined>) {
    let mut li = 0;
    let mut ri = 0;

    while li < left.len() && ri < right.len() {
        match left[li].key.cmp(&right[ri].key) {
            Ordering::Less => {li += 1;}
            Ordering::Greater => {ri += 1;}
            Ordering::Equal => {
                let key = left[li].key;
                debug_assert!(left[li].key == right[ri].key);
                
                let l_start = li;
                while li < left.len() && left[li].key == key { li += 1; }
                
                let r_start = ri;
                while ri < right.len() && right[ri].key == key { ri += 1; }

                for i in l_start..li {
                    for j in r_start..ri {
                        output.push(Joined::new(key, left[i].payload, right[j].payload));
                    }
                }
            }
        }
    }
}

fn basic_sort_merge_join(mut left: Vec<Tuple>, mut right: Vec<Tuple>) -> Vec<Joined> {
    left.sort_by_key(|t| t.key);
    right.sort_by_key(|t| t.key);

    let mut output = Vec::new();
    merge_join_sorted(&left, &right, &mut output);
    output
}

fn basic_mpsm(mut left: Vec<Tuple>, mut right: Vec<Tuple>, thread_count: usize) -> Vec<Vec<Joined>>{
    assert!(thread_count > 0);
    
    // Sort the public data among thread_count workers
    parallel::sort_runs_parallel(&mut right, thread_count);

    // Borrow right as an immutable reference so that all threads
    // can share the data.
    let public: &[Tuple] = &right;
    let private_chunk_size = left.len().div_ceil(thread_count);
    let public_chunk_size = right.len().div_ceil(thread_count);

    // Sort each private data chunk and then merge against the 
    // entire public data.
    let mut outputs = Vec::new();
    thread::scope(|s| {
        let mut handles = Vec::new();
        for private_chunk in left.chunks_mut(private_chunk_size) {
            handles.push(s.spawn(move || {
                private_chunk.sort_by_key(|t| t.key);
                
                let mut output = Vec::new();
                for public_chunk in public.chunks(public_chunk_size) {
                    merge_join_sorted(private_chunk, public_chunk, &mut output);
                }
                output
            }));
        }
        for h in handles {
            outputs.push(h.join().unwrap());
        }
    });

    outputs
}

fn partitioned_mpsm(left: Vec<Tuple>, mut right: Vec<Tuple>, thread_count: usize) -> Vec<Vec<Joined>> {
    assert!(thread_count > 0);

    // left = private data = R
    // right = public data = S

    let public_chunk_size = right.len().div_ceil(thread_count);

    // Phase 1 -- https://arxiv.org/abs/1207.0145
    // Sort the public data among thread_count workers
    parallel::sort_runs_parallel(&mut right, thread_count);

    // Phase 2
    // Compute thread_count histograms on the private data using thread_count workers
    let histograms = parallel::chunk_histograms(&left, thread_count);
    // Compute prefix sums
    let prefix_sums = histograms::prefix_sums(&histograms);
    // Scatter the private data into partitioned chunks
    let mut private_chunks = parallel::scatter(&left, thread_count, &prefix_sums);

    // Reborrow public as mutable so that all threads can share the data
    let public: &[Tuple] = &right;

    // Sort each private chunk and then merge against a run of public data
    let mut outputs = Vec::new();
    thread::scope(|s| {
        let mut handles = Vec::new();
        for private_chunk in &mut private_chunks {
            handles.push(s.spawn(move || {
                // Phase 3
                private_chunk.sort_by_key(|t| t.key);

                // Phase 4
                let mut output = Vec::new();
                for public_chunk in public.chunks(public_chunk_size) {
                    merge_join_sorted(private_chunk, public_chunk, &mut output);
                }
                output
            }));
        }
        for h in handles {
            outputs.push(h.join().unwrap());
        }
    });

    outputs
}

#[cfg(test)]
mod test {
    use rand::{rngs::StdRng, SeedableRng};

    use crate::infrastructure;

    use super::*;

    #[test]
    fn basic_sort_merge_join_test() {
        // Dimension table
        let left = vec![
            Tuple::new(5, 10),
            Tuple::new(6, 100),
            Tuple::new(2, 34),
            Tuple::new(7, 18)
        ];
        // Fact table. First column is a foreign key
        // to dimension table
        let right = vec![
            Tuple::new(5, 15),
            Tuple::new(8, 16),
            Tuple::new(2, 36),
            Tuple::new(2, 18),
            Tuple::new(2, 8),
            Tuple::new(9, 9)
        ];

        let mut output = basic_sort_merge_join(left, right).into_iter();
        assert_eq!(output.next(), Some(Joined::new(2, 34, 36)));
        assert_eq!(output.next(), Some(Joined::new(2, 34, 18)));
        assert_eq!(output.next(), Some(Joined::new(2, 34, 8)));
        assert_eq!(output.next(), Some(Joined::new(5, 10, 15)));
        assert_eq!(output.next(), None);
    }

    #[test]
    fn nested_loop_join_test() {
        // Dimension table
        let left = vec![
            Tuple::new(5, 10),
            Tuple::new(6, 100),
            Tuple::new(2, 34),
            Tuple::new(7, 18)
        ];
        // Fact table. First column is a foreign key
        // to dimension table
        let right = vec![
            Tuple::new(5, 15),
            Tuple::new(8, 16),
            Tuple::new(2, 36),
            Tuple::new(2, 18),
            Tuple::new(2, 8),
            Tuple::new(9, 9)
        ];

        let mut output = nested_loop_join(&left, &right).into_iter();
        assert_eq!(output.next(), Some(Joined::new(5, 10, 15)));
        assert_eq!(output.next(), Some(Joined::new(2, 34, 36)));
        assert_eq!(output.next(), Some(Joined::new(2, 34, 18)));
        assert_eq!(output.next(), Some(Joined::new(2, 34, 8)));
        assert_eq!(output.next(), None);
    }

    #[test]
    fn compare_sort_merge_nested_loop() {
        let mut rng = StdRng::seed_from_u64(101);
        let (lt, rt) = infrastructure::gen_tables(10000, 0.7, &mut rng);

        let nl_output = nested_loop_join(&lt, &rt);
        let sm_output = basic_sort_merge_join(lt, rt);

        assert!(infrastructure::table_eq(&nl_output, &sm_output))
    }

    #[test]
    fn compare_basic_mpsm_nested_loop() {
        let mut rng = StdRng::seed_from_u64(101);
        let (lt, rt) = infrastructure::gen_tables(10000, 0.7, &mut rng);

        let nl_output = nested_loop_join(&lt, &rt);
        let mpsm_output = basic_mpsm(lt, rt, 4).into_iter().flatten().collect::<Vec<Joined>>();

        assert!(infrastructure::table_eq(&nl_output, &mpsm_output));
    }

    #[test]
    fn compare_partitioned_mpsm_nested_loop() {
        let mut rng = StdRng::seed_from_u64(101);
        let (lt, rt) = infrastructure::gen_tables(10000, 0.7, &mut rng);

        let nl_output = nested_loop_join(&lt, &rt);
        let mpsm_output = partitioned_mpsm(lt, rt, 4).into_iter().flatten().collect::<Vec<Joined>>();

        assert!(infrastructure::table_eq(&nl_output, &mpsm_output));
    }
}