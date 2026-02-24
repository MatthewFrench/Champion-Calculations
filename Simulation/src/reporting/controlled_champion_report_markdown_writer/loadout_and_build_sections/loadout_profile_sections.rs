use super::*;

use crate::{ChampionBase, Item, ResolvedLoadout, Stats};

pub(crate) fn append_base_stats_section(
    content: &mut String,
    controlled_champion_name: &str,
    controlled_champion_base_level: &ChampionBase,
) {
    content.push_str(&format!(
        "## {} Base Stats At Level\n",
        controlled_champion_name
    ));
    content.push_str(&format!(
        "- HP: {}, Armor: {}, MR: {}, AD: {}, AS: {}, MS: {}\n\n",
        format_f64_with_commas(controlled_champion_base_level.base_health, 1),
        format_f64_with_commas(controlled_champion_base_level.base_armor, 1),
        format_f64_with_commas(controlled_champion_base_level.base_magic_resist, 1),
        format_f64_with_commas(controlled_champion_base_level.base_attack_damage, 1),
        format_f64_with_commas(controlled_champion_base_level.base_attack_speed, 3),
        format_f64_with_commas(controlled_champion_base_level.base_move_speed, 1)
    ));
}

pub(crate) fn append_loadout_selection_and_effect_sections(
    content: &mut String,
    controlled_champion_name: &str,
    controlled_champion_loadout: &ResolvedLoadout,
    enemy_loadout: &ResolvedLoadout,
    controlled_champion_unmodeled_item_effect_names: &[String],
) {
    content.push_str("## Selected Rune Page And Shards\n");
    content.push_str(&format!("- {}:\n", controlled_champion_name));
    for selection_label in &controlled_champion_loadout.selection_labels {
        content.push_str(&format!("  - {}\n", selection_label));
    }
    if enemy_loadout.selection_labels.is_empty() {
        content.push_str(
            "- Opponents: champion-specific preset rune pages are listed in Enemy Builds.\n\n",
        );
    } else {
        content.push_str("- Opponents (shared):\n");
        for selection_label in &enemy_loadout.selection_labels {
            content.push_str(&format!("  - {}\n", selection_label));
        }
        content.push('\n');
    }
    if !controlled_champion_loadout.applied_notes.is_empty()
        || !enemy_loadout.applied_notes.is_empty()
    {
        content.push_str("- Applied deterministic loadout effects:\n");
        for note in &controlled_champion_loadout.applied_notes {
            content.push_str(&format!("  - {}: {}\n", controlled_champion_name, note));
        }
        for note in &enemy_loadout.applied_notes {
            content.push_str(&format!("  - Enemies: {}\n", note));
        }
    }
    if !controlled_champion_loadout.skipped_notes.is_empty()
        || !enemy_loadout.skipped_notes.is_empty()
    {
        content.push_str("- Skipped unsupported/non-deterministic effects:\n");
        for note in &controlled_champion_loadout.skipped_notes {
            content.push_str(&format!("  - {}: {}\n", controlled_champion_name, note));
        }
        for note in &enemy_loadout.skipped_notes {
            content.push_str(&format!("  - Enemies: {}\n", note));
        }
    }
    if !controlled_champion_loadout.unmodeled_rune_names.is_empty() {
        content.push_str(
            "- Controlled champion runes with no modeled deterministic/runtime combat effect:\n",
        );
        for rune_name in &controlled_champion_loadout.unmodeled_rune_names {
            content.push_str(&format!("  - {}\n", rune_name));
        }
    }
    if !controlled_champion_unmodeled_item_effect_names.is_empty() {
        content.push_str(
            "- Controlled champion items with unmodeled passive/active/structured runtime effects:\n",
        );
        for item_name in controlled_champion_unmodeled_item_effect_names {
            content.push_str(&format!("  - {}\n", item_name));
        }
    }
    content.push('\n');
}

pub(crate) fn append_best_build_section(content: &mut String, best_build: &[Item]) {
    content.push_str("## Best Build\n");
    content.push_str(&format!("- {}\n\n", item_names(best_build)));
}

pub(crate) fn append_end_stats_section(
    content: &mut String,
    controlled_champion_name: &str,
    controlled_champion_end_stats: &Stats,
) {
    content.push_str(&format!(
        "## {} End Stats (Best Build)\n",
        controlled_champion_name
    ));
    content.push_str(&format!(
        "- HP: {}, Armor: {}, MR: {}, AP: {}, AD: {}, Ability Haste: {}, Move Speed (flat bonus): {}, Move Speed (% bonus): {}\n\n",
        format_f64_with_commas(controlled_champion_end_stats.health, 1),
        format_f64_with_commas(controlled_champion_end_stats.armor, 1),
        format_f64_with_commas(controlled_champion_end_stats.magic_resist, 1),
        format_f64_with_commas(controlled_champion_end_stats.ability_power, 1),
        format_f64_with_commas(controlled_champion_end_stats.attack_damage, 1),
        format_f64_with_commas(controlled_champion_end_stats.ability_haste, 1),
        format_f64_with_commas(controlled_champion_end_stats.move_speed_flat, 1),
        format_f64_with_commas(controlled_champion_end_stats.move_speed_percent, 1)
    ));
}

pub(crate) fn append_stack_overrides_section(content: &mut String, stack_notes: &[String]) {
    content.push_str("## Stack Overrides\n");
    if stack_notes.is_empty() {
        content
            .push_str("- No explicit stack overrides triggered for selected best build items.\n\n");
        return;
    }
    for note in stack_notes {
        content.push_str(&format!("- {}\n", note));
    }
    content.push('\n');
}
