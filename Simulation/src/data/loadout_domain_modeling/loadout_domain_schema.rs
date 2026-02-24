use super::*;

#[derive(Debug, Clone)]
pub(crate) struct RunePathDomain {
    pub(crate) slot_runes: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub(crate) struct LoadoutDomain {
    pub(crate) rune_paths: Vec<RunePathDomain>,
    pub(crate) shard_slots: [Vec<String>; 3],
}

pub(crate) fn build_loadout_domain() -> LoadoutDomain {
    let runes_data = load_json(&rune_data_dir().join("RunesReforged.json")).unwrap_or(Value::Null);

    let rune_paths = runes_data
        .get("paths")
        .and_then(Value::as_array)
        .map(|paths| {
            paths
                .iter()
                .filter_map(|path| {
                    let slots = path.get("slots").and_then(Value::as_array)?;
                    let slot_runes = slots
                        .iter()
                        .map(|slot| {
                            slot.get("runes")
                                .and_then(Value::as_array)
                                .map(|runes| {
                                    runes
                                        .iter()
                                        .filter_map(|r| r.get("name").and_then(Value::as_str))
                                        .map(ToOwned::to_owned)
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_default()
                        })
                        .collect::<Vec<_>>();
                    if slot_runes.len() >= 4 {
                        Some(RunePathDomain { slot_runes })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let shard_slots = {
        let slots = runes_data
            .get("stat_shards")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let read_slot = |idx: usize| -> Vec<String> {
            slots
                .get(idx)
                .and_then(|s| s.get("options"))
                .and_then(Value::as_array)
                .map(|options| {
                    options
                        .iter()
                        .filter_map(|o| o.get("stat").and_then(Value::as_str))
                        .map(ToOwned::to_owned)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        };
        [read_slot(0), read_slot(1), read_slot(2)]
    };

    LoadoutDomain {
        rune_paths,
        shard_slots,
    }
}
