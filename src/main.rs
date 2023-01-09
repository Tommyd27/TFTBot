use crate::champions::SummonedChampion;
/* Imports */
use crate::{board::Board, champions::PlacedChampion, location::Location};
use std::collections::VecDeque;
use std::env;
use std::time::{Duration, Instant};
#[macro_use]
extern crate log;

/* Crate Modules */
mod board;
mod champions;
mod location;
mod projectiles;
mod shields;
mod status_effects;
mod utils;

fn main() {
    env::set_var("RUST_LOG", "error");
    env_logger::init();
    info!("Program Start Up");

    let p1_champs: VecDeque<PlacedChampion> = VecDeque::from([PlacedChampion::new(
        0,
        0,
        [0, 0, 0],
        Location { x: 0, y: 0 },
    )]);
    let p2_champs: VecDeque<PlacedChampion> = VecDeque::from([PlacedChampion::new(
        1,
        0,
        [0, 0, 0],
        Location { x: 4, y: 5 },
    )]);
    let mut outcomes = [0, 0, 0];
    let start = Instant::now();

    for _ in 0..1000 {
        //let mut board: Board = Board::generate_random_board(10);
        //println!("Board {}", board);
        let mut board = Board::new(&p1_champs, &p2_champs, 10);
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
