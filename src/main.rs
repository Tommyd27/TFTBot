#![allow(non_snake_case)] //allows snake case because.

use rand::Rng;

struct Champion
{
    id : u8, //champ id
    cost : u8, //gold cost
    
    hp : [i32; 3], //health points (scales with star level)
    sm : u8, //starting mana
    mc : u8, //ability mana cost
    ar : u32, //armor
    mr : u8, //magic resist
    ad : [u32; 3], //attack damage (scales with star level)
    aS : u8, //attack speed, divide by ten
    ra : u8, //auto attack range
    
    aID : u8, //ability ID

    traits : [u8 ; 3], //traits
}
//
struct PlacedChampion
{
    id : usize, //champ id

    star : usize, //star level
    items : [u8 ; 3], //items given
    location : [u8; 2] //location on board
}

struct SummonedChampion
{
	location : [i8 ; 2],
	movementProgress : [i8 ; 2],
	health : i32,
	sm : u8,
	dc : u8, //dodge chance
	cr : u8, // crit rate
	mc : u8,
	ar : u32,
	mr : u8,
	ad : u32,
	aS : u8,
	ra : u8,
	aID : u8,
	id : u8,
	targetCountDown : i8,
	autoAttackDelay : i16,
	attackSpeedIncrease : u8,
	target : u8,
	targetCells : [i8 ; 2],
	items : [u8 ; 3], //item abilities 
	//tIDs : Vec<[u8; 2]>, //trait abilities
}

