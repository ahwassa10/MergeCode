use std::{iter::Peekable, slice::Iter};

pub struct Merge<'a> {
    pub dataset: Vec<Peekable<Iter<'a, i64>>>,
    pub winner_index: usize,
    pub loser_tree: Vec<usize>
}

impl Merge <'_> {
    // Returns the (winner, loser) indexes
    fn choose (
        dataset: &mut [Peekable<Iter<'_, i64>>],
        li: usize,
        ri: usize,
    ) -> (usize, usize) {
        match (dataset[li].peek().copied(), dataset[ri].peek().copied()) {
            (None, None) => (li, ri),
            (Some(_), None) => (li, ri),
            (None, Some(_)) => (ri, li),
            (Some(lv), Some(rv)) => if lv <= rv { (li, ri) } else { (ri, li) } 
        }
    }

    // Returns the index of the winner of the subtree and updates the subtree sturcture in ds.
    fn build_subtree (node: usize, lo: usize, hi: usize, losers: &mut [usize], ds: &mut [Peekable<Iter<'_, i64>>]) -> usize {
        if hi - lo == 1 {
            return lo;
        }

        let mid = (lo+hi) / 2;
        let lnode = 2 * node + 1;
        let rnode = 2 * node + 2;

        let lw = Self::build_subtree(lnode, lo, mid, losers, ds);
        let rw = Self::build_subtree(rnode, mid, hi, losers, ds);

        let (w, l) = Self::choose(ds, lw, rw);
        losers[node] = l;
        w
    }


    fn initialize_loser_tree(ds: &mut [Peekable<Iter<'_, i64>>]) -> (Vec<usize>, usize) {
        let k = ds.len();
        let mut loser_tree = vec![usize::MAX; k - 1];
        println!("init len {}", loser_tree.len());

        let winner_index = Self::build_subtree(0, 0, k, &mut loser_tree, ds);

        (loser_tree, winner_index)
    }

    pub fn new(mut dataset: Vec<Peekable<Iter<'_, i64>>>) -> Merge {
        let (loser_tree, winner_index) = Self::initialize_loser_tree(&mut dataset);
        Merge {dataset, winner_index, loser_tree}
    }

    fn bubble_up(&mut self, dataset_index: usize) -> usize {
        // Gets the loser_tree node corresponding to a dataset index.
        // Note that two dataset indices map to one loser_tree node.
        let mut node = (dataset_index + self.dataset.len() - 2) / 2;
        let mut winner = dataset_index;

        loop {
            let opponent = self.loser_tree[node];
            let (w, l) = Self::choose(&mut self.dataset, winner, opponent);
            self.loser_tree[node] = l;
            winner = w;
            if node == 0 {
                return winner
            }
            // Get the parent of the node
            node = (node - 1) / 2;
        }
    }
}

impl Iterator for Merge<'_> {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        match self.dataset[self.winner_index].next() {
            None => None,
            Some(value) => {
                self.winner_index = self.bubble_up(self.winner_index);
                Some(*value)
            }
        }
    }
}