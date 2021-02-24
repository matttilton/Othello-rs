
// TODO clean this up.
// TODO rewrite to follow a more rustic style
use std::fmt;
use crate::engine::enums;

#[derive(Debug, Clone, Copy)]
pub struct Move {
	pub row: i8,
	pub col: i8
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Board {
	pub white_bitboard: u64,
	pub black_bitboard: u64,
	pub valid_moves: u64,
	pub turn: enums::Player,
	pub state: enums::State,
	pub turn_number: u8
}

impl Board {
	pub fn new() -> Board {
		let mut initial_board_state = [[enums::Position::Empty; 8]; 8];
		initial_board_state[3][3] = enums::Position::White;
		initial_board_state[4][4] = enums::Position::White;
		initial_board_state[4][3] = enums::Position::Black;
		initial_board_state[3][4] = enums::Position::Black;
		let white_bitboard = 0b0000000000000000000000000001000000001000000000000000000000000000;
		let black_bitboard = 0b0000000000000000000000000000100000010000000000000000000000000000;
		let board = Board {white_bitboard: white_bitboard, black_bitboard: black_bitboard, turn: enums::Player::Black, state: enums::State::Normal, valid_moves: get_valid_moves_bitboard(black_bitboard, white_bitboard), turn_number: 0};
		board
	}

	// TODO swap return value to array of bitboards, may speed it up
	pub fn get_valid_moves(&self) -> Vec<u64> {
		let bitmasks: [u64; 64] = [
			0b0000000000000000000000000000000000000000000000000000000000000001,
			0b0000000000000000000000000000000000000000000000000000000000000010,
			0b0000000000000000000000000000000000000000000000000000000000000100,
			0b0000000000000000000000000000000000000000000000000000000000001000,
			0b0000000000000000000000000000000000000000000000000000000000010000,
			0b0000000000000000000000000000000000000000000000000000000000100000,
			0b0000000000000000000000000000000000000000000000000000000001000000,
			0b0000000000000000000000000000000000000000000000000000000010000000,
			0b0000000000000000000000000000000000000000000000000000000100000000,
			0b0000000000000000000000000000000000000000000000000000001000000000,
			0b0000000000000000000000000000000000000000000000000000010000000000,
			0b0000000000000000000000000000000000000000000000000000100000000000,
			0b0000000000000000000000000000000000000000000000000001000000000000,
			0b0000000000000000000000000000000000000000000000000010000000000000,
			0b0000000000000000000000000000000000000000000000000100000000000000,
			0b0000000000000000000000000000000000000000000000001000000000000000,
			0b0000000000000000000000000000000000000000000000010000000000000000,
			0b0000000000000000000000000000000000000000000000100000000000000000,
			0b0000000000000000000000000000000000000000000001000000000000000000,
			0b0000000000000000000000000000000000000000000010000000000000000000,
			0b0000000000000000000000000000000000000000000100000000000000000000,
			0b0000000000000000000000000000000000000000001000000000000000000000,
			0b0000000000000000000000000000000000000000010000000000000000000000,
			0b0000000000000000000000000000000000000000100000000000000000000000,
			0b0000000000000000000000000000000000000001000000000000000000000000,
			0b0000000000000000000000000000000000000010000000000000000000000000,
			0b0000000000000000000000000000000000000100000000000000000000000000,
			0b0000000000000000000000000000000000001000000000000000000000000000,
			0b0000000000000000000000000000000000010000000000000000000000000000,
			0b0000000000000000000000000000000000100000000000000000000000000000,
			0b0000000000000000000000000000000001000000000000000000000000000000,
			0b0000000000000000000000000000000010000000000000000000000000000000,
			0b0000000000000000000000000000000100000000000000000000000000000000,
			0b0000000000000000000000000000001000000000000000000000000000000000,
			0b0000000000000000000000000000010000000000000000000000000000000000,
			0b0000000000000000000000000000100000000000000000000000000000000000,
			0b0000000000000000000000000001000000000000000000000000000000000000,
			0b0000000000000000000000000010000000000000000000000000000000000000,
			0b0000000000000000000000000100000000000000000000000000000000000000,
			0b0000000000000000000000001000000000000000000000000000000000000000,
			0b0000000000000000000000010000000000000000000000000000000000000000,
			0b0000000000000000000000100000000000000000000000000000000000000000,
			0b0000000000000000000001000000000000000000000000000000000000000000,
			0b0000000000000000000010000000000000000000000000000000000000000000,
			0b0000000000000000000100000000000000000000000000000000000000000000,
			0b0000000000000000001000000000000000000000000000000000000000000000,
			0b0000000000000000010000000000000000000000000000000000000000000000,
			0b0000000000000000100000000000000000000000000000000000000000000000,
			0b0000000000000001000000000000000000000000000000000000000000000000,
			0b0000000000000010000000000000000000000000000000000000000000000000,
			0b0000000000000100000000000000000000000000000000000000000000000000,
			0b0000000000001000000000000000000000000000000000000000000000000000,
			0b0000000000010000000000000000000000000000000000000000000000000000,
			0b0000000000100000000000000000000000000000000000000000000000000000,
			0b0000000001000000000000000000000000000000000000000000000000000000,
			0b0000000010000000000000000000000000000000000000000000000000000000,
			0b0000000100000000000000000000000000000000000000000000000000000000,
			0b0000001000000000000000000000000000000000000000000000000000000000,
			0b0000010000000000000000000000000000000000000000000000000000000000,
			0b0000100000000000000000000000000000000000000000000000000000000000,
			0b0001000000000000000000000000000000000000000000000000000000000000,
			0b0010000000000000000000000000000000000000000000000000000000000000,
			0b0100000000000000000000000000000000000000000000000000000000000000,
			0b1000000000000000000000000000000000000000000000000000000000000000,
		];
		let valid_moves;
		if self.turn == enums::Player::White {
			valid_moves = get_valid_moves_bitboard(self.white_bitboard, self.black_bitboard)
		} else {
			valid_moves = get_valid_moves_bitboard(self.black_bitboard, self.white_bitboard)
		}
		
		// valid_moves = valid_moves & (valid_moves ^ self.white_bitboard);
		if valid_moves & self.white_bitboard & self.black_bitboard != 0{
			println!("error invalid bitboard");
		}
		let mut moves = Vec::new();
		for each in bitmasks.iter() {
			if valid_moves | each == valid_moves {
				moves.push(*each);
			}
		}
		moves
	}

