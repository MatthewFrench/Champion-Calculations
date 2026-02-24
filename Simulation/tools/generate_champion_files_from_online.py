#!/usr/bin/env python3
"""Generate canonical Characters/<Champion>.json files from From Online champion data.

This tool is data-authoring only. It does not modify runtime code.
"""

from __future__ import annotations

import argparse
import json
import math
import re
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Tuple

ROOT = Path(__file__).resolve().parents[2]
FROM_ONLINE_DIR = ROOT / "From Online" / "champions"
CHARACTERS_DIR = ROOT / "Characters"

DEFAULT_DATA_VERSION = "16.3.1"
GENERATED_DATE = "2026-02-24"
TEMPLATE_VERSION = "1.0.0"

ABILITY_KEY_MAP = {
    "passive": "P",
    "basic_ability_1": "Q",
    "basic_ability_2": "W",
    "basic_ability_3": "E",
    "ultimate": "R",
}


def read_json(path: Path) -> Dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Dict[str, Any]) -> None:
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def slugify(value: str) -> str:
    token = re.sub(r"[^a-zA-Z0-9]+", "_", value.strip().lower()).strip("_")
    return token or "effect"


def first_sentence(text: str) -> str:
    compact = " ".join(text.split())
    if not compact:
        return ""
    match = re.search(r"[.!?]", compact)
    if not match:
        return compact[:260]
    return compact[: match.end()].strip()


def parse_numbers(value: Any) -> List[float]:
    if value is None:
        return []
    if isinstance(value, (int, float)):
        return [float(value)]
    if isinstance(value, str):
        lowered = value.strip().lower()
        if lowered in {"none", "auto", "passive", "instant", "null", ""}:
            return []
        return [float(token) for token in re.findall(r"-?\d+(?:\.\d+)?", value)]
    return []


def parse_cast_time_seconds(value: Any) -> Optional[float]:
    if value is None:
        return None
    if isinstance(value, (int, float)):
        return float(value)
    if isinstance(value, str):
        lowered = value.strip().lower()
        if lowered in {"none", "auto", "instant"}:
            return 0.0
        nums = parse_numbers(value)
        if nums:
            return nums[0]
    return None


def parse_effect_radius(value: Any) -> Optional[float]:
    nums = [n for n in parse_numbers(value) if n > 0]
    if not nums:
        return None
    # Prefer smallest positive value as hitbox footprint for mixed strings like "1000 / 400".
    return min(nums)


def infer_data_version(champion: Dict[str, Any]) -> str:
    icon = champion.get("icon")
    if isinstance(icon, str):
        match = re.search(r"/cdn/([0-9]+\.[0-9]+\.[0-9]+)/", icon)
        if match:
            return match.group(1)
    release_patch = champion.get("releasePatch")
    if isinstance(release_patch, str) and release_patch.strip():
        return release_patch.strip()
    return DEFAULT_DATA_VERSION


def title_case_role(role: str) -> str:
    return " ".join(part.capitalize() for part in role.lower().split("_"))


def get_stat(stats: Dict[str, Any], key: str) -> Tuple[float, float]:
    value = stats.get(key)
    if isinstance(value, dict):
        flat = float(value.get("flat", 0) or 0)
        per_level = float(value.get("perLevel", 0) or 0)
        return flat, per_level
    return 0.0, 0.0


def resource_keys(resource_type: str) -> Tuple[Optional[str], Optional[str]]:
    mapping = {
        "MANA": ("mana", "manaRegen"),
        "ENERGY": ("energy", "energyRegen"),
    }
    return mapping.get(resource_type, (None, None))


def normalize_resource_type(resource_type: str) -> str:
    return resource_type.strip().lower() if resource_type else "none"


