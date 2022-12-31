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
    let player_one_champs: VecDeque<PlacedChampion> = VecDeque::from([PlacedChampion::new(
        0,
        0,
        [0, 0, 0],
        Location { x: 3, y: 0 },
    )]);
    let player_two_champs: VecDeque<PlacedChampion> = VecDeque::from([PlacedChampion::new(
        1,
        0,
        [0, 0, 0],
        Location { x: 6, y: 7 },
    )]);
    let mut board: Board = Board::new(&player_one_champs, &player_two_champs, 10);
    println!("Debug : Starting Battle");
    let board_outcome = board.start_battle();
    println!("Debug : Iteration Count {}", board_outcome);
}
