use super::*;

fn is_rune_name_in_slot(slot_runes: &[String], rune_name: &str) -> bool {
    let key = to_norm_key(rune_name);
    slot_runes.iter().any(|r| to_norm_key(r) == key)
}

fn is_shard_name_in_slot(slot_shards: &[String], shard_name: &str) -> bool {
    let key = to_norm_key(shard_name);
    slot_shards.iter().any(|s| to_norm_key(s) == key)
}

pub(crate) fn is_legal_rune_page_selection(
    selection: &LoadoutSelection,
    loadout_domain: &LoadoutDomain,
) -> bool {
    if selection.rune_names.len() != 6 || selection.shard_stats.len() != 3 {
        return false;
    }
    if loadout_domain.rune_paths.len() < 2 {
        return false;
    }
    for (idx, shard_name) in selection.shard_stats.iter().enumerate() {
        let valid = loadout_domain
            .shard_slots
            .get(idx)
            .map(|slot| is_shard_name_in_slot(slot, shard_name))
            .unwrap_or(false);
        if !valid {
            return false;
        }
    }

    let primary_keystone = &selection.rune_names[0];
    let primary_minor_1 = &selection.rune_names[1];
    let primary_minor_2 = &selection.rune_names[2];
    let primary_minor_3 = &selection.rune_names[3];
    let secondary_minor_1 = &selection.rune_names[4];
    let secondary_minor_2 = &selection.rune_names[5];

    for (primary_idx, primary_path) in loadout_domain.rune_paths.iter().enumerate() {
        if primary_path.slot_runes.len() < 4 {
            continue;
        }
        if !is_rune_name_in_slot(&primary_path.slot_runes[0], primary_keystone)
            || !is_rune_name_in_slot(&primary_path.slot_runes[1], primary_minor_1)
            || !is_rune_name_in_slot(&primary_path.slot_runes[2], primary_minor_2)
            || !is_rune_name_in_slot(&primary_path.slot_runes[3], primary_minor_3)
        {
            continue;
        }

        for (secondary_idx, secondary_path) in loadout_domain.rune_paths.iter().enumerate() {
            if secondary_idx == primary_idx || secondary_path.slot_runes.len() < 4 {
                continue;
            }
            let secondary_minor_1_slot = (1..=3).find(|slot| {
                is_rune_name_in_slot(&secondary_path.slot_runes[*slot], secondary_minor_1)
            });
            let secondary_minor_2_slot = (1..=3).find(|slot| {
                is_rune_name_in_slot(&secondary_path.slot_runes[*slot], secondary_minor_2)
            });
            if let (Some(slot_a), Some(slot_b)) = (secondary_minor_1_slot, secondary_minor_2_slot)
                && slot_a < slot_b
            {
                return true;
            }
        }
    }

    false
}

pub(crate) fn validate_rune_page_selection(
    selection: &LoadoutSelection,
    loadout_domain: &LoadoutDomain,
) -> Result<()> {
    if is_legal_rune_page_selection(selection, loadout_domain) {
        return Ok(());
    }
    bail!(
        "Invalid rune page selection. Provide ordered runes_reforged.rune_names with six runes [primary keystone, primary slot2, primary slot3, primary slot4, secondary slot2/3/4 rune A, secondary higher-slot rune B], plus ordered shard_stats for the three shard slots."
    );
}
