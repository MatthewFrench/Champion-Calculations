use super::*;

pub(crate) fn item_pool_from_names(
    items: &HashMap<String, Item>,
    names: &[String],
) -> Result<Vec<Item>> {
    let mut out = Vec::new();
    for name in names {
        let resolved = resolve_evolved_item_name(items, name);
        out.push(
            items
                .get(&resolved)
                .cloned()
                .ok_or_else(|| anyhow!("Item not found: {}", resolved))?,
        );
    }
    Ok(out)
}

fn resolve_evolved_item_name(items: &HashMap<String, Item>, name: &str) -> String {
    for (source, evolved) in ITEM_EVOLUTION_REPLACEMENTS {
        if *source == name && items.contains_key(*evolved) {
            return (*evolved).to_string();
        }
    }
    name.to_string()
}

fn is_legendary(item: &Item) -> bool {
    item.rank.iter().any(|rank| rank == LEGENDARY_RANK)
}

fn is_pre_evolution_item(items: &HashMap<String, Item>, item_name: &str) -> bool {
    ITEM_EVOLUTION_REPLACEMENTS
        .iter()
        .any(|(source, evolved)| *source == item_name && items.contains_key(*evolved))
}

fn is_evolution_target(item_name: &str) -> bool {
    ITEM_EVOLUTION_REPLACEMENTS
        .iter()
        .any(|(_, evolved)| *evolved == item_name)
}

fn looks_arena_or_non_summoners_rift(item: &Item) -> bool {
    let lower = item.name.to_ascii_lowercase();
    let arena_like_tokens = [
        "dragonheart",
        "hemomancer",
        "runecarver",
        "gambler",
        "golden spatula",
        "black hole gauntlet",
        "reaper",
        "demon king",
        "pyromancer",
        "molten stone",
        "wooglet",
        "entropy",
        "decapitator",
        "regicide",
        "lucky dice",
    ];
    arena_like_tokens.iter().any(|token| lower.contains(token))
}

pub(crate) fn item_is_allowed_in_urf(item_name: &str, urf: &UrfBuffs) -> bool {
    let key = to_norm_key(item_name);
    let mapped = match key.as_str() {
        "ludensecho" => "ludenscompanion",
        _ => key.as_str(),
    };
    urf.allowed_item_keys.contains(mapped)
}

pub(crate) fn default_item_pool(items: &HashMap<String, Item>, urf: &UrfBuffs) -> Vec<Item> {
    let mut pool = items
        .values()
        .filter(|item| item.shop_purchasable || is_evolution_target(&item.name))
        .filter(|item| is_legendary(item))
        .filter(|item| !is_pre_evolution_item(items, &item.name))
        .filter(|item| !looks_arena_or_non_summoners_rift(item))
        .filter(|item| item_is_allowed_in_urf(&item.name, urf))
        .cloned()
        .collect::<Vec<_>>();
    pool.sort_by(|a, b| a.name.cmp(&b.name));
    pool
}
