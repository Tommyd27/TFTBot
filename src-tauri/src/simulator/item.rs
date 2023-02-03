use serde::{Serialize, Deserialize};
use surrealdb::sql::{Object, Value};
use crate::prelude::*;
pub const DEFAULT_ITEMS : [Item ; 47] = [
    Item {id : 1, ad : 10.0, health : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 2, ap : 0.1, health : 0.0, ad : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 3, health : 150.0, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 4, ar : 0.2, health : 0.0, ad : 0.0, ap : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 5, mr : 0.2, health : 0.0, ad : 0.0, ap : 0.0, ar : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 6, attack_speed_modifier : 1.1, health : 0.0, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 7, cr : 5, dc : 10, health : 0.0, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 8, cm : 15, health : 0.0, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 11, ad : 15.0, health : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 12, ad : 10.0, ap : 0.1, health : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 13, ad : 10.0, health : 150.0, attack_speed_modifier : 1.3, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 14, ad : 10.0, ar : 0.2, health : 0.0, ap : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 15, ad : 10.0, mr : 0.2, omnivamp : 0.25, health : 0.0, ap : 0.0, ar : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, crit_damage : 0.0},
    Item {id : 16, ad : 10.0, attack_speed_modifier : 1.1, health : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 17, ad : 10.0, cm : 75, crit_damage : 0.1, health : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, omnivamp : 0.0},
    Item {id : 18, ad : 10.0, cm : 15, health : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 19, ad : 10.0, health : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 22, ap : 0.75, health : 0.0, ad : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 23, ap : 0.4, health : 150.0, ad : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 24, ap : 0.1, ar : 0.2, health : 0.0, ad : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 25, ap : 0.1, mr : 0.2, health : 0.0, ad : 0.0, ar : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 26, ap : 0.1, attack_speed_modifier : 1.1, health : 0.0, ad : 0.0, ar : 0.0, mr : 0.0, ra : 0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 27, ap : 0.5, cr : 15, crit_damage : 0.4, health : 0.0, ad : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, dc : 0, cm : 0, omnivamp : 0.0},
    Item {id : 28, ap : 0.1, cm : 15, health : 0.0, ad : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 29, ap : 0.1, health : 0.0, ad : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 33, health : 1000.0, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 34, health : 300.0, ar : 0.2, ad : 0.0, ap : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 35, health : 150.0, mr : 0.2, ad : 0.0, ap : 0.0, ar : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 36, health : 150.0, attack_speed_modifier : 1.1, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 37, health : 150.0, dc : 15, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 38, health : 150.0, cm : 15, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 39, health : 150.0, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 44, ar : 0.8, health : 0.0, ad : 0.0, ap : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 45, ar : 0.2, mr : 0.2, health : 0.0, ad : 0.0, ap : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 46, ar : 0.2, attack_speed_modifier : 1.1, health : 0.0, ad : 0.0, ap : 0.0, mr : 0.0, ra : 0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 47, ar : 0.2, dc : 15, health : 0.0, ad : 0.0, ap : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 48, ar : 0.2, cm : 15, health : 0.0, ad : 0.0, ap : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 55, mr : 1.2, health : 0.0, ad : 0.0, ap : 0.0, ar : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 56, mr : 0.2, ad : 10.0, attack_speed_modifier : 1.1, health : 0.0, ap : 0.0, ar : 0.0, ra : 0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 57, mr : 0.2, dc : 15, attack_speed_modifier : 1.2, health : 0.0, ad : 0.0, ap : 0.0, ar : 0.0, ra : 0, cr : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 58, cm : 15, mr : 0.2, ap : 0.3, health : 0.0, ad : 0.0, ar : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 66, attack_speed_modifier : 1.55, ra : 1, health : 0.0, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, cr : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 67, attack_speed_modifier : 1.21, cr : 15, health : 0.0, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, dc : 0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 68, attack_speed_modifier : 1.21, cm : 15, health : 0.0, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, cr : 0, dc : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 77, cr : 15, dc : 15, health : 0.0, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cm : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 78, cm : 10, cr : 15, health : 0.0, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, dc : 0, omnivamp : 0.0, crit_damage : 0.0},
    Item {id : 88, cm : 50, health : 0.0, ad : 0.0, ap : 0.0, ar : 0.0, mr : 0.0, ra : 0, attack_speed_modifier : 1.0, cr : 0, dc : 0, omnivamp : 0.0, crit_damage : 0.0},
];

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Item {
    pub id : u8,
    pub health : f32,
    pub ad : f32,
    pub ap : f32,
    pub ar : f32,
    pub mr : f32,
    pub attack_speed_modifier : f32,
    pub ra : i8,
    pub cr : u8,
    pub dc : u8,
    pub cm : i16,
    pub omnivamp : f32,
    pub crit_damage : f32,
}

impl Item {
    pub fn into_values(&self) -> [(String, Value) ; 13] {
        [
            ("id".into(), self.id.into()),
            ("health".into(), self.health.into()),
            ("ad".into(), self.ad.into()),
            ("ap".into(), self.ap.into()),
            ("ar".into(), self.ar.into()),
            ("mr".into(), self.mr.into()),
            ("attack_speed_modifier".into(), self.attack_speed_modifier.into()),
            ("ra".into(), self.ra.into()),
            ("cr".into(), self.cr.into()),
            ("dc".into(), self.dc.into()),
            ("cm".into(), self.cm.into()),
            ("omnivamp".into(), self.omnivamp.into()),
            ("crit_damage".into(), self.crit_damage.into())
        ]
    }
}

impl TryFrom<Object> for Item {
    type Error = Error;
    fn try_from(mut obj: Object) -> Result<Self> {
        let ad = obj.remove("ad").unwrap().as_float() as f32;
        let ap = obj.remove("ap").unwrap().as_float() as f32;
        let ar = obj.remove("ar").unwrap().as_float() as f32;
        let mr = obj.remove("mr").unwrap().as_float() as f32;
        let attack_speed_modifier = obj.remove("attack_speed_modifier").unwrap().as_float() as f32;
        let cm = obj.remove("cm").unwrap().as_int() as i16;
        let cr = obj.remove("cr").unwrap().as_int() as u8;
        let crit_damage = obj.remove("crit_damage").unwrap().as_float() as f32;
        let dc = obj.remove("dc").unwrap().as_int() as u8;
        let health = obj.remove("health").unwrap().as_float() as f32;
        let omnivamp = obj.remove("omnivamp").unwrap().as_float() as f32;
        let ra = obj.remove("ra").unwrap().as_int() as i8;
        let id : u8 = Value::from(obj.remove("id").unwrap().record().unwrap().id).as_int() as u8;
        Ok(Item { id, ad, ap, attack_speed_modifier, health, ar, mr, ra, cr, dc, cm, omnivamp, crit_damage}) 
    }
}
impl Default for Item {
    fn default() -> Self {
        Item {
            id : 0,
            health : 0.0,
            ad : 0.0, 
            ap : 0.0,
            ar : 0.0,
            mr : 0.0,
            ra : 0,
            attack_speed_modifier : 1.0,
            cr : 0,
            dc : 0,
            cm : 0,
            omnivamp : 0.0,
            crit_damage : 0.0,
        }
    }
}