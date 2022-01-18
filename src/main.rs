#![allow(non_snake_case)] //Allows snake ase because I like it.

struct Champion
{
    id : u8,
    cost : u8,
    
    hp : [u16; 3],
    sm : u8,
    mc : u8,
    ar : u8,
    mr : u8,
    ad : [u8; 3],
    aS : u8,
    ra : u8,
    
    aID : u8,

    synergies : [u8 ; 3],
}

struct SummonedChampion
{
    id : u8,

    star : u8,
    items : [u8; 3],
    location : [u8; 2]
}

struct Player
{
    id : u8,
    gold : u8,
    health : u8,
    level : u8,
    xp : u8,

    champions : [u8 ; 25],

}

fn main() {
    let champions = [Champion{id : 0, cost : 1, hp : [700, 1260, 2268], sm : 0, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, synergies : [1, 2, 0]}, 
                 Champion{id : 1, cost : 2, hp : [900, 1620, 2916], sm : 50, mc : 100, ar : 40, mr : 40, ad : [77, 138, 248], aS : 7, ra : 3, aID : 0, synergies : [2, 3, 0]}, 
                 Champion{id : 2, cost : 3, hp : [700, 1260, 2268], sm : 35, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, synergies : [4, 5, 0]}];
    let mut Chadden = SummonedChampion{id : 0, star : 1, items : [0, 0, 0]};
    let mut SomeGuy = SummonedChampion{id : 1, star : 2, items : [0, 0, 0]};

}
