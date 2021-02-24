use std::fmt;
use std::ops::Not;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Player {
	White,
	Black
}

impl Not for Player {
	type Output = Player;
	
	fn not(self) -> Player {
		match self {
			Player::White => Player::Black,
			Player::Black => Player::White
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Position {
	White,
	Black,
	Empty,
	Valid
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
	Normal,
	SkippedSingle,
	End
}

// Overloading the print operator for the player
impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let result = match self {
			Player::White => "White",
			Player::Black => "Black"
		};

		write!(f, "{}", result)
	}
}