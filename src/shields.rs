///Implementation for Shields
struct Shield {
	///duration of shield
	duration : i16,
	///number of damage blocked
	size : f32,
	///Optional choice for whether it only blocks a certain type
	blocksType : Option<DamageType>,

	///Whether it pops after receiving any damage
	pop : bool,
}

impl Shield {
	fn updateShield(&mut self, timeUnit : i8) -> bool { //updates self
		self.duration -= timeUnit as i16; //(!O)
		return self.duration > 0 && self.size > 0.0
	}
}
///Default for shield
impl Default for Shield {
	fn default() -> Shield {
		Shield
		{
			duration : 0,
			size : 0.0,
			blocksType : None,
			pop : false
		}
	}
}
