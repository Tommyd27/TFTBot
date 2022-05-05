#![allow(non_snake_case)] //allows snake case because.
struct Champion
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
struct PlacedChampion
{
    id : usize, //champ id

    star : usize, //star level
    items : [u8 ; 3], //items given
    location : [u8; 2] //location on board
}

struct SummonedChampion
{
	location : [i8 ; 4],
	health : u16,
	sm : u8,
	dc : u8, //dodge chance
	mc : u8,
	ar : u8,
	mr : u8,
	ad : u8,
	aS : u8,
	ra : u8,
	aID : u8,
	id : u8,
	targetCountDown : i8,
	autoAttackDelay : i8,
	attackSpeedIncrease : u8,
	target : u8,
	targetCell : [i8 ; 2],
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
		SummonedChampion { location: [placedChampion.location[0] as i8, placedChampion.location[1] as i8, 0, 0],
						   health: ofChampion.hp[starLevel], 
						   sm: ofChampion.sm, 
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
						   target : 255,
						   targetCell : [-1, -1], //Optimisation, list in path
						   aID: ofChampion.aID, 
						   items: placedChampion.items, 
						   //tIDs: Vec::new(),
						}
	}
	fn takeTurn(self : &mut SummonedChampion, friendlyChampions : &Vec<SummonedChampion>, enemyChampions : &Vec<SummonedChampion>, timeUnit : u8)
	{
		self.targetCountDown -= timeUnit as i8;//Reduce cooldown to check target/ find new target
		self.autoAttackDelay -= timeUnit as i8;//Risks going out of bounds as auto attack value may not be called for some time

		//does auto attack delay need to reset on pathing? does attack instantly after reaching path/ in range


		let mut index : usize = 0;//Cache index of target in enemyChampions
		let mut distanceToTarget : i8 = 127;//Distance to target (is set either while finding target or when target found)
		let mut needNewTargetCell : bool = false;//Bool to store whether new path is needed
		if self.targetCountDown <= 0
		{
			self.targetCountDown = 25;
			self.target = 0;
			let mut distance : i8 = 0;
			needNewTargetCell = true;

			for (i, enemyChampion) in enemyChampions.iter().enumerate()
			{
				distance = (enemyChampion.location[0] - self.location[0]).abs() + (enemyChampion.location[1] - self.location[1]).abs();
				if distance < distanceToTarget
				{
					self.target = enemyChampion.id;
					distanceToTarget = distance;
					index = i;
				}
			}
		}
		else 
		{
			for (i, enemyChampion) in enemyChampions.iter().enumerate()// potential bug if target champion gets killed and therefore not in enemyChampions
			{
				if enemyChampion.id == self.target
				{
					index = i;
					distanceToTarget = (enemyChampion.location[0] - self.location[0]).abs() + (enemyChampion.location[1] - self.location[1]).abs();
				}
			}	
		}
		if distanceToTarget <= self.ra as i8
		{
			if self.autoAttackDelay <= 0
			{
				self.autoAttackDelay =     //attack speed unclear, capped at five yet some champions let you boost beyond it?
			}
		}
		else 
		{
		    if needNewTargetCell || self.location[0..2] == self.targetCell //optimisation?
			{
				//find new path
			}
			//follow current path
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
	p1Champions : Vec<SummonedChampion>,
	p2Champions : Vec<SummonedChampion>,
	champMS : u8, //champ movement speed in tiles per second
	timeUnit : u8, //time unit for board in centiseconds (1/100 of a second
	gridSize : [u8 ; 2],
}


impl Board
{
	fn new(p1PlacedChamps : &Vec<PlacedChampion>, p2PlacedChamps : &Vec<PlacedChampion>, champMS : u8, timeUnit : u8, champions : &[Champion]) -> Board
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
			  champMS : champMS,
			  timeUnit : timeUnit,
			  gridSize : [7, 8],
			}
	}
	fn StartBattle(self : Board)
	{
		for p1Champion in self.p1Champions
		{
			p1Champion.takeTurn(&self.p1Champions, &self.p2Champions, self.timeUnit)
		}

	}
}




fn main() {
    const champions : [Champion ; 3] = [Champion{id : 0, cost : 1, hp : [700, 1260, 2268], sm : 0, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, traits : [1, 2, 0]}, 
                 						Champion{id : 1, cost : 2, hp : [900, 1620, 2916], sm : 50, mc : 100, ar : 40, mr : 40, ad : [77, 138, 248], aS : 7, ra : 3, aID : 0, traits : [2, 3, 0]}, 
                 						Champion{id : 2, cost : 3, hp : [700, 1260, 2268], sm : 35, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, traits : [4, 5, 0]}];
    //let mut Chadden = Summ1dChampion{id : 0, star : 1, items : [0, 0, 0]};
    //let mut SomeGuy = Summ1dChampion{id : 1, star : 2, items : [0, 0, 0]};

}
