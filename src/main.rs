#![allow(non_snake_case)] //Allows snake case

use rand::Rng; //Used for generating random numbers for crits
struct Champion //Basic structure to store the base stats of a champ
{
    id : u8, //champ id
    cost : u8, //gold cost
    
    hp : [i32; 3], //health points (scales with star level)
    sm : u8, //starting mana
    mc : u8, //ability mana cost
    ar : u32, //armor
    mr : u8, //magic resist
    ad : [u32; 3], //attack damage (scales with star level)
    aS : u8, //attack speed in attacks per 10 seconds
    ra : u8, //auto attack range
    
    aID : u8, //ability ID

    traits : [u8 ; 3], //traits
}
//
struct PlacedChampion //Structure for champions placed on the board (but not in a battle), includes bench
{
    id : usize, //champ id

    star : usize, //star level
    items : [u8 ; 3], //items given
    location : [i8; 2] //location on board
}

struct SummonedChampion //Structure for chapions on board in battle
{
	location : [i8 ; 2], //array of p, q coordinates, r can be calculated with r = -p - q
	movementProgress : [i8 ; 2], //progress of movement before moving to a new square, goes to 10 before moving
	health : i32, //health
	cm : u8, //current mana
	dc : u8, //dodge chance
	cr : u8, //crit rate
	mc : u8, //mana/ ability cost
	ar : u32, //armor
	mr : u8,  //magic resist
	ad : u32, //attack damage
	aS : u8, //attacks per 10 seconds
	ra : u8, //auto attack range
	aID : u8, //ability ID
	id : u8, //id
	targetCountDown : i8, //cooldown before target change
	autoAttackDelay : i16, //cooldown before auto attackng again
	attackSpeedIncrease : u8, //increase from items/ from base attack speed
	target : u8, //ID of target
	targetCells : [i8 ; 2], //pathfinding target cell
	items : [u8 ; 3], //item abilities 
	//tIDs : Vec<[u8; 2]>, //trait abilities
}

