//import require types from other files
use super::{
    champions::{DamageType, SummonedChampion},
    location::Location,
    utils::{find_champion_index_from_id, sign},
};
//import serialise and vecdeque
use serde::Serialize;
use std::collections::VecDeque;
///Projectile struct
#[derive(Clone, Serialize)]
pub struct Projectile {
    ///location of projectile
    location: Location,

    ///location progress
    location_progress: [i8; 2],

    ///target location
    target_location: Option<Location>,

    ///enemy Champion index
    target_id: usize,

    ///projectile damage
    damage: f32,

    ///projectile damage type
    damage_type: DamageType,

    ///amount of splash damage
    splash_damage: f32,

    ///speed of projectile
    speed: i8,

    ///id of shooter (so can give item effects etc)
    shooter_id: usize,
}

impl Projectile {
    ///Simulates a single tick of a projectile
    pub fn simulate_tick(
        self: &mut Projectile,
        possible_targets: &mut VecDeque<SummonedChampion>,
        friendly_champions: &mut VecDeque<SummonedChampion>,
        dead_champions: &mut VecDeque<SummonedChampion>,
    ) -> bool {
        info!("Simulating projectile");
        let target_location = match self.target_location //if self has target location, set target location to that, else get the location of the target champion.
		{
			Some(location) => {
                info!("Target {location}");
                location}, //gets target location
			None => {
				let out_location = find_champion_index_from_id(possible_targets, self.target_id);//gets location of target champion
				info!("Finding location from id : {:?}", out_location);
                match out_location
				{
					Some(index) => possible_targets[index].location, //if target is still alive, return its location
					None => Location { x: -1, y: -1 }, //set location to invalid
				}
                


		}};
        if target_location.x == -1 {
            info!("Not valid location, removing");
            return false;
        } //not found, remove projectile

        let subtracted_distance = Location::sub_positions(&target_location, &self.location); //get location difference

        self.location_progress[0] += self.speed * sign(subtracted_distance.x);
        self.location_progress[1] += self.speed * sign(subtracted_distance.y); //add location progress
        if self.location_progress[0].abs() >= 10
        //if above 10, move
        {
            self.location.x += sign(self.location_progress[0]);
        }
        if self.location_progress[1].abs() >= 10 {
            self.location.y += sign(self.location_progress[1]);
        }
        if !self.location.check_valid() {
            info!("Out of grid leaving");
            return false;
        } //if out of grid, remove

        for possible_target in possible_targets.iter_mut()
        //iterate through all possible collisions
        {
            if self.location == possible_target.location
            //has a hit
            {
                info!("has a hit");
                let mut dead = false; //stores whether need to add to dead champions or alive
                let mut shooter: SummonedChampion;
                //if shooter alive, fetch from friendly champions, else fetch from dead champions, this is because deal damage requires damage dealer to apply correct effects
                if let Some(shooter_index) =
                    find_champion_index_from_id(friendly_champions, self.shooter_id)
                {
                    //finds shooter id
                    shooter = friendly_champions.swap_remove_back(shooter_index).unwrap();
                    info!("shooter alive");
                } else {
                    let dead_champion_index =
                        find_champion_index_from_id(dead_champions, self.shooter_id).unwrap(); //fetch from dead champions
                    shooter = dead_champions.swap_remove_back(dead_champion_index).unwrap();
                    dead = true; //remember to add to dead champions later
                    info!("shooter dead")
                }
                shooter.deal_damage(
                    friendly_champions,
                    possible_target,
                    self.damage,
                    self.damage_type,
                    false,
                ); //deals damage
                if self.splash_damage > 0.0
                //if there is splash damage
                {
                    let initial_hit = possible_target.id;
                    info!("dealing splash");
                    for possible_secondary_target in possible_targets
                        .iter_mut()
                        .filter(self.location.get_within_distance(3))
                    //iterate through possible splash hits
                    {
                        if possible_secondary_target.id == initial_hit { continue }
                        shooter.deal_damage(
                            friendly_champions,
                            possible_secondary_target,
                            self.splash_damage,
                            self.damage_type,
                            true,
                        ); //deal secondary dmg
                    }
                }
                if !dead {
                    friendly_champions.push_back(shooter) //push to alive
                } else {
                    dead_champions.push_back(shooter) //push to dead
                }
                return false; //has exploded, so return false
            }
        }
        if self.target_location.is_some() && self.target_location.unwrap() == self.location {
            return false;
        }
        true //still alive
    }
    ///Makes new projectile
    pub fn new(
        location: Location,
        target_location: Option<Location>,
        target_id: usize,
        damage: f32,
        damage_type: DamageType,
        splash_damage: f32,
        speed: i8,
        shooter_id: usize,
    ) -> Projectile {
        Projectile {
            location,
            location_progress: [0, 0],
            target_location,
            target_id,
            damage,
            damage_type,
            splash_damage,
            speed,
            shooter_id,
        }
    }
}
