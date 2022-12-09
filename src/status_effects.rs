///Records champion's stun state.<br />
pub struct ShouldStun {
	///Records whether champion is stunned:<br />
	///0 = not stunned<br />
	///1 = stunned<br />
	///2 = stun immune
	stun : u8
}

///Status Type (enum):<br />
///Holds information about what the status does
#[derive(PartialEq)]
enum StatusType {
	///Attack Speed Buff:<br />
	///(bool : whether the buff has been applied, f32 : actual modifier)
	AttackSpeedBuff(f32),
	
	///Increase Damage Taken:<br />
	///(bool : whether the buff has been applied, i32 : actual modifier in % (so 120 = 120% or 20% increase))
	IncreaseDamageTaken(f32),

	///Stun
	Stun(),

	///Grevious Wounds:<br />
	///Reduces healing by 50%
	GreviousWounds(),

	///Gives edge of night buff<br />:
	EdgeOfNight(),
	
	///Whether the target is targetable
	///bool : Whether the buff has been applied
	Untargetable(),

	///Bloodthirster shield at 40%
	Bloodthirster(),

	///Assassin trait leap
	#[allow(dead_code)]
	Assassin(),

	///Morellonomicon Burn:<br />
	///(f32 : damage per tick, f32 : damage to do, i16 : time til next tick)
	MorellonomiconBurn(f32, f32, i16),

	///Ionic spark effect:<br />
	///Reduces MR by 50%<br />
	IonicSparkEffect(),//maybe discrepencies? awkward cuz only lasts 1 frame?

	///Archangel Staff:<br />
	///(bool : applied. f32 : ap increase)
	ArchangelStaff(f32),

	///Zephyr Item:<br />
	///(bool : applied, i16 : banish duration)
	Zephyr(i16),

	///Banished:<br />
	///(bool : applied)
	Banished(),

	///Taunted:<br />
	///(usize : ID of taunter)
	Taunted(usize),

	///Redemption:<br />
	///(bool : applied)
	RedemptionGive(),

	///Gargoyles Item Effect:<br />
	///(f32: How many were targeting previous frame)
	Gargoyles(f32),
	///Titans Resolve Item Effect:<br />
	///(u8: Number of stacks previous frame)
	TitansResolve(u8),

	///Shroud of Stillness Item Effect:<br />
	///Immediately removed/ used at start of game
	ShroudOfStillness(),

	///Protectors Vow Item Effect:<br />
	ProtectorsVow(),

	///Dragon Claw Heal Item Effect:<br />
	DragonClawHeal(),

	///Immune of CC Effect:<br />
	CrowdControlImmune(),

	///Last Whisper Armor Shred Effect:<br />
	///(bool : applied)
	LastWhisperShred(),

	///Shreds Magic Resist Effect:<br />
	///(bool : applied, f32 : multiplyer/ effect)
	ShredMagicResist(f32),

	///Gives sunfire effect:<br />
	///Not implemented
	GiveSunfire(),

	///None
	NoEffect()
}

///StatusEffect (struct)<br />:
///Stores a status type and a duration
struct StatusEffect {
	///Duration of status effect in centiseconds
	duration : Option<i16>,//optimisation so uses Option<i16> rather than i16

	///Whether the status effect has been applied
	applied : bool,
	///Stores status type
	status_type : StatusType,
	///Whether is negative for shred
	is_negative : bool,
}
///Default Status Effect Values
impl Default for StatusEffect {
	fn default() -> StatusEffect {
		StatusEffect { duration: None, applied : false, status_type: StatusType::NoEffect(), is_negative: false }
	}
}

