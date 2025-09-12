#![allow(dead_code)]

use std::{iter::Peekable, slice::Iter};

pub enum BinaryTree {
    Empty,
    NonEmpty(Box<TreeNode>)
}

pub struct TreeNode {
    dataset_index: usize,
    left: BinaryTree,
    right: BinaryTree
}

impl BinaryTree {
    pub fn new_trivial_bt(dataset_index: usize) -> BinaryTree {
        BinaryTree::NonEmpty(Box::new(TreeNode {
            dataset_index,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty
        } ))
    }

    pub fn new_bt(dataset_index: usize, left: BinaryTree, right: BinaryTree) -> BinaryTree {
        BinaryTree::NonEmpty(Box::new(TreeNode {
            dataset_index,
            left,
            right
        } ))
    }
}

pub fn print_binary_tree(bt: &BinaryTree) {
    fn rec_helper(bt: &BinaryTree) {
        match bt {
            BinaryTree::Empty => (),
            BinaryTree::NonEmpty(tree_node) => {
                print!("{}", tree_node.dataset_index);
                rec_helper(&tree_node.left);
                rec_helper(&tree_node.right);
            }
        }
    }
    rec_helper(bt);
    println!("");
}

pub fn bt_to_vec(bt: &BinaryTree) -> Vec<usize> {
    fn rec_helper(bt: &BinaryTree, v: &mut Vec<usize>) {
        match bt {
            BinaryTree::Empty => (),
            BinaryTree::NonEmpty(tree_node) => {
                v.push(tree_node.dataset_index);
                rec_helper(&tree_node.left, v);
                rec_helper(&tree_node.right, v);
            }
        }
    }
    let mut v = Vec::new();
    rec_helper(bt, &mut v);
    v
}

pub fn build_tree<T: PartialOrd>(data: &Vec<Option<T>>, li: usize, ri: usize) -> (BinaryTree, usize) {
    match ri - li {
        0 | 1 => panic!("data must be atleast len 2"),
        2     => match (&data[li], &data[ri-1]) {
            (None, None)       => (BinaryTree::new_trivial_bt(li), ri-1),
            (None, Some(_))    => (BinaryTree::new_trivial_bt(li), ri-1),
            (Some(_), None)    => (BinaryTree::new_trivial_bt(ri-1), li),
            (Some(l), Some(r)) => if l < r {
                    (BinaryTree::new_trivial_bt(ri-1), li)
                } else {
                    (BinaryTree::new_trivial_bt(li), ri-1)
                }
            }
        _    => {
            let (lt, lw) = build_tree(data, li, ri/2);
            let (rt, rw) = build_tree(data,ri/2, ri);
            match (&data[lw], &data[rw]) {
                (None, None)       => (BinaryTree::new_bt(lw, lt, rt), rw),
                (None, Some(_))    => (BinaryTree::new_bt(lw, lt, rt), rw),
                (Some(_), None)    => (BinaryTree::new_bt(rw, lt, rt), lw),
                (Some(l), Some(r)) => if l < r {
                        (BinaryTree::new_bt(rw, lt, rt), lw)
                    } else {
                        (BinaryTree::new_bt(lw, lt, rt), rw)
                    }
            }
        }
    }
}

pub fn bubble_up(loser_tree: &mut Vec<usize>, dataset_index: usize, dataset: &mut Vec<Peekable<Iter<i64>>>) -> Option<usize> {
    fn choose(a_index: usize, b_index: usize, dataset: &mut Vec<Peekable<Iter<i64>>>) -> Option<(usize, usize)> {
        let a = dataset[a_index].peek().copied();
        let b = dataset[b_index].peek().copied();

        match (a, b) {
            (None, None) => None,
            (Some(_), None) => Some((a_index, b_index)),
            (None, Some(_)) => Some((b_index, a_index)),
            (Some(av), Some(bv)) => {
                if av <= bv {
                    Some((a_index, b_index))
                } else {
                    Some((b_index, a_index))
                }
            }
        }
    }

    let mut node = (dataset_index + dataset.len() - 2) / 2;
    let mut winner = dataset_index;

    loop {
        let opponent = loser_tree[node];
        let Some((w, l)) = choose(winner, opponent, dataset) else {return None;};
        loser_tree[node] = l;
        winner = w;
        if node == 0 {
            return Some(winner);
        }
        node = (node - 1) / 2;
    }
}