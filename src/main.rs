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

}


fn main() {
    let champions = [Champion{id : 0, cost : 1, hp : [700, 1260, 2268], sm : 0, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, synergies : [1, 2, 0]}, 
                 Champion{id : 1, cost : 2, hp : [900, 1620, 2916], sm : 50, mc : 100, ar : 40, mr : 40, ad : [77, 138, 248], aS : 7, ra : 3, aID : 0, synergies : [1, 2, 0]}, 
                 Champion{id : 2, cost : 3, hp : [700, 1260, 2268], sm : 35, mc : 35, ar : 25, mr : 25, ad : [75, 135, 243], aS : 7, ra : 3, aID : 0, synergies : [1, 2, 0]}];



}