impl StatusEffect {
	fn perform_status(&mut self, affected_champion : &mut SummonedChampion, friendly_champions : &mut VecDeque<SummonedChampion>, enemy_champions : &mut VecDeque<SummonedChampion>, time_unit : i8, stun : &mut ShouldStun) -> bool {
		if self.duration.is_some() {
			let mut n_duration = self.duration.unwrap().checked_sub(time_unit.into()).unwrap_or(0); //unwrap duration and do checked subtraction
			
			if affected_champion.shed == 2 && self.is_negative { n_duration = 0; }//if shed and negative set duration to 0
			
			if n_duration <= 0 {
				match self.status_type {//undo status effect/ remove effect. some effects aren't actually removed but just reinitialise	
					StatusType::AttackSpeedBuff(modifier) => {
						affected_champion.attackSpeedModifier /= modifier
					}
					StatusType::IncreaseDamageTaken(modifier) => {
						affected_champion.incomingDMGModifier /= modifier
					}
					StatusType::Untargetable() => {
						affected_champion.targetable = true//(!D) if have 2 untargetable effects this will untarget too early
					}
					StatusType::MorellonomiconBurn(dmg_per_tick, dmg_to_do, time_next_tick) => {
						if affected_champion.shed == 2 { return false; }
						
						if dmg_per_tick > dmg_to_do { 
							affected_champion.health -= dmg_to_do; 
						}
						else {
							n_duration = time_next_tick;
							self.status_type = StatusType::MorellonomiconBurn(dmg_per_tick, dmg_to_do - dmg_per_tick, time_next_tick);
						}
						
					}
					StatusType::IonicSparkEffect() => {
						affected_champion.mr *= 2.0; //(!D) Possible discrepency
						affected_champion.zap = false
					}  
					StatusType::ArchangelStaff(ap_amount) => {
						n_duration = 500; 
						affected_champion.ap += ap_amount;
					}
					StatusType::Banished() => {
						affected_champion.banish = false
					}
					StatusType::RedemptionGive() => {
						n_duration = 100;//increase duration
						for champ in friendly_champions.iter_mut().filter(affected_champion.location.getWithinDistance(3)) {
							champ.heal((champ.initial_hp - champ.health) * 0.12)//discrepency check at multitarget damage time for redemption heal for reduction
						}
						affected_champion.heal((affected_champion.initial_hp - affected_champion.health) * 0.12);
					}
					StatusType::Gargoyles(oldNumTargeting) => {
						n_duration = 100;//increase duration
						let numTargeting : f32 = affected_champion.getNumTargeting(enemy_champions) as f32;
						let difference= numTargeting - oldNumTargeting;//get change
						affected_champion.ar += 0.18 * difference;
						affected_champion.mr += 0.18 * difference;
						self.status_type = StatusType::Gargoyles(numTargeting);
					}
					StatusType::ShroudOfStillness() => {//(!D) not actual shroud affect
						for champ in enemy_champions.iter_mut().filter(|x| x.location.x == affected_champion.location.x) {
							champ.cm -= (7 * champ.mc) / 20;
						}
					}
					StatusType::DragonClawHeal() => {
						n_duration = 200;//reset status effect

						let numTargeting : f32 = affected_champion.getNumTargeting(enemy_champions) as f32;
						affected_champion.heal(affected_champion.initial_hp * 0.012 * numTargeting);
					}
					StatusType::LastWhisperShred() => {
						affected_champion.ar *= 2.0 //discrepency if thingy was reduced during time then
					}	
					StatusType::GiveSunfire() => {//(!U)
						n_duration = 300; 
						for champ in enemy_champions.iter_mut().filter(affected_champion.location.getWithinDistance(3)){
							let dmg = champ.initial_hp / 20.0;
							champ.se.push(StatusEffect {duration : Some(100), status_type : StatusType::MorellonomiconBurn(dmg, dmg / 3.0, 100), ..Default::default()})
						}
					}
					StatusType::EdgeOfNight() => {
						if affected_champion.health <= (affected_champion.initial_hp / 2.0) {
							affected_champion.se.push(StatusEffect { duration : Some(50), status_type : StatusType::Untargetable(), ..Default::default()});//optimisation at every ..Default::default() with instead isNegative : false
							affected_champion.se.push(StatusEffect { duration: None, status_type: StatusType::AttackSpeedBuff(1.3), ..Default::default()}); //(!D) technically attack speed buff comes into play after untargetable wears off
							affected_champion.shed = 1;
						}
						else { return true }
					}
					StatusType::Bloodthirster() => {
						if affected_champion.health <= (0.4 * affected_champion.initial_hp) {
							affected_champion.shields.push(Shield{duration : 500, size : affected_champion.initial_hp / 4.0, ..Default::default()});
						}
						else { return true }
					}
					StatusType::Zephyr(banish_duration) => {
						let opposite_location = Location { x : affected_champion.location.y, y : affected_champion.location.x };//(!D) probs not opposite
						opposite_location.getClosestToLocation(enemy_champions).unwrap().se.push(StatusEffect{ duration: Some(banish_duration), status_type: StatusType::Banished(), ..Default::default() });
					}
					StatusType::Taunted(tauntID) => {
						if findChampionIndexFromID(enemy_champions, tauntID).is_some() {
							affected_champion.target = tauntID;
							affected_champion.targetCountDown = 100;
							n_duration = 20;
						}
					}
					StatusType::TitansResolve(mut oldStackNum) => {
						if oldStackNum != 25 {
							let difference : f32 = (affected_champion.titansResolveStack - oldStackNum).into();
							affected_champion.ad += 2.0 * difference;
							affected_champion.ap += 0.02 * difference;
							oldStackNum = affected_champion.titansResolveStack;
							if oldStackNum == 25 {
								affected_champion.ar += 0.25;
								affected_champion.mr += 0.25;
							}
						}
						return true;
					}
					StatusType::ProtectorsVow() => {
						if affected_champion.health <= (affected_champion.initial_hp / 2.0) {
							affected_champion.mr += 0.25;
							affected_champion.ar += 0.25;
							affected_champion.shields.push( Shield {
								duration : 500,
								size : affected_champion.initial_hp / 4.0,
								..Default::default()
							})
						}
						else {
							return true
						}
					}
					_ => ()
				}
				if n_duration > 0 { self.duration = Some(n_duration); }
				else { return false }
			}
		}
		
		if ! self.applied
		{
			self.applied = true;
			match self.status_type {
				StatusType::AttackSpeedBuff(modifier) => {
					affected_champion.attackSpeedModifier *= modifier;
				}
				StatusType::Stun() => {
					self.applied = false;
					if stun.stun == 0 { stun.stun = 1; }//has to check stun.stun == 0 as if stun.stun == 2 it is immune
				} 
				StatusType::IncreaseDamageTaken(modifier) => { affected_champion.incomingDMGModifier *= modifier; }
				StatusType::Assassin() => {
					if affected_champion.location.y >= 4 { affected_champion.location.y = 0; }
					else { affected_champion.location.y = 0; }//(!D) maybe leap not instantaneous/ first frame?
		
					return false
				}
				StatusType::Untargetable() => { affected_champion.targetable = false } 
				StatusType::IonicSparkEffect() => {	
					affected_champion.mr /= 2.0; 
					affected_champion.zap = true
				}
				StatusType::Banished() => { affected_champion.banish = true }																	
				StatusType::LastWhisperShred() => { affected_champion.ar /= 2.0; }
				StatusType::CrowdControlImmune() => {
					self.applied = false;
					stun.stun = 2;
				}
				_ => ()
			}
		}
		true
	}
}
