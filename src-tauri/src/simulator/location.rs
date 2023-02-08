use super::champions::SummonedChampion;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use rand::Rng;
///Different types of filters
enum FilterType {
    ///i8 : Distance to check
    ///Location : Other Location
    DistanceFilter(i8, Location),
}
///generates a filter based on a given filter type
fn generate_filter(filter: FilterType) -> impl for<'a> Fn(&&mut SummonedChampion) -> bool {
    match filter {
        FilterType::DistanceFilter(dis, location) => {
            move |n: &&mut SummonedChampion| n.location.distance_between_points(&location) < dis
        } //returns a function that moves the given SummonedChampion into the enclosure, and returns a bool depending on whether the distance between a point and the summonedchampion location is low enough
    }
}

///Location class holding x y and lots of useful methods
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Location {
    ///x position
    pub x: i8,
    ///y position
    pub y: i8,
}
impl Location {
    ///calculates the z value of a location
    fn calculate_z(&self) -> i8 {
        -self.x - self.y
    }
    ///calculates the distance between two points
    pub fn distance_between_points(&self, other_pos: &Location) -> i8 {
        (self.x - other_pos.x).abs() //x distance
            + (self.y - other_pos.y).abs() //y distance
            + (self.calculate_z() - other_pos.calculate_z()).abs() //calcalates z then distance between them
    }
    ///subtracts two positions, returning the new location
    pub fn sub_positions(pos_one: &Location, pos_two: &Location) -> Location {
        Location {
            x: pos_one.x - pos_two.x,
            y: pos_one.y - pos_two.y,
        }
    }
    ///returns a new location, of a location added to a array two long
    pub fn add_position_vec(pos_one: &Location, pos_two: [i8; 2]) -> Location {
        Location {
            x: pos_one.x + pos_two[0],
            y: pos_one.y + pos_two[1],
        }
    }
    ///returns a bool of whether the position is valid/ in the grid
    pub fn check_valid(&self) -> bool {
           self.x >= 0
        && self.x < 10
        && self.y >= 0
        && self.y < 8
        && 2 - (self.y / 2) < self.x 
        && 10 - (self.y / 2) >= self.x //last two lines account for slanting border/ x value
    }
    ///generates a random position given a team (what side of the board to generate on)
    pub fn generate_random_position_team(team: bool) -> Location {
        let y: i8 = if team {
            rand::thread_rng().gen_range(0..4) //generates random y
        } else {
            rand::thread_rng().gen_range(4..8)
        };
        let low = 2 - (y / 2) + 1; //calculate max and min values for x as they are dependant on y
        let high = 10 - (y / 2);
        let x: i8 = rand::thread_rng().gen_range(low..high);
        Location { x, y }
    }
    ///given a vector of summonedchamps, return the closest to self
    pub fn get_closest_to_location<'a>(
        &self,
        champion_list: &'a mut VecDeque<SummonedChampion>,
    ) -> Option<&'a mut SummonedChampion> {
        ///iterates through the champion_list, reducing it by comparing the distance to this location (reducing is comparing two sequential elements in the list and keeping the smaller one/ one that fits the bounds and repeating for the entire list, until you have only one)
        champion_list.iter_mut().reduce(|x, y| {
            if x.location.distance_between_points(self) < y.location.distance_between_points(self) {
                x
            } else {
                y
            }
        })
    }
    ///given a vector of summonedchamps, return the closest to self that is targetable
    pub fn get_closest_to_location_targetable<'a>(
        &self,
        enemy_champions: &'a mut VecDeque<SummonedChampion>,
    ) -> Option<&'a mut SummonedChampion> {
        enemy_champions.iter_mut() //turn vector into iterator
            .filter(|x| x.get_is_targetable()) //filters through the iterator, not yielding any elements that are not targetable
            .reduce(|x, y| { //reduce the values by comparing the distance from self
            if x.location.distance_between_points(self) < y.location.distance_between_points(self) {
                return x;
            }
            y
        })
    }
    ///given a vector of summonedchamps, return the closest to self that is targetable, along with its index
    pub fn get_closest_to_location_targetable_index<'a>(
        &self,
        enemy_champions: &'a mut VecDeque<SummonedChampion>,
    ) -> Option<(usize, &'a mut SummonedChampion)> {
        enemy_champions
            .iter_mut() //turn into iterator
            .enumerate() //get the indexes of the champions, BEFORE the filter so that indexes are valid/ accurate to given vector
            .filter(|(_, x)| x.get_is_targetable()) //remove any champions that are not targetable
            
            .reduce(|(i, x), (j, y)| { //reduce by distance to point
                if x.location.distance_between_points(self)
                    < y.location.distance_between_points(self)
                {
                    return (i, x);
                }
                (j, y)
            })
    }
    ///given a distance, generates a filter to be used with .filter
    pub fn get_within_distance( 
        self,
        distance: i8,
    ) -> impl for<'a> Fn(&&mut SummonedChampion) -> bool {
        generate_filter(FilterType::DistanceFilter(distance, self)) //utilises the generate filter function
    }
}


impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x, y, z  {}, {}, {}", self.x, self.y, self.calculate_z())
    }
}
