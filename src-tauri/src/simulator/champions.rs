// Import required types from other modules
use super::location::Location;
use super::projectiles::Projectile;
use super::shields::Shield;
use super::status_effects::{StatusEffect, StatusType, Stun};
use super::utils::{find_champion_index_from_id, find_champion_index_from_id_targetable, sign};
use super::item::Item;
use crate::prelude::*;

// Import functionality from core
use core::fmt;
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use std::mem::take;

// Import surrealdb Value object and serde Serialize.
use surrealdb::sql::{Value, Object};
use serde::{Serialize, Deserialize};


///Champion struct<br />
///Holds basic information about the champion
#[derive(Debug, Serialize, Deserialize)]
pub struct Champion {
    ///index in champions array
    pub id: u8,

    ///healthpoints 
    hp: f32,

    ///starting mana
    sm: i16,

    ///ability mana cost
    mc: i16,

    ///base armor value
    ar: f32,

    ///Base Magic Resist Value
    mr: f32,

    ///attack damage
    ad: f32,

    ///attack speed (attacks per second)
    attack_speed: f32,

    ///attack range
    ra: i8,
}

///Converts from a surrealdb Object to a champion
impl TryFrom<Object> for Champion {
    type Error = Error;
    fn try_from(mut obj: Object) -> Result<Self> {
        //fetch and convert values from the object
        let ad : f32 = obj.remove("ad").unwrap().as_float() as f32;
        let ar : f32 = obj.remove("ar").unwrap().as_float() as f32;
        let attack_speed : f32 = obj.remove("attack_speed").unwrap().as_float() as f32;
        let hp : f32 = obj.remove("hp").unwrap().as_float() as f32;
        let id : u8 = Value::from(obj.remove("id").unwrap().record().unwrap().id).as_int() as u8;
        let mc : i16 = obj.remove("mc").unwrap().as_int() as i16;
        let mr : f32 = obj.remove("mr").unwrap().as_float() as f32;
        let ra : i8 = obj.remove("ra").unwrap().as_int() as i8;
        let sm : i16 = obj.remove("sm").unwrap().as_int() as i16;
        //return new champ
        Ok(Champion { id, hp, sm, mc, ar, mr, ad, attack_speed, ra})
    }
}
/// Default values for champs
impl Default for Champion {
    fn default() -> Self {
        Champion { id: 0, hp: 0.0, sm: 0, mc: 0, ar: 0.0, mr: 0.0, ad: 0.0, attack_speed: 0.0, ra: 0 }
    }
}

impl Champion {
    ///Converts from a champion into String Value array for insertion in database
    pub fn into_values(&self) -> [(String, Value) ; 9] {
        [
            ("id".into(), self.id.into()),
            ("hp".into(), self.hp.into()),
            ("sm".into(), self.sm.into()),
            ("mc".into(), self.mc.into()),
            ("ar".into(), self.ar.into()),
            ("mr".into(), self.mr.into()),
            ("ad".into(), self.ad.into()),
            ("attack_speed".into(), self.attack_speed.into()),
            ("ra".into(), self.ra.into())
        ]
    }
}

/// Default champions array
pub const DEFAULT_CHAMPIONS: [Champion; 4] = [
    //Support
    Champion {
        id: 0,
        hp: 1100.0,
        sm: 70,
        mc: 140,
        ar: 0.25,
        mr: 0.25,
        ad: 70.0,
        attack_speed: 0.6,
        ra: 2,
    },
    //Bruiser
    Champion {
        id: 1,
        hp: 1400.0,
        sm: 50,
        mc: 100,
        ar: 0.45,
        mr: 0.45,
        ad: 100.0,
        attack_speed: 0.7,
        ra: 1,
    },
    //AD Ranged
    Champion {
        id: 2,
        hp: 1200.0,
        sm: 35,
        mc: 100,
        ar: 0.25,
        mr: 0.25,
        ad: 120.0,
        attack_speed: 0.7,
        ra: 3,
    },
    //AP Ranged
    Champion {
        id: 3,
        hp: 1200.0,
        sm: 35,
        mc: 150,
        ar: 0.25,
        mr: 0.25,
        ad: 60.0,
        attack_speed: 0.6,
        ra: 3,
    },
];

///Enum for the 3 damage types Physical, Magical and True
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize)]
pub enum DamageType {
    Physical(),
    Magical(),

    #[allow(dead_code)]
    True(),
}

///PlacedChampion:<br />
///Stores information about a champion's location and status on a board (as well as ID of actual champion)
#[derive(Deserialize, Debug, Serialize)]
pub struct PlacedChampion {
    ///id of the associated champion
    id: usize,

    ///star level of champion
    star: usize,

    ///items
    items: [u8; 3],

    ///location on board
    location: Location,

    ///option for team
    team : Option<u8>
}
///Converts from a surrealdb object from the database to a PlacedChampion
impl TryFrom<Object> for PlacedChampion {
    type Error = Error;
    fn try_from(mut obj: Object) -> Result<Self> {
        let id = obj.remove("of_champ").unwrap().as_int() as usize;
        let item_0 = obj.remove("item_0").unwrap().as_int() as u8;
        let item_1 = obj.remove("item_1").unwrap().as_int() as u8;
        let item_2 = obj.remove("item_2").unwrap().as_int() as u8;
        let star_level = obj.remove("star").unwrap().as_int() as usize;
        let location_x = obj.remove("location_x").unwrap().as_int() as i8;
        let location_y = obj.remove("location_y").unwrap().as_int() as i8;
        let team = obj.remove("team").unwrap().as_int() as u8;
        Ok(PlacedChampion { id, star: star_level, items: [item_0, item_1, item_2], location: Location { x: location_x, y: location_y }, team : Some(team) })
    }
}

impl PlacedChampion {
    ///converts a placed champion into a String value array for insertion into the database
    pub fn into_values(&self) -> [(String, Value); 7] {
        [("of_champ".into(), self.id.into()),
         ("star".into(), self.star.into()),
         ("item_0".into(), self.items[0].into()),
         ("item_1".into(), self.items[1].into()),
         ("item_2".into(), self.items[2].into()),
         ("location_x".into(), self.location.x.into()),
         ("location_y".into(), self.location.y.into())
        ]
    }
    ///generates a random placed champion
    pub fn generate_random_champ(id_range : usize, valid_items : &Vec<u8>, team : bool) -> PlacedChampion {
        //generates a random id and star level
        let id = rand::thread_rng().gen_range(0..id_range);
        let star = rand::thread_rng().gen_range(0..3) as usize;

        //initialise item array
        let mut items : [u8 ; 3] = [0, 0, 0];
        for i in 0..rand::thread_rng().gen_range(0..3) {
            //choose item from valid_items
            items[i] = *valid_items.choose(&mut rand::thread_rng()).unwrap();
        }
        //generate random location
        let location = Location::generate_random_position_team(team);
        
        //returns placed champion
        PlacedChampion { id, star, items, location, team: None }
    }
}

///Struct for champion placed on board in a battle
#[derive(Debug, Clone, Serialize)]
pub struct SummonedChampion {
    ///location
    pub location: Location,

    ///id of associated champion/ placed champion
    of_champ_id : usize,

    ///progress of movement before new square, goes up to 10 then moves
    movement_progress: [i8; 2],

    ///health
    health: f32,

    ///current mana
    cm: i16,

    ///dodge chance in %
    dc: u8,

    ///crit rate in %
    cr: u8,

    ///crit damage
    crit_damage: f32,

    ///ability mana cost
    mc: i16,

    ///armor
    ar: f32,

