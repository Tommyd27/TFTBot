use crate::{
    champions::{DamageType, SummonedChampion},
    location::Location,
    utils::{find_champion_index_from_id, sign},
};
use std::collections::VecDeque;
///Projectile struct
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
        let target_location = match self.target_location //discrepency only checks after move to theoretically could phase through someone
		{
			Some(location) => {location}, //gets target location
			None => {
				let out_location = find_champion_index_from_id(possible_targets, self.target_id);//gets location of target champion
				match out_location
				{
					Some(index) => possible_targets[index].location,
					None => Location { x: -1, y: -1 },
				}


		}};
        if target_location.x == -1 {
            return false;
        } //not found, remove projectile

        let subtracted_distance = Location::sub_positions(&target_location, &self.location);

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
            return false;
        } //if out of grid, remove

        for possible_target in possible_targets.iter_mut()
        //iterate through all possible collisions
        {
            if self.location == possible_target.location
            //has a hit
            {
                let mut dead = false;
                let mut shooter: SummonedChampion;

                if let Some(shooter_index) =
                    find_champion_index_from_id(friendly_champions, self.shooter_id)
                {
                    //finds shooter id
                    shooter = friendly_champions.swap_remove_back(shooter_index).unwrap();
                } else {
                    let dead_champion_index =
                        find_champion_index_from_id(dead_champions, self.shooter_id).unwrap();
                    shooter = dead_champions.swap_remove_back(dead_champion_index).unwrap();
                    dead = true;
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
                    for possible_secondary_target in possible_targets
                        .iter_mut()
                        .filter(self.location.get_within_distance(3))
                    //iterate through possible splash hits
                    {
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
                    friendly_champions.push_back(shooter)
                } else {
                    dead_champions.push_back(shooter)
                }
                return false; //remove self
            }
        }
        true
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