	pub fn place_tile (&mut self, move_bitboard: u64) -> Board {
		// check that the move was valid
		if move_bitboard | self.valid_moves != self.valid_moves {
			return self.clone()
		}

		// check that there is only 1 bit set
		if move_bitboard & move_bitboard-1 != 0 {
			return self.clone()
		}

		// assign boards to the appropriate player
		let mut player_bitboard;
		let mut opponent_bitboard;
		
		if self.turn == enums::Player::White {
			player_bitboard = self.white_bitboard;
			opponent_bitboard = self.black_bitboard;
		} else {
			player_bitboard = self.black_bitboard;
			opponent_bitboard = self.white_bitboard;
		}

		// stores all of the tiles that are captured
		let mut captured_bitboard = 0;

		// shift board right until we hit a player piece, if you hit a wall disregard the shift
		// shift board up until we hit a player piece, if you hit a wall disregard the shift
		// shift board downright until we hit a player piece, if you hit a wall disregard the shift
		// shift board upright until we hit a player piece, if you hit a wall disregard the shift
		for right in 0..4 {
			let mut x = (move_bitboard >> SHIFTS[right]) & opponent_bitboard & AVOID_WRAP_LEFT[right];
			x |= (x >> SHIFTS[right]) & opponent_bitboard & AVOID_WRAP_LEFT[right];
			x |= (x >> SHIFTS[right]) & opponent_bitboard & AVOID_WRAP_LEFT[right];
			x |= (x >> SHIFTS[right]) & opponent_bitboard & AVOID_WRAP_LEFT[right];
			x |= (x >> SHIFTS[right]) & opponent_bitboard & AVOID_WRAP_LEFT[right];
			x |= (x >> SHIFTS[right]) & opponent_bitboard & AVOID_WRAP_LEFT[right];

			// some magic that somehow calculates the disk that the ray stops on
			let bounding_disk = (x >> SHIFTS[right]) & player_bitboard & AVOID_WRAP_LEFT[right];
			captured_bitboard |= match bounding_disk {
				0 => 0,
				_ => x
			}
		}
		// shift board downleft until we hit a player piece, if you hit a wall disregard the shift
		// shift board down until we hit a player piece, if you hit a wall disregard the shift
		// shift board upleft until we hit a player piece, if you hit a wall disregard the shift
		// shift board left until we hit a player piece, if you hit a wall disregard the shift
		for left in 0..4 {
			let mut x = (move_bitboard << SHIFTS[left]) & opponent_bitboard & AVOID_WRAP_RIGHT[left];

			x |= (x << SHIFTS[left]) & opponent_bitboard & AVOID_WRAP_RIGHT[left];
			x |= (x << SHIFTS[left]) & opponent_bitboard & AVOID_WRAP_RIGHT[left];
			x |= (x << SHIFTS[left]) & opponent_bitboard & AVOID_WRAP_RIGHT[left];
			x |= (x << SHIFTS[left]) & opponent_bitboard & AVOID_WRAP_RIGHT[left];
			x |= (x << SHIFTS[left]) & opponent_bitboard & AVOID_WRAP_RIGHT[left];

			// some magic that somehow calculates the disk that the ray stops on
			let bounding_disk = (x << SHIFTS[left]) & player_bitboard & AVOID_WRAP_RIGHT[left];
			captured_bitboard |= match bounding_disk {
				0 => 0,
				_ => x
			}
			// captured_bitboard |= x;
		}
		// This gets us a list of all the possibly valid lines of attack

		// and this with the opponent bitboard to get the tiles that are captured

		// captured or current is the final new bitboard for the current player
		
		// captured xor opponent is the final new bitboard for the opponent
		
		player_bitboard |= captured_bitboard | move_bitboard;
		opponent_bitboard -= player_bitboard & opponent_bitboard;
		let mut valid_moves;
		let mut state = enums::State::Normal;
		let mut turn = !self.turn;
		// println!("{}", turn);
		valid_moves = get_valid_moves_bitboard(opponent_bitboard, player_bitboard);
		if valid_moves == 0 {
			state = enums::State::SkippedSingle;
			turn = !turn;
			valid_moves = get_valid_moves_bitboard(player_bitboard, opponent_bitboard);
			if valid_moves == 0 {
				state = enums::State::End;
				turn = !turn;
			}
			if turn == enums::Player::White {
				return Board {white_bitboard: player_bitboard, black_bitboard: opponent_bitboard, turn: turn, valid_moves: valid_moves, state: state, turn_number: self.turn_number + 1};
			} else {
				return Board {white_bitboard: opponent_bitboard, black_bitboard: player_bitboard, turn: turn, valid_moves: valid_moves, state: state, turn_number: self.turn_number + 1};
			}
		}
		// println!("{}", opponent_bitboard);
		// println!("{}", player_bitboard);
		if turn == enums::Player::White {
			Board {white_bitboard: opponent_bitboard, black_bitboard: player_bitboard, turn: turn, valid_moves: valid_moves, state: state, turn_number: self.turn_number + 1}
		} else {
			Board {white_bitboard: player_bitboard, black_bitboard: opponent_bitboard, turn: turn, valid_moves: valid_moves, state: state, turn_number: self.turn_number + 1}
		}
		
	}

