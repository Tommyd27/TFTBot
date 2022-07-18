#![allow(non_snake_case)] //Allows snake case

use std::{cmp::min};
use rand::{Rng};
use std::collections::HashMap;//Optimisation change default hashing algorithm

///It's in the name
struct ABooleanWithExtraSteps
{
	///A bool
	value : bool,
}
///Champion (struct)<br />:
///Stores the basic information surrounding a champion
struct Champion //Basic structure to store the base stats of a champ
{
	///Champion ID
	///Index in
    id : u8,

	///Cost in Gold
    cost : u8, 
    
	///HP values for each star level
    hp : [i32; 3], 
	///Starting mana
    sm : u8,
	///Ability Mana Cost
    mc : u8,
	///Base Armor Value
    ar : i32,
	///Base Magic Resist Value
    mr : i32,
	///Autoattack damage for each star level
    ad : [i32; 3],
	///Attack speed in attacks per second
    aS : f32,
	///Auto attack range
    ra : u8,
    ///Ability ID, same as index in const CHAMPIONABILITIES
    aID : usize, 
	///Trait IDs
    traits : [u8 ; 3],
}
///Status Type (enum)<br />:
///Holds information about what the status actually is
#[derive(Clone)]
#[derive(PartialEq)]
enum StatusType
{
	///Attack Speed Buff
	///(bool : whether the buff has been applied, f32 : actual modifier)
	AttackSpeedBuff(bool, f32),
	///Increase Damage Taken
	///(bool : whether the buff has been applied, i32 : actual modifier in % (so 120 = 120% or 20% increase))
	IncreaseDamageTaken(bool, i32),
	///Stun
	///()
	Stun(),

	///Grevious Wounds
	///Reduces healing by 50%
	GreviousWounds(),
	///Gives edge of night buff
	EdgeOfNight(),
	///Whether the target is targetable
	///bool : Whether the buff has been applied
	Untargetable(bool),

	///Bloodthirster shield at 40%
	Bloodthirster(),
	///Assassin trait leap
	Assassin(),

	///Morellonomicon Burn
	///i32 : damage per tick
	///i32 : damage to do
	///i16 : time til next tick
	MorellonomiconBurn(i32, i32, i16),

	///Ionic spark effect
	///Reduces MR by 50%
	///bool : applied - remove as doesnt need as only lasts 1 frame?
	IonicSparkEffect(),//maybe discrepencies? awkward cuz only lasts 1 frame?

	///None
	NoEffect()
}



///StatusEffect (struct)<br />:
///Stores a status type and a duration
#[derive(Clone)]
struct StatusEffect
{
	///Duration of status effect in centiseconds
	duration : i16,//optimisation so uses Option<i16> rather than i16
	///Stores status type
	statusType : StatusType,
	isNegative : bool,
}

impl Default for StatusEffect{
	fn default() -> StatusEffect
	{
		StatusEffect { duration: 0, statusType: StatusType::NoEffect(), isNegative: false }
	}
}

///CHAMPIONS (const):<br />
///Stores all the champion information
const CHAMPIONS : [Champion ; 3] = [Champion{id : 0, cost : 1, hp : [650, 1170, 2106], sm : 70, mc : 140, ar : 25, mr : 25, ad : [40, 72, 129], aS : 0.7, ra : 3, aID : 0, traits : [1, 2, 0]}, 
                 					Champion{id : 1, cost : 2, hp : [650, 1170, 2106], sm : 50, mc : 100, ar : 45, mr : 45, ad : [55, 99, 178], aS : 0.7, ra : 1, aID : 1, traits : [2, 3, 0]}, 
                 					Champion{id : 2, cost : 3, hp : [700, 1260, 2268], sm : 35, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 0.7, ra : 3, aID : 0, traits : [4, 5, 0]}];
///LuluAbility (func):<br />
///Whimsy
///Lulu enchants the nearest targets. Enchanted allies gain Attack Speed for 1.5 seconds. Enchanted enemies are stunned and transformed into harmless dragonlings, taking increased damage while stunned. If there are fewer than 3 units nearby, Lulu will enchant herself.<br />
///Targets:
///3/4/5<br />
///Attack Speed:
///70/80/120%
fn LuluAbility(friendlyChampions : &mut Vec<SummonedChampion>, enemyChampions : &mut Vec<SummonedChampion>, selfIndex : usize)
{
	let mut playerDistances : Vec<[i8 ; 2]> = Vec::new();
	let starLevel = friendlyChampions[selfIndex].starLevel;
	for (index, champ) in friendlyChampions.iter().enumerate()
	{
		if index == selfIndex
		{
			continue;
		}
		playerDistances.push([DistanceBetweenPoints(champ.location, friendlyChampions[selfIndex].location), (index + 1) as i8])//optimisation
	}
	for (index, champ) in enemyChampions.iter().enumerate()
	{
		if index == selfIndex
		{
			continue;
		}
		playerDistances.push([DistanceBetweenPoints(champ.location, friendlyChampions[selfIndex].location), -((index + 1) as i8)])//optimisation
	}
	playerDistances.sort_unstable_by_key(|a| a[0]);
	let champCount : usize = [3, 4, 5][starLevel];
	let mut i = 0;//optimisation
	for [_, champIndex] in playerDistances//optimise
	{
		if i >= champCount
		{
			break;
		}
		if champIndex > 0
		{//champIndex - 1
			//give allies attack speed for 5 seconds
			friendlyChampions[(champIndex - 1) as usize].se.push(StatusEffect{
																	duration : 500,
																	statusType : StatusType::AttackSpeedBuff(false, 1.7),
																	..Default::default()	
			});
		}
		else //-(champ index + 1)
		{
			//stun enemies for 1.5 seconds and increase damage for 20%
			enemyChampions[-(champIndex + 1) as usize].se.push(StatusEffect { duration: 150, statusType: StatusType::Stun(), isNegative : true });
			enemyChampions[-(champIndex + 1) as usize].se.push(StatusEffect { duration: 150, statusType: StatusType::IncreaseDamageTaken(false, 120), isNegative : true});
		}
		i += 1;
	}
	if i < champCount - 1
	{
		friendlyChampions[selfIndex].se.push(StatusEffect{duration : 500, statusType : StatusType::AttackSpeedBuff(false, 1.7), ..Default::default()});
		println!("attack speed buff");
	}
}

