use champions::SummonedChampion;
/* Imports */
use std::collections::VecDeque;
use std::env;
use std::time::{Duration, Instant};
use {board::Board, champions::PlacedChampion, location::Location};

/* Crate Modules */
mod board;
pub mod champions;
mod location;
mod projectiles;
mod shields;
mod status_effects;
mod utils;
pub mod item;

fn main() {
    let p1_champs: VecDeque<PlacedChampion> = VecDeque::from([PlacedChampion::new(
        0,
        0,
        [0, 0, 0],
        Location::generate_random_position_team(true),
    )]);
    let p2_champs: VecDeque<PlacedChampion> = VecDeque::from([PlacedChampion::new(
        1,
        2,
        [0, 0, 0],
        Location::generate_random_position_team(false),
    )]);
    let mut outcomes = [0, 0, 0];
    let start = Instant::now();

    for _ in 0..1 {
        //let mut board: Board = Board::generate_random_board(10);

        let mut board = Board::new(&p1_champs, &p2_champs, 10);
        println!("Board {}", board);
        let board_outcome = board.start_battle() as usize;
        println!("Outcome {board_outcome}");
        outcomes[board_outcome - 1] += 1;
    }
    let duration = start.elapsed();
    /*info!("Time elapsed in expensive_function() is: {:?}", duration);
    info!("outcomes {:?}", outcomes);
    info!("Program End")*/
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("outcomes {:?}", outcomes);
    println!("Program End")
}