    ///magic resist
    mr: f32,

    ///attack damage
    ad: f32,

    ///attacks per second/ attack speed
    attack_speed: f32,

    ///auto attack range
    ra: i8,

    ///unique id
    id: usize,

    ///cooldown before target chance
    target_cooldown: i8,

    ///cooldown before auto attacking again
    auto_attack_delay: i16,

    ///attack speed modifier from items and effects
    attack_speed_modifier: f32,

    ///id of target
    target: usize,

    ///pathfinding target cell
    target_cells: Location,

    ///Stores all the item IDs the champion is holding.<br />
    ///**Item IDS:**<br />
    ///0 : Null<br />
    ///1  : B.F Sword (+10 Attack Damage)<br />
    ///2  : Needlessly Large Rod (+10 Ability Power)<br />
    ///3  : Giants Belt (+150 health)<br />
    ///4  : Chain Vest (+20 Armor)<br />
    ///5  : Negatron Cloak (+20 Magic Resist)<br />
    ///6  : Recurve Bow (+10% Attack Speed)<br />
    ///7  : *Sparring Gloves* (+5% Crit Chance, +10% Dodge Chance)<br />
    ///8  : Tear of the Goddess (+15 Mana)<br />
    ///9  : Spatula<br />
    ///11 : Deathblade (+40, +70, +100 Attack Damage - Star Level Dependent)<br />
    ///12 : *Hextech Gunblade* (Dealing Magic and True Damage heals the owner and lowest health ally for 25% of the damage)<br />
    ///13 : Zekes Herald (Grants 30% bonus attack speed to the holder and 2 adjacent allies in same row)<br />
    ///14 : Edge of Night (At 50% health - once per combat - the holder briefly becomes untargetable and sheds negative effects. Then they gain 30% attack speed)<br />
    ///15 : Bloodthirster (Damage dealt heals holder for 25%. Once per combat at 40% Health, gain a 25% maximum health shield for up to 5 seconds)<br />
    ///16 : Giant Slayer (Abilities and attacks deal 25% more damage, increased to 50% if the holder has over 2200 maximum health)<br />
    ///17 : Infinity Edge (+10 Attack Damage, +225% Crit Chance, +10% Crit Damage, Converts every 1% excess critical strike chance into 1% bonus critical strike damage)<br />
    ///18 : Spear of Shojin (Basic attacks restore an additional 8 mana on-attack)<br />
    ///19 : Shimmerscale Emblem (Wearer becomes a shimmerscale, cannot equip on a shimmersclae)<br />
    ///22 : Rabadons Deathcap (+975 Ability Power)<br />
    ///23 : Morellonomicon (+30 Ability Power, magic or true damage from an ability burns the holders target, dealing 25% of the targets maximum health as trude damage over 10 seconds and applying grevious wounds for the duration)<br />
    ///24 : Locket of the Iron Solari (At the start of combat, the wearer and all allies within 2 hexes in the same row gain a 300 / 350 / 400 health shield for 15 seconds - star level dependent)<br />
    ///25 : Ionic Spark (Enemies within 3 hexes have their magic resistance reduced by 50% (does not stack). When enemies within 3 hexes cast their ability, they are dealt 250% of their maximum mana as magic damage)<br />
    ///26 : Guinsoos Rageblade (Basic attacks grant 6% bonus attack speed for the rest of combat, stacks with no upper limit)<br />
    ///27 : *Jeweled Gauntlet* (+15% Crit Chance, +40% Crit Damage, +10 Ability Power, The holders magic adn true damage from abilities can critically strike)<br />
    ///28 : Archangels Staff (Grants the wearer 20 ability power every 5 seconds)<br />
    ///29 : Dragonmancer Emblem (Wearer becomes an dragonmancer, cannot equip on an dragonmancer)<br />
    ///33 : Warmogs Armor (+1000 Health)<br />
    ///34 : Sunfire Cape (+400 Health. At the start of combat and every 2 seconds thereafter, applies a 10% maximum health burn as true damage over 10 seconds and applying grevious wounds for the duration)<br />
    ///35 : Zephyr (At the start of combat, banishes for 5 seconds the unit that mirrors the wielders placement on the other side of the board. Pierces through CC immunity effects)<br />
    ///36 : ZZ Rot Portal (At the start of combat, the wearer taunts enemies within 4 hexes. When the wearer dies, a Voidspawn arises, taunting nearby enemies. Summoned units can spawn Voidspawns at 25% effectiveness)<br />
    ///37 : *Banshees Claw* (+15% Dodge Chance, +150 Health, At the beginning of each round, the holder and allies within 1 hex in the same row gain a shield that blocks the first enemy ability, up to 600 damage)<br />
    ///38 : Redemption (Every 5 seconds, the wearer radiates an aura to allies within 1 hex, healing them for 12% missing health. Affected allies take 25% reduced damage from AOE attacks for  seconds)<br />
    ///39 : Guardian Emblem (Wearer becomes a guardian, cannot equip on a guardian)<br />
    ///44 : Bramble Vest (+60 Armor. Negatves 75% bonus damage from critical hits. On being hit by an attack, deal 75 / 100 / 150 magic damage to all nearby enemies (once every 2.5 seconds))<br />
    ///45 : Gargoyle Stoneplate (+18 Armor and Magic Resist for each enemy targeting the holder)<br />
    ///46 : *Titans Resolve* (Gain 2 attack damage and ability power when attacking or taking damage. After stacking 25 times, gain 25 armor and magic resist and stop stacking)<br />
    ///47 : *Shroud of Stillness* (Shoot a beam that delays the first cast of affected enemies by 35%)<br />
    ///48 : Frozen Heart (Reduce the attack speed of enemies within 2 hexes by 25%)<br />
    ///49 : Cavalier Emblem (Wearer becomes a cavalier, cannot equip on a cavalier)<br />
    ///55 : Dragons Claw (+120 Magic Resist, every 2 seconds, regenerate 1.2% maximum health for each enemy targeting the holder. If holder is a dragon, increase all bonuses and effects by 20%)<br />
    ///56 : *Runaans Hurricane* (+10 Atttack Damage, attacks fire a bolt at a nearby enemy, dealing 70% of the holders attack damage as physical damage)<br />
    ///57 : *Quicksilver* (+20% attack speed. Immune to crowd control for 15 secnds)<br />
    ///58 : Chalice of Power (+30 Ability Power to holder and 2 adjacent allies on same row)<br />
    ///59 : Mirage Emblem (Wearer becomes a mirage, cannot equip on a mirage)<br />
    ///66 : Rapid Firecannon (+50% attack speed and +1 attack range, attacks cannot miss)<br />
    ///67 : *Last Whisper* (Dealing physical damage reduces the targets armor by 50% for 5 seconds, does not stack)<br />
    ///68 : Statikk Shiv (+15% attack speed, every 3rd attack shocks enemies for 70 magic damage and reduces their magic resist by 50% for 5 seconds)<br />
    ///69 : Ragewing Emblem (Wearer becomes a ragewing, cannot equip on a ragewing)<br />
    ///77 : *Thiefs Gloves* (Each round equip 2 random items, improve with player level, you cannot equip other items)<br />
    ///78 : *Hand of Justice* (+15 attack damage, +15% ability power. Attacks and abilities heal for 15% of damage dealt. Each round randomly increase 1 effect by 30%)<br />
    ///79 : *Assassin Emblem* (Wearer becomes an assassin, cannot equip on an assassin)<br />
    ///88 : Blue Buff (+20 Starting Mana. Gain 20 mana after casting an ability)<br />
    ///89 : Mage Emblem (Wearer becomes a mage, cannot equip on a mage)<br />
    ///99 : Tacticians Crown (Increase board unit size by 1)<br />
    items: [u8; 3],

