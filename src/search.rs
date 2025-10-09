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

pub fn lb_linear_search<T: Ord>(target: T, input: &[T]) -> Option<usize> {
    for i in 0..input.len() {
        if target <= input[i] {
            return Some(i)
        }
    }
    return None
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
    fn lb_search_test1() {
        let mut rng = rand::rng();
        let mut input = infrastructure::gen_keys(10000, &mut rng);
        input.sort();
        let target = rng.next_u64();

        let res1 = lb_linear_search(target, &input);
        let res2 = lb_binary_search(target, &input);
        
        assert_eq!(res1, res2);
    }
}