	pub fn get_board(&self) -> [[enums::Position; 8]; 8] {
		let mut board = [[enums::Position::Empty; 8]; 8];
		let mut count = 0;
		let mut white = self.white_bitboard;
		let mut black = self.black_bitboard;
		let mut legal = self.valid_moves;
		while count < 64 {
			if white != 0 {
				if white % 2 == 1{
					let row: usize = (count / 8) as usize;
					let col: usize = (count % 8) as usize;
					board[row][col] = enums::Position::White;
				}
				white = white >> 1;
			}
			if black != 0 {
				if black % 2 == 1{
					let row: usize = (count / 8) as usize;
					let col: usize = (count % 8) as usize;
					board[row][col] = enums::Position::Black;
				}
				black = black >> 1;
			}
			if legal != 0 {
				if legal % 2 == 1{
					let row: usize = (count / 8) as usize;
					let col: usize = (count % 8) as usize;
					board[row][col] = enums::Position::Valid;
				}
				legal = legal >> 1;
			}
			count += 1;
		}
		return board;
    }
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut board = [[enums::Position::Empty; 8]; 8];
		let mut count = 0;
		let mut white = self.white_bitboard;
		let mut black = self.black_bitboard;
		let mut legal = self.valid_moves;
		while count < 64 {
			if white != 0 {
				if white % 2 == 1{
					let row: usize = (count / 8) as usize;
					let col: usize = (count % 8) as usize;
					board[row][col] = enums::Position::White;
				}
				white = white >> 1;
			}
			if black != 0 {
				if black % 2 == 1{
					let row: usize = (count / 8) as usize;
					let col: usize = (count % 8) as usize;
					board[row][col] = enums::Position::Black;
				}
				black = black >> 1;
			}
			if legal != 0 {
				if legal % 2 == 1{
					let row: usize = (count / 8) as usize;
					let col: usize = (count % 8) as usize;
					board[row][col] = enums::Position::Valid;
				}
				legal = legal >> 1;
			}
			count += 1;
		}
		let mut string = String::with_capacity(143);
		string.push_str("  A B C D E F G H\n");
		for row in 0..8 {
			string = format!("{}{}|", string, row + 1);
			for col in 0..8 {
				match board[row][col] {
					enums::Position::Empty => string.push_str(" |"),
					enums::Position::White => string.push_str("●|"),
					enums::Position::Black => string.push_str("○|"),
					enums::Position::Valid => string = format!("{}x|", string)
				}
			}
			string.push_str("\n");
		}
		write!(f, "{}", string)
    }
}