    ///ability power
    ap: f32,

    ///vec of status effects
    se: Vec<StatusEffect>,

    ///generate mana delay (can't generate mana 1 secomnd after casting ability)
    gain_mana_delay: i16,

    ///star level
    star_level: usize,

    ///incoming DMG modifier
    incoming_damage_modifier: f32,

    ///starting HP
    initial_hp: f32,

    ///can be targeted or not
    targetable: bool,

    ///needs to shed negative status effects
    shed: u8,

    ///vec of all shields
    shields: Vec<Shield>,

    ///whether zapped from ionic spark
    zap: bool,

    ///whether zenith banished
    banish: bool,

    ///titan's resolve stacks
    titans_resolve_stacks: u8,

    ///omnivamp (% of healing from damage done)
    omnivamp: f32,

    ///shiv attack count
    shiv_attack_count: u8,
}

impl SummonedChampion {
    ///converts PlacedChampion into SummonChampion
    pub fn new(placed_champion: &PlacedChampion, id: usize) -> SummonedChampion {
        info!(
            "New Summoned Champion ID : {} Champion ID : {}",
            id, placed_champion.id
        );
        //create summoned champ with all details
        SummonedChampion {
            location: placed_champion.location, 
            of_champ_id: placed_champion.id,
            movement_progress: [0, 0],
            initial_hp: 0.0,
            dc: 0,
            cr: 25,
            crit_damage: 0.3,
            id,
            target_cooldown: 0,
            auto_attack_delay: 0,
            attack_speed_modifier: 1.0,
            target: 255,
            target_cells: Location { x: -1, y: -1 }, //(!O)
            items: placed_champion.items,
            ap: 1.0,
            se: Vec::new(),
            gain_mana_delay: 0,
            star_level : placed_champion.star,
            incoming_damage_modifier: 1.0,
            targetable: true,
            shed: 0,
            shields: Vec::new(),
            //sortBy : 0,
            //traits : traits,
            zap: false, //discrepency maybe if order of status Effects is ever affected, alternative would be to iterate through status Effects and check for ionic spark
            banish: false, //discrepency with this and many others if one status effect banishing ends and another is still going on etc.
            titans_resolve_stacks: 0,
            omnivamp: 0.0,
            shiv_attack_count: 0,
            ..Default::default()
        }
    }
    ///setup champion
    pub fn setup(
        &mut self,
        friendly_champions: &mut VecDeque<SummonedChampion>,
        enemy_champions: &mut VecDeque<SummonedChampion>,
        champions : &[Champion],
        items : &[Item]
    ) {
        info!("Setup of Champion {}", self.id);
        {
            //sets stats to initial values specified by Champion.
            let of_champion = &champions[self.of_champ_id];
            self.health = of_champion.hp;
            self.cm = of_champion.sm;
            self.ar = of_champion.ar;
            self.mr = of_champion.mr;
            self.ad = of_champion.ad;
            self.attack_speed = of_champion.attack_speed;
            self.ra = of_champion.ra * 2;
            self.mc = of_champion.mc;
        }
        {
            //for each item, give item effect
            for item in self.items {
                info!("Giving item effect {}", item);
                self.give_item_effect(item, friendly_champions, enemy_champions, items)
            }
        }
        //set initial hp to current health, accounting for items etc
        self.initial_hp = self.health;
        info!("Set HP to {}", self.health);
    }
    ///heal
    fn heal(&mut self, mut healing_amount: f32) {
        info!("{self} - Healing");

        //check for grevious wounds
        if self.se.contains(&StatusEffect {
            status_type: StatusType::GreviousWounds(),
            ..Default::default()
        }) {
            info!(
                "Has Grevious Wounds cutting halving healing before: {}",
                healing_amount
            );
            healing_amount /= 2.0; //cut healing in half
            info!("After {}", healing_amount);
        }

        self.health = self.initial_hp.min(self.health + healing_amount); //make sure health doesn't go higher than initial amount
        info!("{self}");
    }
    
