#![allow(non_snake_case)] //Allows snake case

use std::iter::Sum;

use rand::Rng; //Used for generating random numbers for crits
struct Champion //Basic structure to store the base stats of a champ
{
    id : u8, //champ id
    cost : u8, //gold cost
    
    hp : [i32; 3], //health points (scales with star level)
    sm : u8, //starting mana
    mc : u8, //ability mana cost
    ar : i32, //armor
    mr : i8, //magic resist
    ad : [i32; 3], //attack damage (scales with star level)
    aS : f32, //attack speed in attacks per 1 second
    ra : u8, //auto attack range
    
    aID : usize, //ability ID

    traits : [u8 ; 3], //traits
}
//

struct StatusEffect
{
	duration : i16,
	effect : StatusKind
}
enum StatusKind
{
	//             Performed, Modifier, Duration
	AttackSpeedBuff(bool, f32)
}

const CHAMPIONS : [Champion ; 3] = [Champion{id : 0, cost : 1, hp : [700, 1260, 2268], sm : 0, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 0.7, ra : 3, aID : 0, traits : [1, 2, 0]}, 
                 					Champion{id : 1, cost : 2, hp : [900, 1620, 2916], sm : 50, mc : 100, ar : 40, mr : 40, ad : [77, 138, 248], aS : 0.7, ra : 3, aID : 0, traits : [2, 3, 0]}, 
                 					Champion{id : 2, cost : 3, hp : [700, 1260, 2268], sm : 35, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 0.7, ra : 3, aID : 0, traits : [4, 5, 0]}];

fn LuluAbility(friendlyChampions : &mut Vec<SummonedChampion>, enemyChampions : &mut Vec<SummonedChampion>, thisChamp : &mut SummonedChampion)
{
	let mut playerDistances : Vec<[i8 ; 2]> = Vec::new();
	let starLevel = thisChamp.starLevel;
	for (index, champ) in friendlyChampions.iter().enumerate()
	{
		if champ.id == thisChamp.id
		{
			continue;
		}
		playerDistances.push([DistanceBetweenPoints(&champ.location, &thisChamp.location), (index + 1) as i8])//optimisation
	}
	for (index, champ) in enemyChampions.iter().enumerate()
	{
		playerDistances.push([DistanceBetweenPoints(&champ.location, &thisChamp.location), -((index + 1) as i8)])//optimisation
	}
	playerDistances.sort_unstable_by_key(|a| a[0]);
	let champCount : usize = [3, 4, 5][starLevel];
	let mut i = 0;//optimisation
	for [_distance, champIndex] in playerDistances
	{
		if i >= champCount
		{
			break;
		}
		if champIndex > 0
		{//champIndex - 1
			//give allies attack speed for 5 seconds
		}
		else //-(champ index + 1)
		{
			//stun enemies for 1.5 seconds and increase damage for 20%
		}
		i += 1;
	}
	if i < champCount - 1
	{
		//enchant herself
	}
}

fn AatroxAbility(friendlyChampions : &mut Vec<SummonedChampion>, enemyChampions : &mut Vec<SummonedChampion>, thisChamp : &mut SummonedChampion)
{
	let starLevel = thisChamp.starLevel;
	//can strike from out of range
	let mut targetIndex = thisChamp.target;
	if targetIndex != enemyChampions[thisChamp.target as usize].id
	{
		for (i, champ) in enemyChampions.iter().enumerate()
		{
			if champ.id == targetIndex
			{
				targetIndex == i;
			}
		}
	}
	thisChamp.health += ((300 + 50 * starLevel as i32) * thisChamp.ap) / 100;

	enemyChampions[targetIndex].health -= (100 / 100 + enemyChampions[targetIndex].ar) * (300 + 5 * starLevel as i32) * thisChamp.ad;
}
const CHAMPIONABILITIES : [fn(friendlyChampions : &mut Vec<SummonedChampion>, enemyChampions : &mut Vec<SummonedChampion>, thisChamp : &mut SummonedChampion) ; 2]	= 
	[LuluAbility, AatroxAbility];








struct PlacedChampion //Structure for champions placed on the board (but not in a battle), includes bench
{
    id : usize, //champ id

    star : usize, //star level
    items : [u8 ; 3], //items given
    location : [i8; 2] //location on board
}