pub fn get_index_from_move(move_: Move) -> u32 {
	(move_.row*8 + move_.col) as u32
}
pub fn get_bitmask_for_index(index: u32) -> u64 {
	2u64.pow(index)
}

// get a bitboard with the valid moves
// TODO currently uses dumb7fill may want to replace with the kooge_stone algorithm at some point
pub fn get_valid_moves_bitboard(gen: u64, pro: u64) -> u64 {
	let mut moves = 
	  dumb7fill_generic_right(gen, pro, 0)
	| dumb7fill_generic_right(gen, pro, 1)
	| dumb7fill_generic_right(gen, pro, 2)
	| dumb7fill_generic_right(gen, pro, 3)
	| dumb7fill_generic_left(gen, pro, 0)
	| dumb7fill_generic_left(gen, pro, 1)
	| dumb7fill_generic_left(gen, pro, 2)
	| dumb7fill_generic_left(gen, pro, 3);
	moves = moves - (moves & gen);
	moves = moves & (moves ^ gen);
	moves = moves & (moves ^ pro);
	moves
}

pub fn print_bitboard(board: u64) {
	println!("{}", Board {
		white_bitboard: board,
		black_bitboard: 0,
		valid_moves: 0,
		turn: enums::Player::White,
		state: enums::State::Normal,
		turn_number: 0
	})
}

// positve left, negative right shifts
static SHIFTS: [i8; 4] = [9, 1, 7, 8];

static AVOID_WRAP_LEFT: [u64; 4] =
[
   
/*
	* |0|0|0|0|0|0|0|0|
	* |0|1|1|1|1|1|1|1|
	* |0|1|1|1|1|1|1|1|
	* |0|1|1|1|1|1|1|1|
	* |0|1|1|1|1|1|1|1|
	* |0|1|1|1|1|1|1|1|
	* |0|1|1|1|1|1|1|1|
	* |0|1|1|1|1|1|1|1|
	*/
   0x007f7f7f7f7f7f7f, // south west
   /*
   0000000011111111111111111111111111111111111111111111111111111111
   0000000000000000000000000000000000000000000000000000000000000000
   |0|1|1|1|1|1|1|1|
   |0|1|1|1|1|1|1|1|
   |0|1|1|1|1|1|1|1|
   |0|1|1|1|1|1|1|1|
   |0|1|1|1|1|1|1|1|
   |0|1|1|1|1|1|1|1|
   |0|1|1|1|1|1|1|1|
   |0|1|1|1|1|1|1|1|
   */
   0x7f7f7f7f7f7f7f7f, // west
   /*
	|0|0|0|0|0|0|0|0|
	|1|1|1|1|1|1|1|0|
	|1|1|1|1|1|1|1|0|
	|1|1|1|1|1|1|1|0|
	|1|1|1|1|1|1|1|0|
	|1|1|1|1|1|1|1|0|
	|1|1|1|1|1|1|1|0|
	|1|1|1|1|1|1|1|0|
   */
   0x00fefefefefefefe, // south east
   /*
   |0|0|0|0|0|0|0|0|
   |1|1|1|1|1|1|1|1|
   |1|1|1|1|1|1|1|1|
   |1|1|1|1|1|1|1|1|
   |1|1|1|1|1|1|1|1|
   |1|1|1|1|1|1|1|1|
   |1|1|1|1|1|1|1|1|
   |1|1|1|1|1|1|1|1|
   */
   0x00ffffffffffffff, // south
];

static AVOID_WRAP_RIGHT: [u64; 4] = [
	0xfefefefefefefe00, // north east
	0xfefefefefefefefe, // east
	0x7f7f7f7f7f7f7f00, // north west
   	0xffffffffffffff00, // north
];