impl SummonedChampion 
{
	//Method for converting PlacedChampion into SummonChampion
	fn new(placedChampion : &PlacedChampion, ofChampion : &Champion, id : u8) -> SummonedChampion
	{
		// Pass in grid of friendly champions rather than list of placed champions that will not be used
		let starLevel = placedChampion.star;
		/*nLocation = [placedChampion.location[0], placedChampion.location[1], 0, 0];
		
		nHealth = ofChampion.hp[starLevel];
		nStartingMana = ofChampion.sm;
		nDodgeChance = 0;
		nManaCost = ofChampion.mc;
		nArmour = ofChampion.ar;
		nMagicResist = ofChampion.mr;
		nAttackDamage = ofChampion.ad[starLevel];
		nAttackSpeed = ofChampion.aS;
		nRange = ofChampion.ra;
		nAbilityID = ofChampion.aID;
		nItems = placedChampion.items;
		nTraits = ofChampion.traits;*/
		SummonedChampion { location: [placedChampion.location[0] as i8, placedChampion.location[1] as i8],
						   movementProgress : [0, 0],
						   health: ofChampion.hp[starLevel], 
						   sm: ofChampion.sm, 
						   dc: 0, 
						   cr : 25,
						   mc: ofChampion.mc, 
						   ar: ofChampion.ar, 
						   mr: ofChampion.mr, 
						   ad: ofChampion.ad[starLevel], 
						   aS: ofChampion.aS, 
						   ra: ofChampion.ra,
						   id : id,
						   targetCountDown : 0,
						   autoAttackDelay : 0,
						   attackSpeedIncrease : 0,
						   target : 255,
						   targetCells : [-1, -1], //Optimisation, list in path
						   aID: ofChampion.aID, 
						   items: placedChampion.items, 
						   //tIDs: Vec::new(),
						}
	}
	fn takeTurn(self : &mut SummonedChampion, friendlyChampionsLocations : &Vec<[i8 ; 2]>, enemyChampions : &mut Vec<SummonedChampion>, timeUnit : u8, movementAmount : i8, /*gridSize : [i8 ; 2]*/)
	{
		self.targetCountDown -= timeUnit as i8;//Reduce cooldown to check target/ find new target
		self.autoAttackDelay -= timeUnit as i16;//Risks going out of bounds as auto attack value may not be called for some time

		//does auto attack delay need to reset on pathing? does attack instantly after reaching path/ in range


		let mut index : usize = 99;//Cache index of target in enemyChampions
		let mut distanceToTarget : i8 = 127;//Distance to target (is set either while finding target or when target found)
		let mut needNewTargetCell : bool = false;//Bool to store whether new path is needed
		if self.targetCountDown > 0
		{
			for (i, enemyChampion) in enemyChampions.iter().enumerate()// potential bug if target champion gets killed and therefore not in enemyChampions
			{
				if enemyChampion.id == self.target
				{
					println!("Debug : Found Target");
					index = i;
					distanceToTarget = DistanceBetweenPoints(&enemyChampion.location[0..2], &self.location[0..2]);
					break;
				}
			}	
		}
		if index == 99
		{
			println!("Debug : Looking for Target");
			self.targetCountDown = 100;
			self.target = 0;
			let mut distance : i8 = 0;
			needNewTargetCell = true;

			for (i, enemyChampion) in enemyChampions.iter().enumerate()
			{
				distance = DistanceBetweenPoints(&enemyChampion.location, &self.location);
				if distance < distanceToTarget
				{
					self.target = enemyChampion.id;
					distanceToTarget = distance;
					index = i;
				}
			}
		}
		
		if distanceToTarget <= self.ra as i8
		{
			println!("Debug : Target in Range");
			println!("Debug : Auto Attack Delay Remaining {0}", self.autoAttackDelay);
			if self.autoAttackDelay <= 0
			{
				println!("Debug : Delay Smaller than 0 - Attacking");
				/* 
				self.aS = attacks per 10 seconds
				self.autoAttackDelay = time in 1/10 of second until next attack
				self.attackSpeedIncrease = percentage increase in attack speed
				
				

				autoAttacKDelay (seconds) = 1 (second) / 0.7 (attacks per seconds)
				autoAttackDelay (centiseconds) = 100 (centisecond) / 0.7 (attacks per second)
				autoAttackDelay (centiseconds) = 1000 (centisecond * 10) / 7 (attacks per 10 seconds) + 7 * attackSpeedIncrease
				
				*/
				self.autoAttackDelay = 1000 / (self.aS + self.aS * self.attackSpeedIncrease) as i16; //attack speed unclear, capped at five yet some champions let you boost beyond it?
				//optimisation definitely here
				if enemyChampions[index].dc > 0 && 
				enemyChampions[index].health -= ((100 * self.ad) / (100 + enemyChampions[index].ar)) as i32; //discrepency
				//enemyChampions[index].health -= ((100 * 75) / (100 + 25)) as u32; 
				println!("Debug : Enemy Champion Health is {0}", enemyChampions[index].health);
				if enemyChampions[index].health <= 0
				{
					println!("Debug : Health Lower than 0 - Removing");
					enemyChampions.swap_remove(index);
				}

			}
		}
		else 
		{
			println!("Debug : Not in Range");
		    if needNewTargetCell || self.location[0..2] == self.targetCells //optimisation?, accuracy vs performance cost
			{
				println!("Debug : Need Target Cell");
				let mut lowestDistance : i8 = 100;
				let mut newPosition : [i8 ; 2] = self.location;
				for possibleMove in [[0, -1], [1, -1], [1, 0], [-1, 0], [-1, 1], [0, 1]] //optimisation?
				{
					newPosition = [self.location[0] + possibleMove[0], self.location[1] + possibleMove[1]];
					distanceToTarget = DistanceBetweenPoints(&newPosition, &enemyChampions[index].location);
					if distanceToTarget < lowestDistance
					{
						let mut failed = false;
						if ! InGridHexagon(newPosition)
						{
							continue;
						}
						for friendlyChampionLocation in friendlyChampionsLocations
						{
							if friendlyChampionLocation[0] == newPosition[0] && friendlyChampionLocation[1] == newPosition[1]
							{
								failed = true;
								break
							}
						}
						if failed
						{
							continue;
						}
						println!("Debug : Found a Target Cell");
						lowestDistance = distanceToTarget;
						self.targetCells = newPosition;
					}
					
				}
			}
			
			println!("Debug : Moving to Target Cell");
			self.movementProgress[0] += movementAmount * sign(self.targetCells[0] - self.location[0]);//optimisation here
			println!("Debug : Position ({0},{1}) -- Movement Progress ({2},{3})", self.location[0], self.location[1], self.movementProgress[0], self.movementProgress[1]);
			if self.movementProgress[0].abs() == 10
			{
				self.location[0] += sign(self.movementProgress[0]);
				self.movementProgress[0] = 0;
				
			}
			self.movementProgress[1] += movementAmount * sign(self.targetCells[1] - self.location[1]);
			if self.movementProgress[1].abs() == 10
			{
				self.location[1] += sign(self.movementProgress[1]);
				self.movementProgress[1] = 0;
				
			}
		}
	}
}

struct Player
{
    id : u8, //p id
    gold : u8, //gold stored
    winstreak : i8, //win streak, can be +-
    health : u8, //p health
    level : u8, //p level
    xp : u8, //p xp


    champions : Vec<PlacedChampion>, //all p champions
	augments : [u8 ; 3] //augments
}

struct Board
{
	p1Champions : Vec<SummonedChampion>, //Vector of player 1's champs
	p2Champions : Vec<SummonedChampion>, //Vector of player 2's champs
	timeUnit : u8, //time unit for board in centiseconds (1/100 of a second
	//gridSize : [i8 ; 2], //grid size [x, y, gridType]
	movementAmount : i8, //will be calculated, const / timeUnit
}


