use serde::Serialize;

use super::champions::{PlacedChampion, SummonedChampion, Champion};
use super::item::Item;
use super::projectiles::Projectile;
use core::fmt;
use std::collections::VecDeque;
const MOVEMENT_AMOUNT_CONST: i8 = 10;
///Board Struct:<br />
///Simulates battles
#[derive(Clone, Serialize)]
pub struct Board {
    ///vecdeque of player 1's champs
    p1_champions: VecDeque<SummonedChampion>,

    ///vecdeque of player 2's champs
    p2_champions: VecDeque<SummonedChampion>,

    ///time unit for board in centiseconds (1/100 of a second)
    time_unit: i8,

    ///movement amount per tick, is calculated by const / time unit
    movement_amount: i8,

    ticks_till_draw : u32,

    tick_count : u32,

    p1_projectiles: Vec<Projectile>,

    p2_projectiles: Vec<Projectile>,

    dead_champs: VecDeque<SummonedChampion>
}

impl Board {
    pub fn new (p1_placed_champs: &VecDeque<PlacedChampion>, p2_placed_champs: &VecDeque<PlacedChampion>, champions : &Vec<Champion>, items : &Vec<Item>, time_unit: i8, ticks_till_draw : u32) -> Board {
        info!("New Board");
        let mut p1_champions = VecDeque::new();
        let mut p2_champions = VecDeque::new();
        let len: usize = p1_placed_champs.len();
        info!("Creating Champions");
        for (i, p1_champ) in p1_placed_champs.iter().enumerate() {
            //(!O) converts placed champions to summoned champions
            p1_champions.push_back(SummonedChampion::new(p1_champ, i)); //converts into summoned champ
        }

        for (i, p2_champ) in p2_placed_champs.iter().enumerate() {
            p2_champions.push_back(SummonedChampion::new(p2_champ, i + len)); //converts into summoned champ
        }

        for _ in 0..p1_champions.len() {
            let mut champ = p1_champions.pop_front().unwrap();
            champ.setup(&mut p1_champions, &mut p2_champions, champions, items);
            p1_champions.push_back(champ);
        }

        for _ in 0..p2_champions.len() {
            let mut champ = p2_champions.pop_front().unwrap();
            champ.setup(&mut p2_champions, &mut p1_champions, champions, items);
            p2_champions.push_back(champ);
        }

        info!("Champions Created");
        Board {
            p1_champions,
            p2_champions,
            time_unit,
            movement_amount: MOVEMENT_AMOUNT_CONST / time_unit, //(!O)
            ticks_till_draw,
            tick_count: 0,
            p1_projectiles: Vec::new(),
            p2_projectiles: Vec::new(),
            dead_champs: VecDeque::new(),
        } //creates new board
    }
    /*pub fn generate_random_board(time_unit: i8, champions : &Vec<Champion>, items : &Vec<Item>, ticks_till_draw : u32) -> Board {
        let num_p1_champs: usize = rand::thread_rng().gen_range(1..4);
        let num_p2_champs: usize = rand::thread_rng().gen_range(1..4);
        let p1_champions: VecDeque<SummonedChampion> = (0..num_p1_champs)
            .map(|f: usize| SummonedChampion::generate_random_champ(false, f, champions))
            .collect();
        let p2_champions: VecDeque<SummonedChampion> = (num_p1_champs
            ..num_p1_champs + num_p2_champs)
            .map(|f: usize| SummonedChampion::generate_random_champ(false, f, champions))
            .collect();
        Board::new(p1_placed_champs, p2_placed_champs, champions, items, time_unit, ticks_till_draw)
    }*/
    pub fn simulate_battle(&mut self, ticks_to_simulate : Option<u32>) {
        info!("Starting Battle");
        let upper = match ticks_to_simulate {
            Some(cnt) => self.ticks_till_draw.min(self.tick_count + cnt),
            None => self.ticks_till_draw
        };
        for _ in self.tick_count..upper
        //take turns while there are champions alive
        {
            info!("Battle Iteration : {}", self.tick_count);
            self.tick_count += 1; //count turns
            info!("Taking Champion Turns");
            for _champ_count in 0..self.p1_champions.len() {
                //take turn for all p1Champs
                let mut this_champ = self.p1_champions.pop_front().unwrap();
                let alive = this_champ.take_turn(
                    &mut self.p1_champions,
                    &mut self.p2_champions,
                    self.time_unit,
                    self.movement_amount,
                    &mut self.p1_projectiles
                );
                if alive {
                    self.p1_champions.push_back(this_champ);
                } else {
                    self.dead_champs.push_back(this_champ);
                }
            }

            for _champ_count in 0..self.p2_champions.len() {
                //take turn for all p1Champs
                let mut this_champ = self.p2_champions.pop_front().unwrap();
                let alive = this_champ.take_turn(
                    &mut self.p2_champions,
                    &mut self.p1_champions,
                    self.time_unit,
                    self.movement_amount,
                    &mut self.p2_projectiles,
                );
                if alive {
                    self.p2_champions.push_back(this_champ);
                } else {
                    self.dead_champs.push_back(this_champ);
                }
            }

            info!("Simulating Projectiles");

            self.p1_projectiles.retain_mut(|f| {
                f.simulate_tick(
                    &mut self.p2_champions,
                    &mut self.p1_champions,
                    &mut self.dead_champs,
                )
            });

            self.p2_projectiles.retain_mut(|f| {
                f.simulate_tick(
                    &mut self.p1_champions,
                    &mut self.p2_champions,
                    &mut self.dead_champs,
                )
            }); //simulate projectile ticks
            info!("End of Turn");
            if self.p1_champions.is_empty() {
                info!("Player 2 Wins");
                break;
            } else if self.p2_champions.is_empty() {
                info!("Player 1 Wins");
                break;
            }
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p1: {:?},\np2: {:?}",
            self.p1_champions, self.p2_champions
        )
    }
}