def build_base_stats(champion: Dict[str, Any]) -> Dict[str, Any]:
    stats = champion.get("stats", {})
    resource_type = str(champion.get("resource", "NONE") or "NONE").upper()

    health_base, health_per = get_stat(stats, "health")
    health_regen_base, health_regen_per = get_stat(stats, "healthRegen")
    armor_base, armor_per = get_stat(stats, "armor")
    mr_base, mr_per = get_stat(stats, "magicResistance")
    ad_base, ad_per = get_stat(stats, "attackDamage")
    atk_speed_base, atk_speed_per = get_stat(stats, "attackSpeed")

    move_speed = float(stats.get("movespeed", {}).get("flat", 0) or 0)
    attack_range = float(stats.get("attackRange", {}).get("flat", 0) or 0)

    resource_stat_key, resource_regen_stat_key = resource_keys(resource_type)
    res_base, res_per = get_stat(stats, resource_stat_key) if resource_stat_key else (0.0, 0.0)
    res_regen_base, res_regen_per = (
        get_stat(stats, resource_regen_stat_key) if resource_regen_stat_key else (0.0, 0.0)
    )

    resource = {
        "type": normalize_resource_type(resource_type),
        "base": res_base,
        "per_level": res_per,
    }
    if resource_type in {"NONE", "HEALTH"}:
        resource["context_notes"] = (
            "Champion does not use mana/energy as a standard regenerating resource for ability costs."
        )

    return {
        "health": {"base": health_base, "per_level": health_per},
        "resource": resource,
        "move_speed": move_speed,
        "armor": {"base": armor_base, "per_level": armor_per},
        "magic_resist": {"base": mr_base, "per_level": mr_per},
        "attack_range": attack_range,
        "health_regeneration": {"base": health_regen_base, "per_level": health_regen_per},
        "resource_regeneration": {
            "base": res_regen_base,
            "per_level": res_regen_per,
        },
        "crit_chance": {"base": 0.0, "per_level": 0.0},
        "attack_damage": {"base": ad_base, "per_level": ad_per},
        "attack_speed": {
            "base": atk_speed_base,
            "per_level_percent": atk_speed_per,
        },
    }


def build_basic_attack(champion: Dict[str, Any]) -> Dict[str, Any]:
    stats = champion.get("stats", {})
    attack_type = str(champion.get("attackType", "RANGED") or "RANGED").lower()
    is_projectile = attack_type == "ranged"

    attack_total_time = float(stats.get("attackTotalTime", {}).get("flat", 0) or 0)
    attack_cast_time = float(stats.get("attackCastTime", {}).get("flat", 0) or 0)
    attack_delay_offset = float(stats.get("attackDelayOffset", {}).get("flat", 0) or 0)
    gameplay_radius = float(stats.get("gameplayRadius", {}).get("flat", 0) or 0)

    if attack_total_time <= 0:
        attack_speed_base = float(stats.get("attackSpeed", {}).get("flat", 0) or 0)
        if attack_speed_base > 0:
            attack_total_time = 1.0 / attack_speed_base
        else:
            attack_total_time = 1.0
    if attack_cast_time <= 0:
        attack_cast_time = max(0.05, attack_total_time * 0.3)

    missile_speed = 2000.0 if is_projectile else 0.0
    windup_fraction = attack_cast_time / attack_total_time if attack_total_time > 0 else 0.3

    notes = []
    if is_projectile:
        notes.append(
            "Missile speed uses canonical ranged fallback (2000) because source champion payload omits basic-attack missile speed."
        )

    return {
        "attack_type": attack_type,
        "is_projectile": is_projectile,
        "base_attack_period_seconds": attack_total_time,
        "base_windup_seconds": attack_cast_time,
        "windup_fraction": windup_fraction,
        "windup_percent": windup_fraction * 100.0,
        "missile_speed": missile_speed,
        "missile_travel_time_seconds_per_unit": (1.0 / missile_speed) if missile_speed > 0 else 0.0,
        "raw_timing_stats": {
            "attack_cast_time": attack_cast_time,
            "attack_total_time": attack_total_time,
            "attack_delay_offset": attack_delay_offset,
            "gameplay_radius": gameplay_radius,
        },
        "calculation_notes": (
            " ".join(notes)
            if notes
            else "Basic-attack timing values are sourced from From Online stats payload."
        ),
    }


def extract_values(modifier_block: Any) -> List[float]:
    if not isinstance(modifier_block, dict):
        return []
    values = modifier_block.get("values")
    if not isinstance(values, list):
        return []
    out: List[float] = []
    for raw in values:
        if isinstance(raw, (int, float)):
            out.append(float(raw))
        elif isinstance(raw, str):
            try:
                out.append(float(raw))
            except ValueError:
                pass
    return out


def extract_rank_values(container: Any) -> List[float]:
    if not isinstance(container, dict):
        return []
    modifiers = container.get("modifiers")
    if not isinstance(modifiers, list):
        return []
    for block in modifiers:
        values = extract_values(block)
        if values:
            return values
    return []


