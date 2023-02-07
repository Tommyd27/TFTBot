use super::champions::SummonedChampion;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use rand::Rng;
enum FilterType {
    ///i8 : Distance to check
    ///Location : Other Location
    DistanceFilter(i8, Location),
}

fn generate_filter(filter: FilterType) -> impl for<'a> Fn(&&mut SummonedChampion) -> bool {
    match filter {
        FilterType::DistanceFilter(dis, location) => {
            move |n: &&mut SummonedChampion| n.location.distance_between_points(&location) < dis
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Location {
    pub x: i8,
    pub y: i8,
}
impl Location {
    fn calculate_z(&self) -> i8 {
        -self.x - self.y
    }
    pub fn distance_between_points(&self, other_pos: &Location) -> i8 {
        (self.x - other_pos.x).abs()
            + (self.y - other_pos.y).abs()
            + (self.calculate_z() - other_pos.calculate_z()).abs()
    }
    pub fn _add_positions(pos_one: &Location, pos_two: &Location) -> Location {
        Location {
            x: pos_one.x + pos_two.x,
            y: pos_one.y + pos_two.y,
        }
    }
    pub fn sub_positions(pos_one: &Location, pos_two: &Location) -> Location {
        Location {
            x: pos_one.x - pos_two.x,
            y: pos_one.y - pos_two.y,
        }
    }
    pub fn add_position_vec(pos_one: &Location, pos_two: [i8; 2]) -> Location {
        Location {
            x: pos_one.x + pos_two[0],
            y: pos_one.y + pos_two[1],
        }
    }
    pub fn check_valid(&self) -> bool {
           self.x >= 0
        && self.x < 10
        && self.y >= 0
        && self.y < 8
        && 2 - (self.y / 2) < self.x
        && 10 - (self.y / 2) >= self.x
    }
    pub fn generate_random_position_team(team: bool) -> Location {
        let y: i8 = if team {
            rand::thread_rng().gen_range(0..4)
        } else {
            rand::thread_rng().gen_range(4..8)
        };
        let low = 2 - (y / 2) + 1;
        let high = 10 - (y / 2);
        let x: i8 = rand::thread_rng().gen_range(low..high);
        Location { x, y }
    }
    pub fn get_closest_to_location<'a>(
        &self,
        enemy_champions: &'a mut VecDeque<SummonedChampion>,
    ) -> Option<&'a mut SummonedChampion> {
        enemy_champions.iter_mut().reduce(|x, y| {
            if x.location.distance_between_points(self) < y.location.distance_between_points(self) {
                x
            } else {
                y
            }
        })
    }
    pub fn get_closest_to_location_targetable<'a>(
        &self,
        enemy_champions: &'a mut VecDeque<SummonedChampion>,
    ) -> Option<&'a mut SummonedChampion> {
        enemy_champions.iter_mut().reduce(|x, y| {
            if !x.get_is_targetable() {
                return y;
            } else if !y.get_is_targetable() {
                return x;
            }

            if x.location.distance_between_points(self) < y.location.distance_between_points(self) {
                return x;
            }
            y
        })
    }
    pub fn get_closest_to_location_targetable_index<'a>(
        &self,
        enemy_champions: &'a mut VecDeque<SummonedChampion>,
    ) -> Option<(usize, &'a mut SummonedChampion)> {
        enemy_champions
            .iter_mut()
            .filter(|x| x.get_is_targetable())
            .enumerate()
            .reduce(|(i, x), (j, y)| {
                if x.location.distance_between_points(self)
                    < y.location.distance_between_points(self)
                {
                    return (i, x);
                }
                (j, y)
            })
    }
    pub fn get_within_distance(
        self,
        distance: i8,
    ) -> impl for<'a> Fn(&&mut SummonedChampion) -> bool {
        generate_filter(FilterType::DistanceFilter(distance, self))
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x, y, z  {}, {}, {}", self.x, self.y, self.calculate_z())
    }
}