    ///takes the turn for a summonedchamp, returns whether the champion is alive at end of turn
    pub fn take_turn(
        &mut self,
        friendly_champions: &mut VecDeque<SummonedChampion>,
        enemy_champions: &mut VecDeque<SummonedChampion>,
        time_unit: i8,
        movement_amount: i8,
        projectiles: &mut Vec<Projectile>,
    ) -> bool {
        info!("Taking turn for {self}");
        if self.health <= 0.0 {
            info!("Health below zero, removing self");
            return false;
            //champ is dead
        }

        self.target_cooldown = self.target_cooldown.checked_sub(time_unit).unwrap_or(-1); //Reduce cooldown to check target/ find new target
        self.auto_attack_delay = self
            .auto_attack_delay
            .checked_sub(time_unit as i16)
            .unwrap_or(-1); //Risks going out of bounds as auto attack value may not be called for some time, so checked subtraction
        self.gain_mana_delay = self
            .gain_mana_delay
            .checked_sub(time_unit as i16)
            .unwrap_or(-1);


        {
            info!("Simulating status effects");
            let mut status_effects = take(&mut self.se); //takes status effect vec
            let mut stun = Stun { stun: 0 }; // setting stun to 0
            status_effects.retain_mut(|x| {
                self.perform_status(x, friendly_champions, enemy_champions, time_unit, &mut stun)
            }); //perform status for each status effect

            if self.health <= 0.0 {
                info!("Health below zero from status effect, removing");
                //died from status effect
                return false;
            }

            self.se.extend(status_effects); //extend se by status effects
            //do NOT set self.se = status_effects, as new status may have been added

            self.update_shed(); //updates shed

            self.shields.retain_mut(|x| x.update_shield(time_unit)); //updates all shields

            if stun.stun == 1 {
                info!("Is stunned");
                //stunned
                return true;
            }
        }
        if self.banish {
            //banished
            info!("Is banished");
            return true;
        }

        {
            info!("Calculating auto attack or movement");
            let mut need_new_target_cell: bool = false; //bool to store whether new path is needed

            let mut target_object: Option<SummonedChampion> = None;

            if self.target_cooldown >= 0 {
                info!("Cooldown above zero, trying to find target {}", self.target);
                //if already has target and doesnt want to change targets
                if let Some(index) =
                    find_champion_index_from_id_targetable(enemy_champions, self.target) //try to find target
                {
                    target_object = enemy_champions.swap_remove_back(index);
                    //target found
                    info!("Target found? : {}", target_object.is_some());
                }
            }

            if target_object.is_none() {
                //target_object not found
                info!("Could not find target or need new target");
                self.target_cooldown = 100; //reset target cooldown

                need_new_target_cell = true; //tells us to recalculate pathfinding later
                
                let mut index: Option<usize> = None;
                if let Some((i, champ)) = self
                    .location
                    .get_closest_to_location_targetable_index(enemy_champions) {
                    index = Some(i);//get closest to location that is targetable
                }
                if index.is_none() {
                    //no targetable champions, ending turn
                    info!("No targetable champions, ending turn");
                    return true;
                }

                target_object = enemy_champions.swap_remove_back(index.unwrap());
            }

            let mut target_object: SummonedChampion = target_object.unwrap();
            info!("Target is {target_object}");
            self.target = target_object.id; //set target

            let distance_to_target = self
                .location
                .distance_between_points(&target_object.location); //get distance to target
            info!("Distance to target {distance_to_target}");

            if distance_to_target <= self.ra {
                //if target in range
                info!("Target in range, attacking or reducing auto attack cooldown");
                info!("Auto Attack Delay Remaining {0}", self.auto_attack_delay);
                if self.auto_attack_delay <= 0
                //if autoattack ready
                {
                    info!("Ready to attack");
                    /*
                    self.aS = attacks per 1 second
                    self.autoAttackDelay = time in 1/10 of second until next attack
                    self.attackSpeedIncrease = percentage increase in attack speed


                    autoAttackDelay (seconds) = 1 / (attackSpeed * attackSpeedMod)
                    autoAttackDelay (centiseconds) = 100 / (attackSpeed * attackSpeedMod)

                    */
                    info!(
                        "as: {}, mod: {}",
                        self.attack_speed, self.attack_speed_modifier
                    );
                    self.auto_attack_delay =
                        ((100.0 / (self.attack_speed * self.attack_speed_modifier)) as i16).max(20); //calculating auto attack delay
                    info!("Auto attack delay set to {}", self.auto_attack_delay);
                    if self.items.contains(&26) {
                        self.attack_speed_modifier *= 1.06;
                        info!("Increasing speed with Rageblade")
                    }

                    if self.gain_mana_delay <= 0 {
                        //if can gain mana
                        self.cm += 10;
                        info!("Gaining mana");
                        if self.items.contains(&18) {
                            self.cm += 8;
                            info!("Additional mana from shojin");
                        }
                        info!("Current mana {}", self.cm);
                    }
                    self.shiv_attack_count += 1; //increase shiv count
                    if self.items.contains(&68) && self.shiv_attack_count == 3 {
                        info!("Has shiv and on third auto, applying affect");
                        //shiv effect
                        self.deal_damage(
                            friendly_champions,
                            &mut target_object,
                            50.0,
                            DamageType::Magical(),
                            false,
                        );
                        target_object.se.push(StatusEffect {
                            duration: Some(500),
                            status_type: StatusType::ShredMagicResist(2.0),
                            is_negative: true,
                            ..Default::default()
                        });
                        //for other champs affected by shiv effect
                        for (i, enemy_champ) in enemy_champions.iter_mut().enumerate() {
                            self.deal_damage(
                                friendly_champions,
                                enemy_champ,
                                50.0,
                                DamageType::Magical(),
                                false,
                            );
                            enemy_champ.se.push(StatusEffect {
                                duration: Some(500),
                                status_type: StatusType::ShredMagicResist(2.0),
                                is_negative: true,
                                ..Default::default()
                            });

                            if i > 2 {
                                break;
                            }
                        }
                    }
                    self.shiv_attack_count %= 3; //make sure shiv count sticks between 0 to 2
                    if self.items.contains(&56) {
                        //doing runaan's second auto attack
                        info!("Has runaan's, performing second auto");
                        warn!("Runaan's can be dodged, treated like normal auto");
                        let closest_other_enemy = self
                            .location
                            .get_closest_to_location_targetable(enemy_champions); //fetch closest other champ
                        if let Some(target) = closest_other_enemy {
                            self.deal_damage(
                                friendly_champions,
                                target,
                                self.ad * 0.7,
                                DamageType::Physical(),
                                false,
                            ) //do damage
                        }
                    }
                    info!("Auto attacking");
                    if target_object.dc == 0
                        || target_object.dc < rand::thread_rng().gen_range(0..100)
                        || self.items.contains(&66)
                    //calculating whether to dodge
                    {
                        //no dodge
                        info!("Not dodged");
                        self.deal_damage( //deal auto damage
                            friendly_champions,
                            &mut target_object,
                            self.ad,
                            DamageType::Physical(),
                            false,
                        );

                        info!("Enemy Champion Health is {0}", target_object.health);
                    } else {
                        info!("Dodged Attack");
                    }
                }
                //return target
                enemy_champions.push_back(target_object);
            } else {
                info!("Not in Range");
                if need_new_target_cell || self.location == self.target_cells {
                    //checks if need new pathfinding target
                    info!("Need Target Cell");
                    self.target_cells = self.location; //setting target cells to location so if it does not find a target this frame will try to do it again
                            
                    let mut lowest_distance: i8 = i8::MAX; //setting lowestDistance to high value
                    let mut new_position;
                    for possible_move in [[0, -1], [1, -1], [1, 0], [-1, 0], [-1, 1], [0, 1]]
                    //for every possible move
                    {
                        //calculate distance
                        new_position = Location::add_position_vec(&self.location, possible_move);
                        let distance_to_target = target_object
                            .location
                            .distance_between_points(&new_position);
                        if distance_to_target < lowest_distance {
                            if (!new_position.check_valid())
                                || friendly_champions
                                    .iter()
                                    .any(|f| f.location == new_position)
                            {
                                continue;
                            }
                            //if distance lower, position valid and there is no other friendly champion in cell
                            info!("Found target cell {}", new_position);
                            lowest_distance = distance_to_target;//set new target cell
                            self.target_cells = new_position;
                        }
                    }
                }

                info!("Moving to Target Cell {}", self.target_cells);
                self.movement_progress[0] +=
                    movement_amount * sign(self.target_cells.x - self.location.x); //add movement progress
                info!(
                    "Position ({0:?}) -- Movement Progress ({1:?})",
                    self.location, self.movement_progress
                );
                if self.movement_progress[0].abs() >= 10 {
                    //move self
                    self.location.x += sign(self.movement_progress[0]);
                    self.movement_progress[0] = 0;
                }
                self.movement_progress[1] +=
                    movement_amount * sign(self.target_cells.y - self.location.y);
                if self.movement_progress[1].abs() >= 10 {
                    self.location.y += sign(self.movement_progress[1]);
                    self.movement_progress[1] = 0;
                }

                enemy_champions.push_back(target_object); //return target
            }
        }
        
        if self.items.contains(&25) {
            //giving ionic spark effect
            info!("giving ionic spark effect");
            for champ in enemy_champions
                .iter_mut()
                .filter(self.location.get_within_distance(7))
            {
                //give effect to all enemy champs within range
                info!("Push to {champ}");
                champ.se.push(StatusEffect {
                    duration: Some((time_unit + 1).into()),
                    status_type: StatusType::IonicSparkEffect(),
                    is_negative: true,
                    ..Default::default()
                })
            }
        }

        if self.cm >= self.mc {
            //cast ability
            info!("Enough mana casting ability");
            if self.zap {
                //zap from ionic spakr
                info!("Zap");
                self.health -= (self.mc as f32) * 2.5;
            }
            self.cm = 0;
            if self.items.contains(&88) {
                info!("Bluebuff");
                self.cm = 20;
            }
            self.gain_mana_delay = 100;
            self.cast_ability(friendly_champions, enemy_champions, projectiles); //cast ability
        }
        true
    }
    ///deals damage to a target, requires a source Summoned Champion
    pub fn deal_damage(
        &mut self,
        friendly_champions: &mut VecDeque<SummonedChampion>,
        target: &mut SummonedChampion,
        damage_amount: f32,
        damage_type: DamageType,
        _is_splash: bool,
    ) {
        info!("Dealing {damage_amount} from {self} to {target}");
        //calculate base damage amount
        let mut damage: f32 = damage_amount * target.incoming_damage_modifier;
        info!("Increased to {damage}");
        let can_crit; //initialise can_crit bool
        let mut crit_damage = self.crit_damage; //initialise crit damage variable

        match damage_type {
            DamageType::Physical() => {
                //can crit true, decrease damage by armor
                can_crit = true;
                damage /= 1.0 + target.ar;
                if self.items.contains(&67) {
                    //apply armor shred from last whisper
                    if !target.se.contains(&StatusEffect {
                        status_type: StatusType::LastWhisperShred(),
                        ..Default::default()
                    }) {
                        //give last whisper shred if target does not have it
                        target.se.push(StatusEffect {
                            duration: Some(500),
                            status_type: StatusType::LastWhisperShred(),
                            is_negative: true,
                            ..Default::default()
                        })
                    }
                }
                if self.cr > 100 && self.items.contains(&17) {
                    //give extra crit damage from infinity edge
                    crit_damage += (self.cr - 100) as f32
                }
            }
            DamageType::Magical() => {
                can_crit = self.items.contains(&27);
                damage /= 1.0 + target.mr;//decrease damage by mr
                //can crit depends on jeweled gauntlet
            }
            DamageType::True() => {
                can_crit = self.items.contains(&27); //can crit depends on jeweled
            }
        }

        if can_crit && self.cr > rand::thread_rng().gen_range(0..100) { //check for crit
            info!("Crit");
            let mut additional_crit_damage = damage * crit_damage;
            if target.items.contains(&44) {
                //reduce dmg if target has bramble vest
                additional_crit_damage /= 4.0;
            }
            damage += additional_crit_damage //increase dmg by crit dmg
        }

        if self.items.contains(&16) {
            //give bonus giant's slayer attack dmg
            if target.initial_hp >= 2200.0 {
                damage *= 1.45
            } else {
                damage *= 1.2
            }
        }

        if damage_type != DamageType::Physical() {
            //give gunblade and morellos
            if self.items.contains(&12) {
                //give gunblade healing
                let healing = damage / 4.0; //calculate healing
                self.heal(healing); //heal self

                if let Some(lowest_hp_champ) =
                    friendly_champions
                        .iter_mut()
                        .reduce(|x, y| if x.health < y.health { x } else { y })
                {
                    //get lowest HP ally
                    //if there are any allies
                    lowest_hp_champ.heal(healing)
                }
            }
            if self.items.contains(&23) {
                //if self has morellos give morellos effect
                target.se.push(StatusEffect {
                    duration: Some(1000),
                    status_type: StatusType::GreviousWounds(),
                    is_negative: true,
                    ..Default::default()
                });
                let damage_to_do = target.initial_hp / 4.0;
                target.se.push(StatusEffect {
                    duration: Some(100),
                    status_type: StatusType::MorellonomiconBurn(
                        damage_to_do / 10.0,
                        damage_to_do,
                        100,
                    ),
                    is_negative: true,
                    ..Default::default()
                }) 
            }
        }

        self.heal(damage * self.omnivamp); //give omnivamp healing

        info!("Accounting for shield");
        for shield in &mut target.shields {
            //reduce damage due to shields
            info!("shield : {}", shield.size);
            damage = shield.handle_damage(damage, damage_type); //handle damage
            info!("damage : {}", damage);
            if damage <= 0.0 {
                break;
                //no damage left, stop checking shields
            }
        }

        self.titans_resolve_stacks = 25.min(self.titans_resolve_stacks + 1); //add titan's resolve stacks
        target.titans_resolve_stacks = 25.min(target.titans_resolve_stacks + 1); //give enemy titan's resolve stacks

        target.health -= damage; //deal damage

        if target.gain_mana_delay <= 0 {
            // give mana is delay off
            target.cm = target
                .cm
                .checked_add((0.7 * damage) as i16)
                .unwrap_or(target.mc);
            //give bonus mana
        }
    }
    ///cast ability
    fn cast_ability(
        &mut self,
        friendly_champions: &mut VecDeque<SummonedChampion>,
        enemy_champions: &mut VecDeque<SummonedChampion>,
        projectiles: &mut Vec<Projectile>,
    ) {
        info!("casting ability");
        //match id to its selected ability
        match self.id {
            0 => { //support ability


                let mut player_distances: Vec<(i8, &mut SummonedChampion, bool)> =
                    friendly_champions
                        .iter_mut()
                        .map(|x| (self.location.distance_between_points(&x.location), x, true))
                        .collect(); //create a vector of distances from the SummonedChamp, SummonedChamp and bool representing its team (true = same team, false = other team)
                player_distances.extend( 
                    enemy_champions
                        .iter_mut()
                        .map(|x| (self.location.distance_between_points(&x.location), x, false)),
                ); //extend with enemy champions
                let star_level = self.star_level; //gets current star level

                player_distances.sort_unstable_by_key(|a| a.0); //sorts the player distances so the smallest distances are first
                let number_affected: usize = [3, 4, 5][star_level]; //how many champions it can hit/ effect
                let mut i = 0; //count number already affected
                let ap = self.ap; //get ability power
                for (_, champ, on_team) in player_distances //iterate through play distances
                {
                    if i >= number_affected { //already affected all it can
                        break;
                    }
                    if on_team
                    //if friendly champ
                    {
                        //give allies attack speed for 5 seconds
                        champ.se.push(StatusEffect {
                            duration: Some(500),
                            status_type: StatusType::AttackSpeedBuff(1.7 * ap),
                            ..Default::default()
                        });
                    } else {
                        //enemy champ
                        //stun enemies for 1.5 seconds and increase damage for 20%
                        champ.se.push(StatusEffect {
                            duration: Some(150),
                            status_type: StatusType::Stun(),
                            is_negative: true,
                            ..Default::default()
                        });
                        champ.se.push(StatusEffect {
                            duration: Some(150),
                            status_type: StatusType::IncreaseDamageTaken(1.2 * ap),
                            is_negative: true,
                            ..Default::default()
                        });
                    }
                    i += 1; //add 1 to count of hit enemies
                }
                if i < number_affected
                //give self effect if there aren't enough champs to hit
                {
                    self.se.push(StatusEffect {
                        duration: Some(500),
                        status_type: StatusType::AttackSpeedBuff(1.7 * ap),
                        ..Default::default()
                    });
                }
            }
            1 => { //bruiser ability
                let star_level = self.star_level; //get star level
                let target_index =
                    find_champion_index_from_id(enemy_champions, self.target).unwrap_or(0); //get target object or the first in vec
                self.heal((300.0 + 50.0 * star_level as f32) * self.ap); //heals

                //deals damage
                self.deal_damage(
                    friendly_champions,
                    &mut enemy_champions[target_index],
                    (25.0 * star_level as f32) * 4.0 * self.ad,
                    DamageType::Physical(),
                    false,
                )
            }
            2 => {
                let target = find_champion_index_from_id(enemy_champions, self.target).unwrap_or(0); //get target
                let target_location = enemy_champions[target].location;
                let damage: f32 = self.ad * 3.0 * (self.star_level as f32); //calculate damage
                projectiles.push(Projectile::new(
                    self.location,
                    Option::Some(target_location),
                    self.target,
                    damage,
                    DamageType::Physical(),
                    0.0,
                    5,
                    self.id,
                )) //create new projectile
            }
            3 => {
                //fetches target index
                let target = find_champion_index_from_id(enemy_champions, self.target).unwrap_or(0); //(!D) Can strike from out of range
                                                                                                     //gets their location
                let target_location = enemy_champions[target].location;
                //calculates damage
                let damage: f32 = 250.0 * self.ap * (self.star_level as f32);
                //adds projectile to vec
                projectiles.push(Projectile::new(
                    self.location,
                    Option::Some(target_location),
                    self.target,
                    damage,
                    DamageType::Magical(),
                    damage / 3.0,
                    3,
                    self.id,
                ))
            }
            _ => println!("Unimplemented"),
        }
    }
    /// get number of enemy champions targetting
    pub fn get_num_targeting(&self, enemy_champions: &VecDeque<SummonedChampion>) -> usize {
        enemy_champions
            .iter()
            .filter(|p| p.target == self.id)
            .count() //counts the number of enemy champions with target equal to id
    }
    /// retrieve whether the champion is targetable
    pub fn get_is_targetable(&self) -> bool {
        self.targetable && !self.banish
    }
    ///Gives an item effect to a champion<br />
    ///**Item IDS:**<br />
    ///0 : Null<br />1  : B.F Sword (+10 Attack Damage)<br />2  : Needlessly Large Rod (+10 Ability Power)<br />3  : Giants Belt (+150 health)<br />4  : Chain Vest (+20 Armor)<br />5  : Negatron Cloak (+20 Magic Resist)<br />6  : Recurve Bow (+10% Attack Speed)<br />7  : *Sparring Gloves* (+5% Crit Chance, +10% Dodge Chance)<br />8  : Tear of the Goddess (+15 Mana)<br />9  : Spatula<br />11 : Deathblade (+40, +70, +100 Attack Damage - Star Level Dependent)<br /> 12 : *Hextech Gunblade* (Dealing Magic and True Damage heals the owner and lowest health ally for 25% of the damage)<br />13 : Zekes Herald (Grants 30% bonus attack speed to the holder and 2 adjacent allies in same row)<br />14 : Edge of Night (At 50% health - once per combat - the holder briefly becomes untargetable and sheds negative effects. Then they gain 30% attack speed)<br />15 : Bloodthirster (Damage dealt heals holder for 25%. Once per combat at 40% Health, gain a 25% maximum health shield for up to 5 seconds)<br />16 : Giant Slayer (Abilities and attacks deal 25% more damage, increased to 50% if the holder has over 2200 maximum health)<br />17 : Infinity Edge (+10 Attack Damage, +75% Crit Chance, +10% Crit Damage, Converts every 1% excess critical strike chance into 1% bonus critical strike damage)<br />18 : Spear of Shojin (✓) (Basic attacks restore an additional 8 mana on-attack)<br />19 : Shimmerscale Emblem (Wearer becomes a shimmerscale, cannot equip on a shimmersclae)<br />22 : Rabadons Deathcap (+75 Ability Power)<br />23 : Morellonomicon (+30 Ability Power, magic or true damage from an ability burns the holders target, dealing 25% of the targets maximum health as trude damage over 10 seconds and applying grevious wounds for the duration)<br />24 : Locket of the Iron Solari (At the start of combat, the wearer and all allies within 2 hexes in the same row gain a 300 / 350 / 400 health shield for 15 seconds - star level dependent)<br />25 : Ionic Spark (Enemies within 3 hexes have their magic resistance reduced by 50% (does not stack). When enemies within 3 hexes cast their ability, they are dealt 250% of their maximum mana as magic damage)<br />26 : Guinsoos Rageblade (Basic attacks grant 6% bonus attack speed for the rest of combat, stacks with no upper limit)<br />27 : *Jeweled Gauntlet* (+15% Crit Chance, +40% Crit Damage, +10 Ability Power, The holders magic adn true damage from abilities can critically strike)<br />28 : Archangels Staff (Grants the wearer 20 ability power every 5 seconds)<br />29 : Dragonmancer Emblem (Wearer becomes an dragonmancer, cannot equip on an dragonmancer)<br />33 : Warmogs Armor (+1000 Health)<br />34 : Sunfire Cape (+400 Health. At the start of combat and every 2 seconds thereafter, applies a 10% maximum health burn as true damage over 10 seconds and applying grevious wounds for the duration)<br />35 : Zephyr (At the start of combat, banishes for 5 seconds the unit that mirrors the wielders placement on the other side of the board. Pierces through CC immunity effects)<br />36 : ZZ Rot Portal (At the start of combat, the wearer taunts enemies within 4 hexes. When the wearer dies, a Voidspawn arises, taunting nearby enemies. Summoned units can spawn Voidspawns at 25% effectiveness)<br />37 : *Banshees Claw* (+15% Dodge Chance, +150 Health, At the beginning of each round, the holder and allies within 1 hex in the same row gain a shield that blocks the first enemy ability, up to 600 damage)<br />38 : Redemption (Every 5 seconds, the wearer radiates an aura to allies within 1 hex, healing them for 12% missing health. Affected allies take 25% reduced damage from AOE attacks for  seconds)<br />39 : Guardian Emblem (Wearer becomes a guardian, cannot equip on a guardian)<br />44 : Bramble Vest (+60 Armor. Negatves 75% bonus damage from critical hits. On being hit by an attack, deal 75 / 100 / 150 magic damage to all nearby enemies (once every 2.5 seconds))<br />45 : Gargoyle Stoneplate (+18 Armor and Magic Resist for each enemy targeting the holder)<br />46 : *Titans Resolve* (Gain 2 attack damage and ability power when attacking or taking damage. After stacking 25 times, gain 25 armor and magic resist and stop stacking)<br />47 : *Shroud of Stillness* (Shoot a beam that delays the first cast of affected enemies by 35%)<br />48 : Frozen Heart (Reduce the attack speed of enemies within 2 hexes by 25%)<br />49 : Cavalier Emblem (Wearer becomes a cavalier, cannot equip on a cavalier)<br />55 : Dragons Claw (+120 Magic Resist, every 2 seconds, regenerate 1.2% maximum health for each enemy targeting the holder. If holder is a dragon, increase all bonuses and effects by 20%)<br />56 : *Runaans Hurricane* (+10 Atttack Damage, attacks fire a bolt at a nearby enemy, dealing 70% of the holders attack damage as physical damage)<br />57 : *Quicksilver* (+20% attack speed. Immune to crowd control for 15 secnds)<br />58 : Chalice of Power (+30 Ability Power to holder and 2 adjacent allies on same row)<br />59 : Mirage Emblem (Wearer becomes a mirage, cannot equip on a mirage)<br />66 : Rapid Firecannon (+50% attack speed and +1 attack range, attacks cannot miss)<br />67 : *Last Whisper* (Dealing physical damage reduces the targets armor by 50% for 5 seconds, does not stack)<br />68 : Statikk Shiv (+15% attack speed, every 3rd attack shocks enemies for 70 magic damage and reduces their magic resist by 50% for 5 seconds)<br />69 : Ragewing Emblem (Wearer becomes a ragewing, cannot equip on a ragewing)<br />77 : *Thiefs Gloves* (Each round equip 2 random items, improve with player level, you cannot equip other items)<br />78 : *Hand of Justice* (+15 attack damage, +15% ability power. Attacks and abilities heal for 15% of damage dealt. Each round randomly increase 1 effect by 30%)<br />79 : *Assassin Emblem* (Wearer becomes an assassin, cannot equip on an assassin)<br />88 : Blue Buff (+20 Starting Mana. Gain 20 mana after casting an ability)<br />89 : Mage Emblem (Wearer becomes a mage, cannot equip on a mage)<br />99 : Tacticians Crown (Increase board unit size by 1)<br />
    fn give_item_effect(
        &mut self,
        item: u8,
        friendly_champions: &mut VecDeque<SummonedChampion>,
        enemy_champions: &mut VecDeque<SummonedChampion>,
        items : &[Item],
    ) {
        info!("giving item {}", item);
        if item == 0 { return } //no actual item
        let item_obj = items[(item as usize) - 1]; //fetch item object
        {
            self.health += item_obj.health;
            self.ad += item_obj.ad;
            self.ap += item_obj.ap;
            self.ar += item_obj.ar;
            self.mr += item_obj.mr;
            self.attack_speed_modifier *= item_obj.attack_speed_modifier;
            self.ra += item_obj.ra;
            self.cr += item_obj.cr;
            self.dc += item_obj.dc;
            self.cm += item_obj.cm;
            self.omnivamp += item_obj.omnivamp;
            self.crit_damage += item_obj.crit_damage;
            //give plain stat buffs
        }

        //give item specific effects
        match item {
            11 => {self.ad += item_obj.ad * (self.star_level as f32)}, //increasing ad with star level
            13 => {
                for friendly_champion in friendly_champions
                    .iter_mut()
                    .filter(self.location.get_within_distance(3))
                {
                    if friendly_champion.location.y == self.location.y {
                        friendly_champion.attack_speed_modifier *= 1.3;
                    }
                } //attack speed buff to all allied champs adjacent on same row
            }
            14 => {
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::EdgeOfNight(),
                    ..Default::default()
                })
            } //gives edge of night buff
            15 => {
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::Bloodthirster(),
                    ..Default::default()
                }); //gives bloodthirster buff
            }
            24 => {
                let shield_amount = [300.0, 350.0, 400.0][self.star_level];
                self.shields.push(Shield {
                    duration: 1500,
                    size: shield_amount,
                    ..Default::default()
                });
                for friendly_champion in friendly_champions
                    .iter_mut()
                    .filter(self.location.get_within_distance(3))
                {
                    if friendly_champion.location.y == self.location.y {
                        friendly_champion.shields.push(Shield {
                            duration: 1500,
                            size: shield_amount,
                            ..Default::default()
                        });
                    }
                }//gives shield to self and all adjacent allies on same row
            }
            28 => {
                self.se.push(StatusEffect {
                    duration: Some(500),
                    status_type: StatusType::ArchangelStaff(0.2),
                    ..Default::default()
                })//gives archangel buf
            }
            34 => {
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::GiveSunfire(),
                    ..Default::default()
                }) //gives sunfire buff
            } //(!U)
            35 => {
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::Zephyr(500),
                    ..Default::default()
                })
            } //gives zephyr effect
            36 => {
                for enemy_champion in enemy_champions
                    .iter_mut()
                    .filter(self.location.get_within_distance(9))
                {
                    enemy_champion.se.push(StatusEffect {
                        duration: Some(0),
                        status_type: StatusType::Taunted(self.id),
                        is_negative: true,
                        ..Default::default()
                    }) //taunts all enemies within range
                }
            }
            37 => {
                self.shields.push(Shield {
                    duration: 1500,
                    size: 600.0,
                    blocks_type: Some(DamageType::Magical()),
                    pop: true,
                }); //gives self banshee shield
                for friendly_champion in friendly_champions
                    .iter_mut()
                    .filter(self.location.get_within_distance(3))
                {
                    if friendly_champion.location.y == self.location.y
                    //gives banshee's shield
                    {
                        friendly_champion.shields.push(Shield {
                            duration: 1500,
                            size: 600.0,
                            blocks_type: Some(DamageType::Magical()),
                            pop: true,
                        });
                    }
                }
            }
            38 => {
                self.se.push(StatusEffect {
                    duration: Some(100),
                    status_type: StatusType::RedemptionGive(),
                    ..Default::default()
                })
            } //Gives redemption effect
            45 => {
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::Gargoyles(0.0),
                    ..Default::default()
                }) //gives gargoyles effect
            }
            46 => {
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::TitansResolve(0),
                    ..Default::default()
                }) //gives titans resolve effect
            }
            47 => {
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::ShroudOfStillness(),
                    ..Default::default()
                }) //gives shroud of stillness effect
            }
            48 => {
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::ProtectorsVow(),
                    ..Default::default()
                }) //gives protectors vow effect
            }
            55 => {
                self.se.push(StatusEffect {
                    duration: Some(200),
                    status_type: StatusType::DragonClawHeal(),
                    ..Default::default()
                }) //gives dragon claw heal
            }
            57 => {
                self.se.push(StatusEffect {
                    duration: Some(15000),
                    status_type: StatusType::CrowdControlImmune(),
                    ..Default::default()
                }); //gives cc immunity for 15 seconds
            }
            58 => {
                for friendly_champion in friendly_champions
                    .iter_mut()
                    .filter(self.location.get_within_distance(3))
                {
                    if friendly_champion.location.y == self.location.y
                    {
                        friendly_champion.ap += 0.3;
                    }
                } //gives ap to friendly champs adjacent in same row
            }
            78 => {
                //50% chance to give bonus ad ap, 50% chance to give bonus omnivamp
                if rand::thread_rng().gen_range(0..100) > 50
                {
                    self.ad += 30.0;
                    self.ap += 0.3;
                    self.omnivamp += 0.15;

                } else {
                    self.ad += 15.0;
                    self.ap += 0.15;
                    self.omnivamp += 0.3;
                }
            }
            _ => (),
        }
    }
    ///checks if id equal
    pub fn equal_id(&self, id: usize) -> bool {
        self.id == id //checks if id equal
    }
    ///checks if shed == 2
    pub fn is_shed(&self) -> bool {
        self.shed == 2 //checks if shed
    }
    ///updates shed
    fn update_shed(&mut self) {
        //if shed is 1, set to 2, else set to 0
        if self.shed == 1 {
            self.shed = 2;
        } else {
            self.shed = 0;
        }
    }
    ///performs a status effect, returning true if status effect should be kept or false if removed
    fn perform_status(
        &mut self,
        status_effect: &mut StatusEffect,
        friendly_champions: &mut VecDeque<SummonedChampion>,
        enemy_champions: &mut VecDeque<SummonedChampion>,
        time_unit: i8,
        stun: &mut Stun,
    ) -> bool {
        info!("Performing status");
        //if status has duration
        if status_effect.duration.is_some() {
            let mut n_duration = status_effect
                .duration
                .unwrap()
                .checked_sub(time_unit.into())
                .unwrap_or(0); //unwrap duration and do checked subtraction
            if self.is_shed() && status_effect.is_negative {
                n_duration = 0;
            } //if shed and negative set duration to 0

            if n_duration <= 0 {
                match status_effect.status_type {
                    //undo status effect/ remove effect. some effects aren't actually removed but just reinitialise
                    StatusType::AttackSpeedBuff(modifier) => self.attack_speed_modifier /= modifier,
                    StatusType::IncreaseDamageTaken(modifier) => {
                        self.incoming_damage_modifier /= modifier
                    }
                    StatusType::Untargetable() => {
                        self.targetable = true //(!D) if have 2 untargetable effects this will untarget too early
                    }
                    StatusType::MorellonomiconBurn(dmg_per_tick, dmg_to_do, time_next_tick) => {
                        if self.is_shed() {
                            return false; //remove if shed
                        }

                        if dmg_per_tick > dmg_to_do {
                            self.health -= dmg_to_do; //deal remainiong damage
                        } else {
                            n_duration = time_next_tick; //set duration to next tick time
                            self.health -= dmg_per_tick;
                            status_effect.status_type = StatusType::MorellonomiconBurn(
                                dmg_per_tick,
                                dmg_to_do - dmg_per_tick,
                                time_next_tick,
                            ); //update status
                        }
                    }
                    StatusType::IonicSparkEffect() => {
                        self.mr *= 2.0; //(!D) Possible discrepency
                        self.zap = false
                    }
                    StatusType::ArchangelStaff(ap_amount) => {
                        n_duration = 500;
                        self.ap += ap_amount;
                    }
                    StatusType::Banished() => self.banish = false,
                    StatusType::RedemptionGive() => {
                        n_duration = 100; //increase duration
                        for champ in friendly_champions
                            .iter_mut()
                            .filter(self.location.get_within_distance(3))
                        {
                            champ.heal((champ.initial_hp - champ.health) * 0.12)
                            //discrepency check at multitarget damage time for redemption heal for reduction
                        }
                        self.heal((self.initial_hp - self.health) * 0.12); //gives healing to self and all adjacent
                    }
                    StatusType::Gargoyles(old_num_targeting) => {
                        n_duration = 100; //increase duration
                        let num_targeting: f32 = self.get_num_targeting(enemy_champions) as f32;
                        let difference = num_targeting - old_num_targeting; //get change
                        self.ar += 0.18 * difference;
                        self.mr += 0.18 * difference;
                        status_effect.status_type = StatusType::Gargoyles(num_targeting);
                    }
                    StatusType::ShroudOfStillness() => {
                        for champ in enemy_champions
                            .iter_mut()
                            .filter(|x| x.location.x == self.location.x)
                        {
                            champ.cm -= (7 * champ.mc) / 20;
                        } //mana debuff to all in row
                    }
                    StatusType::DragonClawHeal() => {
                        n_duration = 200; //reset status effect

                        let num_targeting: f32 = self.get_num_targeting(enemy_champions) as f32;
                        self.heal(self.initial_hp * 0.012 * num_targeting);
                    }
                    StatusType::LastWhisperShred() => {
                        self.ar *= 2.0
                    }
                    StatusType::GiveSunfire() => {
                        //(!U)
                        n_duration = 300;
                        for champ in enemy_champions
                            .iter_mut()
                            .filter(self.location.get_within_distance(3))
                        {
                            let dmg = champ.initial_hp / 20.0;
                            champ.se.push(StatusEffect {
                                duration: Some(100),
                                status_type: StatusType::MorellonomiconBurn(dmg, dmg / 3.0, 100),
                                ..Default::default()
                            })
                        }
                    }
                    StatusType::EdgeOfNight() => {
                        //if health below half
                        if self.health <= (self.initial_hp / 2.0) {
                            self.se.push(StatusEffect {
                                duration: Some(50),
                                status_type: StatusType::Untargetable(),
                                ..Default::default()
                            }); //optimisation at every ..Default::default() with instead isNegative : false
                            self.se.push(StatusEffect {
                                duration: None,
                                status_type: StatusType::AttackSpeedBuff(1.3),
                                ..Default::default()
                            }); //(!D) technically attack speed buff comes into play after untargetable wears off
                            self.shed = 1; //push effect and shred
                        } else {
                            return true; //do not remove
                        }
                    }
                    StatusType::Bloodthirster() => {
                        //if health below 40%
                        if self.health <= (0.4 * self.initial_hp) {
                            self.shields.push(Shield {
                                duration: 500,
                                size: self.initial_hp / 4.0,
                                ..Default::default()
                            }); //push shield
                        } else {
                            return true; //do not remove
                        }
                    }
                    StatusType::Zephyr(banish_duration) => {
                        let opposite_location = Location {
                            x: self.location.y,
                            y: self.location.x,
                        }; //gets opposite location
                        opposite_location
                            .get_closest_to_location(enemy_champions)
                            .unwrap()
                            .se
                            .push(StatusEffect {
                                duration: Some(banish_duration),
                                status_type: StatusType::Banished(),
                                ..Default::default()
                            }); //banishes champ closest to that location
                    }
                    StatusType::Taunted(taunt_id) => {
                        //if taunter still alive
                        if find_champion_index_from_id(enemy_champions, taunt_id).is_some() {
                            self.target = taunt_id;
                            self.target_cooldown = 100;
                            n_duration = 20; //set target to taunter
                        }
                    }
                    StatusType::TitansResolve(old_stack_num) => {
                        if old_stack_num != 25 { //if old stacks not maxed out
                            let difference: f32 =
                                (self.titans_resolve_stacks - old_stack_num).into();
                            self.ad += 2.0 * difference; //gives bonus from difference
                            self.ap += 0.02 * difference;
                            if self.titans_resolve_stacks == 25 {
                                self.ar += 0.25;
                                self.mr += 0.25;
                            } //give bonus if maxed out
                            status_effect.status_type =
                                StatusType::TitansResolve(self.titans_resolve_stacks); //update to new stack count
                        }
                        return true;
                    }
                    StatusType::ProtectorsVow() => {
                        //if health below half
                        if self.health <= (self.initial_hp / 2.0) {
                            self.mr += 0.25; //give protectors vow effect
                            self.ar += 0.25; 
                            self.shields.push(Shield {
                                duration: 500,
                                size: self.initial_hp / 4.0,
                                ..Default::default()
                            })
                        } else {
                            return true;
                        }
                    }
                    _ => (),
                }
                //if duration updated
                if n_duration > 0 {
                    status_effect.duration = Some(n_duration); //do not remove
                } else {
                    return false; //remove
                }
            }
        }

        //if status is not applied yet
        if !status_effect.applied {
            status_effect.applied = true; //is now
            //match status type and give associated effect
            match status_effect.status_type {
                StatusType::AttackSpeedBuff(modifier) => {
                    self.attack_speed_modifier *= modifier;
                }
                StatusType::Stun() => {
                    status_effect.applied = false;
                    if stun.stun == 0 {
                        stun.stun = 1;
                    } //has to check stun.stun == 0 as if stun.stun == 2 it is immune
                }
                StatusType::IncreaseDamageTaken(modifier) => {
                    self.incoming_damage_modifier *= modifier;
                }
                StatusType::Assassin() => {
                    if self.location.y >= 4 {
                        self.location.y = 0;
                    } else {
                        self.location.y = 7; 
                    }

                    return false;
                }
                StatusType::Untargetable() => self.targetable = false,
                StatusType::IonicSparkEffect() => {
                    self.mr /= 2.0;
                    self.zap = true
                }
                StatusType::Banished() => self.banish = true,
                StatusType::LastWhisperShred() => {
                    self.ar /= 2.0;
                }
                StatusType::CrowdControlImmune() => {
                    status_effect.applied = false;
                    stun.stun = 2;
                }
                _ => (),
            }
        }
        true
    }
    ///sets id
    pub fn _set_id(&mut self, id: usize) {
        self.id = id //sets id
    }
}

impl Default for SummonedChampion {
    //default values for summoned champion
    fn default() -> Self {
        SummonedChampion {
            location: Location {
                ..Default::default()
            },
            of_champ_id: 0,
            movement_progress: [0, 0],
            health: 0.0,
            cm: 0,
            dc: 0,
            cr: 0,
            crit_damage: 0.0,
            mc: 0,
            ar: 0.0,
            mr: 0.0,
            ad: 0.0,
            attack_speed: 0.0,
            ra: 0,
            id: 0,
            target_cooldown: 0,
            auto_attack_delay: 0,
            attack_speed_modifier: 0.0,
            target: 0,
            target_cells: Location { x: 0, y: 0 },
            items: [0, 0, 0],
            ap: 0.0,
            se: Vec::new(),
            gain_mana_delay: 0,
            star_level: 0,
            incoming_damage_modifier: 0.0,
            initial_hp: 0.0,
            targetable: false,
            shed: 0,
            shields: Vec::new(),
            zap: false,
            banish: false,
            titans_resolve_stacks: 0,
            omnivamp: 0.0,
            shiv_attack_count: 0,
        }
    }
}

impl fmt::Display for SummonedChampion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.health)
    }
}