// fills to the south according to the dumb7fill algorithm. 
// the loop was unrolled to prevent branch prediction.
// It may be faster to detect if the loop finishes early but this should be plenty fast for now.
// gen is players pieces
// pro is opponent pieces
fn dumb7fill_generic_right(mut gen: u64, mut pro: u64, dirindex: usize) -> u64 {
	let dir = SHIFTS[dirindex];
	let wrap = AVOID_WRAP_RIGHT[dirindex];
	pro &= wrap;
	
	let mut gen_orig = gen;
	let mut flood = gen;
	
	flood |= gen;
	flood &= wrap;
	gen = ((gen >> dir)) & pro;
	
	flood |= gen;
	flood &= wrap;
	gen = ((gen >> dir)) & pro;
	
	flood |= gen;
	flood &= wrap;
	gen = ((gen >> dir)) & pro;
	
	flood |= gen;
	flood &= wrap;
	gen = ((gen >> dir)) & pro;
	
	flood |= gen;
	flood &= wrap;
	gen = ((gen >> dir)) & pro;
	
	flood |= gen;
	flood &= wrap;
	gen = ((gen >> dir)) & pro;

	flood |= ((gen >> dir)) & pro;
	flood &= wrap;
	gen_orig = gen_orig >> dir;
  	
	return ((flood >> dir)) & !gen_orig;
}

fn dumb7fill_generic_left (mut gen: u64, mut pro: u64, dirindex: usize) -> u64 {
	let dir = SHIFTS[dirindex];
	let wrap = AVOID_WRAP_LEFT[dirindex];
	
	pro &= wrap;
	
	let mut gen_orig = gen;
	let mut flood = gen;
	
	flood |= gen;
	flood &= wrap;
	gen = (gen << dir) & pro;
	
	flood |= gen;
	flood &= wrap;
	gen = (gen << dir) & pro;
	
	flood |= gen;
	flood &= wrap;
	gen = (gen << dir) & pro;
	
	flood |= gen;
	flood &= wrap;
	gen = (gen << dir) & pro;
	
	flood |= gen;
	flood &= wrap;
	gen = (gen << dir) & pro;
	
	flood |= gen;
	flood &= wrap;
	gen = (gen << dir) & pro;
	
	flood |= ((gen << dir)) & pro;
	flood &= wrap;
	gen_orig = gen_orig << dir;
  	
	return ((flood << dir)) & !gen_orig;
}


#[cfg(test)]
mod tests {
	use super::*;
	use crate::engine::enums::*;

    #[test]
	fn test_missing_valid_move_vertical() {
		/************
		 * Black has a missing valid move in this position denoted by m
		 * 
		 * |b|b|b|b|b|b|b|b|
		 * |b|w|b|b|b|b|b|b|
		 * |b|w|b|b|b|b|b|b|
		 * |b|w|b|b|b|b| |b|
		 * |v|w|v|b|b|b| | |
		 * |v|w|v| |b|b| | |
		 * | |m| | | | | | |
		 * | | | | | | | | |
		 ***********/
		let white_bitboard: u64 = 0b0000000000000000000000100000001000000010000000100000001000000000;
		let black_bitboard: u64 = 0b0000000000000000001100000011100010111101111111011111110111111111;

		let board: Board = Board {
			white_bitboard: white_bitboard,
			black_bitboard: black_bitboard,
			turn: Player::Black,
			state: State::Normal,
			turn_number: 33,
			valid_moves: get_valid_moves_bitboard(black_bitboard, white_bitboard)
		};

		println!("{}", board);
	}
	
	// #[test]
	// fn test_missing_valid_move_vertical2() {
	// 	/************
	// 	 * Black has a missing valid move in this position denoted by m
	// 	 * 
	// 	 * |b|b|b|b|b|b|b|b|
	// 	 * |b|w|b|b|b|b|b|b|
	// 	 * |b|w|b|b|b|b|b|b|
	// 	 * |b|w|b|b|b|b| |b|
	// 	 * |v|w|v|b|b|b| | |
	// 	 * |v|w|v| |b|b| | |
	// 	 * | |m| | | | | | |
	// 	 * | | | | | | | | |
	// 	 ***********/

	// 	let board: Board = Board::new();
	// 	// let board = board.place_tile(get_bitmask_for_index(get_index_from_move(Move {row: 0, col: 0})))

	// 	println!("{}", board);
    // }
}