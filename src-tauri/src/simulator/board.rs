// Import Serialize from serde so Board can be sent to frontend.
use serde::Serialize;

// Import the necessary types from other modules.
use super::champions::{PlacedChampion, SummonedChampion, Champion};
use super::item::Item;
use super::projectiles::Projectile;

// Import format, VecDeque and Rng.
use core::fmt;
use std::collections::VecDeque;
use rand::Rng;

/// Constant for movement amount.
const MOVEMENT_AMOUNT_CONST: i8 = 10;

/// Board Struct:<br/>
/// Simulates Battles
#[derive(Clone, Serialize)]
pub struct Board {
    /// A vector deque (double ended vector) of player 1's summoned champions.
    p1_champions: VecDeque<SummonedChampion>,

    /// A vector deque of player 2's summoned champions.
    p2_champions: VecDeque<SummonedChampion>,

    /// Time unit for board in centiseconds (1/100 of a second).
    time_unit: i8,

    /// Movement amount per tick, is calculated by const * time unit.
    movement_amount: i8,

    /// Number of ticks until the battle is declared a draw.
    ticks_till_draw : u32,

    /// The current count/ number of ticks that has passed.
    tick_count : u32,

    /// A vector of player 1's projectiles.
    p1_projectiles: Vec<Projectile>,

    /// A vector of player 2's projectiles.
    p2_projectiles: Vec<Projectile>,

    /// A vector deque of the dead summoned champions.
    dead_champs: VecDeque<SummonedChampion>
}

