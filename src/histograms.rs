pub fn prefix_sums(histograms: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let num_histograms = histograms.len();
    assert!(num_histograms > 0);
    
    let mut ps: Vec<Vec<u64>> = Vec::new();
    let num_bins = histograms[0].len();
    
    // The first prefix sum is all zeroes since that data gets written to the
    // start of each chunk.
    let mut cur_ps = vec![0; num_bins];
    ps.push(cur_ps.clone());

    for i in 0..num_histograms {
        let histogram = &histograms[i];
        for j in 0..num_bins {
            cur_ps[j] += histogram[j];
        }
        ps.push(cur_ps.clone());
    }

    ps
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn prefix_sums_test() {
        let histograms = vec![vec![3, 5], vec![1, 7]];
        let expected_ps = vec![vec![0, 0], vec![3, 5]];

        let ps = prefix_sums(&histograms);
        assert_eq!(ps, expected_ps);
    }

    #[test]
    fn prefix_sums_test2() {
        let histograms = vec![vec![10, 7, 3], vec![5, 5, 10], vec![13, 5, 2]];
        let expected_ps = vec![vec![0, 0, 0], vec![10, 7, 3], vec![15, 12, 13]];

        let ps = prefix_sums(&histograms);
        assert_eq!(ps, expected_ps);
    }
}