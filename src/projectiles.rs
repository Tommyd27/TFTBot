use std::collections::VecDeque;
use crate::{champions::{SummonedChampion, DamageType}, location::Location, utils::{findChampionIndexFromID, sign}};
///Projectile struct
pub struct Projectile
{
	///location of projectile
	location : Location,

	///location progress
	locationProgress : [i8 ; 2],

	///target location
	targetLocation : Option<Location>,

	///enemy Champion index
	targetID : usize,

	///projectile damage
	damage : f32,

	///projectile damage type
	damageType : DamageType,

	///amount of splash damage
	splashDamage : f32,

	///speed of projectile
	speed : i8,

	///id of shooter (so can give item effects etc)
	shooterID : usize,
}

impl Projectile
{
	///Simulates a single tick of a projectile
	pub fn SimulateTick(self : &mut Projectile, possibleTargets : &mut VecDeque<SummonedChampion>, friendlyChampions : &mut VecDeque<SummonedChampion>) -> bool
	{
		let targetLocation = match self.targetLocation //discrepency only checks after move to theoretically could phase through someone
		{
			Some(location) => {location}, //gets target location
			None => {
				let outLocation = findChampionIndexFromID(&possibleTargets, self.targetID);//gets location of target champion
				match outLocation
				{
					Some(index) => possibleTargets[index].location,
					None => Location { x: -1, y: -1 },
				}


		}};
		if targetLocation.x == -1 { return false }//not found, remove projectile
		
		let subtractedDistance = Location::subPositions(&targetLocation, &self.location);
	
		self.locationProgress[0] += self.speed * sign(subtractedDistance.x);
		self.locationProgress[1] += self.speed * sign(subtractedDistance.y);//add location progress
		if self.locationProgress[0].abs() >= 10//if above 10, move
		{
			self.location.x += sign(self.locationProgress[0]);
		}
		if self.locationProgress[1].abs() >= 10
		{
			self.location.y += sign(self.locationProgress[1]);
		}
		if ! self.location.checkValid() { return false; }//if out of grid, remove
		
		
			
		
		for possibleTarget in possibleTargets.iter_mut()//iterate through all possible collisions
		{
			if self.location == possibleTarget.location//has a hit
			{
				let mut shooter = SummonedChampion {..Default::default()}; //(!O) has to be better way
				let mut addBack = false;
				if let Some(shooterIndex) = findChampionIndexFromID(&friendlyChampions, self.shooterID) {//finds shooter id
					shooter = friendlyChampions.swap_remove_back(shooterIndex).unwrap();
					addBack = true;
				};
				shooter.dealDamage(friendlyChampions, possibleTarget, self.damage, self.damageType, false);//deals damage
				if self.splashDamage > 0.0//if there is splash damage
				{
					for possibleSecondaryTarget in possibleTargets.iter_mut().filter(self.location.getWithinDistance(3))//iterate through possible splash hits
					{
						shooter.dealDamage(friendlyChampions, possibleSecondaryTarget, self.splashDamage, self.damageType, true);//deal secondary dmg
					}
				}
				if addBack {
					friendlyChampions.push_back(shooter)
				}
				return false//remove self
			}
		}
		true
	}
	///Makes new projectile
	pub fn new(location : Location, targetLocation : Option<Location>, targetID : usize, damage : f32,
		damageType : DamageType,
		splashDamage : f32,
		speed : i8,
		shooterID : usize) -> Projectile
		{
			Projectile {
				location : location,
				locationProgress : [0, 0],
				targetLocation : targetLocation,
				targetID : targetID,
				damage : damage,
				damageType : damageType,
				splashDamage : splashDamage,
				speed : speed,
				shooterID : shooterID,
			}
		}
}
