use crate::champions::SummonedChampion;
use std::collections::VecDeque;
///findChampionIndexFromID:<br />
///champions : &Vec<SummonedChampion> - List of champions to iterate through<br />
///id : usize - ID wanted<br />
///returns : Option<usize> - Some(correct id) or None if not found
pub fn find_champion_index_from_id(champions: &VecDeque<SummonedChampion>, id: usize) -> Option<usize> {
    //(!D) swap this out for check targetable as well

    if champions[id].equal_id(id) {
        return Some(id);
    }

    for champ in champions {
        if champ.equal_id(id) {
            return Some(id);
        }
    }
    None
}
///Same as find champ index from id but also checks it is targetable/ not banished
pub fn find_champion_index_from_id_targetable(
    champions: &VecDeque<SummonedChampion>,
    id: usize,
) -> Option<usize> {
    let mut out: Option<usize> = None;
    if champions[id].equal_id(id) {
        out = Some(id)
    } else {
        for champ in champions {
            if champ.equal_id(id) {
                out = Some(id);
                break;
            }
        }
    }
    if out.is_some() && champions[out.unwrap()].get_is_targetable() {
        return out;
    }
    None
}

///0 if num is 0, 1 if num > 0, -1 if num < 0
pub fn sign(num: i8) -> i8 {
    if num == 0 {
        return 0;
    } else if num > 0 {
        return 1;
    }
    -1
}
