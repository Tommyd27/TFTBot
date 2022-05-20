#![allow(non_snake_case)] //allows snake case because.



struct Champion //Basic structure to store the base stats of a champ
{
    id : u8, //champ id
    cost : u8, //gold cost
    
    hp : [u16; 3], //health points (scales with star level)
    sm : u8, //starting mana
    mc : u8, //ability mana cost
    ar : u8, //armor
    mr : u8, //magic resist
    ad : [u8; 3], //attack damage (scales with star level)
    aS : u8, //attack speed, divide by ten
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
	location : [i8 ; 2], //location (x, y) on board
	movementProgress : [i8 ; 2], //progress before moving to next square (range +-10)
	health : u16, //health
	cm : u8, //current mana
	dc : u8, //dodge chance
	mc : u8, //mana cost
	ar : u8, //armor
	mr : u8, //magic resist
	ad : u8, //attack damage
	aS : u8, //attack speed
	ra : u8, //range
	aID : u8, //ability ID
	id : u8, //id 
	targetCountDown : i8, //cooldown before change target
	autoAttackDelay : i16, //cooldown before next auto
	attackSpeedIncrease : u8, //increase of base attack speed
	target : u8, //id of target
	targetCells : [i8 ; 2], //target cell to move to
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
	fn takeTurn(self : &mut SummonedChampion, friendlyChampionsLocations : &Vec<[i8 ; 2]>, enemyChampions : &mut Vec<SummonedChampion>, timeUnit : u8, movementAmount : i8, gridSize : [i8 ; 3])
	{
		/*
		self : this champion
		friendlyChampionsLocations : location of all friend champs (array of positions), for pathfinding
		enemyChampions : all enemy champions, for targetting
		timeUnit : time unit of a frame, in centiseconds
		movementAmount : precalculated movement distance for 1 frame
		gridSize : currently unused
		 */
		self.targetCountDown -= timeUnit as i8;//Reduce cooldown to check target/ find new target
		self.autoAttackDelay -= timeUnit as i16;//Risks going out of bounds as auto attack value may not be called for some time

		//does auto attack delay need to reset on pathing? does attack instantly after reaching path/ in range


		let mut index : usize = 0;//Cache index of target in enemyChampions
		let mut distanceToTarget : i8 = 127;//Distance to target (is set either while finding target or when target found)
		let mut needNewTargetCell : bool = false;//Bool to store whether new path is needed
		if self.targetCountDown <= 0 //find new target
		{
			self.targetCountDown = 25; //resetting targetCoolDown
			self.target = 0; //setting target to default value of 0 (first enemy champ)
			let mut distance : i8 = 0; //setting distance to any value (will be overwritten)
			needNewTargetCell = true; //setting this to true for later, will reset pathfinding

			for (i, enemyChampion) in enemyChampions.iter().enumerate() //for every champ
			{
				distance = DistanceBetweenPoints(&enemyChampion.location, &self.location); //calculate distance
				if distance < distanceToTarget //if distance to current enemy champion in loop is lower than distance to current target
				{
					self.target = enemyChampion.id; //change target
					distanceToTarget = distance;
					index = i; //setting index
				}
			}
		}
		else 
		{
			for (i, enemyChampion) in enemyChampions.iter().enumerate()// //finding target in enemy champs, cannot be done by index as index may have changed from death, maybe optimisation if accepting changing target when one dies.
			{
				if enemyChampion.id == self.target //if correct id
				{
					index = i;
					distanceToTarget = DistanceBetweenPoints(&enemyChampion.location, &self.location);//setting distance to correct distance
				}
			}	
		}
		if distanceToTarget <= self.ra as i8 //if target in range
		{
			if self.autoAttackDelay <= 0 //if auto attack ready
			{
				self.autoAttackDelay = 1000 / (self.aS + self.aS * self.attackSpeedIncrease) as i16; //sets attack delay to valid account dependant on base and % increase
				//attack speed unclear, capped at five yet some champions let you boost beyond it?
				//optimisation definitely here

				enemyChampions[index].health -= ((100 / (100 + enemyChampions[index].ar)) * self.ad) as u16; 
				if enemyChampions[index].health <= 0
				{
					enemyChampions.swap_remove(index);
				}

			}
		}
		else 
		{
		    if needNewTargetCell || self.location[0..2] == self.targetCells //optimisation?, accuracy vs performance cost
			{
				let mut lowestDistance : i8 = 100;
				let mut newPosition : [i8 ; 2] = self.location;
				for possibleMove in [[0, -1], [1, -1], [1, 0], [-1, 0], [-1, 1], [0, 1]] //optimisation?
				{
					newPosition = [self.location[0] + possibleMove[0], self.location[1] + possibleMove[1]];
					distanceToTarget = DistanceBetweenPoints(&newPosition, &enemyChampions[index].location);
					if distanceToTarget < lowestDistance
					{
						let mut failed = false;
						if ! InGrid(newPosition, gridSize)
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
						lowestDistance = distanceToTarget;
						self.targetCells = newPosition;
					}
					
				}
			}
			self.movementProgress[0] += movementAmount * sign(self.targetCells[0] - self.location[0]);//optimisation here
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
	gridSize : [i8 ; 3], //grid size [x, y, gridType]
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
			  gridSize : [7, 8, 1],
			  movementAmount : 10 / timeUnit as i8, //optimisation
			}
	}
	fn StartBattle(mut self : Board)
	{
		let mut p1Positions : Vec<[i8 ; 2]> = Vec::new();
		let mut p2Positions : Vec<[i8 ; 2]> = Vec::new();
		while self.p1Champions.len() > 0 && self.p2Champions.len() > 0
		{
			for champion in &self.p1Champions
			{
				p1Positions.push(champion.location);
			}
			for p1Champion in &mut self.p1Champions
			{
				p1Champion.takeTurn(&p1Positions, &mut self.p2Champions, self.timeUnit, self.movementAmount, self.gridSize);
			}

			for champion in &self.p2Champions
			{
				p2Positions.push(champion.location);
			}
			for p2Champion in &mut self.p2Champions
			{
				p2Champion.takeTurn(&p2Positions, &mut self.p1Champions, self.timeUnit, self.movementAmount, self.gridSize);
			}
		}
	}
		
}




