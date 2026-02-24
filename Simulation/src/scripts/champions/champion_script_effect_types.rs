#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ChampionScriptEvent {
    WarwickInfiniteDuress,
    VayneTumbleEmpower,
    MorganaDarkBinding,
    MorganaSoulShackles,
    MorganaSoulShacklesDetonate,
    SonaCrescendo,
    DoctorMundoInfectedBonesaw,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ChampionScriptPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ScriptedEffectHitbox {
    Circle { radius: f64 },
}

impl ScriptedEffectHitbox {
    pub(crate) fn radius(self) -> f64 {
        match self {
            Self::Circle { radius } => radius.max(0.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChampionScriptExecutionInput {
    pub event: ChampionScriptEvent,
    pub actor_position: ChampionScriptPoint,
    pub actor_level: usize,
    pub distance_to_target: f64,
    pub physical_hit_damage: f64,
    pub actor_ability_power: f64,
    pub actor_bonus_attack_damage: f64,
    pub target_current_health: f64,
    pub target_max_health: f64,
    pub now: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ChampionScriptAction {
    AddNextAttackBonusPhysical {
        amount: f64,
        trace_message: &'static str,
    },
    ApplyDamage {
        source: ChampionScriptPoint,
        projectile_speed: f64,
        hitbox: ScriptedEffectHitbox,
        physical: f64,
        magic: f64,
        true_damage: f64,
        stun_duration: f64,
    },
    ScheduleFollowup {
        delay_seconds: f64,
        priority: i32,
        event: ChampionScriptEvent,
    },
}