def build_effects(raw_effects: Any, ability_slug: str) -> List[Dict[str, Any]]:
    if not isinstance(raw_effects, list):
        return []

    output: List[Dict[str, Any]] = []
    used_ids: set[str] = set()

    def next_id(base: str) -> str:
        candidate = base
        index = 2
        while candidate in used_ids:
            candidate = f"{base}_{index}"
            index += 1
        used_ids.add(candidate)
        return candidate

    for effect_index, effect in enumerate(raw_effects):
        if not isinstance(effect, dict):
            continue
        effect_description = str(effect.get("description", "") or "").strip()
        leveling = effect.get("leveling")

        if isinstance(leveling, list) and leveling:
            for level_index, entry in enumerate(leveling):
                if not isinstance(entry, dict):
                    continue
                attribute = str(entry.get("attribute", "") or "").strip()
                base_id = slugify(attribute) if attribute else f"{ability_slug}_effect_{effect_index+1}_{level_index+1}"
                effect_out: Dict[str, Any] = {
                    "id": next_id(base_id),
                    "description": attribute or f"{ability_slug} effect",
                }

                modifiers = entry.get("modifiers")
                values: List[float] = []
                if isinstance(modifiers, list):
                    for modifier in modifiers:
                        values = extract_values(modifier)
                        if values:
                            break
                if values:
                    effect_out["base_by_rank"] = values

                if effect_description and effect_description != effect_out["description"]:
                    effect_out["context_notes"] = first_sentence(effect_description)
                output.append(effect_out)
        else:
            base_id = f"{ability_slug}_effect_{effect_index+1}"
            effect_out = {
                "id": next_id(base_id),
                "description": effect_description or f"{ability_slug} effect detail",
            }
            output.append(effect_out)

    return output


def build_ability(
    champion_name: str,
    champion_key: str,
    canonical_key: str,
    source_entries: Any,
) -> Dict[str, Any]:
    source_slot = ABILITY_KEY_MAP[canonical_key]
    first = source_entries[0] if isinstance(source_entries, list) and source_entries else {}
    if not isinstance(first, dict):
        first = {}

    targeting = str(first.get("targeting", "") or "")
    type_value = "Passive" if canonical_key == "passive" or targeting.lower() == "passive" else "Active"

    ability: Dict[str, Any] = {
        "name": str(first.get("name", source_slot) or source_slot),
        "type": type_value,
    }

    if canonical_key != "passive":
        ability["slot"] = source_slot
        ability["default_keybinding"] = source_slot

    blurb = str(first.get("blurb", "") or "").strip()
    if blurb:
        ability["description_source"] = blurb

    effects = build_effects(first.get("effects"), f"{champion_key.lower()}_{source_slot.lower()}")
    if effects:
        ability["effects"] = effects

    primary_description = ""
    if isinstance(first.get("effects"), list) and first["effects"]:
        raw_effect_0 = first["effects"][0]
        if isinstance(raw_effect_0, dict):
            primary_description = str(raw_effect_0.get("description", "") or "").strip()
    ability["description"] = primary_description or blurb or f"{champion_name} {ability['name']}"

    cooldown_values = extract_rank_values(first.get("cooldown"))
    if cooldown_values:
        ability["cooldown_seconds_by_rank"] = cooldown_values

    cost_values = extract_rank_values(first.get("cost"))
    resource = str(first.get("resource", "") or "").strip().lower()
    if cost_values:
        cost_obj: Dict[str, Any] = {
            "resource": resource if resource else "none",
            "value_by_rank": cost_values,
            "value_type": "flat",
        }
        ability["cost"] = cost_obj

    context_notes: List[str] = []
    cast_time_raw = first.get("castTime")
    speed_raw = first.get("speed")
    effect_radius_raw = first.get("effectRadius")

    execution: Dict[str, float] = {}
    if type_value == "Active":
        cast_seconds = parse_cast_time_seconds(cast_time_raw)
        if cast_seconds is not None:
            execution["cast_windup_seconds"] = cast_seconds

        speed_values = parse_numbers(speed_raw)
        if speed_values:
            execution["projectile_speed"] = speed_values[0]

        effect_radius = parse_effect_radius(effect_radius_raw)
        if effect_radius is not None:
            execution["effect_hitbox_radius"] = effect_radius

        if execution:
            ability["execution"] = execution

    if targeting:
        context_notes.append(f"Source targeting: {targeting}.")
    if cast_time_raw is not None:
        context_notes.append(f"Source castTime: {cast_time_raw}.")
    if speed_raw is not None:
        context_notes.append(f"Source speed: {speed_raw}.")
    if effect_radius_raw is not None:
        context_notes.append(f"Source effectRadius: {effect_radius_raw}.")

    notes_raw = str(first.get("notes", "") or "").strip()
    if notes_raw:
        context_notes.append(f"Source notes (summary): {first_sentence(notes_raw)}")

    if context_notes:
        ability["context_notes"] = context_notes

    return ability


