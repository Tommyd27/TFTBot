///findChampionIndexFromID:<br />
///champions : &Vec<SummonedChampion> - List of champions to iterate through<br />
///id : usize - ID wanted<br />
///returns : Option<usize> - Some(correct id) or None if not found
fn findChampionIndexFromID(champions : &VecDeque<SummonedChampion>, id : usize) -> Option<usize> { //(!D) swap this out for check targetable as well

	if champions[id].id == id { return Some(id) }

	for champ in champions { 
		if champ.id == id { 
			return Some(id); 
		} 
	}
	None
}
///Same as find champ index from id but also checks it is targetable/ not banished
fn findChampionIndexFromIDTargetable(champions : &VecDeque<SummonedChampion>, id : usize) -> Option<usize> {
	let mut out : Option<usize> = None;
	if champions[id].id == id { out = Some(id) }
	else {
		for champ in champions { 
			if champ.id == id { 
				out = Some(id); 
				break; 
			} 
		}
	}
	if out.is_some() {
		if champions[out.unwrap()].getIsTargetable() {
			return out
		}
	}
	None
}


///0 if num is 0, 1 if num > 0, -1 if num < 0
fn sign(num : i8) -> i8 {
	if num == 0 { return 0 }
	else if num > 0 { return 1 }
	-1
}
