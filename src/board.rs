
///Board Struct:<br />
///Simulates battles
struct Board
{
	///Vec of player 1's champs
	p1Champions : VecDeque<SummonedChampion>, 
	
	///Vec of player 2's champs
	p2Champions : VecDeque<SummonedChampion>, 

	///Time unit for board in centiseconds (1/100 of a second)
	timeUnit : i8, 

	///movement amount per tick, is calculated by const / time unit
	movementAmount : i8, 
}

impl Board
{
	fn new(p1PlacedChamps : &VecDeque<PlacedChampion>, p2PlacedChamps : &VecDeque<PlacedChampion>, timeUnit : i8) -> Board
	{
		let mut p1Champions = VecDeque::new();
		let mut p2Champions = VecDeque::new();
		for (i, p1Champion) in p1PlacedChamps.iter().enumerate()//(!O) converts placed champions to summoned champions
		{
			p1Champions.push_back(SummonedChampion::new(&p1Champion, i));//converts into summoned champ

		}

		for (i, p2Champion) in p2PlacedChamps.iter().enumerate()
		{
			p2Champions.push_back(SummonedChampion::new(&p2Champion, i));//converts into summoned champ
		}
		
		Board{p1Champions : p1Champions,
			  p2Champions : p2Champions,
			  timeUnit : timeUnit,
			  movementAmount : 10 / timeUnit as i8, //(!O)
			}//creates new board
	}



	fn StartBattle(mut self : Board) -> i8
	{
		let mut debugCount : u32 = 0;
		let mut p1Projectiles : Vec<Projectile> = Vec::new();//instantiate projectiles vecs
		let mut p2Projectiles : Vec<Projectile> = Vec::new();
		while self.p1Champions.len() > 0 && self.p2Champions.len() > 0//take turns while there are champions alive
		{
			println!("Debug : Iteration {}", debugCount);
			debugCount += 1;//count turns
			for _champCount in 0..self.p1Champions.len()//take turn for all p1Champs
			{
				let mut thisChamp = self.p1Champions.pop_front().unwrap();
				thisChamp.setup(&mut self.p1Champions, &mut self.p2Champions);
				let alive = thisChamp.takeTurn(&mut self.p1Champions, &mut self.p2Champions, self.timeUnit, self.movementAmount, &mut p1Projectiles);
				if alive{
					self.p1Champions.push_back(thisChamp)
				}
			}



			

			for _champCount in 0..self.p2Champions.len()//take turn for all p1Champs
			{
				let mut thisChamp = self.p2Champions.pop_front().unwrap();
				thisChamp.setup(&mut self.p2Champions, &mut self.p1Champions);
				let alive = thisChamp.takeTurn(&mut self.p2Champions, &mut self.p1Champions, self.timeUnit, self.movementAmount, &mut p2Projectiles);
				if alive{
					self.p2Champions.push_back(thisChamp)
				}
			}
			p1Projectiles.retain_mut(|f| f.SimulateTick(&mut self.p2Champions, &mut self.p1Champions));
			p2Projectiles.retain_mut(|f| f.SimulateTick(&mut self.p1Champions, &mut self.p2Champions));//simulate projectile ticks
		}
		println!("Debug : Battle Over");
		if self.p1Champions.len() == 0//check winner and get champ information
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
