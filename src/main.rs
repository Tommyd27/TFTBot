/* Imports */
use crate::{board::Board, champions::PlacedChampion, location::Location};
use std::collections::VecDeque;

/* Crate Modules */
mod board;
mod champions;
mod location;
mod projectiles;
mod shields;
mod status_effects;
mod utils;

fn main() {
    let playerOneChamps: VecDeque<PlacedChampion> = VecDeque::from([PlacedChampion::new(
        0,
        0,
        [0, 0, 0],
        Location { x: 3, y: 0 },
    )]);
    let playerTwoChamps: VecDeque<PlacedChampion> = VecDeque::from([PlacedChampion::new(
        1,
        0,
        [0, 0, 0],
        Location { x: 6, y: 7 },
    )]);
    let mut board: Board = Board::new(&playerOneChamps, &playerTwoChamps, 10);
    println!("Debug : Starting Battle");
    let boardOutcome = board.start_battle();
    println!("Debug : Iteration Count {}", boardOutcome);
}
