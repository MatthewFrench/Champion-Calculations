use super::shared_parsing_primitives::parse_stack_overrides_map;
use super::*;

pub(crate) fn parse_enemy_config(
    data: &Value,
    champion_bases: &HashMap<String, ChampionBase>,
    default_level: usize,
    default_stack_overrides: &HashMap<String, f64>,
) -> Result<EnemyConfig> {
    let champion = data
        .get("champion")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("Opponent actor requires champion"))?;
    let base = lookup_champion_base(champion_bases, champion)?;
    let actor_id = data
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or(champion)
        .to_string();
    if data.get("combat").is_some() {
        bail!(
            "Opponent actor '{}' uses deprecated combat proxy settings. Remove actor.combat and model champion behavior through champion scripts/data.",
            actor_id
        );
    }
    let placement = data.get("placement").unwrap_or(&Value::Null);
    let spawn_position_xy = if let Some(position) = placement.get("position") {
        let position_object = position.as_object().ok_or_else(|| {
            anyhow!("Opponent actor placement.position must be an object with x/y fields")
        })?;
        let x = position_object
            .get("x")
            .and_then(Value::as_f64)
            .ok_or_else(|| anyhow!("Opponent actor placement.position.x is required"))?;
        let y = position_object
            .get("y")
            .and_then(Value::as_f64)
            .ok_or_else(|| anyhow!("Opponent actor placement.position.y is required"))?;
        Some((x, y))
    } else {
        None
    };
    let movement_mode = match placement.get("movement").and_then(Value::as_str) {
        Some(movement) => match to_norm_key(movement).as_str() {
            "holdposition" | "hold" | "static" => OpponentMovementMode::HoldPosition,
            "maintaincombatrange" | "maintainrange" | "orbit" | "kite" => {
                OpponentMovementMode::MaintainCombatRange
            }
            _ => bail!(
                "Opponent actor '{}' has unsupported placement.movement '{}'. Allowed values: hold_position, maintain_combat_range.",
                actor_id,
                movement
            ),
        },
        None => OpponentMovementMode::MaintainCombatRange,
    };
    let level = data
        .get("level")
        .and_then(Value::as_u64)
        .unwrap_or(default_level as u64)
        .max(1) as usize;
    if data.get("assumptions").is_some() {
        bail!(
            "Opponent actor '{}' uses deprecated assumptions. Use actor.stack_overrides instead.",
            actor_id
        );
    }
    if data.get("item_stacks_at_level_20").is_some() {
        bail!(
            "Opponent actor '{}' uses deprecated item_stacks_at_level_20. Use actor.stack_overrides instead.",
            actor_id
        );
    }
    let mut stack_overrides = default_stack_overrides.clone();
    stack_overrides.extend(parse_stack_overrides_map(data.get("stack_overrides"))?);

    Ok(EnemyConfig {
        id: actor_id,
        name: base.name.clone(),
        level,
        base,
        spawn_position_xy,
        movement_mode,
        loadout_item_names: Vec::new(),
        loadout_rune_names: Vec::new(),
        loadout_shards: Vec::new(),
        stack_overrides,
    })
}