impl Board
{
	fn new(p1PlacedChamps : &Vec<PlacedChampion>, p2PlacedChamps : &Vec<PlacedChampion>, timeUnit : u8, champions : &[Champion]) -> Board
	{
		/*P1 and P2 placed champs to convert into Summoned Champs for  */
		let mut p1Champions = Vec::new();
		let mut p2Champions = Vec::new();
		for (i, p1Champion) in p1PlacedChamps.iter().enumerate()//place for optimisation
		{
			p1Champions.push(SummonedChampion::new(&p1Champion, &champions[p1Champion.id], i as u8));//converts into summoned champ
		}

		for (i, p2Champion) in p2PlacedChamps.iter().enumerate()//place for optimisation
		{
			p2Champions.push(SummonedChampion::new(&p2Champion, &champions[p2Champion.id], i as u8));//converts into summoned champ
		}
		
		Board{p1Champions : p1Champions,
			  p2Champions : p2Champions,
			  timeUnit : timeUnit,
			  //gridSize : [7, 8],
			  movementAmount : 10 / timeUnit as i8, //optimisation
			}
	}
	fn StartBattle(mut self : Board)
	{
		let mut p1Positions : Vec<[i8 ; 2]> = Vec::new();
		let mut p2Positions : Vec<[i8 ; 2]> = Vec::new();
		let mut debugCount : u32 = 0;
		while self.p1Champions.len() > 0 && self.p2Champions.len() > 0
		{
			println!("Debug : Iteration {}", debugCount);
			debugCount += 1;
			for champion in &self.p1Champions
			{
				p1Positions.push(champion.location);
			}
			for p1Champion in &mut self.p1Champions
			{
				p1Champion.takeTurn(&p1Positions, &mut self.p2Champions, self.timeUnit, self.movementAmount, /*self.gridSize*/);
			}

			for champion in &self.p2Champions
			{
				p2Positions.push(champion.location);
			}
			for p2Champion in &mut self.p2Champions
			{
				p2Champion.takeTurn(&p2Positions, &mut self.p1Champions, self.timeUnit, self.movementAmount, /*self.gridSize*/);
			}
		}
		println!("Debug : Battle Over");
		if self.p1Champions.len() == 0
		{
			println!("Debug : Player 2 Won");
			for champion in &self.p2Champions
			{
				println!("Champ Remaining ID,  Health : {0} {1}", champion.id, champion.health)
			} 
		}
		else 
		{
			println!("Debug : Player 1 Won");
			for champion in &self.p1Champions
			{
				println!("Champ Remaining ID,  Health : {0} {1}", champion.id, champion.health)
			} 
		}
	}
		
}




fn main() {
    const champions : [Champion ; 3] = [Champion{id : 0, cost : 1, hp : [700, 1260, 2268], sm : 0, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, traits : [1, 2, 0]}, 
                 						Champion{id : 1, cost : 2, hp : [900, 1620, 2916], sm : 50, mc : 100, ar : 40, mr : 40, ad : [77, 138, 248], aS : 7, ra : 3, aID : 0, traits : [2, 3, 0]}, 
                 						Champion{id : 2, cost : 3, hp : [700, 1260, 2268], sm : 35, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, traits : [4, 5, 0]}];
    let playerOneChamps : Vec<PlacedChampion> = vec![PlacedChampion{id : 0, star : 1, items : [0, 0, 0], location : [3, 0]}, PlacedChampion{id : 0, star : 1, items : [0, 0, 0], location : [9, 0]}];
	let playerTwoChamps : Vec<PlacedChampion> = vec![PlacedChampion{id : 0, star : 2, items : [0, 0, 0], location : [6, 7]}];
	let board : Board = Board::new(&playerOneChamps, &playerTwoChamps, 10, &champions);
	println!("Debug : Starting Battle");
	board.StartBattle()
										 //let mut Chadden = Summ1dChampion{id : 0, star : 1, items : [0, 0, 0]};
    //let mut SomeGuy = Summ1dChampion{id : 1, star : 2, items : [0, 0, 0]};

}

fn DistanceBetweenPoints(point1 : &[i8], point2 : &[i8]) -> i8
{
	(point1[0] - point2[0]).abs() + (point1[1] - point2[1]).abs()
}

fn sign(num : i8) -> i8
{
	if num == 0
	{
		0
	}
	else if num > 0
	{
		1
	}
	else
	{
		-1
	}
}

/*fn InGridHexagon(pos : [i8 ; 2], gridSize : [i8 ; 3]) -> bool//need to check for correct gridsize
{
	if pos[0] >= 0 && pos[0] < gridSize[0] &&
	   pos[1] >= 0 && pos[1] < gridSize[1]
	{
		if gridSize[2] == 1 //optimisation
		{
			if 2 - (pos[1] / 2) < pos[0] && //doesnt work for different grid sizes has to be changed manually
			   10 - (pos[1] / 2) > pos[0]
			{
				return true
			}
			return false
		}
		return true
	}
	return false
}*/

fn InGridHexagon(pos : [i8 ; 2]) -> bool//not going to attempt getting it working for different grid sizes yet
{
	if pos[0] >= 0 && pos[0] < 10 &&
	   pos[1] >= 0 && pos[1] < 8
	{
		if 2 - (pos[1] / 2) < pos[0] && //doesnt work for different grid sizes has to be changed manually
		   10 - (pos[1] / 2) > pos[0]
		{
			return true
		}
	}
	return false
}