use super::champions::{PlacedChampion, SummonedChampion, DEFAULT_CHAMPIONS, Champion};
use super::item::Item;
use super::projectiles::Projectile;
use core::fmt;
use rand::Rng;
use std::collections::VecDeque;
const MOVEMENT_AMOUNT_CONST: i8 = 10;
///Board Struct:<br />
///Simulates battles
pub struct Board<'a> {
    ///vecdeque of player 1's champs
    p1_champions: VecDeque<SummonedChampion>,

    ///vecdeque of player 2's champs
    p2_champions: VecDeque<SummonedChampion>,

    ///time unit for board in centiseconds (1/100 of a second)
    time_unit: i8,

    ///movement amount per tick, is calculated by const / time unit
    movement_amount: i8,

    champions : &'a Vec<Champion>,

    items : &'a Vec<Item>,

    ticks_till_draw : u32,
}

impl Board <'_> {
    pub fn new <'a>(p1_placed_champs: &VecDeque<PlacedChampion>, p2_placed_champs: &VecDeque<PlacedChampion>, champions : &'a Vec<Champion>, items : &'a Vec<Item>, time_unit: i8, ticks_till_draw : u32) -> Board<'a> {
        info!("New Board");
        let mut p1_champions = VecDeque::new();
        let mut p2_champions = VecDeque::new();
        let len: usize = p1_placed_champs.len();
        info!("Creating Champions");
        for (i, p1_champ) in p1_placed_champs.iter().enumerate() {
            //(!O) converts placed champions to summoned champions
            p1_champions.push_back(SummonedChampion::new(p1_champ, i, champions)); //converts into summoned champ
        }

        for (i, p2_champ) in p2_placed_champs.iter().enumerate() {
            p2_champions.push_back(SummonedChampion::new(p2_champ, i + len, champions)); //converts into summoned champ
        }
        info!("Champions Created");
        Board {
            p1_champions,
            p2_champions,
            time_unit,
            movement_amount: MOVEMENT_AMOUNT_CONST / time_unit, //(!O)
            champions,
            items,
            ticks_till_draw
        } //creates new board
    }
    pub fn generate_random_board <'a> (time_unit: i8, champions : &'a Vec<Champion>, items : &'a Vec<Item>, ticks_till_draw : u32) -> Board <'a> {
        let num_p1_champs: usize = rand::thread_rng().gen_range(1..4);
        let num_p2_champs: usize = rand::thread_rng().gen_range(1..4);
        let p1_champions: VecDeque<SummonedChampion> = (0..num_p1_champs)
            .map(|f: usize| SummonedChampion::generate_random_champ(false, f, champions))
            .collect();
        let p2_champions: VecDeque<SummonedChampion> = (num_p1_champs
            ..num_p1_champs + num_p2_champs)
            .map(|f: usize| SummonedChampion::generate_random_champ(false, f, champions))
            .collect();
        Board {
            p1_champions,
            p2_champions,
            time_unit,
            movement_amount: MOVEMENT_AMOUNT_CONST / time_unit,
            champions,
            items,
            ticks_till_draw
        }
    }
    pub fn start_battle(mut self) -> i8 {
        let mut debug_count: u32 = 0;
        let mut p1_projectiles: Vec<Projectile> = Vec::new(); //instantiate projectiles vecs
        let mut p2_projectiles: Vec<Projectile> = Vec::new();
        let mut dead_champs: VecDeque<SummonedChampion> = VecDeque::new();
        info!("Starting Battle");
        loop
        //take turns while there are champions alive
        {
            info!("Battle Iteration : {}", debug_count);
            debug_count += 1; //count turns
            info!("Taking Champion Turns");
            for _champ_count in 0..self.p1_champions.len() {
                //take turn for all p1Champs
                let mut this_champ = self.p1_champions.pop_front().unwrap();
                this_champ.setup(&mut self.p1_champions, &mut self.p2_champions);
                let alive = this_champ.take_turn(
                    &mut self.p1_champions,
                    &mut self.p2_champions,
                    self.time_unit,
                    self.movement_amount,
                    &mut p1_projectiles,
                );
                if alive {
                    self.p1_champions.push_back(this_champ);
                } else {
                    dead_champs.push_back(this_champ);
                }
            }

            for _champ_count in 0..self.p2_champions.len() {
                //take turn for all p1Champs
                let mut this_champ = self.p2_champions.pop_front().unwrap();
                this_champ.setup(&mut self.p2_champions, &mut self.p1_champions);
                let alive = this_champ.take_turn(
                    &mut self.p2_champions,
                    &mut self.p1_champions,
                    self.time_unit,
                    self.movement_amount,
                    &mut p2_projectiles,
                );
                if alive {
                    self.p2_champions.push_back(this_champ);
                } else {
                    dead_champs.push_back(this_champ);
                }
            }

            info!("Simulating Projectiles");

            p1_projectiles.retain_mut(|f| {
                f.simulate_tick(
                    &mut self.p2_champions,
                    &mut self.p1_champions,
                    &mut dead_champs,
                )
            });

            p2_projectiles.retain_mut(|f| {
                f.simulate_tick(
                    &mut self.p1_champions,
                    &mut self.p2_champions,
                    &mut dead_champs,
                )
            }); //simulate projectile ticks
            info!("End of Turn");
            if self.p1_champions.is_empty() {
                info!("Player 2 Wins");
                return 2;
            } else if self.p2_champions.is_empty() {
                info!("Player 1 Wins");
                return 1;
            } else if debug_count > self.ticks_till_draw {
                return 3;
            }
        }
    }
}

impl std::fmt::Display for Board <'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p1: {:?},\np2: {:?}",
            self.p1_champions, self.p2_champions
        )
    }
}