struct SummonedChampion //Structure for champions on board in battle
{
	location : [i8 ; 2], //array of p, q coordinates, r can be calculated with r = -p - q
	movementProgress : [i8 ; 2], //progress of movement before moving to a new square, goes to 10 before moving
	health : i32, //health
	cm : u8, //current mana
	dc : u8, //dodge chance
	cr : u8, //crit rate
	mc : u8, //mana/ ability cost
	ar : i32, //armor
	mr : i8,  //magic resist
	ad : i32, //attack damage
	aS : f32, //attacks per 10 seconds
	ra : u8, //auto attack range
	aID : usize, //ability ID
	id : usize, //id
	targetCountDown : i8, //cooldown before target change
	autoAttackDelay : i16, //cooldown before auto attackng again
	attackSpeedModifier : f32, //increase from items/ from base attack speed
	target : usize, //ID of target
	targetCells : [i8 ; 2], //pathfinding target cell
	items : [u8 ; 3], //item abilities 
	ap : i32, //ability power
	se : Vec<StatusEffect>, //status effects
	gMD : i8, //generate mana delay, after abilities 1 second before can start generating mana again
	starLevel : usize,
	//sortBy : i8,
	//tIDs : Vec<[u8; 2]>, //trait abilities
}
/*
Summoned Champions Status Effects
Need to Implement:
-Attack Speed Increase
-CC/ Stunlock
-Self Damage Increase


Implementation Type:
[statusID, statusDuration, statusStrength, statusPerformed]
*/

