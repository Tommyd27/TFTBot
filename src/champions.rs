use crate::location::Location;
use crate::projectiles::Projectile;
use crate::shields::Shield;
use crate::status_effects::{StatusEffect, StatusType, Stun};
use crate::utils::{find_champion_index_from_id, find_champion_index_from_id_targetable, sign};
use core::fmt;
use rand::Rng;
use std::collections::VecDeque;
use std::mem::take;
///Stores basic information surrounding a champion
pub struct Champion {
    ///index in champions array
    _id: u8,

    ///healthpoints (star level dependent)
    hp: [f32; 3],

    ///starting mana
    sm: u16,

    ///ability mana cost
    mc: u16,

    ///base armor value
    ar: f32,

    ///Base Magic Resist Value
    mr: f32,

    ///attack damage (star level dependent)
    ad: [f32; 3],

    ///attack speed (attacks per second)
    attack_speed: f32,

    ///attack range
    ra: i8,

    ///ability id: index in abilities array
    a_id: usize,
}

///CHAMPIONS (const):<br />
///Stores all the champion information
pub const CHAMPIONS: [Champion; 4] = [
    Champion {
        _id: 0,
        hp: [650.0, 1100.0, 2100.0],
        sm: 70,
        mc: 140,
        ar: 0.25,
        mr: 0.25,
        ad: [40.0, 70.0, 130.0],
        attack_speed: 0.6,
        ra: 2,
        a_id: 0,
    }, //Support
    Champion {
        _id: 1,
        hp: [800.0, 1400.0, 2500.0],
        sm: 50,
        mc: 100,
        ar: 0.45,
        mr: 0.45,
        ad: [75.0, 100.0, 175.0],
        attack_speed: 0.7,
        ra: 1,
        a_id: 1,
    }, //Bruiser
    Champion {
        _id: 2,
        hp: [700.0, 1200.0, 2200.0],
        sm: 35,
        mc: 100,
        ar: 0.25,
        mr: 0.25,
        ad: [65.0, 120.0, 240.0],
        attack_speed: 0.7,
        ra: 3,
        a_id: 2,
    }, //AD Ranged
    Champion {
        _id: 2,
        hp: [700.0, 1200.0, 2200.0],
        sm: 35,
        mc: 150,
        ar: 0.25,
        mr: 0.25,
        ad: [50.0, 60.0, 70.0],
        attack_speed: 0.6,
        ra: 3,
        a_id: 3,
    },
]; //AP Ranged

///Enum for the 3 damage types Physical, Magical and True
#[derive(PartialEq, Eq, Clone, Copy)] //derives clone copy and partial equal
pub enum DamageType {
    Physical(),
    Magical(),

    #[allow(dead_code)]
    True(),
}

///PlacedChampion (struct):
///Stores information about a champion's location and status on a board (as well as ID of actual champion)
///Not used in battles, only for planning phase
pub struct PlacedChampion {
    ///id given at instantiation
    id: usize,

    ///star level of champion
    star: usize,

    ///items
    items: [u8; 3],

    ///location on board
    location: Location,
}

impl PlacedChampion {
    pub fn new(id: usize, star: usize, items: [u8; 3], location: Location) -> PlacedChampion {
        PlacedChampion {
            id,
            star,
            items,
            location,
        }
    }
}
///Struct for champion placed on board in a battle
pub struct SummonedChampion {
    ///array of p, q coordinates, r can be calculated with r = -p - q
    pub location: Location,

    ///progress of movement before new square, goes up to 10 then moves
    movement_progress: [i8; 2],

    ///health
    health: f32,
    ///current mana
    cm: u16,

    ///dodge chance in %
    dc: u8,
    ///crit rate in %
    cr: u8,
    ///crit damage
    crit_damage: f32,

    ///ability mana cost
    mc: u16,

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

    ///ability ID/ index
    a_id: usize,

    ///id
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

    /*///trait abilities
    traits : Vec<u8>,*/
    ///whether zapped from ionic spark
    zap: bool,

    ///whether zenith banished
    banish: bool,

    ///titan's resolve stacks
    titans_resolve_stacks: u8,

    ///omnivamp (% of healing from damage done)
    omnivamp: f32,

    is_setup: bool,
}

impl SummonedChampion {
    ///converts PlacedChampion into SummonChampion
    pub fn new(placed_champion: &PlacedChampion, id: usize) -> SummonedChampion {
        let star_level = placed_champion.star; //get star level
        let of_champ = &CHAMPIONS[placed_champion.id]; //get champ info
        SummonedChampion {
            location: placed_champion.location, //create summoned champ with all details
            movement_progress: [0, 0],
            health: of_champ.hp[star_level],
            initial_hp: 0.0,
            cm: of_champ.sm, //update current mana to starting mana
            dc: 0,
            cr: 25,
            crit_damage: 0.3,
            mc: of_champ.mc,
            ar: of_champ.ar,
            mr: of_champ.mr,
            ad: of_champ.ad[star_level],
            attack_speed: of_champ.attack_speed,
            ra: of_champ.ra * 2, //because distanceBetweenPoints returns value twice as large
            id,
            target_cooldown: 0,
            auto_attack_delay: 0,
            attack_speed_modifier: 1.0,
            target: 255,
            target_cells: Location { x: -1, y: -1 }, //(!O)
            a_id: of_champ.a_id,
            items: placed_champion.items,
            ap: 1.0,
            se: Vec::new(),
            gain_mana_delay: 0,
            star_level,
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
            is_setup: false,
        }
    }