impl SummonedChampion 
{
	//Method for converting PlacedChampion into SummonChampion
	fn new(placedChampion : &PlacedChampion, ofChampion : &Champion, id : u8) -> SummonedChampion
	{
		let starLevel = placedChampion.star; //Get STart Level
		SummonedChampion { location: [placedChampion.location[0], placedChampion.location[1]], //create summoned champ with all details
						   movementProgress : [0, 0],
						   health: ofChampion.hp[starLevel], 
						   cm: ofChampion.sm, //update current mana to starting mana
						   dc: 0, 
						   cr : 25,
						   mc: ofChampion.mc, 
						   ar: ofChampion.ar * 2, //when calculating distances in cube grid, 1 adjacent hex is calculated as "2" away due to the p, q, r coordinate system, thus attack range is doubled.
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
	//fn takeTurn(self : &mut SummonedChampion, friendlyChampionsLocations : &Vec<[i8 ; 2]>, enemyChampions : &mut Vec<SummonedChampion>, timeUnit : u8, movementAmount : i8, randomGen : &mut rand::rngs::ThreadRng/*gridSize : [i8 ; 2]*/)
	fn takeTurn(self : &mut SummonedChampion, friendlyChampions : &mut Vec<SummonedChampion>, enemyChampions : &mut Vec<SummonedChampion>, timeUnit : u8, movementAmount : i8, randomGen : &mut rand::rngs::ThreadRng/*gridSize : [i8 ; 2]*/)
	{
		/*
		self : this champion
		friendlyChampionsLocations : location of all friend champs (array of positions), for pathfinding
		enemyChampions : all enemy champions, for targetting
		timeUnit : time unit of a frame, in centiseconds
		movementAmount : precalculated movement distance for 1 frame
		gridSize : depreciated
		 */
		self.targetCountDown -= timeUnit as i8;//Reduce cooldown to check target/ find new target
		self.autoAttackDelay -= timeUnit as i16;//Risks going out of bounds as auto attack value may not be called for some time

		//does auto attack delay need to reset on pathing? does attack instantly after reaching path/ in range


		let mut index : usize = 99;//Cache index of target in enemyChampions
		let mut distanceToTarget : i8 = 127;//Distance to target (is set either while finding target or when target found)
		let mut needNewTargetCell : bool = false;//Bool to store whether new path is needed
		if self.targetCountDown > 0 //if already has target and doesnt want to change targets 
		{
			for (i, enemyChampion) in enemyChampions.iter().enumerate() //every enemy champ
			{
				if enemyChampion.id == self.target //if they share id
				{
					println!("Debug : Found Target");
					index = i;//set index
					distanceToTarget = DistanceBetweenPoints(&enemyChampion.location, &self.location);//calculate distance
					break;
				}
			}	
		}
		if index == 99 //index not updating from initial intilialisation of 99, therefore need new target
		{
			println!("Debug : Looking for Target");
			self.targetCountDown = 100;//reset target cooldown
			self.target = 0;//reset target
			let mut distance; //cache to store distance between enemy and location
			needNewTargetCell = true; //tells us to recalculate pathfinding later

			for (i, enemyChampion) in enemyChampions.iter().enumerate() //for every champ
			{
				distance = DistanceBetweenPoints(&enemyChampion.location, &self.location); //calculate distance
				if distance < distanceToTarget //if distance to current enemy champion in loop is lower than distance to current target
				{
					self.target = enemyChampion.id; //change target
					distanceToTarget = distance; //updating distance to new lower value
					index = i; //setting index
				}
			}
		}
		
		if distanceToTarget <= self.ra as i8//if target in range
		{
			println!("Debug : Target in Range");
			println!("Debug : Auto Attack Delay Remaining {0}", self.autoAttackDelay);
			if self.autoAttackDelay <= 0//if autoattack ready
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
				self.autoAttackDelay = 1000 / (self.aS + self.aS * self.attackSpeedIncrease) as i16; //calculating auto attack delay
				//attack speed unclear, capped at five yet some champions let you boost beyond it?
				//optimisation definitely here
				if enemyChampions[index].dc <= 0 || enemyChampions[index].dc < randomGen.gen_range(0..100) //calculating whether to dodge
				{
					let damage : i32 = ((100 * self.ad) / (100 + enemyChampions[index].ar)).try_into().unwrap(); //calculating damage
					enemyChampions[index].health -=  damage; 
					//discrepency
					if self.cr > randomGen.gen_range(0..100)//calculating crit
					{
						enemyChampions[index].health -= damage * 3 / 10;
						println!("Debug : Critical Hit");
					}
					println!("Debug : Enemy Champion Health is {0}", enemyChampions[index].health);
					if enemyChampions[index].health <= 0 //if enemy champion dead
					{
						println!("Debug : Health Lower than 0 - Removing");
						enemyChampions.swap_remove(index);
					}
				}
				else 
				{
					println!("Debug : Dodged Attack");
				}
				

			}
		}
		else 
		{
			println!("Debug : Not in Range");
		    if needNewTargetCell || self.location == self.targetCells //if need to update pathfinding or at pathfinding target
			//optimisation?, accuracy vs performance cost
			{
				println!("Debug : Need Target Cell");
				self.targetCells = self.location; //setting target cells to location so if it does not find a target this frame will try to do it again
				//optimisation does not need to check every frame

				let mut lowestDistance : i8 = 100; //setting lowestDistance to high value
				let mut newPosition;
				for possibleMove in [[0, -1], [1, -1], [1, 0], [-1, 0], [-1, 1], [0, 1]] //for every possible move
				//optimisation
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
						for friendlyChampionLocation in friendlyChampions
						{
							if friendlyChampionLocation.location[0] == newPosition[0] && friendlyChampionLocation.location[1] == newPosition[1]
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
	fn StartBattle(mut self : Board) -> i8
	{
		let mut debugCount : u32 = 0;
		let mut randomGen = rand::thread_rng();
		while self.p1Champions.len() > 0 && self.p2Champions.len() > 0
		{
			println!("Debug : Iteration {}", debugCount);
			debugCount += 1;
			/*for p1ChampionIndex in 0..self.p1Champions.len()
			{
				self.p1Champions[p1ChampionIndex].takeTurn(&mut self.p1Champions, &mut self.p2Champions, self.timeUnit, self.movementAmount, &mut randomGen/*self.gridSize*/);
			}
			for p2ChampionIndex in 0..self.p2Champions.len()
			{
				self.p2Champions[p2ChampionIndex].takeTurn(&mut self.p2Champions, &mut self.p2Champions, self.timeUnit, self.movementAmount, &mut randomGen/*self.gridSize*/);
			}*/
			/*for p1Champion in &mut *self.p1Champions
			{
				p1Champion.takeTurn(&mut self.p1Champions, &mut self.p2Champions, self.timeUnit, self.movementAmount, &mut randomGen)
			}*/
		}
		println!("Debug : Battle Over");
		if self.p1Champions.len() == 0
		{
			println!("Debug : Player 2 Won");
			for champion in &self.p2Champions
			{
				println!("Champ Remaining ID,  Health : {0} {1}", champion.id, champion.health)
			}
			return 2;
		}
		else 
		{
			println!("Debug : Player 1 Won");
			for champion in &self.p1Champions
			{
				println!("Champ Remaining ID,  Health : {0} {1}", champion.id, champion.health)
			}
			return 1;
		}
	}
		
}




fn main() {
    const CHAMPIONS : [Champion ; 3] = [Champion{id : 0, cost : 1, hp : [700, 1260, 2268], sm : 0, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, traits : [1, 2, 0]}, 
                 						Champion{id : 1, cost : 2, hp : [900, 1620, 2916], sm : 50, mc : 100, ar : 40, mr : 40, ad : [77, 138, 248], aS : 7, ra : 3, aID : 0, traits : [2, 3, 0]}, 
                 						Champion{id : 2, cost : 3, hp : [700, 1260, 2268], sm : 35, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, traits : [4, 5, 0]}];
    let playerOneChamps : Vec<PlacedChampion> = vec![PlacedChampion{id : 0, star : 1, items : [0, 0, 0], location : [3, 0]}, PlacedChampion{id : 0, star : 1, items : [0, 0, 0], location : [9, 0]}, PlacedChampion{id : 0, star : 1, items : [0, 0, 0], location : [6, 0]}];
	let playerTwoChamps : Vec<PlacedChampion> = vec![PlacedChampion{id : 0, star : 2, items : [0, 0, 0], location : [6, 7]}];
	let mut boardOutcome = 1;
	let mut iterationCount = 0;
	while boardOutcome != 2
	{
		iterationCount += 1;
		let board : Board = Board::new(&playerOneChamps, &playerTwoChamps, 10, &CHAMPIONS);
		println!("Debug : Starting Battle");
		boardOutcome = board.StartBattle()
		
	}
	println!("Debug : Iteration Count {}", iterationCount);
	
										 //let mut Chadden = Summ1dChampion{id : 0, star : 1, items : [0, 0, 0]};
    //let mut SomeGuy = Summ1dChampion{id : 1, star : 2, items : [0, 0, 0]};

}

fn DistanceBetweenPoints(point1 : &[i8], point2 : &[i8]) -> i8
{
	let zPoints : [i8 ; 2] = [-point1[0] - point1[1], -point2[0] - point2[1]];
	(point1[0] - point2[0]).abs() + (point1[1] - point2[1]).abs() + (zPoints[0] - zPoints[1]).abs()
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