///AatroxAbility (func):
///Deathbringer Strike
///Aatrox strikes his target for physical damage and heals himself.
///Attack Damage:
///300/305/310%
///Heal:
///300/350/400
fn AatroxAbility(friendlyChampions : &mut Vec<SummonedChampion>, enemyChampions : &mut Vec<SummonedChampion>, selfIndex : usize)
{
	let starLevel = friendlyChampions[selfIndex].starLevel;
	//can strike from out of range
	let mut targetIndex = friendlyChampions[selfIndex].target;
	if targetIndex != enemyChampions[friendlyChampions[selfIndex].target as usize].id
	{
		for (i, champ) in enemyChampions.iter().enumerate()
		{
			if champ.id == targetIndex
			{
				targetIndex == i;
			}
		}
	}
	let ap = friendlyChampions[selfIndex].ap;
	friendlyChampions[selfIndex].heal(((300 + 50 * starLevel as i32) * ap) / 100);

	dealDamage(selfIndex, friendlyChampions, &mut enemyChampions[targetIndex], (300 + 5 * starLevel as i32) * friendlyChampions[selfIndex].ad, 0)
}
///const CHAMPIONABILITIES :
///Stores all the champ abilities (index = abilityID)
///All abilities are called in the form 
///(friendlyChampions : &mut Vec<SummonedChampion>, enemyChampions : &mut Vec<SummonedChampion>, selfIndex : usize)
///Arguments:
///friendlyChampions : Mutable reference to allied champions
///enemyChampions : Mutable reference to enemy champions
///selfIndex : Index of champion (in friendlyChampions) who casted this ability
const CHAMPIONABILITIES : [fn(friendlyChampions : &mut Vec<SummonedChampion>, enemyChampions : &mut Vec<SummonedChampion>, selfIndex : usize) ; 2]	= 
	[LuluAbility, AatroxAbility];

//discrepency, cast time = 0.5 seconds apparently





///PlacedChampion (struct):
///Stores information about a champion's location and status on a board (as well as ID of actual champion)
///Not used in battles, only for planning phase
struct PlacedChampion 
{
	///
    id : usize, 

    star : usize, //star level
    items : [u8 ; 3], //items given
    location : [i8; 2] //location on board
}
struct Shield
{
	duration : i16,
	size : i32,
}
struct SummonedChampion //Structure for champions on board in battle
{
	location : [i8 ; 2], //array of p, q coordinates, r can be calculated with r = -p - q
	movementProgress : [i8 ; 2], //progress of movement before moving to a new square, goes to 10 before moving
	health : i32, //health
	cm : u8, //current mana
	dc : u8, //dodge chance
	cr : u8, //crit rate
	critD : i32, // crit damage
	mc : u8, //mana/ ability cost
	ar : i32, //armor
	mr : i32,  //magic resist
	ad : i32, //attack damage
	aS : f32, //attacks per second
	ra : u8, //auto attack range
	aID : usize, //ability ID
	id : usize, //id
	targetCountDown : i8, //cooldown before target change
	autoAttackDelay : i16, //cooldown before auto attackng again
	attackSpeedModifier : f32, //increase from items/ from base attack speed
	target : usize, //ID of target
	targetCells : [i8 ; 2], //pathfinding target cell
	///Stores all the item IDs the champion is holding.<br />
	///**Item IDS:**<br />
	///0 : Null<br />
	///1  : B.F Sword (+10 Attack Damage)<br />
	///2  : Needlessly Large Rod (+10 Ability Power)<br />
	///3  : Giants Belt (+150 health)<br />
	///4  : Chain Vest (+20 Armor)<br />
	///5  : Negatron Cloak (+20 Magic Resist)<br />
	///6  : Recurve Bow (+10% Attack Speed)<br />
	///7  : *Sparring Gloves* (+5% Crit Chance, +10% Dodge Chance)<br />
	///8  : Tear of the Goddess (+15 Mana)<br />
	///9  : Spatula<br />
	///11 : Deathblade (+40, +70, +100 Attack Damage - Star Level Dependent)<br />
 	///12 : *Hextech Gunblade* (Dealing Magic and True Damage heals the owner and lowest health ally for 25% of the damage)<br />
	///13 : Zekes Herald (Grants 30% bonus attack speed to the holder and 2 adjacent allies in same row)<br />
	///14 : Edge of Night (At 50% health - once per combat - the holder briefly becomes untargetable and sheds negative effects. Then they gain 30% attack speed)<br />
	///15 : Bloodthirster (Damage dealt heals holder for 25%. Once per combat at 40% Health, gain a 25% maximum health shield for up to 5 seconds)<br />
	///16 : Giant Slayer (Abilities and attacks deal 25% more damage, increased to 50% if the holder has over 2200 maximum health)<br />
	///17 : Infinity Edge (+10 Attack Damage, +75% Crit Chance, +10% Crit Damage, Converts every 1% excess critical strike chance into 1% bonus critical strike damage)<br />
	///18 : Spear of Shojin (Basic attacks restore an additional 8 mana on-attack)<br />
	///19 : Shimmerscale Emblem (Wearer becomes a shimmerscale, cannot equip on a shimmersclae)<br />
	///22 : Rabadons Deathcap (+75 Ability Power)<br />
	///23 : Morellonomicon (+30 Ability Power, magic or true damage from an ability burns the holders target, dealing 25% of the targets maximum health as trude damage over 10 seconds and applying grevious wounds for the duration)<br />
	///24 : Locket of the Iron Solari (At the start of combat, the wearer and all allies within 2 hexes in the same row gain a 300 / 350 / 400 health shield for 15 seconds - star level dependent)<br />
	///25 : Ionic Spark (Enemies within 3 hexes have their magic resistance reduced by 50% (does not stack). When enemies within 3 hexes cast their ability, they are dealt 250% of their maximum mana as magic damage)<br />
	///26 : Guinsoos Rageblade (Basic attacks grant 6% bonus attack speed for the rest of combat, stacks with no upper limit)<br />
	///27 : *Jeweled Gauntlet* (+15% Crit Chance, +40% Crit Damage, +10 Ability Power, The holders magic adn true damage from abilities can critically strike)<br />
	///28 : Archangels Staff (Grants the wearer 20 ability power every 5 seconds)<br />
	///29 : Dragonmancer Emblem (Wearer becomes an dragonmancer, cannot equip on an dragonmancer)<br />
	///33 : Warmogs Armor (+1000 Health)<br />
	///34 : Sunfire Cape (+400 Health. At the start of combat and every 2 seconds thereafter, applies a 10% maximum health burn as true damage over 10 seconds and applying grevious wounds for the duration)<br />
	///35 : Zephyr (At the start of combat, banishes for 5 seconds the unit that mirrors the wielders placement on the other side of the board. Pierces through CC immunity effects)<br />
	///36 : ZZ Rot Portal (At the start of combat, the wearer taunts enemies within 4 hexes. When the wearer dies, a Voidspawn arises, taunting nearby enemies. Summoned units can spawn Voidspawns at 25% effectiveness)<br />
	///37 : *Banshees Claw* (+15% Dodge Chance, +150 Health, At the beginning of each round, the holder and allies within 1 hex in the same row gain a shield that blocks the first enemy ability, up to 600 damage)<br />
	///38 : Redemption (Every 5 seconds, the wearer radiates an aura to allies within 1 hex, healing them for 12% missing health. Affected allies take 25% reduced damage from AOE attacks for  seconds)<br />
	///39 : Guardian Emblem (Wearer becomes a guardian, cannot equip on a guardian)<br />
	///44 : Bramble Vest (+60 Armor. Negatves 75% bonus damage from critical hits. On being hit by an attack, deal 75 / 100 / 150 magic damage to all nearby enemies (once every 2.5 seconds))<br />
	///45 : Gargoyle Stoneplate (+18 Armor and Magic Resist for each enemy targeting the holder)<br />
	///46 : *Titans Resolve* (Gain 2 attack damage and ability power when attacking or taking damage. After stacking 25 times, gain 25 armor and magic resist and stop stacking)<br />
	///47 : *Shroud of Stillness* (Shoot a beam that delays the first cast of affected enemies by 35%)<br />
	///48 : Frozen Heart (Reduce the attack speed of enemies within 2 hexes by 25%)<br />
	///49 : Cavalier Emblem (Wearer becomes a cavalier, cannot equip on a cavalier)<br />
	///55 : Dragons Claw (+120 Magic Resist, every 2 seconds, regenerate 1.2% maximum health for each enemy targeting the holder. If holder is a dragon, increase all bonuses and effects by 20%)<br />
	///56 : *Runaans Hurricane* (+10 Atttack Damage, attacks fire a bolt at a nearby enemy, dealing 70% of the holders attack damage as physical damage)<br />
	///57 : *Quicksilver* (+20% attack speed. Immune to crowd control for 15 secnds)<br />
	///58 : Chalice of Power (+30 Ability Power to holder and 2 adjacent allies on same row)<br />
	///59 : Mirage Emblem (Wearer becomes a mirage, cannot equip on a mirage)<br />
	///66 : Rapid Firecannon (+50% attack speed and +1 attack range, attacks cannot miss)<br />
	///67 : *Last Whisper* (Dealing physical damage reduces the targets armor by 50% for 5 seconds, does not stack)<br />
	///68 : Statikk Shiv (+15% attack speed, every 3rd attack shocks enemies for 70 magic damage and reduces their magic resist by 50% for 5 seconds)<br />
	///69 : Ragewing Emblem (Wearer becomes a ragewing, cannot equip on a ragewing)<br />
	///77 : *Thiefs Gloves* (Each round equip 2 random items, improve with player level, you cannot equip other items)<br />
	///78 : *Hand of Justice* (+15 attack damage, +15% ability power. Attacks and abilities heal for 15% of damage dealt. Each round randomly increase 1 effect by 30%)<br />
	///79 : *Assassin Emblem* (Wearer becomes an assassin, cannot equip on an assassin)<br />
	///88 : Blue Buff (+20 Starting Mana. Gain 20 mana after casting an ability)<br />
	///89 : Mage Emblem (Wearer becomes a mage, cannot equip on a mage)<br />
	///99 : Tacticians Crown (Increase board unit size by 1)<br />
	items : [u8 ; 3], //item abilities 
	ap : i32, //ability power
	se : Vec<StatusEffect>, //status effects
	gMD : i16, //generate mana delay, after abilities 1 second before can start generating mana again
	starLevel : usize,
	incomingDMGModifier : i32,
	initialHP : i32,
	targetable : bool,
	shed : u8,
	shields : Vec<Shield>,
	//sortBy : i8,
	traits : Vec<u8>, //trait abilities
	zap : bool //zap for ionic spark on ability cast
}