    pub fn setup(
        &mut self,
        friendly_champions: &mut VecDeque<SummonedChampion>,
        enemy_champions: &mut VecDeque<SummonedChampion>,
    ) {
        if self.is_setup {
            return;
        }

        if self.items[0] == 77 {
            //(!D) doesnt give accurate item pairs
            let level = true; //implement getting level
            if level {
                self.items[1] =
                    rand::thread_rng().gen_range(0..9) * 10 + rand::thread_rng().gen_range(0..9);
                self.items[2] = rand::thread_rng().gen_range(0..9); //discrepency do this properly later
            } else {
                self.items[1] =
                    rand::thread_rng().gen_range(0..9) * 10 + rand::thread_rng().gen_range(0..9);
                self.items[2] =
                    rand::thread_rng().gen_range(0..9) * 10 + rand::thread_rng().gen_range(0..9);
            }
        }
        for item in self.items {
            self.give_item_effect(item, friendly_champions, enemy_champions)
        }
        self.initial_hp = self.health
    }
    ///fn to heal set amount
    fn heal(&mut self, mut healing_amount: f32) {
        if self.se.contains(&StatusEffect {
            status_type: StatusType::GreviousWounds(),
            ..Default::default()
        }) {
            healing_amount /= 2.0;
        }

        self.health = self.initial_hp.min(self.health + healing_amount);
    }
    ///simulates a tick/ turn for a champion<br />
    ///friendlyChampions[selfIndex] : this champion<br />
    ///friendlyChampionsLocations : location of all friend champs (array of positions), for pathfinding<br />
    ///enemyChampions : all enemy champions, for targetting<br />
    ///timeUnit : time unit of a frame, in centiseconds<br />
    ///movementAmount : precalculated movement distance for 1 frame<br />
    fn turn_to_void_spawn(&mut self) {
        println!("Unimplemented")
    }
    pub fn take_turn(
        &mut self,
        friendly_champions: &mut VecDeque<SummonedChampion>,
        enemy_champions: &mut VecDeque<SummonedChampion>,
        time_unit: i8,
        movement_amount: i8,
        projectiles: &mut Vec<Projectile>,
    ) -> bool {
        if self.health <= 0.0 {
            return false;
        }

        self.target_cooldown -= time_unit; //Reduce cooldown to check target/ find new target
        self.auto_attack_delay -= time_unit as i16; //Risks going out of bounds as auto attack value may not be called for some time
        self.gain_mana_delay -= time_unit as i16;

        if self.banish {
            return true;
        }

        {
            let mut status_effects = take(&mut self.se);
            let mut stun = Stun { stun: 0 };
            status_effects.retain_mut(|x| {
                self.perform_status(x, friendly_champions, enemy_champions, time_unit, &mut stun)
            });

            if self.health <= 0.0 {
                return false;
            }

            self.se.extend(status_effects);

            self.update_shred();

            self.shields.retain_mut(|x| x.update_shield(time_unit));

            if stun.stun == 1 {
                return true;
            }
        }

        //does auto attack delay need to reset on pathing? does attack instantly after reaching path/ in range

        {
            //targetObject/ pathfinding block
            let mut need_new_target_cell: bool = false; //Bool to store whether new path is needed

            let mut target_object: Option<SummonedChampion> = None;

            if self.target_cooldown >= 0 {
                //if already has target and doesnt want to change targets
                if let Some(index) = find_champion_index_from_id_targetable(enemy_champions, self.target) {
                    target_object = enemy_champions.swap_remove_back(index);
                }
            }

            if target_object.is_none() {
                //index not updating from initial intilialisation of 99, therefore need new target
                println!("Debug : Looking for Target");
                self.target_cooldown = 100; //reset target cooldown

                need_new_target_cell = true; //tells us to recalculate pathfinding later
                                          //discrepency what if target has moved regardless
                let mut index: Option<usize> = None;
                if let Some((i, champ)) = self
                    .location
                    .get_closest_to_location_targetable_index(enemy_champions) {
                    if champ.get_is_targetable() {
                        index = Some(i)
                    }
                    
                }
                if index.is_none() {
                    return true;
                }

                target_object = enemy_champions.swap_remove_back(index.unwrap());
            }

            let mut target_object: SummonedChampion = target_object.unwrap();
            self.target = target_object.id;
            let distance_to_target = self.location.distance_between_points(&target_object.location);

            if distance_to_target <= self.ra {
                //if target in range
                println!("Debug : Target in Range");
                println!(
                    "Debug : Auto Attack Delay Remaining {0}",
                    self.auto_attack_delay
                ); //discrepency, does auto attack "charge" while moving
                if self.auto_attack_delay <= 0
                //if autoattack ready
                {
                    println!("Debug : Delay Smaller than 0 - Attacking");
                    /*
                    self.aS = attacks per 1 second
                    self.autoAttackDelay = time in 1/10 of second until next attack
                    self.attackSpeedIncrease = percentage increase in attack speed


                    autoAttackDelay (seconds) = 1 / (attackSpeed * attackSpeedMod)
                    autoAttackDelay (centiseconds) = 100 / (attackSpeed * attackSpeedMod)

                    */
                    println!(
                        "as: {}, mod: {}",
                        self.attack_speed, self.attack_speed_modifier
                    );
                    self.auto_attack_delay =
                        ((100.0 / (self.attack_speed * self.attack_speed_modifier)) as i16).max(20); //calculating auto attack delay
                    println!("Auto attack delay set");
                    if self.items.contains(&26) {
                        self.attack_speed_modifier *= 1.06
                    } //(!D) if attack speed doesnt increase when attack misses/ is dodged

                    //attack speed unclear, capped at five yet some champions let you boost beyond it?
                    //optimisation definitely here
                    if self.gain_mana_delay <= 0 {
                        self.cm += 10;
                        if self.items.contains(&18) {
                            self.cm += 8;
                        }
                        println!("gain mana");
                    }
                    if self.items.contains(&68) {
                        //(!O) go through foreach in items and match statement
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
                        let mut count = 0;

                        for enemy_champ in enemy_champions.iter_mut() {
                            count += 1;

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

                            if count >= 3 {
                                break;
                            }
                        }
                    }

                    if self.items.contains(&56) {
                        //(!D) can be dodged
                        let closest_other_enemy =
                            self.location.get_closest_to_location_targetable(enemy_champions);
                        if let Some(target) = closest_other_enemy {
                            self.deal_damage(
                                friendly_champions,
                                target,
                                self.ad * 0.7,
                                DamageType::Physical(),
                                false,
                            ) //discrepency runaans can miss
                        }
                    }
                    println!("maybe dodge");
                    if target_object.dc == 0
                        || target_object.dc < rand::thread_rng().gen_range(0..100)
                        || self.items.contains(&66)
                    //calculating whether to dodge
                    {
                        //(!O) from not generating random gen
                        println!("No Dodge");
                        self.deal_damage(
                            friendly_champions,
                            &mut target_object,
                            self.ad,
                            DamageType::Physical(),
                            false,
                        );

                        println!("Debug : Enemy Champion Health is {0}", target_object.health);
                        if target_object.health <= 0.0
                        //if enemy champion dead
                        {
                            println!("Debug : Health Lower than 0 - Removing");

                            if target_object.items.contains(&36) {
                                target_object.turn_to_void_spawn();
                                //(!D) cant be asked to set everything to default)
                                //(!D) stats change depending on stage
                            }
                            //(!D), only checks for champion death when it is auto attacked
                        }
                    } else {
                        println!("Debug : Dodged Attack");
                    }
                }
                enemy_champions.push_back(target_object);
            } else {
                println!("Debug : Not in Range");
                if need_new_target_cell || self.location == self.target_cells {
                    //if need to update pathfinding or at pathfinding target
                    //optimisation?, accuracy vs performance cost
                    println!("Debug : Need Target Cell");
                    self.target_cells = self.location; //setting target cells to location so if it does not find a target this frame will try to do it again
                                                       //optimisation does not need to check every frame

                    let mut lowest_distance: i8 = i8::MAX; //setting lowestDistance to high value
                    let mut new_position;
                    for possible_move in [[0, -1], [1, -1], [1, 0], [-1, 0], [-1, 1], [0, 1]]
                    //for every possible move
                    //optimisation
                    {
                        new_position = Location::add_position_vec(&self.location, possible_move);
                        let distance_to_target =
                            target_object.location.distance_between_points(&new_position);
                        if distance_to_target < lowest_distance {
                            if (!new_position.check_valid())
                                || friendly_champions.iter().any(|f| f.location == new_position)
                            {
                                continue;
                            }
                            println!("Debug : Found a Target Cell");
                            lowest_distance = distance_to_target;
                            self.target_cells = new_position;
                        }
                    }
                }

                println!("Debug : Moving to Target Cell");
                self.movement_progress[0] +=
                    movement_amount * sign(self.target_cells.x - self.location.x); //optimisation here
                println!(
                    "Debug : Position ({0:?}) -- Movement Progress ({1:?})",
                    self.location, self.movement_progress
                );
                if self.movement_progress[0].abs() == 10 {
                    self.location.x += sign(self.movement_progress[0]);
                    self.movement_progress[0] = 0;
                }
                self.movement_progress[1] +=
                    movement_amount * sign(self.target_cells.y - self.location.y);
                if self.movement_progress[1].abs() == 10 {
                    self.location.y += sign(self.movement_progress[1]);
                    self.movement_progress[1] = 0;
                }

                enemy_champions.push_back(target_object);
            }
        }
        //Ionic spark, optimisation, could be status effect but enemies not passed into function? also doesnt need to be check every turn
        if self.items.contains(&25) {
            for champ in enemy_champions
                .iter_mut()
                .filter(self.location.get_within_distance(7))
            {
                champ.se.push(StatusEffect {
                    duration: Some((time_unit + 1).into()),
                    status_type: StatusType::IonicSparkEffect(),
                    is_negative: true,
                    ..Default::default()
                })
            }
        }

        if self.cm >= self.mc {
            if self.zap {
                self.health -= (self.mc as f32) * 2.5;
            }
            self.cm = 0;
            if self.items.contains(&88) {
                self.cm = 20;
            }
            self.gain_mana_delay = 100;
            self.cast_ability(friendly_champions, enemy_champions, projectiles);
        }
        true
    }
    pub fn deal_damage(
        &mut self,
        friendly_champions: &mut VecDeque<SummonedChampion>,
        target: &mut SummonedChampion,
        damage_amount: f32,
        damage_type: DamageType,
        _is_splash: bool,
    ) {
        let mut damage: f32 = damage_amount * target.incoming_damage_modifier;
        let can_crit;
        let mut crit_damage = self.crit_damage;

        match damage_type {
            DamageType::Physical() => {
                can_crit = true;
                damage /= 1.0 + target.ar;
                if self.items.contains(&67) {
                    //apply armor shred from last whisper
                    if !target.se.contains(&StatusEffect {
                        status_type: StatusType::LastWhisperShred(),
                        ..Default::default()
                    }) {
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
                damage /= 1.0 + target.mr;
            }
            DamageType::True() => {
                can_crit = self.items.contains(&27);
            }
        }

        if can_crit && self.cr > rand::thread_rng().gen_range(0..100) {
            let mut additional_crit_damage = damage * crit_damage;
            if target.items.contains(&44) {
                //reduce dmg if target has bramble vest
                additional_crit_damage /= 4.0;
            }
            damage += additional_crit_damage
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

                if let Some(lowest_hp_champ) = friendly_champions
                        .iter_mut()
                        .reduce(|x, y| if x.health < y.health { x } else { y }) {//get lowest HP ally 
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
                    status_type: StatusType::MorellonomiconBurn(damage_to_do / 10.0, damage_to_do, 100),
                    is_negative: true,
                    ..Default::default()
                }) //discrepency unsure whether burn just reapplies itself
            }
        }

        self.heal(damage * self.omnivamp); //give omnivamp healing

        for shield in &mut target.shields {
            //reduce damage due to shields
            damage = shield.handle_damage(damage, damage_type);
            if damage <= 0.0 {
                break;
            }
        }

        self.titans_resolve_stacks = 25.min(self.titans_resolve_stacks + 1); //add titan's resolve stacks
        target.titans_resolve_stacks = 25.min(target.titans_resolve_stacks + 1); //give enemy titan's resolve stacks

        target.health -= damage;

        if target.gain_mana_delay <= 0 {
            // give mana is delay off
            target.cm += (0.7 * damage) as u16; //(!D) should be 1% of premitigation and 7% of post.
        }
    }
    fn cast_ability(
        &mut self,
        friendly_champions: &mut VecDeque<SummonedChampion>,
        enemy_champions: &mut VecDeque<SummonedChampion>,
        projectiles: &mut Vec<Projectile>,
    ) {
        match self.a_id {
            0 => {
                //let mut playerDistances : Vec<[i8 ; 2]> = Vec::new(); //instantiates empty vec to hold distance to friendly and enemy champions

                let mut player_distances: Vec<(i8, &mut SummonedChampion, bool)> = friendly_champions
                    .iter_mut()
                    .map(|x| (self.location.distance_between_points(&x.location), x, true))
                    .collect();
                player_distances.extend(
                    enemy_champions
                        .iter_mut()
                        .map(|x| (self.location.distance_between_points(&x.location), x, false)),
                );
                let star_level = self.star_level; //gets current star level

                player_distances.sort_unstable_by_key(|a| a.0); //sorts the player distances
                let number_affected: usize = [3, 4, 5][star_level]; //how many champions it can hit/ effect
                let mut i = 0; //(!O) counts how many have been given effect
                let ap = self.ap; //get ability power
                for (_, champ, on_team) in player_distances
                //(!O) just fetch the champion index, distance is irrelevant as already sorted
                {
                    if i >= number_affected {
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
            1 => {
                let star_level = self.star_level;
                let target_index = find_champion_index_from_id(enemy_champions, self.target).unwrap_or(0); //(!D) Can strike from out of range, should search for closest
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
                let target = find_champion_index_from_id(enemy_champions, self.target).unwrap_or(0); //(!D) Can strike from out of range
                let target_location = enemy_champions[target].location;
                let damage: f32 = self.ad * 3.0 * (self.star_level as f32);
                projectiles.push(Projectile::new(
                    self.location,
                    Option::Some(target_location),
                    self.target,
                    damage,
                    DamageType::Physical(),
                    0.0,
                    5,
                    self.id,
                ))
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
    pub fn get_num_targeting(&self, enemy_champions: &VecDeque<SummonedChampion>) -> usize {
        enemy_champions
            .iter()
            .filter(|p| p.target == self.id)
            .count()
    }
    pub fn get_is_targetable(&self) -> bool {
        self.targetable && !self.banish
    }
    ///GiveItemEffect : (func)<br />
    ///Gives an item effect to a champion<br />
    ///**Item IDS:**<br />
    ///0 : Null<br />1  : B.F Sword (+10 Attack Damage)<br />2  : Needlessly Large Rod (+10 Ability Power)<br />3  : Giants Belt (+150 health)<br />4  : Chain Vest (+20 Armor)<br />5  : Negatron Cloak (+20 Magic Resist)<br />6  : Recurve Bow (+10% Attack Speed)<br />7  : *Sparring Gloves* (+5% Crit Chance, +10% Dodge Chance)<br />8  : Tear of the Goddess (+15 Mana)<br />9  : Spatula<br />11 : Deathblade (+40, +70, +100 Attack Damage - Star Level Dependent)<br /> 12 : *Hextech Gunblade* (Dealing Magic and True Damage heals the owner and lowest health ally for 25% of the damage)<br />13 : Zekes Herald (Grants 30% bonus attack speed to the holder and 2 adjacent allies in same row)<br />14 : Edge of Night (At 50% health - once per combat - the holder briefly becomes untargetable and sheds negative effects. Then they gain 30% attack speed)<br />15 : Bloodthirster (Damage dealt heals holder for 25%. Once per combat at 40% Health, gain a 25% maximum health shield for up to 5 seconds)<br />16 : Giant Slayer (Abilities and attacks deal 25% more damage, increased to 50% if the holder has over 2200 maximum health)<br />17 : Infinity Edge (+10 Attack Damage, +75% Crit Chance, +10% Crit Damage, Converts every 1% excess critical strike chance into 1% bonus critical strike damage)<br />18 : Spear of Shojin (✓) (Basic attacks restore an additional 8 mana on-attack)<br />19 : Shimmerscale Emblem (Wearer becomes a shimmerscale, cannot equip on a shimmersclae)<br />22 : Rabadons Deathcap (+75 Ability Power)<br />23 : Morellonomicon (+30 Ability Power, magic or true damage from an ability burns the holders target, dealing 25% of the targets maximum health as trude damage over 10 seconds and applying grevious wounds for the duration)<br />24 : Locket of the Iron Solari (At the start of combat, the wearer and all allies within 2 hexes in the same row gain a 300 / 350 / 400 health shield for 15 seconds - star level dependent)<br />25 : Ionic Spark (Enemies within 3 hexes have their magic resistance reduced by 50% (does not stack). When enemies within 3 hexes cast their ability, they are dealt 250% of their maximum mana as magic damage)<br />26 : Guinsoos Rageblade (Basic attacks grant 6% bonus attack speed for the rest of combat, stacks with no upper limit)<br />27 : *Jeweled Gauntlet* (+15% Crit Chance, +40% Crit Damage, +10 Ability Power, The holders magic adn true damage from abilities can critically strike)<br />28 : Archangels Staff (Grants the wearer 20 ability power every 5 seconds)<br />29 : Dragonmancer Emblem (Wearer becomes an dragonmancer, cannot equip on an dragonmancer)<br />33 : Warmogs Armor (+1000 Health)<br />34 : Sunfire Cape (+400 Health. At the start of combat and every 2 seconds thereafter, applies a 10% maximum health burn as true damage over 10 seconds and applying grevious wounds for the duration)<br />35 : Zephyr (At the start of combat, banishes for 5 seconds the unit that mirrors the wielders placement on the other side of the board. Pierces through CC immunity effects)<br />36 : ZZ Rot Portal (At the start of combat, the wearer taunts enemies within 4 hexes. When the wearer dies, a Voidspawn arises, taunting nearby enemies. Summoned units can spawn Voidspawns at 25% effectiveness)<br />37 : *Banshees Claw* (+15% Dodge Chance, +150 Health, At the beginning of each round, the holder and allies within 1 hex in the same row gain a shield that blocks the first enemy ability, up to 600 damage)<br />38 : Redemption (Every 5 seconds, the wearer radiates an aura to allies within 1 hex, healing them for 12% missing health. Affected allies take 25% reduced damage from AOE attacks for  seconds)<br />39 : Guardian Emblem (Wearer becomes a guardian, cannot equip on a guardian)<br />44 : Bramble Vest (+60 Armor. Negatves 75% bonus damage from critical hits. On being hit by an attack, deal 75 / 100 / 150 magic damage to all nearby enemies (once every 2.5 seconds))<br />45 : Gargoyle Stoneplate (+18 Armor and Magic Resist for each enemy targeting the holder)<br />46 : *Titans Resolve* (Gain 2 attack damage and ability power when attacking or taking damage. After stacking 25 times, gain 25 armor and magic resist and stop stacking)<br />47 : *Shroud of Stillness* (Shoot a beam that delays the first cast of affected enemies by 35%)<br />48 : Frozen Heart (Reduce the attack speed of enemies within 2 hexes by 25%)<br />49 : Cavalier Emblem (Wearer becomes a cavalier, cannot equip on a cavalier)<br />55 : Dragons Claw (+120 Magic Resist, every 2 seconds, regenerate 1.2% maximum health for each enemy targeting the holder. If holder is a dragon, increase all bonuses and effects by 20%)<br />56 : *Runaans Hurricane* (+10 Atttack Damage, attacks fire a bolt at a nearby enemy, dealing 70% of the holders attack damage as physical damage)<br />57 : *Quicksilver* (+20% attack speed. Immune to crowd control for 15 secnds)<br />58 : Chalice of Power (+30 Ability Power to holder and 2 adjacent allies on same row)<br />59 : Mirage Emblem (Wearer becomes a mirage, cannot equip on a mirage)<br />66 : Rapid Firecannon (+50% attack speed and +1 attack range, attacks cannot miss)<br />67 : *Last Whisper* (Dealing physical damage reduces the targets armor by 50% for 5 seconds, does not stack)<br />68 : Statikk Shiv (+15% attack speed, every 3rd attack shocks enemies for 70 magic damage and reduces their magic resist by 50% for 5 seconds)<br />69 : Ragewing Emblem (Wearer becomes a ragewing, cannot equip on a ragewing)<br />77 : *Thiefs Gloves* (Each round equip 2 random items, improve with player level, you cannot equip other items)<br />78 : *Hand of Justice* (+15 attack damage, +15% ability power. Attacks and abilities heal for 15% of damage dealt. Each round randomly increase 1 effect by 30%)<br />79 : *Assassin Emblem* (Wearer becomes an assassin, cannot equip on an assassin)<br />88 : Blue Buff (+20 Starting Mana. Gain 20 mana after casting an ability)<br />89 : Mage Emblem (Wearer becomes a mage, cannot equip on a mage)<br />99 : Tacticians Crown (Increase board unit size by 1)<br />
    fn give_item_effect(
        &mut self,
        item: u8,
        friendly_champions: &mut VecDeque<SummonedChampion>,
        enemy_champions: &mut VecDeque<SummonedChampion>,
    ) {
        match item {
            0 => (),
            1 => self.ad += 10.0,                   //BF Sword
            2 => self.ap += 0.1,                    //Needlessly Large Rod
            3 => self.health += 150.0,              //Giants Belt
            4 => self.ar += 0.2,                    //Chain Vest
            5 => self.mr += 0.2,                    //Negatron Cloak
            6 => self.attack_speed_modifier *= 1.1, //Recurve Bow
            7 => {
                self.cr += 5;
                self.dc += 10
            } //Sparring Glove
            8 => self.cm += 15,                     //Tear of the Goddess
            11 => self.ad += [15.0, 30.0, 45.0][self.star_level],
            12 => {
                self.ad += 10.0;
                self.ap += 0.1
            }
            13 => {
                self.ad += 10.0;
                self.health += 150.0;
                self.attack_speed_modifier *= 1.3;
                for friendly_champion in friendly_champions
                    .iter_mut()
                    .filter(self.location.get_within_distance(3))
                {
                    if friendly_champion.location.y == self.location.y {
                        friendly_champion.attack_speed_modifier *= 1.3;
                    }
                }
            }
            14 => {
                self.ad += 10.0;
                self.ar += 0.2;
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::EdgeOfNight(),
                    ..Default::default()
                })
            } //gives edge of night buff
            15 => {
                self.ad += 10.0;
                self.mr += 0.2;
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::Bloodthirster(),
                    ..Default::default()
                }); //gives bloodthirster buff
                self.omnivamp += 0.25;
            }
            16 => {
                self.ad += 10.0;
                self.attack_speed_modifier *= 0.1
            } //
            17 => {
                self.ad += 10.0;
                self.cr += 225;
                self.crit_damage += 0.1
            } //(!D)?
            18 => {
                self.ad += 10.0;
                self.cm += 15
            } //
            19 => {
                self.ad += 10.0;
            } //(!U)
            22 => self.ap += 0.75,
            23 => {
                self.ap += 0.40;
                self.health += 150.0
            } //
            24 => {
                self.ap += 0.1;
                self.ar += 0.2; //Gives locket shield
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
                    } //gives shield
                }
            }
            25 => {
                self.ap += 0.1;
                self.mr += 0.2;
            } //
            26 => {
                self.ap += 0.1;
                self.attack_speed_modifier *= 0.1
            } //
            27 => {
                self.ap += 0.5;
                self.cr += 15;
                self.crit_damage += 0.4
            } // //(!D) does bonus ability damage include from components? //
            28 => {
                self.ap += 0.1;
                self.cm += 15;
                self.se.push(StatusEffect {
                    duration: Some(500),
                    status_type: StatusType::ArchangelStaff(0.2),
                    ..Default::default()
                })
            }
            29 => {
                self.ap += 0.1;
            } //add next trait
            33 => self.health += 1000.0,
            34 => {
                self.health += 300.0;
                self.ar += 0.2;
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::GiveSunfire(),
                    ..Default::default()
                })
            } //(!U)
            35 => {
                self.health += 150.0;
                self.mr += 0.2;
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::Zephyr(500),
                    ..Default::default()
                })
            } //gives zephyr effect
            36 => {
                self.health += 150.0;
                self.attack_speed_modifier *= 0.1; //close enough, doesnt reset fully
                for enemy_champion in enemy_champions
                    .iter_mut()
                    .filter(self.location.get_within_distance(9))
                {
                    enemy_champion.se.push(StatusEffect {
                        duration: Some(0),
                        status_type: StatusType::Taunted(self.id),
                        is_negative: true,
                        ..Default::default()
                    }) //(!D) does shed cleanse taunt? gives taunt effect
                }
            }
            37 => {
                self.health += 150.0;
                self.dc += 15;
                self.shields.push(Shield {
                    duration: 1500,
                    size: 600.0,
                    blocks_type: Some(DamageType::Magical()),
                    pop: true,
                });
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
                        }); //(!D) shouldn't stack whether from multiple items on 1 person or from multiple champs
                    }
                }
            }
            38 => {
                self.health += 150.0;
                self.cm += 15;
                self.se.push(StatusEffect {
                    duration: Some(100),
                    status_type: StatusType::RedemptionGive(),
                    ..Default::default()
                })
            } //Gives redemption effect
            39 => self.health += 150.0, //(!U)
            44 => self.ar += 0.8, //(!D) says grants 40 bonus armor, is that the 40 from the two chain vests?
            45 => {
                self.ar += 0.2;
                self.mr += 0.2; //
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::Gargoyles(0.0),
                    ..Default::default()
                }) //(!D) only updates every second
            }
            46 => {
                self.ar += 0.2;
                self.attack_speed_modifier *= 1.1;
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::TitansResolve(0),
                    ..Default::default()
                })
            }
            47 => {
                self.ar += 0.2;
                self.dc += 15;
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::ShroudOfStillness(),
                    ..Default::default()
                })
            }
            48 => {
                self.ar += 0.2;
                self.cm += 15;
                self.se.push(StatusEffect {
                    duration: Some(0),
                    status_type: StatusType::ProtectorsVow(),
                    ..Default::default()
                })
            }
            55 => {
                self.mr += 1.2;
                self.se.push(StatusEffect {
                    duration: Some(200),
                    status_type: StatusType::DragonClawHeal(),
                    ..Default::default()
                })
            }
            56 => {
                self.mr += 0.2;
                self.attack_speed_modifier *= 1.1;
                self.ad += 10.0
            } //
            57 => {
                self.mr += 0.2;
                self.dc += 15;
                self.attack_speed_modifier *= 1.2;
                self.se.push(StatusEffect {
                    duration: Some(15000),
                    status_type: StatusType::CrowdControlImmune(),
                    ..Default::default()
                });
            }
            58 => {
                self.cm += 15;
                self.mr += 0.2;
                self.ap += 0.3;
                for friendly_champion in friendly_champions
                    .iter_mut()
                    .filter(self.location.get_within_distance(3))
                {
                    if friendly_champion.location.y == self.location.y
                    //discrepency distances
                    {
                        friendly_champion.ap += 0.3; //(!D) shouldn't stack whether from multiple items on 1 person or from multiple champs
                    }
                }
            }
            66 => {
                self.attack_speed_modifier *= 1.55;
                self.ra += 1;
            }
            67 => {
                self.attack_speed_modifier *= 1.21;
                self.cr += 15;
            } //discrepency
            68 => {
                self.attack_speed_modifier *= 1.21;
                self.cm += 15;
            }
            77 => {
                self.cr += 15;
                self.dc += 15;
            }
            78 => {
                self.cm += 10;
                self.cr += 15;

                if rand::thread_rng().gen_range(0..100) > 50
                //(!D) does this even mf'ing work
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
            88 => {
                self.cm += 50;
            }
            _ => println!("Unimplemented Item"),
        }
    }
    pub fn equal_id(&self, id: usize) -> bool {
        self.id == id
    }
    pub fn is_shred(&self) -> bool {
        self.shed == 2
    }
    pub fn update_shred(&mut self) {
        if self.shed == 1 {
            self.shed = 2;
        } else {
            self.shed = 0;
        }
    }
    pub fn perform_status(
        &mut self,
        status_effect: &mut StatusEffect,
        friendly_champions: &mut VecDeque<SummonedChampion>,
        enemy_champions: &mut VecDeque<SummonedChampion>,
        time_unit: i8,
        stun: &mut Stun,
    ) -> bool {
        if status_effect.duration.is_some() {
            let mut n_duration = status_effect
                .duration
                .unwrap()
                .checked_sub(time_unit.into())
                .unwrap_or(0); //unwrap duration and do checked subtraction

            if self.is_shred() && status_effect.is_negative {
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
                        if self.is_shred() {
                            return false;
                        }

                        if dmg_per_tick > dmg_to_do {
                            self.health -= dmg_to_do;
                        } else {
                            n_duration = time_next_tick;
                            status_effect.status_type = StatusType::MorellonomiconBurn(
                                dmg_per_tick,
                                dmg_to_do - dmg_per_tick,
                                time_next_tick,
                            );
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
                        self.heal((self.initial_hp - self.health) * 0.12);
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
                        //(!D) not actual shroud affect
                        for champ in enemy_champions
                            .iter_mut()
                            .filter(|x| x.location.x == self.location.x)
                        {
                            champ.cm -= (7 * champ.mc) / 20;
                        }
                    }
                    StatusType::DragonClawHeal() => {
                        n_duration = 200; //reset status effect

                        let num_targeting: f32 = self.get_num_targeting(enemy_champions) as f32;
                        self.heal(self.initial_hp * 0.012 * num_targeting);
                    }
                    StatusType::LastWhisperShred() => {
                        self.ar *= 2.0 //discrepency if thingy was reduced during time then
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
                            self.shed = 1;
                        } else {
                            return true;
                        }
                    }
                    StatusType::Bloodthirster() => {
                        if self.health <= (0.4 * self.initial_hp) {
                            self.shields.push(Shield {
                                duration: 500,
                                size: self.initial_hp / 4.0,
                                ..Default::default()
                            });
                        } else {
                            return true;
                        }
                    }
                    StatusType::Zephyr(banish_duration) => {
                        let opposite_location = Location {
                            x: self.location.y,
                            y: self.location.x,
                        }; //(!D) probs not opposite
                        opposite_location
                            .get_closest_to_location(enemy_champions)
                            .unwrap()
                            .se
                            .push(StatusEffect {
                                duration: Some(banish_duration),
                                status_type: StatusType::Banished(),
                                ..Default::default()
                            });
                    }
                    StatusType::Taunted(taunt_id) => {
                        if find_champion_index_from_id(enemy_champions, taunt_id).is_some() {
                            self.target = taunt_id;
                            self.target_cooldown = 100;
                            n_duration = 20;
                        }
                    }
                    StatusType::TitansResolve(old_stack_num) => {
                        if old_stack_num != 25 {
                            let difference: f32 = (self.titans_resolve_stacks - old_stack_num).into();
                            self.ad += 2.0 * difference;
                            self.ap += 0.02 * difference;
                            if self.titans_resolve_stacks == 25 {
                                self.ar += 0.25;
                                self.mr += 0.25;
                            }
                            status_effect.status_type = StatusType::TitansResolve(self.titans_resolve_stacks);
                        }
                        return true;
                    }
                    StatusType::ProtectorsVow() => {
                        if self.health <= (self.initial_hp / 2.0) {
                            self.mr += 0.25;
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
                if n_duration > 0 {
                    status_effect.duration = Some(n_duration);
                } else {
                    return false;
                }
            }
        }

        if !status_effect.applied {
            status_effect.applied = true;
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
                        self.location.y = 7; //(!D) cant remember if its 7
                    } //(!D) maybe leap not instantaneous/ first frame?

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
    pub fn _set_id(&mut self, id: usize) {
        self.id = id
    }
}

impl Default for SummonedChampion {
    fn default() -> Self {
        SummonedChampion {
            location: Location {
                ..Default::default()
            },
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
            a_id: 0,
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
            is_setup: false,
        }
    }
}

impl fmt::Display for SummonedChampion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} {}", self.id, self.a_id, self.health)
    }
}

impl From<PlacedChampion> for SummonedChampion {
    fn from(champ: PlacedChampion) -> Self {
        let star_level = champ.star; //get star level
        let of_champion = &CHAMPIONS[champ.id]; //get champ info
        SummonedChampion {
            location: champ.location, //create summoned champ with all details
            movement_progress: [0, 0],
            health: of_champion.hp[star_level],
            initial_hp: 0.0,
            cm: of_champion.sm, //update current mana to starting mana
            dc: 0,
            cr: 25,
            crit_damage: 0.3,
            mc: of_champion.mc,
            ar: of_champion.ar,
            mr: of_champion.mr,
            ad: of_champion.ad[star_level],
            attack_speed: of_champion.attack_speed,
            ra: of_champion.ra * 2, //because distanceBetweenPoints returns value twice as large
            id: 0,
            target_cooldown: 0,
            auto_attack_delay: 0,
            attack_speed_modifier: 1.0,
            target: 255,
            target_cells: Location { x: -1, y: -1 }, //(!O)
            a_id: of_champion.a_id,
            items: champ.items,
            ap: 1.0,
            se: Vec::new(),
            gain_mana_delay: 0,
            star_level,
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
            is_setup: false,
        }
    }
}
