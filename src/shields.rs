use crate::champions::DamageType;
///Implementation for Shields
pub struct Shield {
    ///duration of shield
    pub duration: i16,
    ///number of damage blocked
    pub size: f32,
    ///Optional choice for whether it only blocks a certain type
    pub blocks_type: Option<DamageType>,

    ///Whether it pops after receiving any damage
    pub pop: bool,
}

impl Shield {
    pub fn update_shield(&mut self, time_unit: i8) -> bool {
        //updates self
        self.duration -= time_unit as i16; //(!O)
        info!("updating shield {} {}", self.duration, self.size);
        self.duration > 0 && self.size > 0.0
    }
    pub fn handle_damage(&mut self, damage: f32, damage_type: DamageType) -> f32 {
        if self.blocks_type.is_none() || self.blocks_type.unwrap() == damage_type {
            let out = damage - self.size;
            self.size -= damage;
            if self.pop {
                self.size = 0.0;
            }
            return out.min(0.0);
        }
        damage
    }
}
///Default for shield
impl Default for Shield {
    fn default() -> Shield {
        Shield {
            duration: 0,
            size: 0.0,
            blocks_type: None,
            pop: false,
        }
    }
}
