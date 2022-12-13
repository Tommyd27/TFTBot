#![allow(non_snake_case)] //Allows snake case

use std::collections::VecDeque;

use crate::{champions::PlacedChampion, board::Board, location::Location};


mod utils;
mod status_effects;
mod champions;
mod location;
mod board;
mod projectiles;
mod shields;














fn main() {
	let playerOneChamps : VecDeque<PlacedChampion> = VecDeque::from([PlacedChampion::new(0, 0, [0, 0, 0], Location { x: 3, y: 0 })]);
	let playerTwoChamps : VecDeque<PlacedChampion> = VecDeque::from([PlacedChampion::new(1, 0, [0, 0, 0], Location { x: 6, y: 7 })]);
	let mut boardOutcome = 1;
	let mut iterationCount = 0;
	while boardOutcome != 2
	{
		iterationCount += 1;
		let board : Board = Board::new(&playerOneChamps, &playerTwoChamps, 10);
		println!("Debug : Starting Battle");
		boardOutcome = board.StartBattle()
		
	}
	println!("Debug : Iteration Count {}", iterationCount);

}



