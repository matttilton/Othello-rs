use crate::engine::board;
use crate::engine::enums;

pub fn main_heuristic(board: &board::Board, player: enums::Player) -> i16 {
    if board.state == enums::State::End {
        let tiles = count_tiles(board, player);
        if tiles > 0 {
            return 30000;
        } else if tiles == 0 {
            return 0;
        } else if tiles < 0 {
            return -30000;
        }
    }
    let count_weight = 1;
    let location_weight = 3;
    (count_tiles(board, player) * count_weight) + (evaluate_locations(board, player) * location_weight)
}

pub fn count_tiles(board: &board::Board, player: enums::Player) -> i16 {
    let black_count: i16 = board.black_bitboard.count_ones() as i16;
    let white_count: i16 = board.white_bitboard.count_ones() as i16;
    
    if player == enums::Player::White {
        (white_count - black_count) as i16
    } else {
        (black_count - white_count) as i16
    }
}

// returns an into from -100 to 100 rating the overall position of the board.
pub fn evaluate_locations(board: &board::Board, player: enums::Player) -> i16{
    // TODO add ability to consider c squares as positive if they are next to an owned corner square.

    let player_pieces;
    let opponent_pieces;
    if player == enums::Player::White {
        player_pieces = board.white_bitboard;
        opponent_pieces = board.black_bitboard;
    } else {
        player_pieces = board.black_bitboard;
        opponent_pieces = board.white_bitboard;
    }

    let corner_bitmask = 0b1000000100000000000000000000000000000000000000000000000010000001;
    let c_square_bitmask = 0b0100001010000001000000000000000000000000000000001000000101000010;
    let x_square_bitmask = 0b0000000001000010000000000000000000000000000000000100001000000000;
    let center_square_bitmask = 0b0000000000000000000000000001100000011000000000000000000000000000;
    let edge_square_bitmask = 0b0011110000000000100000011000000110000001100000010000000000111100;

    let corner_weight = 2;
    let c_weight: i64 = -1;
    let x_weight: i64 = -2;
    let center_weight = 0;
    let edge_weight = 1;

    let corner_value = corner_weight * (corner_bitmask & player_pieces).count_ones() as i64;
    let c_value = c_weight * (c_square_bitmask & player_pieces).count_ones() as i64;
    let x_value = x_weight * (x_square_bitmask & player_pieces).count_ones() as i64;
    let center_value = center_weight * (center_square_bitmask & player_pieces).count_ones() as i64;
    let edge_value = edge_weight * (edge_square_bitmask & player_pieces).count_ones() as i64;

    let o_corner_value = -((corner_weight * (corner_bitmask & opponent_pieces).count_ones() as i64) as i64);
    let o_c_value = -(c_weight * (c_square_bitmask & opponent_pieces).count_ones() as i64);
    let o_x_value = -(x_weight * (x_square_bitmask & opponent_pieces).count_ones() as i64);
    let o_center_value = -((center_weight * (center_square_bitmask & opponent_pieces).count_ones() as i64) as i64);
    let o_edge_value = -((edge_weight * (edge_square_bitmask & opponent_pieces).count_ones() as i64) as i64);

    (corner_value + c_value + x_value + center_value + edge_value + o_c_value + o_center_value + o_corner_value + o_edge_value + o_x_value) as i16
}

// check for tempo in frontiers

// check for parity

// check for stability

// 

// #[test]
// fn test_heuristic() {
//     let board = board::Board::new();
//     let player = enums::Player::Black;
// 	// println!("{}", board);
//     evaluate_locations(&board, player);
// }