def champion_payload(source_path: Path) -> Dict[str, Any]:
    champion = read_json(source_path)
    champion_key = source_path.stem
    champion_name = str(champion.get("name", champion_key) or champion_key)

    abilities_obj = champion.get("abilities", {})
    if not isinstance(abilities_obj, dict):
        abilities_obj = {}

    abilities: Dict[str, Any] = {}
    for canonical_key, source_slot in ABILITY_KEY_MAP.items():
        source_entries = abilities_obj.get(source_slot, [])
        abilities[canonical_key] = build_ability(
            champion_name,
            champion_key,
            canonical_key,
            source_entries,
        )

    payload = {
        "name": champion_name,
        "data_version": infer_data_version(champion),
        "role_tags": [title_case_role(str(role)) for role in champion.get("roles", []) if role],
        "base_stats": build_base_stats(champion),
        "basic_attack": build_basic_attack(champion),
        "abilities": abilities,
        "sources": [
            {
                "type": "Generated champion data",
                "path": f"From Online/champions/{champion_key}.json",
                "accessed": GENERATED_DATE,
                "used_for": "canonical champion baseline generation for stats, abilities, and execution metadata",
            }
        ],
        "notes": [
            "Generated from From Online champion payload as part of champion corpus parity coverage.",
            "Ability execution metadata is source-derived from castTime/speed/effectRadius when present.",
            "This baseline should be manually refined for high-impact simulation nuances before runtime scripting expansion.",
        ],
        "schema_notes": {
            "base_by_rank": "Arrays indexed by ability rank (1-based in game, 0-based in data).",
            "base_by_champion_level": "Arrays indexed by champion level minus one.",
            "formula": "Formula objects use explicit op/terms/output_stat when reliable source math is available.",
            "ratio_type": "Common ratio types: total_ad, bonus_ad, ap, target_max_health, target_missing_health.",
            "value_percent": "Percent values are represented as whole percentages unless noted otherwise.",
            "basic_attack": "Basic attack timing and projectile defaults are source-derived with documented fallbacks.",
            "basic_attack_raw_timing_stats": "Raw timing stats preserve source payload fields for auditability.",
        },
        "simulation": {
            "status": "data_only_unscripted",
            "template_version": TEMPLATE_VERSION,
            "generated_from_source_key": champion_key,
        },
    }
    return payload


def existing_character_keys() -> set[str]:
    keys: set[str] = set()
    for path in CHARACTERS_DIR.glob("*.json"):
        if path.stem == "ChampionDefaults":
            continue
        keys.add(path.stem)
    return keys


def champion_source_keys() -> List[str]:
    return sorted(path.stem for path in FROM_ONLINE_DIR.glob("*.json"))


def determine_targets(args: argparse.Namespace) -> List[str]:
    source_keys = champion_source_keys()
    existing = existing_character_keys()

    if args.champions:
        requested = [name.strip() for name in args.champions.split(",") if name.strip()]
        unknown = [name for name in requested if name not in source_keys]
        if unknown:
            raise ValueError(f"Unknown champion keys: {', '.join(unknown)}")
        return requested

    if args.all_missing:
        return [key for key in source_keys if key not in existing]

    limit = max(args.limit, 0)
    missing = [key for key in source_keys if key not in existing]
    return missing[:limit]


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate canonical champion files from From Online")
    parser.add_argument(
        "--all-missing",
        action="store_true",
        help="Generate files for every missing champion key",
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=10,
        help="Generate at most N missing champions when --all-missing is not used (default: 10)",
    )
    parser.add_argument(
        "--champions",
        type=str,
        default="",
        help="Comma-separated explicit champion keys to generate",
    )
    parser.add_argument(
        "--overwrite",
        action="store_true",
        help="Overwrite existing canonical champion files",
    )
    args = parser.parse_args()

    targets = determine_targets(args)
    if not targets:
        print("No target champions selected.")
        return 0

    generated = 0
    skipped = 0
    for key in targets:
        source_path = FROM_ONLINE_DIR / f"{key}.json"
        target_path = CHARACTERS_DIR / f"{key}.json"
        if target_path.exists() and not args.overwrite:
            skipped += 1
            continue
        payload = champion_payload(source_path)
        write_json(target_path, payload)
        generated += 1

    print(
        f"Generated {generated} champion files"
        + (f", skipped {skipped} existing files" if skipped else "")
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
