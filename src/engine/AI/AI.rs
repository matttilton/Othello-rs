/*
This fill is where most of the function that the ai uses are defined.
The actual minimax algorithm isnt defined here for convenience
TODO Make this stronger when playing as black
*/

use crate::engine::enums;
use crate::engine::board;
use crate::engine::AI::tree;
use std::time::Instant;
use fnv::FnvHashMap;

// this get a move using minimax, 
// can choose wether or not to use alphabeta pruning
pub fn get_minimax_move(board: board::Board, mut hashmap: &mut FnvHashMap<(u64, u64), (i16, u8)>) -> u64 {
    let start = Instant::now();
    let mut stats = tree::Stats{nodes_visited: 0, number_of_prunes: 0, cache_hit: 0};
    let mut root_node = tree::Node {
        board_state: board, 
        value: 0,
        position: 3
    };
    let result = root_node.expand(20, &mut stats, true, Vec::new(), true, &mut hashmap);
    let elapsed_time = start.elapsed().as_nanos() as f64 / 1000000000 as f64;
    let nodes_per_second: f64 = stats.nodes_visited as f64 / elapsed_time as f64;
    // let newboard = board
    println!("resultant move: {:#064b}, estimated value: {}", result.0, result.1);
    println!("Number of nodes visited: {}", stats.nodes_visited);
    println!("Nodes per second: {}", nodes_per_second);
    println!("Number of branches pruned: {}", stats.number_of_prunes);
    println!("Number of cache hits: {}", stats.cache_hit);
    return result.0;
}