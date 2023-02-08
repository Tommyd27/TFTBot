use serde::Serialize;

use super::champions::DamageType;
///Implementation for Shields
#[derive(Debug, Clone, Serialize)]
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
    ///updates shield, reducing duration with time unit returns bool whether should be kept or removed
    pub fn update_shield(&mut self, time_unit: i8) -> bool {
        //updates duration
        self.duration -= time_unit as i16;
        info!("updating shield {} {}", self.duration, self.size);
        //returns whether duration and size is still above zero
        self.duration > 0 && self.size > 0.0
    }
    ///handles incoming damage, returning the remaining damage
    pub fn handle_damage(&mut self, damage: f32, damage_type: DamageType) -> f32 {
        if self.blocks_type.is_none() || self.blocks_type.unwrap() == damage_type { //if it blocks all types or the specific damage type
            let out = damage - self.size; //reduce the damage
            self.size -= damage; //reduce the size
            if self.pop { //if pop ie removes self after any damage
                self.size = 0.0;
            }
            return out.max(0.0); //returns the damage or 0, whatever larger
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
