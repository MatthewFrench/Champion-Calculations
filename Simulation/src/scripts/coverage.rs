use std::collections::HashSet;

use crate::{Item, to_norm_key};

pub(crate) const MODELED_RUNTIME_ITEM_EFFECT_KEYS: &[&str] = &[
    "bladeoftheruinedking",
    "guardianangel",
    "guinsoosrageblade",
    "heartsteel",
    "krakenslayer",
    "liandrystorment",
    "ludensecho",
    "ludenscompanion",
    "protoplasmharness",
    "zhonyashourglass",
];

#[allow(dead_code)]
pub(crate) const MODELED_CONTROLLED_CHAMPION_SCRIPT_KEYS: &[&str] = &["vladimir"];
#[allow(dead_code)]
pub(crate) const MODELED_ENEMY_SCRIPT_EVENT_CHAMPION_KEYS: &[&str] =
    &["drmundo", "morgana", "sona", "vayne", "warwick"];

#[allow(dead_code)]
pub(crate) fn modeled_dynamic_rune_effect_keys() -> &'static [&'static str] {
    crate::scripts::runes::effects::DYNAMIC_RUNE_KEYS
}

pub(crate) fn has_modeled_runtime_item_effect(item_name: &str) -> bool {
    let key = to_norm_key(item_name);
    MODELED_RUNTIME_ITEM_EFFECT_KEYS.contains(&key.as_str())
}

pub(crate) fn item_has_nontrivial_effect_data(item: &Item) -> bool {
    !item.passive_effects_text.is_empty()
        || item.has_active_effect
        || item.structured_effect_count > 0
}

pub(crate) fn is_item_effect_unmodeled(item: &Item) -> bool {
    item_has_nontrivial_effect_data(item) && !has_modeled_runtime_item_effect(&item.name)
}

pub(crate) fn unmodeled_runtime_item_effect_names(items: &[Item]) -> Vec<String> {
    let mut names = items
        .iter()
        .filter(|item| is_item_effect_unmodeled(item))
        .map(|item| item.name.clone())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    names.sort();
    names
}

#[cfg(test)]
#[path = "tests/coverage_tests.rs"]
mod tests;
