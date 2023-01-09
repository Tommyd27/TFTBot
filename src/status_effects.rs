///Records champion's stun state.<br />
pub struct Stun {
    ///Records whether champion is stunned:<br />
    ///0 = not stunned<br />
    ///1 = stunned<br />
    ///2 = stun immune
    pub stun: u8,
}

///Status Type (enum):<br />
///Holds information about what the status does
#[derive(PartialEq, Debug)]
pub enum StatusType {
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
    IonicSparkEffect(), //maybe discrepencies? awkward cuz only lasts 1 frame?

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
    NoEffect(),
}

///StatusEffect (struct)<br />:
///Stores a status type and a duration
#[derive(Debug)]
pub struct StatusEffect {
    ///Duration of status effect in centiseconds
    pub duration: Option<i16>, //optimisation so uses Option<i16> rather than i16

    ///Whether the status effect has been applied
    pub applied: bool,
    ///Stores status type
    pub status_type: StatusType,
    ///Whether is negative for shred
    pub is_negative: bool,
}
///Default Status Effect Values
impl Default for StatusEffect {
    fn default() -> StatusEffect {
        StatusEffect {
            duration: None,
            applied: false,
            status_type: StatusType::NoEffect(),
            is_negative: false,
        }
    }
}

impl PartialEq for StatusEffect {
    fn eq(&self, other: &Self) -> bool {
        self.status_type == other.status_type
    }
}
