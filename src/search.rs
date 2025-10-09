pub fn lb_linear_search<T: Ord>(target: T, input: &[T]) -> Option<usize> {
    for i in 0..input.len() {
        if target <= input[i] {
            return Some(i)
        }
    }
    return None
}

pub fn lb_binary_search<T: Ord>(target: T, input: &[T]) -> Option<usize> {
    let mut lo = 0;
    let mut hi = input.len();

    while lo < hi {
        let mid = (lo + hi) / 2;
        
        if target <= input[mid] {
            hi = mid
        } else {
            lo = mid + 1
        }
    }

    debug_assert!(lo == hi);
    // The target was greater than every element in the input slice.
    if lo >= input.len() {
        return None
    } else {
        return Some(lo)
    }
}

pub fn lb_interpolation_search(target: u64, input: &[u64]) -> Option<usize> {
    if input.len() == 0 {
        return None;
    }

    if target <= input[0] {
        return Some(0)
    }
    if target > input[input.len() - 1] {
        return None;
    }
    
    let mut lo = 0;
    let mut hi = input.len();

    while lo < hi {
        let denominator = (input[hi - 1] - input[lo]) as i128;
        if denominator == 0 {
            if input[lo] >= target {
                return Some(lo)
            } else {
                return if hi < input.len() { Some(hi) } else { None };
            }
        }

        let index_range = (hi - lo - 1) as i128;
        let numerator = (target as i128) - (input[lo] as i128);
        
        let mut cut = ((lo as i128) + ((numerator * index_range) / denominator)) as usize;
        if cut < lo {cut = lo}
        if cut > (hi - 1) {cut = hi - 1}

        if input[cut] < target {
            lo = cut + 1;
        } else {
            hi = cut;
        }
    }

    if lo < input.len() { Some(lo) } else { None }
}

#[cfg(test)]
mod test {
    use rand::RngCore;

    use crate::infrastructure;

    use super::*;

    #[test]
    fn lb_binary_search_test1() {
        assert_eq!(lb_binary_search(8, &Vec::new()), None);
    }

    #[test]
    fn lb_binary_search_test2() {
        assert_eq!(lb_binary_search(8, &vec![5]), None);
    }

    #[test]
    fn lb_binary_search_test3() {
        assert_eq!(lb_binary_search(5, &vec![5]), Some(0));
    }

    #[test]
    fn lb_binary_search_test4() {
        assert_eq!(lb_binary_search(6, &vec![4, 5, 6, 6, 7, 8, 9]), Some(2));
    }

    #[test]
    fn lb_binary_search_test5() {
        assert_eq!(lb_binary_search(6, &vec![4, 8]), Some(1));
    }

    #[test]
    fn lb_search_test1() {
        let mut rng = rand::rng();
        let mut input = infrastructure::gen_keys(10000, &mut rng);
        input.sort();
        let target = rng.next_u64();

        let res1 = lb_linear_search(target, &input);
        let res2 = lb_binary_search(target, &input);
        
        assert_eq!(res1, res2);
    }

    #[test]
    fn lb_interpolation_search_test1() {
        assert_eq!(lb_interpolation_search(8, &Vec::new()), None);
    }

    #[test]
    fn lb_interpolation_search_test2() {
        assert_eq!(lb_interpolation_search(8, &vec![5]), None);
    }

    #[test]
    fn lb_interpolation_search_test3() {
        assert_eq!(lb_interpolation_search(5, &vec![5]), Some(0));
    }

    #[test]
    fn lb_interpolation_search_test5() {
        assert_eq!(lb_interpolation_search(5, &vec![4, 5, 5, 5]), Some(1));
    }

    #[test]
    fn lb_interpolation_search_test6() {
        assert_eq!(lb_interpolation_search(6, &vec![4, 5, 6, 6, 7, 8, 9]), Some(2));
    }

    #[test]
    fn lb_search_test2() {
        let mut rng = rand::rng();
        let mut input = infrastructure::gen_keys(10000, &mut rng);
        input.sort();
        let target = rng.next_u64();

        let res1 = lb_linear_search(target, &input);
        let res2 = lb_interpolation_search(target, &input);
        
        assert_eq!(res1, res2);
    }


}