fn main() {
    const champions : [Champion ; 3] = [Champion{id : 0, cost : 1, hp : [700, 1260, 2268], sm : 0, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, traits : [1, 2, 0]}, 
                 						Champion{id : 1, cost : 2, hp : [900, 1620, 2916], sm : 50, mc : 100, ar : 40, mr : 40, ad : [77, 138, 248], aS : 7, ra : 3, aID : 0, traits : [2, 3, 0]}, 
                 						Champion{id : 2, cost : 3, hp : [700, 1260, 2268], sm : 35, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, traits : [4, 5, 0]}];
    let playerOneChamps : Vec<PlacedChampion> = vec![PlacedChampion{id : 0, star : 1, items : [0, 0, 0], location : [3, 0]}];
	let playerTwoChamps : Vec<PlacedChampion> = vec![PlacedChampion{id : 0, star : 1, items : [0, 0, 0], location : [6, 6]}];
	let board : Board = Board::new(&playerOneChamps, &playerTwoChamps, 10, &champions);
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

fn InGrid(pos : [i8 ; 2], gridSize : [i8 ; 3]) -> bool//need to check for correct gridsize
{
	if pos[0] >= 0 && pos[0] < gridSize[0] &&
	   pos[1] >= 0 && pos[1] < gridSize[1]
	{
		if gridSize[2] == 1 //optimisation
		{
			if 2 - (pos[1] / 2) < pos[0] && //doesnt work for different grid sizes has to be changed manually
			   7 - (pos[1] / 2) > pos[0]
			{
				return true
			}
			return false
		}
		return true
	}
	return false
}