use rand::seq::SliceRandom;

pub fn mem_scan(input: &Vec<usize>) -> usize {
    let mut accumulator = 0;
    for i in 0..input.len() {
        accumulator = accumulator ^ input[i] 
    }
    accumulator
}

pub fn gen_ideal_n(n: usize) -> Vec<usize> {
    let mut rng = rand::rng();

    let mut data = (0..n).collect::<Vec<usize>>();
    data.shuffle(&mut rng);

    data
}

pub fn sort_ideal(input: &Vec<usize>) -> Vec<usize> {
    let mut output = vec![0; input.len()];

    for i in input { 
        output[*i] = *i;
    }

    output
}

pub fn rust_sort(mut input: Vec<usize>) -> Vec<usize> {
    input.sort();
    input
}

pub fn sort_merge_join_ideal(left: &Vec<usize>, right: &Vec<usize>) -> Vec<usize> {
    let left_sorted = sort_ideal(left);
    let right_sorted = sort_ideal(right);
    
    let mut output = Vec::new();

    let mut li = 0;
    let mut ri = 0;
    while li < left_sorted.len() && ri < right_sorted.len() {
        output.push(left_sorted[li] - right_sorted[ri]);
        li +=1;
        ri +=1;
    }

    output
}

pub fn hash_join_ideal(left: &Vec<usize>, right: &Vec<usize>) -> Vec<usize> {
    let right_sorted = sort_ideal(right);

    let mut output = Vec::new();
    for &i in left {
        output.push(right_sorted[i] - i);
    }

    output
}
