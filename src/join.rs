use std::cmp::Ordering;

use crate::tuple::Tuple;

fn nested_loop_join(left: &Vec<Tuple>, right: &Vec<Tuple>) -> Vec<Tuple> {
    let mut output = Vec::new();

    for lt in left {
        for rt in right {
            if lt.key == rt.key {
                output.push(Tuple::new(lt.key, rt.payload));
            }
        }
    }

    output
}

fn basic_sort_merge_join(mut left: Vec<Tuple>, mut right: Vec<Tuple>) -> Vec<Tuple> {
    left.sort_by_key(|t| t.key);
    right.sort_by_key(|t| t.key);

    let mut li = 0;
    let mut ri = 0;
    let mut output = Vec::new();
    
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

                for _ in l_start..li {
                    for j in r_start..ri {
                        output.push(Tuple::new(key, right[j].payload));
                    }
                }
            }
        }
    }

    output
}

#[cfg(test)]
mod test {

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
        assert_eq!(output.next(), Some(Tuple::new(2, 36)));
        assert_eq!(output.next(), Some(Tuple::new(2, 18)));
        assert_eq!(output.next(), Some(Tuple::new(2, 8)));
        assert_eq!(output.next(), Some(Tuple::new(5, 15)));
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
        assert_eq!(output.next(), Some(Tuple::new(5, 15)));
        assert_eq!(output.next(), Some(Tuple::new(2, 36)));
        assert_eq!(output.next(), Some(Tuple::new(2, 18)));
        assert_eq!(output.next(), Some(Tuple::new(2, 8)));
        assert_eq!(output.next(), None);
    }
}