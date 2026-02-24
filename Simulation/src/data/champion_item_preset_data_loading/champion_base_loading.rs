use super::*;

pub(crate) fn normalize_name(input: &str) -> String {
    input
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(|ch| ch.to_lowercase())
        .collect()
}

fn is_character_support_file(stem: &str) -> bool {
    normalize_name(stem) == "championdefaults"
}

pub(crate) fn champion_base_from_character_data(
    character: &Value,
    fallback_name: &str,
) -> Result<ChampionBase> {
    let base_stats = character
        .get("base_stats")
        .ok_or_else(|| anyhow!("Missing base_stats for {}", fallback_name))?;

    let attack_speed = base_stats
        .get("attack_speed")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing attack_speed.base for {}", fallback_name))?;
    let armor = base_stats
        .get("armor")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing armor.base for {}", fallback_name))?;
    let magic_resist = base_stats
        .get("magic_resist")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing magic_resist.base for {}", fallback_name))?;
    let attack_damage = base_stats
        .get("attack_damage")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing attack_damage.base for {}", fallback_name))?;
    let health = base_stats
        .get("health")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing health.base for {}", fallback_name))?;
    let move_speed = base_stats
        .get("move_speed")
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing move_speed for {}", fallback_name))?;
    let attack_speed_per_level_percent = base_stats
        .get("attack_speed")
        .and_then(|v| v.get("per_level_percent"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let armor_per_level = base_stats
        .get("armor")
        .and_then(|v| v.get("per_level"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let magic_resist_per_level = base_stats
        .get("magic_resist")
        .and_then(|v| v.get("per_level"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let attack_damage_per_level = base_stats
        .get("attack_damage")
        .and_then(|v| v.get("per_level"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let health_per_level = base_stats
        .get("health")
        .and_then(|v| v.get("per_level"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);

    let attack_type = character
        .get("basic_attack")
        .and_then(|v| v.get("attack_type"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_lowercase();
    let is_melee = attack_type == "melee";
    let base_attack_projectile_speed = character
        .get("basic_attack")
        .and_then(|v| v.get("missile_speed"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let attack_range = base_stats
        .get("attack_range")
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing attack_range for {}", fallback_name))?;

    let champion_name = character
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or(fallback_name)
        .to_string();

    Ok(ChampionBase {
        name: champion_name,
        base_health: health,
        health_per_level,
        base_armor: armor,
        armor_per_level,
        base_magic_resist: magic_resist,
        magic_resist_per_level,
        base_attack_damage: attack_damage,
        attack_damage_per_level,
        base_attack_speed: attack_speed,
        attack_speed_per_level_percent,
        base_attack_range: attack_range,
        base_attack_projectile_speed,
        base_move_speed: move_speed,
        is_melee,
    })
}

pub(crate) fn load_champion_bases() -> Result<HashMap<String, ChampionBase>> {
    let mut out = HashMap::new();
    let mut entries = fs::read_dir(characters_dir())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("json"))
        .collect::<Vec<_>>();
    entries.sort();

    for path in entries {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid character filename"))?;
        if is_character_support_file(stem) {
            continue;
        }
        let data = load_json(&path)?;
        let base = champion_base_from_character_data(&data, stem)?;
        out.insert(normalize_name(stem), base.clone());
        out.insert(normalize_name(&base.name), base);
    }
    Ok(out)
}

pub(crate) fn lookup_champion_base(
    champion_bases: &HashMap<String, ChampionBase>,
    champion_name: &str,
) -> Result<ChampionBase> {
    champion_bases
        .get(&normalize_name(champion_name))
        .cloned()
        .ok_or_else(|| anyhow!("Champion not found: {}", champion_name))
}
