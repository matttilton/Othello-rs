use crate::engine::board;
use crate::engine::AI::AI;
use crate::engine::AI::heuristics;
use crate::engine::enums;
use rayon::prelude::*;
use std::cmp::Ordering;
use fnv::FnvHashMap;

// Struct used to keep track of stats
// TODO think of more stats to track
#[derive(Copy, Clone)]
pub struct Stats {
    pub nodes_visited: u64,
    pub number_of_prunes: u64,
    pub cache_hit: u64
}

// Each node of the tree
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Node {
    pub board_state: board::Board,
    pub value: i16,
    pub position: u64
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PositionEvaluation {
    value: i16,
    significance: u8
}

impl<'a> Node {
    pub fn expand(&mut self, iterations: u8, stats: &mut Stats, maximize: bool, siblings: Vec<Node>, move_ordering: bool, hashmap: &mut FnvHashMap<(u64, u64), (i16, u8)>) -> (u64, i16) {
        // if move_ordering && hashmap.len() != 0 {
        //     hashmap.clear();
        // }
        // Gets all valid moves from the position
        let valid_moves = self.board_state.get_valid_moves();

        // Quick check to see if there are no valid moves
        if valid_moves.len() == 0 {
            return (3, heuristics::main_heuristic(&self.board_state, enums::Player::White));
        }

        // let length_of_valid_moves = number_of_set_bits(valid_moves);
        stats.nodes_visited = stats.nodes_visited + valid_moves.len() as u64;
        let mut max_value = -30001;
        let mut min_value = 30001;
        let mut max_index = 0;
        let mut min_index = 0;
        let mut children: Vec<Node> = Vec::new();
        let mut children_to_evaluate: Vec<Node> = Vec::new();

        // check the cache to see if any of the moves have been calculated before
        // perform a shallow search to determine if there are any obviously bad moves.
        if iterations != 0 && move_ordering {
            for position in 0..valid_moves.len() {
                let new_board_state = self.board_state.place_tile(valid_moves[position]);
                let mut value = heuristics::main_heuristic(&new_board_state, enums::Player::White);
                let mut node = Node {board_state: new_board_state, value, position: valid_moves[position]}; // make new node
                let possibly_already_calculated = match hashmap.get(&(node.board_state.white_bitboard, node.board_state.black_bitboard)) {
                    None => (0, 0),
                    Some(t) => (t.0, t.1)
                };

                if possibly_already_calculated.1 != 0 {
                    stats.cache_hit += 1;
                }

                if possibly_already_calculated.1 <= node.board_state.turn_number + 15 {
                    node.value = node.expand(15, stats, !maximize, vec![], false, hashmap).1;
                    hashmap.insert((node.board_state.white_bitboard, node.board_state.black_bitboard), (node.value, node.board_state.turn_number + 15));
                } else {
                    node.value = possibly_already_calculated.0;
                }
                children_to_evaluate.push(node);
            }
            // sort the moves with the best first to help the alpha-beta pruning. use the values generated in the previous step.
            children_to_evaluate.sort();
            // let truncate_point = children_to_evaluate.len() / 2;
            // children_to_evaluate.truncate(truncate_point);
            let mut max = -30001;
            for each in children_to_evaluate.clone() {
                if each.value > max {
                    max = each.value;
                }
            }
            let mut tmp: Vec<Node> = Vec::new();
            let cutoff = (max as f32 - (max.abs() as f32 * 0.7)).floor() as i16;
            for each in children_to_evaluate.clone() {
                if each.value >= cutoff {
                    tmp.push(each);
                }
            }
            if tmp.len() == 0 {
                println!("cutoff: {}", cutoff);
                println!("{:?}", children_to_evaluate.clone());
            }
            children_to_evaluate = tmp;
        } else {
            for position in 0..valid_moves.len() {
                let new_board_state = self.board_state.place_tile(valid_moves[position]);
                let mut value = heuristics::main_heuristic(&new_board_state, enums::Player::White);;
                let mut node = Node {board_state: new_board_state, value, position: valid_moves[position]};
                children_to_evaluate.push(node);
            }
        }

        for position in 0..children_to_evaluate.len() {
            // alpha-beta pruning. I know this works.
            if maximize { // if on a maximizing layer
                let mut max = -31000;
                for each in &siblings {
                    if each.value > max {
                        max = each.value;
                    }
                }
                if max > self.value {
                    stats.number_of_prunes = stats.number_of_prunes + 1;
                    return (children_to_evaluate[position].position, self.value);
                }
            } else { // on a minimizing layer
                let mut min = 31000; 
                for each in &siblings {
                    if each.value < min {
                        min = each.value;
                    }
                }
                if min < self.value {
                    stats.number_of_prunes = stats.number_of_prunes + 1;
                    return (children_to_evaluate[position].position, self.value);
                }
            }

            let mut node = children_to_evaluate[position];
            let possibly_already_calculated = match hashmap.get(&(node.board_state.white_bitboard, node.board_state.black_bitboard)) {
                    None => (0, 0),
                    Some(t) => (t.0, t.1)
            };
            if possibly_already_calculated.1 != 0 {
                stats.cache_hit += 1;
            }
            if possibly_already_calculated.1 < iterations {
                if iterations != 0 {
                    node.value = node.expand(iterations - 1, stats, !maximize, children.clone(), move_ordering, hashmap).1; // recursive bit expands the child if there are more than 0 iterations left
                }
            } else {
                node.value = possibly_already_calculated.0;
            }
            if iterations == 0 {
                node.value = heuristics::main_heuristic(&node.board_state, enums::Player::White);
            }
            children.push(node);
            let mut significance = self.board_state.turn_number;
            if significance < node.board_state.turn_number {
                significance = node.board_state.turn_number;
            }
            hashmap.insert((node.board_state.white_bitboard, node.board_state.black_bitboard), (node.value, significance));
            // determine the value of the node based on the value of the children
            if maximize {
                if node.value > max_value {
                    max_value = node.value;
                    max_index = position;
                }
            } else {
                if node.value < min_value {
                    min_value = node.value;
                    min_index = position;
                }
            }
        }
        // if iterations == 9 {
        //     hashmap.drain();
        // }

        if maximize {
            (children_to_evaluate[max_index].position, max_value)
        } else {
            (children_to_evaluate[min_index].position, min_value)
        }
    }

    // first naieve implementation of multithreading
    // pub fn expand_multi(&mut self, iterations: u8, stats: &mut Stats, maximize: bool, siblings: Vec<Node>, move_ordering: bool, hashmap: &mut FnvHashMap<(u64,u64), PositionEvaluation>) -> (u64, i16) {
    //     // Gets all valid moves from the position
    //     let valid_moves = self.board_state.get_valid_moves();

    //     // Quick check to see if there are no valid moves
    //     if valid_moves.len() == 0 {
    //         return (3, heuristics::count_tiles(&self.board_state, enums::Player::White));
    //     }
        
    //     let mut children: Vec<Node> = Vec::new();
    //     for position in 0..valid_moves.len() {
    //         let new_board_state = self.board_state.place_tile(valid_moves[position]);
    //         let mut node = Node {board_state: new_board_state, value: 0, position: valid_moves[position]};
    //         children.push(node);
    //     }

    //     let results: Vec<(u64, i16)> = children.into_par_iter().map(|mut node| node.expand(iterations, &mut stats.clone(), maximize, siblings.clone(), move_ordering, hashmap)).collect();
    //     // println!("{:?}", results);
    //     let mut maxindex = (0,0);
    //     let mut maxvalue = -30000;
    //     for each in results {
    //         if each.1 > maxvalue {
    //             maxvalue = each.1;
    //             maxindex = each;
    //         }
    //     }
    //     maxindex
    // }
}

pub fn number_of_set_bits(i: u64) -> u64 {
    i.count_ones() as u64
}