impl SummonedChampion 
{
	//Method for converting PlacedChampion into SummonChampion
	fn new(placedChampion : &PlacedChampion, id : usize) -> SummonedChampion
	{
		let starLevel = placedChampion.star; //Get STart Level
		let ofChampion = &CHAMPIONS[placedChampion.id];
		let mut traits = ofChampion.traits.to_vec();
		traits.retain(|x| *x != 0);//optimisation
		SummonedChampion { location: [placedChampion.location[0], placedChampion.location[1]], //create summoned champ with all details
						   movementProgress : [0, 0],
						   health: ofChampion.hp[starLevel], 
						   initialHP : 0,
						   cm: ofChampion.sm, //update current mana to starting mana
						   dc: 0, 
						   cr : 25,
						   critD : 130,
						   mc: ofChampion.mc, 
						   ar: ofChampion.ar * 2, //when calculating distances in cube grid, 1 adjacent hex is calculated as "2" away due to the p, q, r coordinate system, thus attack range is doubled.
						   mr: ofChampion.mr, 
						   ad: ofChampion.ad[starLevel], 
						   aS: ofChampion.aS, 
						   ra: ofChampion.ra * 2,//because distanceBetweenPoints returns value twice as large
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
						   incomingDMGModifier : 1,
						   targetable : true,
						   shed : 0,
						   shields : Vec::new(),
						   //sortBy : 0,
						   traits : traits,
						   zap : false, //discrepency maybe if order of status Effects is ever affected, alternative would be to iterate through status Effects and check for ionic spark
						}
	}
	fn heal(&mut self, mut healingAmount : i32)
	{
		for statusEffect in &self.se
		{
			if statusEffect.statusType == StatusType::GreviousWounds()
			{
				healingAmount /= 2;
				break;
			}
		}
		self.health += healingAmount;
		self.health = min(self.health, self.initialHP);
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
	p1Augments : [u8 ; 3],
	p2Augments : [u8 ; 3],
	timeUnit : i8, //time unit for board in centiseconds (1/100 of a second
	//gridSize : [i8 ; 2], //grid size [x, y, gridType]
	movementAmount : i8, //will be calculated, const / timeUnit
}


///GiveItemEffect : (func)<br />
///Gives an item effect to a champion<br />
///**Item IDS:**<br />
///0 : Null<br />1  : B.F Sword (+10 Attack Damage)<br />2  : Needlessly Large Rod (+10 Ability Power)<br />3  : Giants Belt (+150 health)<br />4  : Chain Vest (+20 Armor)<br />5  : Negatron Cloak (+20 Magic Resist)<br />6  : Recurve Bow (+10% Attack Speed)<br />7  : *Sparring Gloves* (+5% Crit Chance, +10% Dodge Chance)<br />8  : Tear of the Goddess (+15 Mana)<br />9  : Spatula<br />11 : Deathblade (+40, +70, +100 Attack Damage - Star Level Dependent)<br /> 12 : *Hextech Gunblade* (Dealing Magic and True Damage heals the owner and lowest health ally for 25% of the damage)<br />13 : Zekes Herald (Grants 30% bonus attack speed to the holder and 2 adjacent allies in same row)<br />14 : Edge of Night (At 50% health - once per combat - the holder briefly becomes untargetable and sheds negative effects. Then they gain 30% attack speed)<br />15 : Bloodthirster (Damage dealt heals holder for 25%. Once per combat at 40% Health, gain a 25% maximum health shield for up to 5 seconds)<br />16 : Giant Slayer (Abilities and attacks deal 25% more damage, increased to 50% if the holder has over 2200 maximum health)<br />17 : Infinity Edge (+10 Attack Damage, +75% Crit Chance, +10% Crit Damage, Converts every 1% excess critical strike chance into 1% bonus critical strike damage)<br />18 : Spear of Shojin (✓) (Basic attacks restore an additional 8 mana on-attack)<br />19 : Shimmerscale Emblem (Wearer becomes a shimmerscale, cannot equip on a shimmersclae)<br />22 : Rabadons Deathcap (+75 Ability Power)<br />23 : Morellonomicon (+30 Ability Power, magic or true damage from an ability burns the holders target, dealing 25% of the targets maximum health as trude damage over 10 seconds and applying grevious wounds for the duration)<br />24 : Locket of the Iron Solari (At the start of combat, the wearer and all allies within 2 hexes in the same row gain a 300 / 350 / 400 health shield for 15 seconds - star level dependent)<br />25 : Ionic Spark (Enemies within 3 hexes have their magic resistance reduced by 50% (does not stack). When enemies within 3 hexes cast their ability, they are dealt 250% of their maximum mana as magic damage)<br />26 : Guinsoos Rageblade (Basic attacks grant 6% bonus attack speed for the rest of combat, stacks with no upper limit)<br />27 : *Jeweled Gauntlet* (+15% Crit Chance, +40% Crit Damage, +10 Ability Power, The holders magic adn true damage from abilities can critically strike)<br />28 : Archangels Staff (Grants the wearer 20 ability power every 5 seconds)<br />29 : Dragonmancer Emblem (Wearer becomes an dragonmancer, cannot equip on an dragonmancer)<br />33 : Warmogs Armor (+1000 Health)<br />34 : Sunfire Cape (+400 Health. At the start of combat and every 2 seconds thereafter, applies a 10% maximum health burn as true damage over 10 seconds and applying grevious wounds for the duration)<br />35 : Zephyr (At the start of combat, banishes for 5 seconds the unit that mirrors the wielders placement on the other side of the board. Pierces through CC immunity effects)<br />36 : ZZ Rot Portal (At the start of combat, the wearer taunts enemies within 4 hexes. When the wearer dies, a Voidspawn arises, taunting nearby enemies. Summoned units can spawn Voidspawns at 25% effectiveness)<br />37 : *Banshees Claw* (+15% Dodge Chance, +150 Health, At the beginning of each round, the holder and allies within 1 hex in the same row gain a shield that blocks the first enemy ability, up to 600 damage)<br />38 : Redemption (Every 5 seconds, the wearer radiates an aura to allies within 1 hex, healing them for 12% missing health. Affected allies take 25% reduced damage from AOE attacks for  seconds)<br />39 : Guardian Emblem (Wearer becomes a guardian, cannot equip on a guardian)<br />44 : Bramble Vest (+60 Armor. Negatves 75% bonus damage from critical hits. On being hit by an attack, deal 75 / 100 / 150 magic damage to all nearby enemies (once every 2.5 seconds))<br />45 : Gargoyle Stoneplate (+18 Armor and Magic Resist for each enemy targeting the holder)<br />46 : *Titans Resolve* (Gain 2 attack damage and ability power when attacking or taking damage. After stacking 25 times, gain 25 armor and magic resist and stop stacking)<br />47 : *Shroud of Stillness* (Shoot a beam that delays the first cast of affected enemies by 35%)<br />48 : Frozen Heart (Reduce the attack speed of enemies within 2 hexes by 25%)<br />49 : Cavalier Emblem (Wearer becomes a cavalier, cannot equip on a cavalier)<br />55 : Dragons Claw (+120 Magic Resist, every 2 seconds, regenerate 1.2% maximum health for each enemy targeting the holder. If holder is a dragon, increase all bonuses and effects by 20%)<br />56 : *Runaans Hurricane* (+10 Atttack Damage, attacks fire a bolt at a nearby enemy, dealing 70% of the holders attack damage as physical damage)<br />57 : *Quicksilver* (+20% attack speed. Immune to crowd control for 15 secnds)<br />58 : Chalice of Power (+30 Ability Power to holder and 2 adjacent allies on same row)<br />59 : Mirage Emblem (Wearer becomes a mirage, cannot equip on a mirage)<br />66 : Rapid Firecannon (+50% attack speed and +1 attack range, attacks cannot miss)<br />67 : *Last Whisper* (Dealing physical damage reduces the targets armor by 50% for 5 seconds, does not stack)<br />68 : Statikk Shiv (+15% attack speed, every 3rd attack shocks enemies for 70 magic damage and reduces their magic resist by 50% for 5 seconds)<br />69 : Ragewing Emblem (Wearer becomes a ragewing, cannot equip on a ragewing)<br />77 : *Thiefs Gloves* (Each round equip 2 random items, improve with player level, you cannot equip other items)<br />78 : *Hand of Justice* (+15 attack damage, +15% ability power. Attacks and abilities heal for 15% of damage dealt. Each round randomly increase 1 effect by 30%)<br />79 : *Assassin Emblem* (Wearer becomes an assassin, cannot equip on an assassin)<br />88 : Blue Buff (+20 Starting Mana. Gain 20 mana after casting an ability)<br />89 : Mage Emblem (Wearer becomes a mage, cannot equip on a mage)<br />99 : Tacticians Crown (Increase board unit size by 1)<br />
fn GiveItemEffect(item : u8, friendlyChampions : &mut Vec<SummonedChampion>, enemyChampions : &mut Vec<SummonedChampion>, selfIndex : usize)
{
	match item
	{
		0 => (),
		1  => friendlyChampions[selfIndex].ad += 10, //
		2  => friendlyChampions[selfIndex].ap += 10, //
		3 => friendlyChampions[selfIndex].health += 150, //
		4 => friendlyChampions[selfIndex].ar += 20, //
		5 => friendlyChampions[selfIndex].mr += 20,//
		6 => friendlyChampions[selfIndex].attackSpeedModifier += 0.1,//
		7 => {friendlyChampions[selfIndex].cr += 5; friendlyChampions[selfIndex].dc += 10},//
		8 => friendlyChampions[selfIndex].cm += 15,//
		11 => friendlyChampions[selfIndex].ad += [40, 70, 100][friendlyChampions[selfIndex].starLevel],//
		12 => {friendlyChampions[selfIndex].ad += 10; friendlyChampions[selfIndex].ap += 10},//
		13 => {friendlyChampions[selfIndex].ad += 10; friendlyChampions[selfIndex].health += 150;//
			  let thisLocation = friendlyChampions[selfIndex].location;
			  for friendlyChamp in friendlyChampions
			  {
				if friendlyChamp.location[1] == thisLocation[1] && DistanceBetweenPoints(friendlyChamp.location, thisLocation) < 3
				{
					friendlyChamp.attackSpeedModifier *= 1.3; //discrepency, if this is activated before another attack speed bonus that should be before combat, it will not be accurate
				}
			  }
			  },
		14 => {friendlyChampions[selfIndex].ad += 10; friendlyChampions[selfIndex].ar += 20; 
			   friendlyChampions[selfIndex].se.push(StatusEffect { duration: 32767, statusType: StatusType::EdgeOfNight(), ..Default::default()})},//
		15 => {friendlyChampions[selfIndex].ad += 10; friendlyChampions[selfIndex].mr += 20;
			   friendlyChampions[selfIndex].se.push(StatusEffect { duration: 32767, statusType: StatusType::Bloodthirster(), ..Default::default()})		
		},
		16 => {friendlyChampions[selfIndex].ad += 10; friendlyChampions[selfIndex].attackSpeedModifier += 0.1},//
		17 => {friendlyChampions[selfIndex].ad += 10; friendlyChampions[selfIndex].cr += 75; friendlyChampions[selfIndex].critD += 10},// //discrepency cuz crit rate ig
		18 => {friendlyChampions[selfIndex].ad += 10; friendlyChampions[selfIndex].cm += 15},//
		19 => {friendlyChampions[selfIndex].ad += 10; /*friendlyChampions[selfIndex].traits.push() - Shimmerscale*/},
		22 => {friendlyChampions[selfIndex].ap += 75},
		23 => {friendlyChampions[selfIndex].ap += 40; friendlyChampions[selfIndex].health += 150}//
		24 => {friendlyChampions[selfIndex].ap += 10; friendlyChampions[selfIndex].ar += 20;//
			   	let shieldAmount = [300, 350, 400][friendlyChampions[selfIndex].starLevel];
			   	let thisLocation = friendlyChampions[selfIndex].location;
				for friendlyChamp in friendlyChampions
				{
					if friendlyChamp.location[1] == thisLocation[1] && DistanceBetweenPoints(friendlyChamp.location, thisLocation) < 3
					{
						friendlyChamp.shields.push(Shield{duration : 1500, size : shieldAmount});
					}
				}
			   
		},
		25 => {friendlyChampions[selfIndex].ap += 10; friendlyChampions[selfIndex].mr += 20;},//

		_ => println!("Unimplemented Item"),
	}
}
impl Board
{
	fn new(p1PlacedChamps : &Vec<PlacedChampion>, p2PlacedChamps : &Vec<PlacedChampion>, timeUnit : i8) -> Board
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
			  p1Augments : [0, 0, 0],
			  p2Augments : [0, 0, 0],
			  timeUnit : timeUnit,
			  //gridSize : [7, 8],
			  movementAmount : 10 / timeUnit as i8, //optimisation
			}
	}



	fn StartBattle(mut self : Board) -> i8
	{
		let mut debugCount : u32 = 0;



		for i in 0..self.p1Champions.len()//optimisation
		{
			for item in self.p1Champions[i].items
			{
				GiveItemEffect(item, &mut self.p1Champions, &mut self.p2Champions, i);
			}
		}
		for i in 0..self.p2Champions.len()
		{
			for item in self.p2Champions[i].items
			{
				GiveItemEffect(item, &mut self.p2Champions, &mut self.p1Champions, i);
			}
		}

		let mut p1Traits : HashMap<u8, u8> = HashMap::new();
		let mut p2Traits : HashMap<u8, u8> = HashMap::new();
		for p1Champ in &mut self.p1Champions
		{
			for champTrait in &p1Champ.traits
			{
				*p1Traits.entry(*champTrait).or_insert(1) += 1;
				match champTrait
				{
					1 => p1Champ.se.push(StatusEffect { duration: 32767, statusType: StatusType::Assassin(), ..Default::default() }),
					_ => ()
				}
			}
		}
		for p2Champ in &self.p2Champions
		{
			for champTrait in &p2Champ.traits
			{
				*p2Traits.entry(*champTrait).or_insert(1) += 1;
			}
		}

		/*Augments:
		0 => None
		1 => Assassin Heart*/
		for augment in self.p1Augments
		{
			match augment
			{
				0 => continue,
				1 => *p1Traits.entry(1).or_insert(1) += 1,
				_ => (),
			}

		}

		for champTrait in p1Traits
		{	
			/*Traits:
			0 - 
			1 - Assassin */



			match champTrait.0
			{
				1 => {
					let mut extraCritChance = 15;
					let mut extraCritDamage = 5;
					if champTrait.1 > 5
					{
						extraCritChance = 45;
						extraCritDamage = 45;
					}
					else if champTrait.1 > 3
					{
						extraCritChance = 30;
						extraCritDamage = 25;
					}
					for p1Champ in &mut self.p1Champions
					{
						if p1Champ.traits.contains(&1)
						{
							p1Champ.cr += extraCritChance;//discrepency maybe doesnt apply like this
							p1Champ.critD += extraCritDamage;
						}
					}}
				_ => ()
			}
		}
		for p1Champ in &mut self.p1Champions
		{
			p1Champ.initialHP = p1Champ.health;
		}
		for p1Champ in &mut self.p1Champions
		{
			p1Champ.initialHP = p1Champ.health;
		}
		while self.p1Champions.len() > 0 && self.p2Champions.len() > 0
		{
			println!("Debug : Iteration {}", debugCount);
			debugCount += 1;
			for p1ChampionIndex in 0..self.p1Champions.len()
			{
				takeTurn(p1ChampionIndex, &mut self.p1Champions, &mut self.p2Champions, self.timeUnit, self.movementAmount)
			}
			for p2ChampionIndex in 0..self.p2Champions.len()
			{
				takeTurn(p2ChampionIndex, &mut self.p2Champions, &mut self.p1Champions, self.timeUnit, self.movementAmount)
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
    //let playerOneChamps : Vec<PlacedChampion> = vec![PlacedChampion{id : 0, star : 1, items : [0, 0, 0], location : [3, 0]}, PlacedChampion{id : 0, star : 1, items : [0, 0, 0], location : [9, 0]}, PlacedChampion{id : 0, star : 1, items : [0, 0, 0], location : [6, 0]}];
	let playerOneChamps : Vec<PlacedChampion> = vec![PlacedChampion{id : 0, star : 0, items : [0, 0, 0], location : [3, 0]}];
	let playerTwoChamps : Vec<PlacedChampion> = vec![PlacedChampion{id : 1, star : 0, items : [0, 0, 0], location : [6, 7]}];
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
///Returns a distance twice as large (a distance of 1 hex returns 2)
fn DistanceBetweenPoints(point1 : [i8 ; 2], point2 : [i8 ; 2]) -> i8//optimisation doesnt need to borrow?
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
///damageType : 0 = physical, 1 = magical, 2 = true
fn dealDamage(selfIndex : usize,
			  friendlyChampions : &mut Vec<SummonedChampion>,
			  target : &mut SummonedChampion,
			  damageAmount : i32,
			  damageType : u8,
			  )
{
	let mut damage : i32 = 0;
	match damageType
	{
		0 => {damage = (100 * damageAmount * target.incomingDMGModifier) / (100 * (100 + target.ar));
			  if friendlyChampions[selfIndex].cr > rand::thread_rng().gen_range(0..100)//optimisation
			  {
				let mut critD = friendlyChampions[selfIndex].critD;
				if friendlyChampions[selfIndex].cr > 100 && friendlyChampions[selfIndex].items.contains(&17)
				{
					critD += (friendlyChampions[selfIndex].cr - 100) as i32
				}
				damage *= friendlyChampions[selfIndex].critD;
				damage /= 100;
			  }
			  if friendlyChampions[selfIndex].items.contains(&15)//maybe not only physical damage, discrepency?
			  {
				friendlyChampions[selfIndex].heal(damage / 4);
			  }

		},
		1 => {damage = (100 * damageAmount * friendlyChampions[selfIndex].ap * target.incomingDMGModifier) / (100 * (100 + target.mr));
			  if friendlyChampions[selfIndex].items.contains(&27)
			  {
				if friendlyChampions[selfIndex].cr > rand::thread_rng().gen_range(0..100)
				{
					damage *= friendlyChampions[selfIndex].critD;
					damage /= 100;
				}
			  }
			  if friendlyChampions[selfIndex].items.contains(&12)
			  {
				let healing = damage / 4;
				friendlyChampions[selfIndex].heal(healing);
				let mut lowestHP : i32 = 999999;
				let mut lowestHPID : usize = 0;
				for (i, champ) in friendlyChampions.iter().enumerate()
				{
					if i != selfIndex && champ.health < lowestHP
					{
						lowestHP = champ.health;
						lowestHPID = i;
					}
				}
				if lowestHPID != selfIndex
				{
					friendlyChampions[lowestHPID].heal(healing);
				}
			  }
			  if friendlyChampions[selfIndex].items.contains(&23)
			  {
				target.se.push(StatusEffect { duration: 1000, statusType: StatusType::GreviousWounds(), isNegative: true });
				let dmgToDo = target.initialHP / 4;
				target.se.push(StatusEffect { duration: 1000, statusType: StatusType::MorellonomiconBurn(dmgToDo / 10, dmgToDo, 100), isNegative : true})//discrepency unsure whether burn just reapplies itself
			}
			},
		2 => {
			if friendlyChampions[selfIndex].items.contains(&12)
			{
			  let healing = damage / 4;
			  friendlyChampions[selfIndex].heal(healing);
			  let mut lowestHP : i32 = 999999;
			  let mut lowestHPID : usize = 0;
			  for (i, champ) in friendlyChampions.iter().enumerate()
			  {
				  if i != selfIndex && champ.health < lowestHP
				  {
					  lowestHP = champ.health;
					  lowestHPID = i;
				  }
			  }
			  if lowestHPID != selfIndex
			  {
				  friendlyChampions[lowestHPID].heal(healing);
			  }}
			if friendlyChampions[selfIndex].items.contains(&23)
			{
				target.se.push(StatusEffect { duration: 1000, statusType: StatusType::GreviousWounds(), isNegative: true });
				let dmgToDo = target.initialHP / 4;
				target.se.push(StatusEffect { duration: 1000, statusType: StatusType::MorellonomiconBurn(dmgToDo / 10, dmgToDo, 100), isNegative : true})//discrepency unsure whether burn just reapplies itself
			}
			},
		_ => ()
	}
	if friendlyChampions[selfIndex].items.contains(&16)
	{
		if target.initialHP >= 2200
		{
			damage += (damage / 20) * 9;//discrepency in division yada
		}
		else {
			damage += damage / 5;
		}
	}


	if target.gMD <= 0
	{
		target.cm += (7 * damage  / 100) as u8; //discrepency, should be 1% of premitigation and 7% of post.
	}
	while target.shields.len() > 0
	{
		if target.shields[0].size < damage
		{
			damage -= target.shields[0].size;
			target.shields.swap_remove(0);
		}
		else 
		{
			target.shields[0].size -= damage;
			damage = 0;	
		}
	}
	target.health -= damage;
}

fn UpdateShield(shield : &mut Shield, timeUnit : i8) -> bool
{
	shield.duration -= timeUnit as i16;//optimisation
	return shield.duration > 0
}
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
fn performStatus(statusEffect : &mut StatusEffect, friendlyChampions : &mut Vec<SummonedChampion>, timeUnit : i8, selfIndex : usize, stun : &mut ABooleanWithExtraSteps, seToAdd : &mut Vec<StatusEffect>) -> bool
{//discrepency on whether the last tick of a status applies or not etc
	statusEffect.duration -= timeUnit as i16;
	if friendlyChampions[selfIndex].shed == 2
	{
		if statusEffect.isNegative
		{
			statusEffect.duration = 0;
		}
	}
	if statusEffect.duration <= 0
	{
		match statusEffect.statusType
			{
				StatusType::AttackSpeedBuff(_, modifier) => friendlyChampions[selfIndex].attackSpeedModifier /= modifier,
				StatusType::IncreaseDamageTaken(_, modifier) => friendlyChampions[selfIndex].incomingDMGModifier *= 100 / modifier,
				StatusType::Untargetable(_) => friendlyChampions[selfIndex].targetable = true,//discrepency if have 2 untargetable effects this will untarget too early
				StatusType::MorellonomiconBurn(_, dmgToDo, _) => friendlyChampions[selfIndex].health -= dmgToDo,
				StatusType::IonicSparkEffect() => {friendlyChampions[selfIndex].mr *= 2; friendlyChampions[selfIndex].zap = false}, //discrepency maybe if something like illaoi/ daega ult reduces mr it wont increase by equal amount 
				
				_ => ()//println!("Unimplemented")
			}
		return false
	}
	match statusEffect.statusType
	{
		StatusType::AttackSpeedBuff(false, modifier) => {friendlyChampions[selfIndex].attackSpeedModifier *= modifier;
															  statusEffect.statusType = StatusType::AttackSpeedBuff(true, modifier)},
		StatusType::Stun() => stun.value = true,
		StatusType::IncreaseDamageTaken(false, modifier) => {friendlyChampions[selfIndex].incomingDMGModifier *= modifier / 100;
																  statusEffect.statusType = StatusType::IncreaseDamageTaken(true, modifier)}
		StatusType::EdgeOfNight() => {if friendlyChampions[selfIndex].health <= (friendlyChampions[selfIndex].initialHP / 2)
									  {
										seToAdd.push(StatusEffect{duration : 50, statusType : StatusType::Untargetable(false), ..Default::default()});//optimisation at every ..Default::default() with instead isNegative : false
										seToAdd.push(StatusEffect { duration: 32767, statusType: StatusType::AttackSpeedBuff(false, 1.3), ..Default::default()}); //discrepency technically attack speed buff comes into play after untargetable wears off
										friendlyChampions[selfIndex].shed = 1;
										return false
									  }}
		StatusType::Bloodthirster() => {if friendlyChampions[selfIndex].health <= (4 * friendlyChampions[selfIndex].initialHP) / 10
										{
											let quarterHP = friendlyChampions[selfIndex].initialHP / 4;
											friendlyChampions[selfIndex].shields.push(Shield{duration : 500, size : quarterHP});
											
											return false
										}}
		StatusType::Assassin() => {if friendlyChampions[selfIndex].location[1] >= 4
		{
			friendlyChampions[selfIndex].location[1] = 0;
		}
		else 
		{
			friendlyChampions[selfIndex].location[1] = 0;//discrepency maybe leap not instantaneous/ first frame?
		}
		return false}
		StatusType::Untargetable(false) => {friendlyChampions[selfIndex].targetable = false; statusEffect.statusType = StatusType::Untargetable(true)}, //optimise with not recreating status Type?
		StatusType::MorellonomiconBurn(dmgPerTick, dmgToDo, duration) => {let newDuration = duration - (timeUnit as i16);
											                        if newDuration <= 0
																	{
																		friendlyChampions[selfIndex].health -= dmgPerTick;
																		statusEffect.statusType = StatusType::MorellonomiconBurn(dmgPerTick, dmgToDo - dmgPerTick, 100);//discrepency maybe apply burn more often like every half second
																	}
																	else 
																	{
																		statusEffect.statusType = StatusType::MorellonomiconBurn(dmgPerTick, dmgToDo, newDuration);	}},
		StatusType::IonicSparkEffect() => {friendlyChampions[selfIndex].mr /= 2; friendlyChampions[selfIndex].zap = true}
																		_ => ()//println!("Unimplemented")
	}
	true
}
fn takeTurn(selfIndex : usize, friendlyChampions : &mut Vec<SummonedChampion>, enemyChampions : &mut Vec<SummonedChampion>, timeUnit : i8, movementAmount : i8)
{
	/*
	friendlyChampions[selfIndex] : this champion
	friendlyChampionsLocations : location of all friend champs (array of positions), for pathfinding
	enemyChampions : all enemy champions, for targetting
	timeUnit : time unit of a frame, in centiseconds
	movementAmount : precalculated movement distance for 1 frame
	gridSize : depreciated
		*/
	//let mut thisChamp = &mut friendlyChampions[selfIndex]; //optimisation, maybe setting friendkyChampions[selfIndex] to a var is much faster than repeatedly calling access to a vector??
	friendlyChampions[selfIndex].targetCountDown -= timeUnit;//Reduce cooldown to check target/ find new target
	friendlyChampions[selfIndex].autoAttackDelay -= timeUnit as i16;//Risks going out of bounds as auto attack value may not be called for some time
	friendlyChampions[selfIndex].gMD -= timeUnit as i16;
	{
		let mut statusEffects = friendlyChampions[selfIndex].se.clone();
		let mut stun = ABooleanWithExtraSteps{value : false};
		let mut seToAdd : Vec<StatusEffect> = Vec::new();
		statusEffects.retain_mut(|x| performStatus(x, friendlyChampions, timeUnit, selfIndex, &mut stun, &mut seToAdd));
		friendlyChampions[selfIndex].se = statusEffects;
		//deffo optimisation around statusEffects
		friendlyChampions[selfIndex].se.extend(seToAdd);
		if friendlyChampions[selfIndex].shed == 1
		{
			friendlyChampions[selfIndex].shed = 2;
		}
		else if friendlyChampions[selfIndex].shed == 2
		{
			friendlyChampions[selfIndex].shed = 0;
		}
		if stun.value
		{
			println!("stunned");
		}
	}
	friendlyChampions[selfIndex].shields.retain_mut(|x| UpdateShield(x, timeUnit));
	//does auto attack delay need to reset on pathing? does attack instantly after reaching path/ in range


	let mut index : usize = 99;//Cache index of target in enemyChampions
	let mut distanceToTarget : i8 = 127;//Distance to target (is set either while finding target or when target found)
	let mut needNewTargetCell : bool = false;//Bool to store whether new path is needed
	if friendlyChampions[selfIndex].targetCountDown >= 0 //if already has target and doesnt want to change targets 
	{
		//maybe optimisation to first check for if enemyChampions[friendlyChampions.target]
		for (i, enemyChampion) in enemyChampions.iter().enumerate() //every enemy champ
		{
			if enemyChampion.id == friendlyChampions[selfIndex].target && friendlyChampions[selfIndex].targetable //if they share id
			{
				println!("Debug : Found Target");
				index = i;//set index
				distanceToTarget = DistanceBetweenPoints(enemyChampion.location, friendlyChampions[selfIndex].location);//calculate distance
				break;
			}
		}	
	}
	if index == 99 //index not updating from initial intilialisation of 99, therefore need new target
	{
		println!("Debug : Looking for Target");
		friendlyChampions[selfIndex].targetCountDown = 100;//reset target cooldown
		friendlyChampions[selfIndex].target = 0;//reset target
		let mut distance; //cache to store distance between enemy and location
		needNewTargetCell = true; //tells us to recalculate pathfinding later
		//discrepency what if target has moved regardless

		for (i, enemyChampion) in enemyChampions.iter().enumerate() //for every champ
		{
			if !enemyChampion.targetable//discrepency zapped with ionic spark if untargetable?
			{
				continue;
			}
			distance = DistanceBetweenPoints(enemyChampion.location, friendlyChampions[selfIndex].location); //calculate distance
			if distance < distanceToTarget //if distance to current enemy champion in loop is lower than distance to current target
			{
				friendlyChampions[selfIndex].target = enemyChampion.id; //change target
				distanceToTarget = distance; //updating distance to new lower value
				index = i; //setting index
			}
		}
	}
	
	if distanceToTarget <= friendlyChampions[selfIndex].ra as i8//if target in range
	{
		println!("Debug : Target in Range");
		println!("Debug : Auto Attack Delay Remaining {0}", friendlyChampions[selfIndex].autoAttackDelay);//discrepency, does auto attack "charge" while moving
		if friendlyChampions[selfIndex].autoAttackDelay <= 0//if autoattack ready
		{
			println!("Debug : Delay Smaller than 0 - Attacking");
			/* 
			friendlyChampions[selfIndex].aS = attacks per 1 second
			friendlyChampions[selfIndex].autoAttackDelay = time in 1/10 of second until next attack
			friendlyChampions[selfIndex].attackSpeedIncrease = percentage increase in attack speed
			
			
			autoAttackDelay (seconds) = 1 / (attackSpeed * attackSpeedMod)
			autoAttackDelay (centiseconds) = 100 / (attackSpeed * attackSpeedMod)
			
			*/
			println!("as: {}, mod: {}", friendlyChampions[selfIndex].aS, friendlyChampions[selfIndex].attackSpeedModifier);
			friendlyChampions[selfIndex].autoAttackDelay = (100.0 / (friendlyChampions[selfIndex].aS * friendlyChampions[selfIndex].attackSpeedModifier)) as i16; //calculating auto attack delay
			println!("Auto attack delay set");
			//attack speed unclear, capped at five yet some champions let you boost beyond it?
			//optimisation definitely here
			if friendlyChampions[selfIndex].gMD <= 0
			{
				friendlyChampions[selfIndex].cm += 10;
				if friendlyChampions[selfIndex].items.contains(&18)
				{
					friendlyChampions[selfIndex].cm += 8;
				}
				println!("gain mana");
			}
			println!("maybe dodge");
			//discrepency maybe can  dodge actual ability
			if enemyChampions[index].dc <= 0 || enemyChampions[index].dc < rand::thread_rng().gen_range(0..100) //calculating whether to dodge
			{//optimisation from not generating random gen
				println!("No Dodge");
				dealDamage(selfIndex, friendlyChampions, &mut enemyChampions[index], friendlyChampions[selfIndex].ad, 0);
				
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
		if needNewTargetCell || friendlyChampions[selfIndex].location == friendlyChampions[selfIndex].targetCells //if need to update pathfinding or at pathfinding target
		//optimisation?, accuracy vs performance cost
		{
			println!("Debug : Need Target Cell");
			friendlyChampions[selfIndex].targetCells = friendlyChampions[selfIndex].location; //setting target cells to location so if it does not find a target this frame will try to do it again
			//optimisation does not need to check every frame

			let mut lowestDistance : i8 = 100; //setting lowestDistance to high value
			let mut newPosition;
			for possibleMove in [[0, -1], [1, -1], [1, 0], [-1, 0], [-1, 1], [0, 1]] //for every possible move
			//optimisation
			{
				newPosition = [friendlyChampions[selfIndex].location[0] + possibleMove[0], friendlyChampions[selfIndex].location[1] + possibleMove[1]];
				distanceToTarget = DistanceBetweenPoints(newPosition, enemyChampions[index].location);
				if distanceToTarget < lowestDistance
				{
					let mut failed = false;
					if ! InGridHexagon(newPosition)
					{
						continue;
					}
					for friendlyChampionLocation in friendlyChampions.iter()
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
					friendlyChampions[selfIndex].targetCells = newPosition;
				}
				
			}
		}
		
		println!("Debug : Moving to Target Cell");
		friendlyChampions[selfIndex].movementProgress[0] += movementAmount * sign(friendlyChampions[selfIndex].targetCells[0] - friendlyChampions[selfIndex].location[0]);//optimisation here
		println!("Debug : Position ({0},{1}) -- Movement Progress ({2},{3})", friendlyChampions[selfIndex].location[0], friendlyChampions[selfIndex].location[1], friendlyChampions[selfIndex].movementProgress[0], friendlyChampions[selfIndex].movementProgress[1]);
		if friendlyChampions[selfIndex].movementProgress[0].abs() == 10
		{
			friendlyChampions[selfIndex].location[0] += sign(friendlyChampions[selfIndex].movementProgress[0]);
			friendlyChampions[selfIndex].movementProgress[0] = 0;
			
		}
		friendlyChampions[selfIndex].movementProgress[1] += movementAmount * sign(friendlyChampions[selfIndex].targetCells[1] - friendlyChampions[selfIndex].location[1]);
		if friendlyChampions[selfIndex].movementProgress[1].abs() == 10
		{
			friendlyChampions[selfIndex].location[1] += sign(friendlyChampions[selfIndex].movementProgress[1]);
			friendlyChampions[selfIndex].movementProgress[1] = 0;
			
		}
	}
	
	//Ionic spark, optimisation, could be status effect but enemies not passed into function? also doesnt need to be check every turn
	if friendlyChampions[selfIndex].items.contains(&25)
	{
		let thisLocation = friendlyChampions[selfIndex].location;
		for enemyChamp in enemyChampions.iter_mut()
		{
			if DistanceBetweenPoints(thisLocation, enemyChamp.location) < 7//discrepency check distance between points returns value twice as large?
			{
				enemyChamp.se.push(StatusEffect { duration: (timeUnit + 1) as i16, statusType: StatusType::IonicSparkEffect(), isNegative: true});
			}
		}
	}
	
	
	if friendlyChampions[selfIndex].cm >= friendlyChampions[selfIndex].mc
	{
		if friendlyChampions[selfIndex].zap
		{
			friendlyChampions[selfIndex].health -= ((friendlyChampions[selfIndex].mc as i32) * 5) / 2;
		}
		friendlyChampions[selfIndex].cm = 0;
		friendlyChampions[selfIndex].gMD = 100;
		CHAMPIONABILITIES[friendlyChampions[selfIndex].aID](friendlyChampions, enemyChampions, selfIndex);
		
	}
}