use crate::engine::board;
use crate::engine::enums;
use board::{Board, Move};
use std::io::{stdin,stdout,Write};
use crate::engine::AI::AI;
use fnv::FnvHashMap;

// this is the main loop for the ui
pub fn tui() {
	println!("Welcome to Othello");
    let mut hashmap: FnvHashMap<(u64, u64), (i16, u8)> = FnvHashMap::default();
    hashmap.reserve(100000000);
	let mut players = 0;
	let mut board = Board::new();
	let mut AI = 0;
	let mut difficulty = 0;
	// get the number of human players. repeat until you get valid input
	while players == 0 {
		print!("How many human players: ");
		let mut s = String::new();
		let _=stdout().flush();
	    stdin().read_line(&mut s).expect("Did not enter a correct string");
		players = match s[0..1].parse::<u8>() {
			Ok(r) => if r == 1 {
				r
			} else if r == 2 {
				r
			} else {
				0
			},
			Err(r) => 0
		};
		println!("{}", players)
	}
	// if playing with 1 player loop until valid input for ai
	if players == 1 {
		while AI == 0 {
			print!("What type of AI?
1 -> Random
2 -> MiniMax
3 -> MiniMax with Alpha-Beta Pruning

: ");
			let mut s = String::new();
			let _=stdout().flush();
			stdin().read_line(&mut s).expect("Did not enter a correct string");
			AI = match s[0..1].parse::<u8>() {
				Ok(r) => if r == 1 || r == 2 || r == 3 {
					r
				} else {
					0
				},
				Err(r) => 0
			};
			println!("{}", AI)
		}
		let mut human = 0;
		// loop until you get valid input for the human player color
		while human == 0 {
			print!("What color for human player?
1 -> Black
2 -> White

: ");
		let mut s = String::new();
		let _=stdout().flush();
		stdin().read_line(&mut s).expect("Did not enter a correct string");
		human = match s[0..1].parse::<u8>() {
			Ok(r) => if r == 1 || r == 2 {
				r
			} else {
				0
			},
			Err(r) => 0
		};
		println!("{}", human)
		}
		let humanplayer = if human == 1 {
			enums::Player::Black
		} else {
			enums::Player::White
		};
		let aiplayer = match humanplayer {
			enums::Player::Black => enums::Player::White,
			enums::Player::White => enums::Player::Black
		};
		// if AI == 2 || AI == 3 {
		// 	while difficulty == 0 { // loop until you get valid input for the difficulty
		// 		print!("set depth limit for minimax (must be an odd number for it to work): ");
		// 		let mut s = String::new();
		// 		let _=stdout().flush();
		// 		stdin().read_line(&mut s).expect("Did not enter a correct string");
		// 		difficulty = match s[0..2].parse::<u8>() {
		// 			Ok(r) => r,
		// 			Err(r) => 0
		// 		};
		// 		println!("{}", difficulty)
		// 	}
		// }
		// let ai = match AI {
		// 	1 => AI::new(AI::Type::Random, 0 as u8, aiplayer),
		// 	2 => AI::new(AI::Type::MiniMax, difficulty, aiplayer),
		// 	3 => AI::new(AI::Type::MiniMaxAB, difficulty, aiplayer),
		// 	_ => AI::new(AI::Type::Random, 0 as u8, aiplayer)
		// };
		
		while board.state != enums::State::End { // main game loop
			if board.turn == humanplayer {
				println!("{}", board);
				print!(": ");
				let mut s = String::new();
				let _=stdout().flush();
				stdin().read_line(&mut s).expect("Did not enter a correct string");
				let col = convert_letter_to_number(&s[..1]);
				let rowchar = &s[1..2];
				let row = match rowchar.parse::<u8>() {
					Ok(r) => if r == 0 {
						r
					} else {
						r - 1
					},
					Err(e) => 0 as u8
				};
				let newmove = Move {row: row as i8, col: col as i8};
				board = board.place_tile(board::get_bitmask_for_index(board::get_index_from_move(newmove)));
			} else {
				println!("{}", board);
                let tmp = board.clone();
				board = board.place_tile(AI::get_minimax_move(tmp, &mut hashmap));
			}
		}
	} else {
		while board.state != enums::State::End {
			println!("{}", board);
			print!(": ");
			let mut s = String::new();
			let _ = stdout().flush();
			stdin().read_line(&mut s).expect("Did not enter a correct string");
			let col = convert_letter_to_number(&s[..1]);
			let rowchar = &s[1..2];
			let row = match rowchar.parse::<u8>() {
				Ok(r) => if r == 0 {
					r
				} else {
					r - 1
				},
				Err(e) => 0 as u8
			};
			let newmove = Move {row: row as i8, col: col as i8};
			board = board.place_tile(board::get_bitmask_for_index(board::get_index_from_move(newmove)));
		}
	}

	// determine winner
	let white_counter = board.white_bitboard.count_ones();
	let black_counter = board.black_bitboard.count_ones();
	println!("Final Board State\n{}", board);
	println!("Final Score");
	println!("Black: {}", black_counter);
	println!("White: {}", white_counter);
	if white_counter == black_counter {
		println!("Draw");
	} else if white_counter > black_counter {
		println!("White Wins");
	} else if white_counter < black_counter {
		println!("Black Wins");
	}
}

fn convert_letter_to_number(letter: &str) -> i8 {
	if letter == "A" || letter == "a" {
		return 0
	} else if letter == "B" || letter == "b" {
		return 1	
	} else if letter == "C" || letter == "c" {
		return 2	
	} else if letter == "D" || letter == "d" {
		return 3	
	} else if letter == "E" || letter == "e" {
		return 4	
	} else if letter == "F" || letter == "f" {
		return 5	
	} else if letter == "G" || letter == "g" {
		return 6	
	} else if letter == "H" || letter == "h" {
		return 7	
	} else {
		return 0
	}
}

pub fn AI_vs_AI() {
	let mut hashmap: FnvHashMap<(u64, u64), (i16, u8)> = FnvHashMap::default();
    hashmap.reserve(100000000);
	let mut board = Board::new();
	while board.state != enums::State::End || board.turn_number > 64 {
		let tmp = board.clone();
		board = board.place_tile(AI::get_minimax_move(tmp, &mut hashmap));
		println!("{}", board);
	}
	println!("Winner: {:?}", board)
}