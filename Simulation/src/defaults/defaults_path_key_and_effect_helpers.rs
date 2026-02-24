use super::*;

pub(super) fn normalize_key(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

pub(super) fn normalize_snake_key(input: &str) -> String {
    let mut normalized = String::new();
    let mut previous_was_separator = false;
    for character in input.chars() {
        if character.is_ascii_alphanumeric() {
            normalized.push(character.to_ascii_lowercase());
            previous_was_separator = false;
        } else if !normalized.is_empty() && !previous_was_separator {
            normalized.push('_');
            previous_was_separator = true;
        }
    }
    while normalized.ends_with('_') {
        normalized.pop();
    }
    normalized
}

pub(super) fn is_character_support_file(stem: &str) -> bool {
    normalize_key(stem) == "championdefaults"
}

pub(super) fn repository_root_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

pub(super) fn load_defaults_from_disk() -> Result<SimulatorDefaults> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("simulator_defaults.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading simulator defaults: {}", path.display()))?;
    serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing simulator defaults: {}", path.display()))
}

pub(super) fn load_protoplasm_lifeline_cooldown_seconds_default() -> Result<f64> {
    load_item_effect_cooldown_seconds(
        "Protoplasm Harness.json",
        "lifeline_gain_bonus_health_below_health_threshold",
        "Protoplasm Harness lifeline",
    )
}

pub(super) fn load_heartsteel_colossal_consumption_cooldown_seconds_default() -> Result<f64> {
    load_item_effect_cooldown_seconds(
        "Heartsteel.json",
        "colossal_consumption_empowered_hit_damage_and_permanent_bonus_health",
        "Heartsteel colossal consumption",
    )
}

pub(super) fn load_luden_echo_cooldown_seconds_default() -> Result<f64> {
    load_item_effect_cooldown_seconds(
        "Ludens Echo.json",
        "echo_consume_stacks_for_primary_and_secondary_magic_damage",
        "Luden's Echo",
    )
}

pub(super) fn load_item_effect_cooldown_seconds(
    item_file_name: &str,
    effect_id: &str,
    effect_label: &str,
) -> Result<f64> {
    let (item_path, item_data) = read_item_file(item_file_name)?;
    let effects = item_effects(&item_data, &item_path)?;
    ability_effect_by_id(effects, effect_id)
        .and_then(|effect| effect.get("cooldown_seconds"))
        .and_then(Value::as_f64)
        .or_else(|| {
            effects
                .iter()
                .find_map(|effect| effect.get("cooldown_seconds").and_then(Value::as_f64))
        })
        .ok_or_else(|| {
            anyhow!(
                "Missing effects_structured[*].cooldown_seconds for {} in {}",
                effect_label,
                item_path.display()
            )
        })
}

pub(super) fn ability_effect_by_id<'a>(effects: &'a [Value], effect_id: &str) -> Option<&'a Value> {
    effects
        .iter()
        .find(|effect| effect.get("id").and_then(Value::as_str) == Some(effect_id))
}

pub(super) fn highest_rank_value(values: &[Value]) -> Option<f64> {
    values.last().and_then(Value::as_f64)
}

pub(super) fn read_champion_file(champion_file_name: &str) -> Result<(std::path::PathBuf, Value)> {
    let path = repository_root_dir()
        .join("Characters")
        .join(champion_file_name);
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading champion file: {}", path.display()))?;
    let data: Value = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing champion file: {}", path.display()))?;
    Ok((path, data))
}

pub(super) fn read_item_file(item_file_name: &str) -> Result<(std::path::PathBuf, Value)> {
    let path = repository_root_dir().join("Items").join(item_file_name);
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading item file: {}", path.display()))?;
    let data: Value = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing item file: {}", path.display()))?;
    Ok((path, data))
}

pub(super) fn item_effects<'a>(
    item_data: &'a Value,
    item_path: &std::path::Path,
) -> Result<&'a [Value]> {
    item_data
        .get("effects_structured")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| anyhow!("Missing effects_structured in {}", item_path.display()))
}
