use crate::to_norm_key;

pub(crate) const DYNAMIC_RUNE_KEYS: &[&str] = &[
    "graspoftheundying",
    "lethaltempo",
    "presstheattack",
    "fleetfootwork",
    "conqueror",
    "aftershock",
    "electrocute",
    "firststrike",
    "phaserush",
    "arcanecomet",
    "summonaery",
    "hailofblades",
    "darkharvest",
    "secondwind",
    "triumph",
    "gatheringstorm",
    "cheapshot",
    "scorch",
    "tasteofblood",
    "absorblife",
    "coupdegrace",
];

pub(crate) fn has_dynamic_rune_effect(rune_name: &str) -> bool {
    let key = to_norm_key(rune_name);
    DYNAMIC_RUNE_KEYS.contains(&key.as_str())
}
