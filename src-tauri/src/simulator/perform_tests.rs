use super::board::Board;
use super::champions::PlacedChampion;
use super::location::Location;
use std::collections::VecDeque;
use super::champions::DEFAULT_CHAMPIONS;
use super::item::DEFAULT_ITEMS;
use std::time::{Duration, Instant};
pub fn perform_test() {
    let start = Instant::now();
    for run in 0..10000 {
        let mut board = Board::generate_complex_random_board(10, &DEFAULT_CHAMPIONS, &DEFAULT_ITEMS, 10000);
        board.simulate_battle(None);
        println!("{run}");
    }
    let duration = start.elapsed();

    println!("Time to simulate 1000 battles is is: {:?}", duration);
}