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

struct PlacedChampion
{
    id : u8, //champ id

    star : u8, //star level
    items : [u8; 3], //items given
    location : [u8; 2] //location on board
}

struct SummonedChampion
{
	location : [u8 ; 4],
	health : u16,
	sm : u8,
	mc : u8,
	ar : u8,
	mr : u8,
	ad : u8,
	aS : u8,
	ra : u8,
	aID : u8,

	tIDs : [u8 ; 3],
}


struct Player
{
    id : u8, //player id
    gold : u8, //gold stored
    winstreak : i8, //win streak, can be +-
    health : u8, //player health
    level : u8, //player level
    xp : u8, //player xp


    champions : [u8 ; 25], //all player champions
	augments : [u8 ; 3] //augments
}

struct board
{
	playerOneChampions : [PlacedChampion ; 12],
	playerTwoChampions : [PlacedChampion ; 12],
}





fn main() {
    let champions = [Champion{id : 0, cost : 1, hp : [700, 1260, 2268], sm : 0, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, traits : [1, 2, 0]}, 
                 Champion{id : 1, cost : 2, hp : [900, 1620, 2916], sm : 50, mc : 100, ar : 40, mr : 40, ad : [77, 138, 248], aS : 7, ra : 3, aID : 0, traits : [2, 3, 0]}, 
                 Champion{id : 2, cost : 3, hp : [700, 1260, 2268], sm : 35, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, traits : [4, 5, 0]}];
    let mut Chadden = SummonedChampion{id : 0, star : 1, items : [0, 0, 0]};
    let mut SomeGuy = SummonedChampion{id : 1, star : 2, items : [0, 0, 0]};

}
