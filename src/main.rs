use std::{iter::Peekable, slice::Iter};

use crate::merge::Merge;

mod merge;

fn main() {
    let data = vec![vec![3, 7, 11], vec![2, 2, 6, 9], vec![1, 3, 8, 10, 11], vec![5, 12, 14]];
    
    let dataset: Vec<Peekable<Iter<i64>>> = data.iter().map(|v| v.iter().peekable()).collect();
    
    let mut m = Merge::new(dataset);
    println!("Initial Tree: {:?}", m.loser_tree);
    println!("Initial winner: {:?}", m.winner_index);

    println!("First: {:?}", m.next());
    println!("Second: {:?}", m.next());
    println!("Third: {:?}", m.next());
    println!("Fourth: {:?}", m.next());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = vec![vec![3], vec![2, 2], vec![1], vec![5]];
        let dataset: Vec<Peekable<Iter<i64>>> = data.iter().map(|v| v.iter().peekable()).collect();

        let mut m = Merge::new(dataset);
        assert_eq!(Some(1), m.next());
        assert_eq!(Some(2), m.next());
        assert_eq!(Some(2), m.next());
        assert_eq!(Some(3), m.next());
        assert_eq!(Some(5), m.next());
        assert_eq!(None, m.next());
        assert_eq!(None, m.next());
    }

    #[test]
    fn test2() {
        let data = vec![vec![3, 7, 11], vec![2, 2, 6, 9], vec![1, 3, 8, 10, 11], vec![5, 12, 14]];
        let dataset: Vec<Peekable<Iter<i64>>> = data.iter().map(|v| v.iter().peekable()).collect();
        let m = Merge::new(dataset);

        let res: Vec<i64> = m.collect();
        assert_eq!(vec![1, 2, 2, 3, 3, 5, 6, 7, 8, 9, 10, 11, 11, 12, 14], res);
    }

    #[test]
    fn test3() {
        let data = vec![vec![], vec![2, 2, 6, 9], vec![1, 3, 8, 10, 11], vec![5, 12, 14]];
        let dataset: Vec<Peekable<Iter<i64>>> = data.iter().map(|v| v.iter().peekable()).collect();
        let m = Merge::new(dataset);

        let res: Vec<i64> = m.collect();
        assert_eq!(vec![1, 2, 2, 3, 5, 6, 8, 9, 10, 11, 12, 14], res);
    }

}