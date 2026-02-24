use anyhow::anyhow;
use serde_json::Value;

use super::*;

pub(super) type ScaledEnemyScenarioSet = Vec<(String, f64, Vec<EnemyConfig>)>;

pub(super) struct ControlledChampionEnemyBuildProjection {
    pub(super) enemy_build_scenarios: Vec<EnemyBuildScenario>,
    pub(super) enemy_builds: Vec<EnemyBuildEntry>,
}

pub(super) fn parse_scaled_enemy_scenarios(
    scenario: &Value,
    champion_bases: &HashMap<String, ChampionBase>,
    champion_level: usize,
    stack_overrides: &HashMap<String, f64>,
) -> Result<ScaledEnemyScenarioSet> {
    let enemy_scenarios_raw: Vec<ParsedOpponentEncounter> =
        parse_opponent_encounters(scenario, champion_bases, champion_level, stack_overrides)?;
    Ok(enemy_scenarios_raw
        .iter()
        .map(|encounter| {
            let scaled = encounter
                .actors
                .iter()
                .cloned()
                .map(|mut enemy| {
                    enemy.base = champion_at_level(&enemy.base, enemy.level);
                    enemy
                })
                .collect::<Vec<_>>();
            (encounter.name.clone(), encounter.weight, scaled)
        })
        .collect::<Vec<_>>())
}

pub(super) fn build_enemy_build_projection(
    enemy_scenarios: &ScaledEnemyScenarioSet,
    enemy_presets: &HashMap<String, EnemyUrfPreset>,
    items: &HashMap<String, Item>,
) -> Result<ControlledChampionEnemyBuildProjection> {
    let mut enemy_build_scenarios: Vec<EnemyBuildScenario> = Vec::new();
    for (name, weight, enemies) in enemy_scenarios {
        let mut builds = Vec::new();
        for enemy in enemies {
            let preset_key = to_norm_key(&enemy.name);
            let preset = enemy_presets.get(&preset_key).ok_or_else(|| {
                anyhow!(
                    "Missing URF preset for enemy champion '{}'. Add it to {}.",
                    enemy.name,
                    enemy_preset_data_path().display()
                )
            })?;
            let build_items = item_pool_from_names(items, &preset.item_names)?;
            let preset_enemy_loadout =
                resolve_loadout(&enemy_loadout_from_preset(preset), enemy.level, false)?;
            let enemy_bonus_stats = preset_enemy_loadout.bonus_stats;
            let mut enemy_with_loadout = enemy.clone();
            enemy_with_loadout.loadout_item_names = preset.item_names.clone();
            enemy_with_loadout.loadout_rune_names = preset.runes.clone();
            enemy_with_loadout.loadout_shards = preset.shards.clone();
            builds.push((enemy_with_loadout, build_items, enemy_bonus_stats));
        }
        enemy_build_scenarios.push((name.clone(), *weight, builds));
    }

    let enemy_builds = enemy_build_scenarios
        .first()
        .map(|(_, _, builds)| builds.clone())
        .unwrap_or_default();
    Ok(ControlledChampionEnemyBuildProjection {
        enemy_build_scenarios,
        enemy_builds,
    })
}

pub(super) fn build_scenario_reference_outcomes(
    enemy_build_scenarios: &[EnemyBuildScenario],
    sim: &SimulationConfig,
    urf: &UrfBuffs,
    controlled_champion_base: &ChampionBase,
) -> Vec<CombatOutcome> {
    enemy_build_scenarios
        .iter()
        .map(|(_, _, enemy_builds)| {
            let damage_reference = enemy_builds
                .iter()
                .map(|(enemy, build, bonus_stats)| {
                    derive_enemy_combat_stats(enemy, build, bonus_stats, sim, urf).max_health
                })
                .sum::<f64>()
                .max(1.0);
            CombatOutcome {
                time_alive_seconds: sim.max_time_seconds.max(1.0),
                damage_dealt: damage_reference,
                healing_done: controlled_champion_base.base_health.max(1.0),
                enemy_kills: enemy_builds.len().max(1),
                invulnerable_seconds: sim.max_time_seconds.max(1.0),
            }
        })
        .collect::<Vec<_>>()
}
