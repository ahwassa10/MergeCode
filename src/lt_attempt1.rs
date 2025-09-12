#![allow(dead_code)]

use core::fmt;
use std::fmt::Debug;

#[derive(Clone, Debug)]
enum Node {
    IndexNode(usize),
    LoserNode
}

struct LoserTree {
    // The first element of each run.
    // Note: this vector stores actual values, not indexes.
    heads: Vec<i64>,

    // An array representation of a binary tree
    // loser[0] is the winner, and loser[1..k-1]
    // are the losers.
    // Note: this vector stores indexes.
    losers: Vec<Node>,

    runs: Vec<Vec<i64>>
}

impl Debug for LoserTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LoserTree {{ Heads: {:?}, ", self.heads)?;
        write!(f, "losers: [")?;
        let mut first = true;
        for n in &self.losers {
            match n {
                Node::IndexNode(index) => {
                    if !first {
                        write!(f, ", ")?
                    }
                    write!(f, "IndexNode({})->{}", *index, self.heads[*index])?;
                    first = false;
                }
                Node::LoserNode => {
                    write!(f, "LoserNode(inf)")?;
                }
            };
        }
        write!(f, "] }}")
    }
}

impl LoserTree {
    fn new(mut runs: Vec<Vec<i64>>) -> LoserTree {
        // For development only
        assert!(runs.len() == 4);
        for run in runs.iter_mut() {
            run.reverse();
        }

        let mut heads = vec![0; runs.len()];
        let mut losers = vec![Node::LoserNode; runs.len()];
        for i in 0..runs.len() {
            heads[i] = runs[i].pop().expect("Each slice should have at least one element");
        }
        Self::initialize_losers(&mut heads, &mut losers);

        let lt = LoserTree {heads, losers, runs};
        lt
    }

    fn initialize_losers(heads: &Vec<i64>, losers: &mut Vec<Node>) {
        let mut winners: Vec<usize> = vec![0; heads.len()];
        
        for i in (1..losers.len()).rev() {
            let left_child_index_index = 2 * i;
            let left_child_index = if left_child_index_index >= losers.len() {
                left_child_index_index - losers.len()
            } else {
                winners[left_child_index_index]
            };
            let left_child = heads[left_child_index];
            
            let right_child_index_index = 2 * i + 1;
            let right_child_index = if right_child_index_index >= losers.len() {
                right_child_index_index - losers.len()
            } else {
                winners[right_child_index_index]
            };
            let right_child = heads[right_child_index];

            if left_child < right_child {
                winners[i] = left_child_index;
                losers[i] = Node::IndexNode(right_child_index);
            } else {
                winners[i] = right_child_index;
                losers[i] = Node::IndexNode(left_child_index);
            }
            
            /*
            println!("i {}", i);
            println!("lci {} rci {}", left_child_index, right_child_index);
            println!("lc {} rc {}", left_child, right_child);
            println!("{:?}", losers);
            */
        }
        // recall that losers[0] is the winner overall
        losers[0] = Node::IndexNode(winners[1]);
    }

    fn winner(&self) -> i64 {
        match self.losers[0] {
            Node::IndexNode(index) => self.heads[index],
            Node::LoserNode => i64::MAX
        }
    }

    fn next(&mut self) -> Option<i64> {
        match self.losers[0] {
            Node::IndexNode(index) => {
                let winner = self.heads[index];
                self.losers[0] = Node::LoserNode;
                match self.runs[index].pop() {
                    Some(next_value) => self.heads[index] = next_value,
                    None => self.heads[index] = i64::MAX
                }
                //self.replay(index);
                Option::Some(winner)
            },
            Node::LoserNode => {
                Option::None
            }
        }
    }

    /*
    // Replays the loser tree from the specified index by
    // traversing up the tree to root.
    fn replay(&mut self, index: usize) {
        let mut contestant_index = index;
        let mut contestant = self.heads[index];
        println!("ci: {}", contestant_index);

        let mut parent_index = (index + self.losers.len()) / 2;
        println!("pi: {}", parent_index);
        println!("{:?}", self.losers);
        while parent_index >= 1 {
            let opponent = self.losers[parent_index];

            match opponent {
                Node::IndexNode(opponent_index) => {
                    println!("con_i: {}, opp_i: {}", contestant_index, opponent_index);
                    let opponent = self.heads[opponent_index as usize];
                    println!("{}, {}", contestant, opponent);
                    if opponent < contestant {
                        println!("Opponent {} beats contestant {}", opponent, contestant);
                        // Opponent wins; contestant is the loser
                        self.losers[parent_index] = contestant_index;
                        contestant = opponent;
                        contestant_index = opponent_index as usize;
                    } else {
                        println!("Contestant {} beats opponent {}", contestant, opponent);
                    }
                },
                Node::LoserNode => {
                    println!("Contestant {} beat LoseNode", contestant)
                }
            }
            
            // Gets the next node up the tree
            parent_index /= 2;
            println!("{:?}", self.losers);
        }
        self.losers[0] = contestant_index;
        println!("{:?}", self.losers);
    }*/
}
