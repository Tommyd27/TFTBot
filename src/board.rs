use crate::champions::{PlacedChampion, SummonedChampion};
use crate::projectiles::Projectile;
use std::collections::VecDeque;

///Board Struct:<br />
///Simulates battles
pub struct Board {
    ///vecdeque of player 1's champs
    p1_champions: VecDeque<SummonedChampion>,

    ///vecdeque of player 2's champs
    p2_champions: VecDeque<SummonedChampion>,

    ///time unit for board in centiseconds (1/100 of a second)
    time_unit: i8,

    ///movement amount per tick, is calculated by const / time unit
    movement_amount: i8,
}

impl Board {
    pub fn new(
        p1_placed_champs: &VecDeque<PlacedChampion>,
        p2_placed_champs: &VecDeque<PlacedChampion>,
        time_unit: i8,
    ) -> Board {
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
        info!("Champions Created");
        Board {
            p1_champions,
            p2_champions,
            time_unit,
            movement_amount: 10 / (time_unit as i8), //(!O)
        } //creates new board
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
            }
        }
    }
}
