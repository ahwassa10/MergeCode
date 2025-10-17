use rand::seq::SliceRandom;

pub fn mem_scan(input: &Vec<usize>) -> usize {
    let mut accumulator = 0;
    for i in 0..input.len() {
        accumulator = accumulator ^ input[i] 
    }
    accumulator
}

pub fn mem_strided_scan(input: &Vec<usize>) -> usize {
    let mut accumulator1 = 0;
    let mut accumulator2 = 0;
    let half = input.len() / 2;
    for i in 0..half {
        accumulator1 = accumulator1 ^ input[i];
        accumulator2 = accumulator2 ^ input[half + i];
    }
    accumulator1 ^ accumulator2
}

pub fn mem_strided_4_scan(input: &Vec<usize>) -> usize {
    let mut accumulator1 = 0;
    let mut accumulator2 = 0;
    let mut accumulator3 = 0;
    let mut accumulator4 = 0;
    let quarter = input.len() / 4;
    let half = input.len() / 2;
    for i in 0..quarter {
        accumulator1 = accumulator1 ^ input[i];
        accumulator2 = accumulator2 ^ input[quarter + i];
        accumulator3 = accumulator3 ^ input[half + i];
        accumulator4 = accumulator4 ^ input[half + quarter + i];
    }
    accumulator1 ^ accumulator2 ^ accumulator3 ^ accumulator4
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
