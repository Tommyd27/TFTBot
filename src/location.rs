enum FilterType {
	///i8 : Distance to check
	///Location : Other Location
	DistanceFilter(i8, Location)
}

fn generate_filter(filter : FilterType) -> impl for<'a> Fn(&&mut SummonedChampion) -> bool { 
	match filter {
		FilterType::DistanceFilter(dis, location) => {
			return move |n : &&mut SummonedChampion| {
				n.location.distanceBetweenPoints(&location) < dis
			  }
		}
	}	
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Location {
	x : i8,
	y : i8
}
impl Location {
	fn calculateZ(&self) -> i8{
		-self.x - self.y
	}
	fn distanceBetweenPoints(&self, otherPos : &Location) -> i8 {
		(self.x - otherPos.x).abs() + (self.y - otherPos.y).abs() + (self.calculateZ() - otherPos.calculateZ()).abs()
	}
	fn addPositions(posOne : &Location, posTwo : &Location) -> Location{
		Location {
			x : posOne.x + posTwo.x,
			y : posOne.y + posTwo.y,
		}
	}
	fn subPositions(posOne : &Location, posTwo : &Location) -> Location{
		Location {
			x : posOne.x - posTwo.x,
			y : posOne.y - posTwo.y,
		}
	}
	fn addPositionVec(posOne : &Location, posTwo : [i8; 2]) -> Location {
		Location {
			x : posOne.x + posTwo[0],
			y : posOne.y + posTwo[1]
		}
	}
	fn checkValid(&self) -> bool {
		if self.x >= 0 && self.x < 10 && self.y >= 0 && self.y < 8 {
			if 2 - (self.y / 2) < self.x && 10 - (self.y / 2) > self.x {
				return true
			}
		}
		false
	}
	fn getClosestToLocation<'a>(&self, enemyChampions : &'a mut VecDeque<SummonedChampion>) -> Option<&'a mut SummonedChampion> {
		enemyChampions.iter_mut().reduce(|x, y| {
			if x.location.distanceBetweenPoints(self) < y.location.distanceBetweenPoints(self) {
				x
			}
			else {
				y
			}
		})
	}
	fn getClosestToLocationTargetable<'a>(&self, enemyChampions : &'a mut VecDeque<SummonedChampion>) -> Option<&'a mut SummonedChampion> {
		enemyChampions.iter_mut().reduce(|x, y| {
			if ! x.getIsTargetable() {
				return y
			}
			else if ! y.getIsTargetable() {
				return x
			}

			if x.location.distanceBetweenPoints(self) < y.location.distanceBetweenPoints(self) {
				return x
			}
			y
		})
	}
	fn getClosestToLocationTargetableIndex<'a>(&self, enemyChampions : &'a mut VecDeque<SummonedChampion>) -> Option<(usize, &'a mut SummonedChampion)> {
		enemyChampions.iter_mut().enumerate().reduce(|(i, x), (j, y)| {
			if ! x.getIsTargetable() {
				return (j, y)
			}
			else if ! y.getIsTargetable() {
				return (i, x)
			}

			if x.location.distanceBetweenPoints(self) < y.location.distanceBetweenPoints(self) {
				return (i, x)
			}
			(j, y)
		})
	}
	fn getWithinDistance(&self, distance : i8) -> impl for<'a> Fn(&&mut SummonedChampion) -> bool {
		generate_filter(FilterType::DistanceFilter(distance, self.clone()))
	}
}
