use rand::seq::SliceRandom;

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