impl SummonedChampion 
{
	//Method for converting PlacedChampion into SummonChampion
	fn new(placedChampion : &PlacedChampion, id : usize) -> SummonedChampion
	{
		let starLevel = placedChampion.star; //Get STart Level
		let ofChampion = &CHAMPIONS[placedChampion.id];
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
						   attackSpeedModifier : 1.0,
						   target : 255,
						   targetCells : [-1, -1], //Optimisation, list in path
						   aID: ofChampion.aID, 
						   items: placedChampion.items,
						   ap : 100,
						   se : Vec::new(),
						   gMD : 0,
						   starLevel : starLevel,
						   //sortBy : 0,
						   //tIDs: Vec::new(),
						}
	}
	//fn takeTurn(self : &mut SummonedChampion, friendlyChampionsLocations : &Vec<[i8 ; 2]>, enemyChampions : &mut Vec<SummonedChampion>, timeUnit : u8, movementAmount : i8, randomGen : &mut rand::rngs::ThreadRng/*gridSize : [i8 ; 2]*/)
	fn copy(Self { location, movementProgress, health, cm, dc, cr, mc, ar, mr, ad, aS, ra, aID, id, targetCountDown, autoAttackDelay, attackSpeedModifier, target, targetCells, items, ap, se, gMD, starLevel }: Self)
	{
		return SummonedChampion{
			location : location,
			movementProgress : movementProgress,
			health : health,
			cm : cm,
			dc : dc,
			cr : cr,
			mc : mc,
			ar : ar,
			mr : mr,
			ar : ar,
			
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
	fn new(p1PlacedChamps : &Vec<PlacedChampion>, p2PlacedChamps : &Vec<PlacedChampion>, timeUnit : u8) -> Board
	{
		/*P1 and P2 placed champs to convert into Summoned Champs for  */
		let mut p1Champions = Vec::new();
		let mut p2Champions = Vec::new();
		for (i, p1Champion) in p1PlacedChamps.iter().enumerate()//place for optimisation
		{
			p1Champions.push(SummonedChampion::new(&p1Champion, i));//converts into summoned champ
		}

		for (i, p2Champion) in p2PlacedChamps.iter().enumerate()//place for optimisation
		{
			p2Champions.push(SummonedChampion::new(&p2Champion, i));//converts into summoned champ
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
			for p1ChampionIndex in 0..self.p1Champions.len()
			{
				let mut thisChamp = self.p1Champions[p1ChampionIndex].copy();
				//takeTurn(p1ChampionIndex, &mut self.p1Champions, &mut self.p2Champions, self.timeUnit, self.movementAmount, &mut randomGen);
				takeTurn(&mut thisChamp, &mut self.p1Champions, &mut self.p2Champions, self.timeUnit, self.movementAmount, &mut randomGen);
				self.p1Champions[p1ChampionIndex] = thisChamp;
			}
			for p2ChampionIndex in 0..self.p2Champions.len()
			{
				//takeTurn(p2ChampionIndex, &mut self.p2Champions, &mut self.p1Champions, self.timeUnit, self.movementAmount, &mut randomGen)
			}
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
    let playerOneChamps : Vec<PlacedChampion> = vec![PlacedChampion{id : 0, star : 1, items : [0, 0, 0], location : [3, 0]}, PlacedChampion{id : 0, star : 1, items : [0, 0, 0], location : [9, 0]}, PlacedChampion{id : 0, star : 1, items : [0, 0, 0], location : [6, 0]}];
	let playerTwoChamps : Vec<PlacedChampion> = vec![PlacedChampion{id : 0, star : 2, items : [0, 0, 0], location : [6, 7]}];
	let mut boardOutcome = 1;
	let mut iterationCount = 0;
	while boardOutcome != 2
	{
		iterationCount += 1;
		let board : Board = Board::new(&playerOneChamps, &playerTwoChamps, 10);
		println!("Debug : Starting Battle");
		boardOutcome = board.StartBattle()
		
	}
	println!("Debug : Iteration Count {}", iterationCount);
	
										 //let mut Chadden = Summ1dChampion{id : 0, star : 1, items : [0, 0, 0]};
    //let mut SomeGuy = Summ1dChampion{id : 1, star : 2, items : [0, 0, 0]};

}

fn DistanceBetweenPoints(point1 : &[i8], point2 : &[i8]) -> i8//optimisation doesnt need to borrow?
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

//fn takeTurn(selfIndex : usize, friendlyChampions : &mut Vec<SummonedChampion>, enemyChampions : &mut Vec<SummonedChampion>, timeUnit : u8, movementAmount : i8, randomGen : &mut rand::rngs::ThreadRng/*gridSize : [i8 ; 2]*/)
fn takeTurn(thisChamp : &mut SummonedChampion, friendlyChampions : &mut Vec<SummonedChampion>, enemyChampions : &mut Vec<SummonedChampion>, timeUnit : u8, movementAmount : i8, randomGen : &mut rand::rngs::ThreadRng/*gridSize : [i8 ; 2]*/)
{
	/*
	thisChamp : this champion
	friendlyChampionsLocations : location of all friend champs (array of positions), for pathfinding
	enemyChampions : all enemy champions, for targetting
	timeUnit : time unit of a frame, in centiseconds
	movementAmount : precalculated movement distance for 1 frame
	gridSize : depreciated
		*/
	//let mut thisChamp = &mut friendlyChampions[selfIndex];
	thisChamp.targetCountDown -= timeUnit as i8;//Reduce cooldown to check target/ find new target
	thisChamp.autoAttackDelay -= timeUnit as i16;//Risks going out of bounds as auto attack value may not be called for some time
	thisChamp.gMD -= timeUnit as i8;
	let mut toRemoveVec : Vec<usize> = Vec::new();
	for (index, statusEffect) in thisChamp.se.iter_mut().enumerate()
	{
		statusEffect.duration -= timeUnit as i16;
		if statusEffect.duration <= 0
		{
			toRemoveVec.push(index);
			continue;
		}
		match statusEffect.effect
		{
			StatusKind::AttackSpeedBuff(false, strength) => thisChamp.attackSpeedModifier *= strength,
			_ => println!("Unimplemented")
		}
	}
	//does auto attack delay need to reset on pathing? does attack instantly after reaching path/ in range


	let mut index : usize = 99;//Cache index of target in enemyChampions
	let mut distanceToTarget : i8 = 127;//Distance to target (is set either while finding target or when target found)
	let mut needNewTargetCell : bool = false;//Bool to store whether new path is needed
	if thisChamp.targetCountDown > 0 //if already has target and doesnt want to change targets 
	{
		for (i, enemyChampion) in enemyChampions.iter().enumerate() //every enemy champ
		{
			if enemyChampion.id == thisChamp.target //if they share id
			{
				println!("Debug : Found Target");
				index = i;//set index
				distanceToTarget = DistanceBetweenPoints(&enemyChampion.location, &thisChamp.location);//calculate distance
				break;
			}
		}	
	}
	if index == 99 //index not updating from initial intilialisation of 99, therefore need new target
	{
		println!("Debug : Looking for Target");
		thisChamp.targetCountDown = 100;//reset target cooldown
		thisChamp.target = 0;//reset target
		let mut distance; //cache to store distance between enemy and location
		needNewTargetCell = true; //tells us to recalculate pathfinding later

		for (i, enemyChampion) in enemyChampions.iter().enumerate() //for every champ
		{
			distance = DistanceBetweenPoints(&enemyChampion.location, &thisChamp.location); //calculate distance
			if distance < distanceToTarget //if distance to current enemy champion in loop is lower than distance to current target
			{
				thisChamp.target = enemyChampion.id; //change target
				distanceToTarget = distance; //updating distance to new lower value
				index = i; //setting index
			}
		}
	}
	
	if distanceToTarget <= thisChamp.ra as i8//if target in range
	{
		println!("Debug : Target in Range");
		println!("Debug : Auto Attack Delay Remaining {0}", thisChamp.autoAttackDelay);
		if thisChamp.autoAttackDelay <= 0//if autoattack ready
		{
			println!("Debug : Delay Smaller than 0 - Attacking");
			/* 
			thisChamp.aS = attacks per 10 seconds
			thisChamp.autoAttackDelay = time in 1/10 of second until next attack
			thisChamp.attackSpeedIncrease = percentage increase in attack speed
			
			

			autoAttacKDelay (seconds) = 1 (second) / 0.7 (attacks per seconds)
			autoAttackDelay (centiseconds) = 100 (centisecond) / 0.7 (attacks per second)
			autoAttackDelay (centiseconds) = 1000 (centisecond * 10) / 7 (attacks per 10 seconds) + 7 * attackSpeedIncrease
			
			*/
			thisChamp.autoAttackDelay = 100 / (thisChamp.aS * thisChamp.attackSpeedModifier) as i16; //calculating auto attack delay
			//attack speed unclear, capped at five yet some champions let you boost beyond it?
			//optimisation definitely here
			if thisChamp.gMD <= 0
			{
				thisChamp.cm += 10;
			}
			
			if enemyChampions[index].dc <= 0 || enemyChampions[index].dc < randomGen.gen_range(0..100) //calculating whether to dodge
			{
				let damage : i32 = ((100 * thisChamp.ad) / (100 + enemyChampions[index].ar)).try_into().unwrap(); //calculating damage
				enemyChampions[index].health -=  damage;
				if enemyChampions[index].gMD <= 0
				{
					enemyChampions[index].cm += (damage / 100 * 7) as u8; //discrepency, should be 1% of premitigation and 7% of post.
				}
				
				//discrepency
				if thisChamp.cr > randomGen.gen_range(0..100)//calculating crit
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
		if needNewTargetCell || thisChamp.location == thisChamp.targetCells //if need to update pathfinding or at pathfinding target
		//optimisation?, accuracy vs performance cost
		{
			println!("Debug : Need Target Cell");
			thisChamp.targetCells = thisChamp.location; //setting target cells to location so if it does not find a target this frame will try to do it again
			//optimisation does not need to check every frame

			let mut lowestDistance : i8 = 100; //setting lowestDistance to high value
			let mut newPosition;
			for possibleMove in [[0, -1], [1, -1], [1, 0], [-1, 0], [-1, 1], [0, 1]] //for every possible move
			//optimisation
			{
				newPosition = [thisChamp.location[0] + possibleMove[0], thisChamp.location[1] + possibleMove[1]];
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
					thisChamp.targetCells = newPosition;
				}
				
			}
		}
		
		println!("Debug : Moving to Target Cell");
		thisChamp.movementProgress[0] += movementAmount * sign(thisChamp.targetCells[0] - thisChamp.location[0]);//optimisation here
		println!("Debug : Position ({0},{1}) -- Movement Progress ({2},{3})", thisChamp.location[0], thisChamp.location[1], thisChamp.movementProgress[0], thisChamp.movementProgress[1]);
		if thisChamp.movementProgress[0].abs() == 10
		{
			thisChamp.location[0] += sign(thisChamp.movementProgress[0]);
			thisChamp.movementProgress[0] = 0;
			
		}
		thisChamp.movementProgress[1] += movementAmount * sign(thisChamp.targetCells[1] - thisChamp.location[1]);
		if thisChamp.movementProgress[1].abs() == 10
		{
			thisChamp.location[1] += sign(thisChamp.movementProgress[1]);
			thisChamp.movementProgress[1] = 0;
			
		}
	}
	if thisChamp.cm >= thisChamp.mc
	{
		thisChamp.cm = 0;
		CHAMPIONABILITIES[thisChamp.aID](friendlyChampions, enemyChampions, thisChamp);
	}
}



