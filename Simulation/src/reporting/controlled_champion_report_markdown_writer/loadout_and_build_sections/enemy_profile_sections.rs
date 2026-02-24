use super::*;
use std::collections::HashMap;

use crate::engine::EnemyDerivedCombatStats;
use crate::{EnemyBuildEntry, EnemyUrfPreset};

pub(crate) fn append_enemy_builds_section(
    content: &mut String,
    enemy_builds: &[EnemyBuildEntry],
    enemy_presets_used: &HashMap<String, EnemyUrfPreset>,
) {
    content.push_str("## Enemy Builds (URF Presets)\n");
    for (enemy, build, _) in enemy_builds {
        content.push_str(&format!("- {}: {}\n", enemy.name, item_names(build)));
        if let Some(preset) = enemy_presets_used.get(&to_norm_key(&enemy.name)) {
            content.push_str(&format!(
                "  - Source: {} (last checked {})\n",
                preset.source_url, preset.last_checked
            ));
            content.push_str(&format!("  - Runes: {}\n", preset.runes.join(", ")));
            content.push_str(&format!("  - Shards: {}\n", preset.shards.join(", ")));
        }
    }
    content.push('\n');
}

pub(crate) fn append_enemy_derived_combat_profiles_section(
    content: &mut String,
    enemy_derived_combat_stats: &[EnemyDerivedCombatStats],
    enemy_similarity_notes: &[String],
) {
    content.push_str("## Enemy Derived Combat Profiles\n");
    for profile in enemy_derived_combat_stats {
        content.push_str(&format!(
            "- {}: HP {:.1}, Armor {:.1}, MR {:.1}, AD {:.1}, AS {:.3} (interval {:.3}s), range {:.0}, projectile speed {:.0}, move speed {:.1}, desired combat range {:.0}, hit physical {:.1}, hit ability {:.1}, burst phys/magic/true {:.1}/{:.1}/{:.1}\n",
            profile.champion,
            profile.max_health,
            profile.armor,
            profile.magic_resist,
            profile.attack_damage,
            profile.attack_speed,
            profile.attack_interval_seconds,
            profile.attack_range,
            profile.attack_projectile_speed,
            profile.move_speed,
            profile.desired_combat_range,
            profile.physical_hit_damage,
            profile.ability_hit_damage,
            profile.burst_physical_damage,
            profile.burst_magic_damage,
            profile.burst_true_damage
        ));
    }
    if !enemy_similarity_notes.is_empty() {
        content.push_str("- Similarity checks:\n");
        for note in enemy_similarity_notes {
            content.push_str(&format!("  - {}\n", note));
        }
    }
    content.push('\n');
}
