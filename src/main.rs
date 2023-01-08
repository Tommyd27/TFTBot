/* Imports */
//use std::time::{Duration, Instant};
use crate::{board::Board, champions::PlacedChampion, location::Location};
use std::collections::VecDeque;
use std::env;
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

    let player_one_champs: VecDeque<PlacedChampion> = VecDeque::from([PlacedChampion::new(
        0,
        0,
        [0, 0, 0],
        Location { x: 3, y: 0 },
    )]);
    let player_two_champs: VecDeque<PlacedChampion> = VecDeque::from([
        PlacedChampion::new(
        1,
        0,
        [0, 0, 0],
        Location { x: 6, y: 7 },
    ),
        PlacedChampion::new(
            1,
            0,
            [0, 0, 0],
            Location { x: 6, y: 7 },
        )]);
    /*let mut outcomes = [0, 0];
    let start = Instant::now();
    for _ in 0..1000{
        let mut board: Board = Board::new(&player_one_champs, &player_two_champs, 10);
        let board_outcome = board.start_battle() as usize;
        outcomes[board_outcome - 1] += 1;
    }
    let duration = start.elapsed();
    /*info!("Time elapsed in expensive_function() is: {:?}", duration);
    info!("outcomes {:?}", outcomes);
    info!("Program End")*/
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    println!("outcomes {:?}", outcomes);
    println!("Program End")*/
    let mut board: Board = Board::new(&player_one_champs, &player_two_champs, 10);
    println!("outcome : {}", board.start_battle());
}
