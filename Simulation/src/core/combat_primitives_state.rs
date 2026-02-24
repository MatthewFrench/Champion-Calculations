#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub(crate) enum StatusEffectKind {
    Stun,
    Silence,
    Root,
    Slow,
    Untargetable,
    Stasis,
    Custom(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum StatusDuration {
    Timed { remaining_seconds: f64 },
    Persistent,
}

impl StatusDuration {
    #[allow(dead_code)]
    pub(crate) fn timed(seconds: f64) -> Self {
        Self::Timed {
            remaining_seconds: seconds.max(0.0),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn remaining_seconds(self) -> Option<f64> {
        match self {
            Self::Timed { remaining_seconds } => Some(remaining_seconds),
            Self::Persistent => None,
        }
    }

    fn refresh_from(&mut self, incoming: StatusDuration) {
        match (*self, incoming) {
            (Self::Persistent, _) | (_, Self::Persistent) => *self = Self::Persistent,
            (
                Self::Timed {
                    remaining_seconds: current,
                },
                Self::Timed {
                    remaining_seconds: incoming,
                },
            ) => {
                *self = Self::Timed {
                    remaining_seconds: current.max(incoming),
                };
            }
        }
    }

    fn tick(&mut self, delta_seconds: f64) {
        if delta_seconds <= 0.0 {
            return;
        }
        if let Self::Timed { remaining_seconds } = self {
            *remaining_seconds = (*remaining_seconds - delta_seconds).max(0.0);
        }
    }

    fn is_expired(self) -> bool {
        matches!(
            self,
            Self::Timed { remaining_seconds } if remaining_seconds <= 0.0
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum StatusPersistence {
    Replace,
    RefreshDuration,
    StackRefreshDuration,
    Independent,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StatusEffect {
    pub kind: StatusEffectKind,
    pub duration: StatusDuration,
    pub stacks: u32,
    pub max_stacks: u32,
    pub persistence: StatusPersistence,
}

impl StatusEffect {
    #[allow(dead_code)]
    pub(crate) fn timed(
        kind: StatusEffectKind,
        duration_seconds: f64,
        stacks: u32,
        persistence: StatusPersistence,
    ) -> Self {
        let mut effect = Self {
            kind,
            duration: StatusDuration::timed(duration_seconds),
            stacks: stacks.max(1),
            max_stacks: u32::MAX,
            persistence,
        };
        effect.clamp_stacks();
        effect
    }

    #[allow(dead_code)]
    pub(crate) fn persistent(
        kind: StatusEffectKind,
        stacks: u32,
        persistence: StatusPersistence,
    ) -> Self {
        let mut effect = Self {
            kind,
            duration: StatusDuration::Persistent,
            stacks: stacks.max(1),
            max_stacks: u32::MAX,
            persistence,
        };
        effect.clamp_stacks();
        effect
    }

    #[allow(dead_code)]
    pub(crate) fn with_max_stacks(mut self, max_stacks: u32) -> Self {
        self.max_stacks = max_stacks.max(1);
        self.clamp_stacks();
        self
    }

    fn clamp_stacks(&mut self) {
        self.max_stacks = self.max_stacks.max(1);
        self.stacks = self.stacks.clamp(1, self.max_stacks);
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct StatusEffectSet {
    effects: Vec<StatusEffect>,
}

impl StatusEffectSet {
    #[allow(dead_code)]
    pub(crate) fn effects(&self) -> &[StatusEffect] {
        &self.effects
    }

    #[allow(dead_code)]
    pub(crate) fn is_active(&self, kind: &StatusEffectKind) -> bool {
        self.effects.iter().any(|effect| &effect.kind == kind)
    }

    #[allow(dead_code)]
    pub(crate) fn total_stacks(&self, kind: &StatusEffectKind) -> u32 {
        self.effects
            .iter()
            .filter(|effect| &effect.kind == kind)
            .map(|effect| effect.stacks)
            .sum()
    }

    pub(crate) fn apply(&mut self, mut incoming: StatusEffect) {
        incoming.clamp_stacks();
        match incoming.persistence {
            StatusPersistence::Independent => self.effects.push(incoming),
            StatusPersistence::Replace => {
                self.effects.retain(|effect| effect.kind != incoming.kind);
                self.effects.push(incoming);
            }
            StatusPersistence::RefreshDuration => {
                if let Some(existing) = self
                    .effects
                    .iter_mut()
                    .find(|effect| effect.kind == incoming.kind)
                {
                    existing.max_stacks = existing.max_stacks.max(incoming.max_stacks);
                    existing.stacks = existing.stacks.max(incoming.stacks);
                    existing.clamp_stacks();
                    existing.duration.refresh_from(incoming.duration);
                } else {
                    self.effects.push(incoming);
                }
            }
            StatusPersistence::StackRefreshDuration => {
                if let Some(existing) = self
                    .effects
                    .iter_mut()
                    .find(|effect| effect.kind == incoming.kind)
                {
                    existing.max_stacks = existing.max_stacks.max(incoming.max_stacks);
                    existing.stacks = existing.stacks.saturating_add(incoming.stacks);
                    existing.clamp_stacks();
                    existing.duration.refresh_from(incoming.duration);
                } else {
                    self.effects.push(incoming);
                }
            }
        }
    }

    pub(crate) fn tick(&mut self, delta_seconds: f64) {
        if delta_seconds <= 0.0 {
            return;
        }
        for effect in &mut self.effects {
            effect.duration.tick(delta_seconds);
        }
        self.effects.retain(|effect| !effect.duration.is_expired());
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CastLockWindow {
    pub windup_seconds: f64,
    pub channel_seconds: f64,
    pub lockout_seconds: f64,
}

impl CastLockWindow {
    pub(crate) fn new(windup_seconds: f64, channel_seconds: f64, lockout_seconds: f64) -> Self {
        Self {
            windup_seconds: windup_seconds.max(0.0),
            channel_seconds: channel_seconds.max(0.0),
            lockout_seconds: lockout_seconds.max(0.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum CastLockPhase {
    Idle,
    Windup,
    Channel,
    Lockout,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct CastLockState {
    windup_remaining_seconds: f64,
    channel_remaining_seconds: f64,
    lockout_remaining_seconds: f64,
}

impl CastLockState {
    #[allow(dead_code)]
    pub(crate) fn begin(&mut self, window: CastLockWindow) {
        self.windup_remaining_seconds = self.windup_remaining_seconds.max(window.windup_seconds);
        self.channel_remaining_seconds = self.channel_remaining_seconds.max(window.channel_seconds);
        self.lockout_remaining_seconds = self.lockout_remaining_seconds.max(window.lockout_seconds);
    }

    #[allow(dead_code)]
    pub(crate) fn phase(self) -> CastLockPhase {
        if self.windup_remaining_seconds > 0.0 {
            CastLockPhase::Windup
        } else if self.channel_remaining_seconds > 0.0 {
            CastLockPhase::Channel
        } else if self.lockout_remaining_seconds > 0.0 {
            CastLockPhase::Lockout
        } else {
            CastLockPhase::Idle
        }
    }

    #[allow(dead_code)]
    pub(crate) fn is_locked(self) -> bool {
        self.phase() != CastLockPhase::Idle
    }

    #[allow(dead_code)]
    pub(crate) fn remaining(self) -> CastLockWindow {
        CastLockWindow {
            windup_seconds: self.windup_remaining_seconds,
            channel_seconds: self.channel_remaining_seconds,
            lockout_seconds: self.lockout_remaining_seconds,
        }
    }

    pub(crate) fn tick(&mut self, delta_seconds: f64) {
        if delta_seconds <= 0.0 {
            return;
        }
        let mut remaining = delta_seconds;
        let windup_spent = self.windup_remaining_seconds.min(remaining);
        self.windup_remaining_seconds -= windup_spent;
        remaining -= windup_spent;

        let channel_spent = self.channel_remaining_seconds.min(remaining);
        self.channel_remaining_seconds -= channel_spent;
        remaining -= channel_spent;

        let lockout_spent = self.lockout_remaining_seconds.min(remaining);
        self.lockout_remaining_seconds -= lockout_spent;
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct CombatPrimitivesState {
    status_effects: StatusEffectSet,
    cast_lock: CastLockState,
}

impl CombatPrimitivesState {
    #[allow(dead_code)]
    pub(crate) fn status_effects(&self) -> &StatusEffectSet {
        &self.status_effects
    }

    pub(crate) fn apply_status(&mut self, effect: StatusEffect) {
        self.status_effects.apply(effect);
    }

    #[allow(dead_code)]
    pub(crate) fn cast_lock(&self) -> CastLockState {
        self.cast_lock
    }

    pub(crate) fn begin_cast_lock(&mut self, window: CastLockWindow) {
        self.cast_lock.begin(window);
    }

    pub(crate) fn tick(&mut self, delta_seconds: f64) {
        if delta_seconds <= 0.0 {
            return;
        }
        self.status_effects.tick(delta_seconds);
        self.cast_lock.tick(delta_seconds);
    }
}
