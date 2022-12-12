use crate::champions::DamageType;
///Implementation for Shields
pub struct Shield {
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
	pub fn updateShield(&mut self, timeUnit : i8) -> bool { //updates self
		self.duration -= timeUnit as i16; //(!O)
		return self.duration > 0 && self.size > 0.0
	}
	pub fn handleDamage(&mut self, damage : f32, damageType : DamageType) -> f32 {
		if self.blocksType.is_none() || self.blocksType.unwrap() == damageType {
			let out = damage - self.size;
			self.size -= damage;
			if self.pop {
				self.size = 0.0;
			}
			return out.min(0.0);
		}
		return damage
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
