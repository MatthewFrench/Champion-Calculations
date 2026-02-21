#!/usr/bin/env python3
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE_DIR = ROOT / "From Online" / "champions"
TARGET_DIR = ROOT / "Characters"

EXISTING_CANONICAL = {
    "DrMundo.json",
    "Morgana.json",
    "Sona.json",
    "Vayne.json",
    "Vladimir.json",
    "Warwick.json",
}


def to_numeric_list(value):
    if isinstance(value, list):
        out=[]
        for entry in value:
            if isinstance(entry,(int,float)):
                out.append(float(entry))
        return out
    if isinstance(value, dict):
        for candidate in ("values", "byRank", "rankValues"):
            if isinstance(value.get(candidate), list):
                return [float(entry) for entry in value[candidate] if isinstance(entry,(int,float))]
    return []

def stat(stats: dict, key: str, default=0.0):
    value = stats.get(key, default)
    if isinstance(value, dict):
        for candidate in ("flat", "base", "value"):
            if candidate in value and isinstance(value[candidate], (int, float)):
                return float(value[candidate])
        return float(default)
    return float(value) if isinstance(value, (int, float)) else float(default)

def ability_entry(slot_key: str, payload: dict, slot_label: str, keybinding: str):
    cooldown_by_rank = to_numeric_list(payload.get("cooldown")) or [0.0]
    cost_by_rank = to_numeric_list(payload.get("cost")) or [0.0]
    ability_range = payload.get("targeting") or payload.get("castRange") or 0.0
    if isinstance(ability_range, list):
        ability_range = ability_range[-1] if ability_range else 0.0

    ability_range_numeric = float(ability_range) if isinstance(ability_range, (int, float)) else 0.0

    return {
        "name": payload.get("name", slot_key),
        "type": "Active",
        "slot": slot_label,
        "default_keybinding": keybinding,
        "description": payload.get("blurb") or payload.get("notes") or "",
        "description_source": payload.get("notes") or payload.get("blurb") or "",
        "cooldown_seconds_by_rank": cooldown_by_rank,
        "range": ability_range_numeric,
        "cost": {
            "resource": (payload.get("resource") or "none").lower(),
            "value_type": "flat",
            "value_by_rank": cost_by_rank,
        },
        "effects": [],
    }


def build_champion_file(source: dict):
    stats = source.get("stats", {})
    attack_type = (source.get("attackType") or "RANGED").lower()
    abilities = source.get("abilities", {})
    passive_payload = (abilities.get("P") or [{}])[0]

    file_data = {
        "data_version": "1.0.0",
        "name": source.get("name", source.get("id", "Unknown")),
        "sources": {
            "canonical": "From Online/champions",
            "import_notes": "Generated baseline canonical champion file from From Online dataset.",
        },
        "notes": [
            "Baseline imported champion data file.",
            "Ability effects are intentionally minimal placeholders until scripted modeling is added.",
        ],
        "schema_notes": {
            "source_of_truth": "From Online/champions/<Champion>.json",
            "generated_by": "Characters/import_from_online.py",
        },
        "role_tags": source.get("roles", []),
        "basic_attack": {
            "attack_type": "melee" if attack_type == "melee" else "ranged",
            "missile_speed": stat(stats, "attackMissileSpeed", 0.0),
            "raw_timing_stats": {
                "base_windup_seconds": stat(stats, "attackCastTime", 0.0),
                "total_attack_time_seconds": stat(stats, "attackTotalTime", 0.0),
                "gameplay_radius": stat(stats, "gameplayRadius", 0.0),
            },
        },
        "base_stats": {
            "health": {
                "base": stat(stats, "health", 0.0),
                "per_level": stat(stats, "healthPerLevel", 0.0),
            },
            "health_regeneration": {
                "base": stat(stats, "healthRegen", 0.0),
                "per_level": stat(stats, "healthRegenPerLevel", 0.0),
            },
            "resource": {
                "base": stat(stats, "mana", 0.0),
                "per_level": stat(stats, "manaPerLevel", 0.0),
            },
            "resource_regeneration": {
                "base": stat(stats, "manaRegen", 0.0),
                "per_level": stat(stats, "manaRegenPerLevel", 0.0),
            },
            "armor": {
                "base": stat(stats, "armor", 0.0),
                "per_level": stat(stats, "armorPerLevel", 0.0),
            },
            "magic_resist": {
                "base": stat(stats, "magicResistance", 0.0),
                "per_level": stat(stats, "magicResistancePerLevel", 0.0),
            },
            "attack_damage": {
                "base": stat(stats, "attackDamage", 0.0),
                "per_level": stat(stats, "attackDamagePerLevel", 0.0),
            },
            "attack_speed": {
                "base": stat(stats, "attackSpeed", 0.0),
                "per_level_percent": stat(stats, "attackSpeedPerLevel", 0.0),
            },
            "attack_range": stat(stats, "attackRange", 0.0),
            "move_speed": stat(stats, "movespeed", 0.0),
            "crit_chance": {
                "base": 0.0,
                "per_level": 0.0,
            },
        },
        "abilities": {
            "passive": {
                "name": passive_payload.get("name", "Passive"),
                "type": "Passive",
                "description": passive_payload.get("blurb") or passive_payload.get("notes") or "",
                "description_source": passive_payload.get("notes") or passive_payload.get("blurb") or "",
                "effects": [],
            },
            "basic_ability_1": ability_entry("Q", (abilities.get("Q") or [{}])[0], "Q", "Q"),
            "basic_ability_2": ability_entry("W", (abilities.get("W") or [{}])[0], "W", "W"),
            "basic_ability_3": ability_entry("E", (abilities.get("E") or [{}])[0], "E", "E"),
            "ultimate": ability_entry("R", (abilities.get("R") or [{}])[0], "R", "R"),
        },
    }
    return file_data


def main():
    created = 0
    for path in sorted(SOURCE_DIR.glob("*.json")):
        if path.name in EXISTING_CANONICAL:
            continue
        target = TARGET_DIR / path.name
        if target.exists():
            continue
        source = json.loads(path.read_text())
        target.write_text(json.dumps(build_champion_file(source), indent=2) + "\n")
        created += 1
    print(f"created={created}")

if __name__ == "__main__":
    main()
