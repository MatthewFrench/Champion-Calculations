use crate::cooldown_after_haste;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct RuntimeBuffState {
    pub ability_haste: f64,
    pub item_haste: f64,
    pub cooldown_rate_multiplier: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum CooldownMetricSource {
    Ability,
    Item,
    Neutral,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum StatQuery {
    CooldownSeconds {
        base_seconds: f64,
        source: CooldownMetricSource,
    },
}

pub(crate) fn resolve_stat(query: StatQuery, buffs: RuntimeBuffState) -> f64 {
    match query {
        StatQuery::CooldownSeconds {
            base_seconds,
            source,
        } => {
            let base = base_seconds.max(0.0);
            let haste = match source {
                CooldownMetricSource::Ability => buffs.ability_haste,
                CooldownMetricSource::Item => buffs.item_haste,
                CooldownMetricSource::Neutral => 0.0,
            };
            let after_haste = cooldown_after_haste(base, haste.max(-99.0));
            let multiplier = if buffs.cooldown_rate_multiplier > 0.0 {
                buffs.cooldown_rate_multiplier
            } else {
                1.0
            };
            (after_haste * multiplier).max(0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_item_cooldown_applies_item_haste() {
        let value = resolve_stat(
            StatQuery::CooldownSeconds {
                base_seconds: 120.0,
                source: CooldownMetricSource::Item,
            },
            RuntimeBuffState {
                item_haste: 300.0,
                ..RuntimeBuffState::default()
            },
        );
        assert!((value - 30.0).abs() < 1e-9);
    }

    #[test]
    fn resolve_ability_cooldown_applies_ability_haste() {
        let value = resolve_stat(
            StatQuery::CooldownSeconds {
                base_seconds: 20.0,
                source: CooldownMetricSource::Ability,
            },
            RuntimeBuffState {
                ability_haste: 300.0,
                ..RuntimeBuffState::default()
            },
        );
        assert!((value - 5.0).abs() < 1e-9);
    }
}