impl Board {
    ///Creates a new board
    pub fn new (p1_placed_champs: &VecDeque<PlacedChampion>, p2_placed_champs: &VecDeque<PlacedChampion>, champions : &[Champion], items : &[Item], time_unit: i8, ticks_till_draw : u32) -> Board {
        info!("New Board");
        //creates empty vecdeque's for player 1s and 2s champs.
        let mut p1_champions = VecDeque::new();
        let mut p2_champions = VecDeque::new();

        //gets number of p1's champions.
        let len: usize = p1_placed_champs.len();
        info!("Creating Champions");


        //for each champ in p1's placed champ, push a summoned champion to p1_champions
        for (i, p1_champ) in p1_placed_champs.iter().enumerate() {
            p1_champions.push_back(SummonedChampion::new(p1_champ, i)); //converts into summoned champ
        }

        //repeat for player 2
        for (i, p2_champ) in p2_placed_champs.iter().enumerate() {
            //adds length of p1_placed_champs to id to ensure they have unique id.
            p2_champions.push_back(SummonedChampion::new(p2_champ, i + len)); //converts into summoned champ
        }

        //pop each champ from the p1_champion, set up the champ and then push back.
        for _ in 0..p1_champions.len() {
            let mut champ = p1_champions.pop_front().unwrap();
            champ.setup(&mut p1_champions, &mut p2_champions, champions, items);
            p1_champions.push_back(champ);
        }
        //repeat for player 2
        for _ in 0..p2_champions.len() {
            let mut champ = p2_champions.pop_front().unwrap();
            champ.setup(&mut p2_champions, &mut p1_champions, champions, items);
            p2_champions.push_back(champ);
        }

        info!("Champions Created");

        //return new board
        Board {
            p1_champions,
            p2_champions,
            time_unit,
            movement_amount: MOVEMENT_AMOUNT_CONST * time_unit / 10, //(!O)
            ticks_till_draw,
            tick_count: 0,
            p1_projectiles: Vec::new(),
            p2_projectiles: Vec::new(),
            dead_champs: VecDeque::new(),
        }
    }
    ///Generates a random new board
    pub fn generate_random_board(time_unit: i8, champions : &[Champion], items : &[Item], ticks_till_draw : u32) -> Board {
        //randomly selects the number of player 1's and 2's champions in the range 1 to 6
        let num_p1_champs: usize = rand::thread_rng().gen_range(1..6);
        let num_p2_champs: usize = rand::thread_rng().gen_range(1..6);

        //fetches all item ids
        let item_ids : Vec<u8> = items.iter().map(|f| f.id).collect();
        let id_range = champions.len();
        //for each champ, generate a random placed champion
        let p1_champions: VecDeque<PlacedChampion> = (0..num_p1_champs)
            .map(|_ : usize| PlacedChampion::generate_random_champ(id_range, &item_ids, false))
            .collect();
        let p2_champions: VecDeque<PlacedChampion> = (num_p1_champs
            ..num_p1_champs + num_p2_champs)
            .map(|_ : usize| PlacedChampion::generate_random_champ(id_range, &item_ids, true))
            .collect();
        //create new board
        Board::new(&p1_champions, &p2_champions, champions, items, time_unit, ticks_till_draw)
    }
    pub fn generate_complex_random_board(time_unit: i8, champions : &[Champion], items : &[Item], ticks_till_draw : u32) -> Board {
        //randomly selects the number of player 1's and 2's champions in the range 3 to 6
        let num_p1_champs: usize = rand::thread_rng().gen_range(3..6);
        let num_p2_champs: usize = rand::thread_rng().gen_range(3..6);

        //fetches all item ids
        let item_ids : Vec<u8> = items.iter().map(|f| f.id).collect();
        let id_range = champions.len();
        //for each champ, generate a random placed champion
        let p1_champions: VecDeque<PlacedChampion> = (0..num_p1_champs)
            .map(|_ : usize| PlacedChampion::generate_random_champ(id_range, &item_ids, false))
            .collect();
        let p2_champions: VecDeque<PlacedChampion> = (num_p1_champs
            ..num_p1_champs + num_p2_champs)
            .map(|_ : usize| PlacedChampion::generate_random_champ(id_range, &item_ids, true))
            .collect();
        //create new board
        Board::new(&p1_champions, &p2_champions, champions, items, time_unit, ticks_till_draw)
    }
    ///simulates a battle from self, with an option input of ticks to simulate.
    pub fn simulate_battle(&mut self, ticks_to_simulate : Option<u32>) {
        info!("Starting Battle");
        
        let upper = match ticks_to_simulate {
            Some(cnt) => self.ticks_till_draw.min(self.tick_count + cnt), //simulates battle till draw or for the tick count, whatever comes first.
            None => self.ticks_till_draw //if none simnulates entire battle till draw
        };

        //for each tick
        for _ in self.tick_count..upper
        {
            info!("Battle Iteration : {}", self.tick_count);
            self.tick_count += 1; //increment ticks
            info!("Taking Champion Turns");
            for _champ_count in 0..self.p1_champions.len() {
                //take turn for all p1Champs

                //pop front champ
                let mut this_champ = self.p1_champions.pop_front().unwrap();
                
                //take turn for champ
                let alive = this_champ.take_turn(
                    &mut self.p1_champions,
                    &mut self.p2_champions,
                    self.time_unit,
                    self.movement_amount,
                    &mut self.p1_projectiles
                );
                if alive {
                    //if alive push to back of p1_champs
                    self.p1_champions.push_back(this_champ);
                } else {
                    //else push to dead champs
                    self.dead_champs.push_back(this_champ);
                }
            }
            
            //repeat for p2 champions
            for _champ_count in 0..self.p2_champions.len() {
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
            

            //simulate tick for each p1 projectile
            self.p1_projectiles.retain_mut(|f| {
                f.simulate_tick(
                    &mut self.p2_champions,
                    &mut self.p1_champions,
                    &mut self.dead_champs,
                )
            });
            
            //simulate tick for each p2 projectile
            self.p2_projectiles.retain_mut(|f| {
                f.simulate_tick(
                    &mut self.p1_champions,
                    &mut self.p2_champions,
                    &mut self.dead_champs,
                )
            });
            info!("End of Turn");

            //check for a winner
            if self.p1_champions.is_empty() {
                info!("Player 2 Wins");
                break; //break if there is a winner
            } else if self.p2_champions.is_empty() {
                info!("Player 1 Wins");
                break;
            }
        }
    }
    pub fn get_winner(&self) -> i8 {
        if self.p1_champions.is_empty() {
            return 2
        }
        if self.p2_champions.is_empty() {
            return 1
        }
        return 0
    }
}

///implements display for board
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p1: {:?},\np2: {:?}",
            self.p1_champions, self.p2_champions
        )
    }
}
