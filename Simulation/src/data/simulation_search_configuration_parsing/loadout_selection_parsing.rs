use super::*;

pub(crate) fn parse_loadout_selection(data: Option<&Value>) -> Result<LoadoutSelection> {
    let mut out = LoadoutSelection::default();
    let Some(obj) = data.and_then(Value::as_object) else {
        return Ok(out);
    };
    if obj.get("season2016_masteries").is_some() {
        bail!("loadout.season2016_masteries is no longer supported. Use loadout.runes_reforged.");
    }

    if let Some(runes_obj) = obj.get("runes_reforged").and_then(Value::as_object) {
        if runes_obj.get("rune_ids").is_some() {
            bail!(
                "loadout.runes_reforged.rune_ids is no longer supported. Use loadout.runes_reforged.rune_names."
            );
        }
        if let Some(names) = runes_obj.get("rune_names").and_then(Value::as_array) {
            out.rune_names = names
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect();
        }
        if let Some(shards) = runes_obj.get("shard_stats").and_then(Value::as_array) {
            out.shard_stats = shards
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect();
        }
    }
    Ok(out)
}

pub(crate) fn loadout_selection_key(sel: &LoadoutSelection) -> String {
    format!(
        "r={}|s={}",
        sel.rune_names.join(","),
        sel.shard_stats.join(",")
    )
}
