import json
import os
import re
from datetime import datetime, timezone

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
ITEMS_DIR = os.path.join(BASE_DIR, "items")
OUT_DIR = os.path.join(BASE_DIR, "items_normalized")
REPORT_PATH = os.path.join(BASE_DIR, "items_normalized_report.json")
SCHEMA_PATH = os.path.join(BASE_DIR, "items_schema.json")

NUMBER_RE = re.compile(r"-?\d+\.?\d*")
RANGE_RE = re.compile(r"(\d+\.?\d*)\s*[\-–]\s*(\d+\.?\d*)")
DURATION_RE = re.compile(r"for\s+(\d+\.?\d*)\s*(seconds|second|s)", re.I)
COOLDOWN_RE = re.compile(r"cooldown:?\s*(\d+\.?\d*)\s*(seconds|second|s)", re.I)
INTERVAL_RE = re.compile(r"every\s+(\d+\.?\d*)\s*(seconds|second|s)", re.I)
WITHIN_UNITS_RE = re.compile(r"within\s+(\d+\.?\d*)\s*units", re.I)
STACKS_RE = re.compile(r"stack(?:ing)?\s+up\s+to\s+(\d+)", re.I)
PERCENT_STAT_RE = re.compile(r"(\d+\.?\d*)%\s*(bonus|base|maximum)?\s*(health|hp|armor|magic resistance|mr|attack damage|ad|ability power|ap)", re.I)

STAT_MAP = {
    "attack damage": "attack_damage",
    "ability power": "ability_power",
    "armor": "armor",
    "magic resist": "magic_resistance",
    "magic resistance": "magic_resistance",
    "health": "health",
    "hp": "health",
    "health regen": "health_regen",
    "mana": "mana",
    "mana regen": "mana_regen",
    "attack speed": "attack_speed",
    "movement speed": "movement_speed",
    "ability haste": "ability_haste",
    "critical strike": "critical_strike",
    "crit": "critical_strike",
    "lifesteal": "lifesteal",
    "omnivamp": "omnivamp",
    "tenacity": "tenacity",
    "magic penetration": "magic_penetration",
    "armor penetration": "armor_penetration",
    "shield": "shield",
    "healing": "healing",
    "damage": "damage",
    "true damage": "true_damage",
    "magic damage": "magic_damage",
    "physical damage": "physical_damage",
    "gold": "gold",
    "experience": "experience",
}

STATUS_KEYWORDS = {
    "stasis": "stasis",
    "untargetable": "untargetable",
    "invulnerable": "invulnerable",
    "invisible": "invisible",
    "stealth": "stealth",
    "root": "root",
    "stun": "stun",
    "slow": "slow",
    "silence": "silence",
    "disarm": "disarm",
    "grounded": "grounded",
    "suppressed": "suppressed",
    "knockup": "knockup",
    "resurrection": "resurrection",
    "ghosted": "ghosted",
}

TRIGGERS = {
    "on hit": "on_hit",
    "on-hit": "on_hit",
    "on basic attack": "on_hit",
    "on ability": "on_ability",
    "on cast": "on_cast",
    "on damage": "on_damage",
    "on takedown": "on_kill",
    "on kill": "on_kill",
    "active": "active",
    "passive": "passive",
    "aura": "aura",
    "spellblade": "on_hit",
}

APPLIES = {
    "enemy champion": "enemy_champion",
    "enemy champions": "enemy_champion",
    "enemy": "enemy",
    "allies": "ally",
    "ally": "ally",
    "minion": "minion",
    "minions": "minion",
    "monster": "monster",
    "monsters": "monster",
    "structure": "structure",
    "turret": "structure",
    "tower": "structure",
    "self": "self",
    "you": "self",
    "nearby": "area",
}

SCHEMA = {
    "schema_name": "lol_item_effects_v1",
    "description": "Calculation-ready item schema with structured effects. Raw text preserved.",
    "effect_fields": {
        "effect_type": "damage|healing|shield|stat_modifier|status|resource|utility|summon|aura|proc|cooldown",
        "stat": "canonical stat key when applicable",
        "value": "numeric value if flat",
        "values": "array of numeric values if rank-based",
        "value_range": "[min,max] if range",
        "unit": "percent|seconds|health|gold|stacks|units|none",
        "duration_seconds": "duration if applicable",
        "cooldown_seconds": "cooldown if applicable",
        "interval_seconds": "interval for periodic effects",
        "trigger": "on_hit|on_cast|active|passive|on_kill|on_damage|on_ability|aura|toggle|unknown",
        "applies_to": "self|ally|enemy|enemy_champion|minion|monster|structure|area|unknown",
        "conditions": "array of conditions parsed from text",
        "status_effects": "array of status flags like stasis, untargetable, invulnerable",
        "scaling_terms": "array of scaling terms like 5% AP or 10% bonus health",
        "raw": "original text fragment",
        "parse_confidence": "0..1",
    },
    "canonical_stats": sorted(set(STAT_MAP.values())),
}

# Archetype shortcuts (item name -> effect overrides)
ARCHETYPE_OVERRIDES = {
    "Refillable Potion": [
        {
            "raw": "Holds charges that refill upon visiting the shop.",
            "effect_type": "resource",
            "stat": "charges",
            "trigger": "passive",
            "applies_to": "self",
            "conditions": ["refill_at_shop"],
            "parse_confidence": 0.9,
        }
    ],
    "Phantom Dancer": [
        {
            "raw": "Become permanently ghosted.",
            "effect_type": "status",
            "status_effects": ["ghosted"],
            "trigger": "passive",
            "applies_to": "self",
            "parse_confidence": 0.9,
        }
    ],
    "Quicksilver Sash": [
        {
            "raw": "Remove all crowd control debuffs (except airborne).",
            "effect_type": "utility",
            "status_effects": ["cleanse"],
            "trigger": "active",
            "applies_to": "self",
            "conditions": ["cc_cleanse"],
            "parse_confidence": 0.9,
        }
    ],
    "Ohmwrecker (Turret Item)": [
        {
            "raw": "Turret attacks cannot be dodged.",
            "effect_type": "utility",
            "stat": "turret_attack_undodgeable",
            "trigger": "passive",
            "applies_to": "structure",
            "parse_confidence": 0.9,
        }
    ],
}

# Archetype helpers
JUNGLE_COMPANION_KEYWORDS = ["hatchling", "seedling", "pup", "jungle companions"]
SUPPORT_QUEST_KEYWORDS = ["support quest", "quest", "upgrade this item", "ward", "charges refill"]
TETHER_KEYWORDS = ["tether", "worthy", "promise", "promised"]
WARD_ACTIVE_KEYWORDS = ["stealth ward", "vision", "sight", "ward"]


def split_effects(text: str):
    if not text:
        return []
    cleaned = text.replace("<br>", ". ").replace("<br/>", ". ").replace("<br />", ". ")
    cleaned = cleaned.replace("|", " ")
    parts = re.split(r"[.;]\s+", cleaned)
    return [p.strip() for p in parts if p.strip()]


def parse_sentence(raw: str):
    lower = raw.lower()

    values = [float(x) if "." in x else int(x) for x in NUMBER_RE.findall(raw)]
    value_range = None
    match = RANGE_RE.search(raw)
    if match:
        a = float(match.group(1)) if "." in match.group(1) else int(match.group(1))
        b = float(match.group(2)) if "." in match.group(2) else int(match.group(2))
        value_range = [a, b]

    unit = None
    if "%" in raw:
        unit = "percent"
    elif "second" in lower:
        unit = "seconds"

    duration_seconds = None
    m = DURATION_RE.search(raw)
    if m:
        duration_seconds = float(m.group(1))

    cooldown_seconds = None
    m = COOLDOWN_RE.search(raw)
    if m:
        cooldown_seconds = float(m.group(1))

    interval_seconds = None
    m = INTERVAL_RE.search(raw)
    if m:
        interval_seconds = float(m.group(1))

    stat = None
    for key, value in STAT_MAP.items():
        if key in lower:
            stat = value
            break

    trigger = None
    for key, value in TRIGGERS.items():
        if key in lower:
            trigger = value
            break

    applies_to = None
    for key, value in APPLIES.items():
        if key in lower:
            applies_to = value
            break

    status_effects = []
    for key, value in STATUS_KEYWORDS.items():
        if key in lower:
            status_effects.append(value)

    conditions = []
    if "below" in lower:
        conditions.append("threshold")
    if "while" in lower:
        conditions.append("while_condition")
    if "after" in lower:
        conditions.append("after_condition")
    if "in combat" in lower:
        conditions.append("in_combat")
    if WITHIN_UNITS_RE.search(raw):
        conditions.append("within_units")
    if "maximum stacks" in lower:
        conditions.append("at_max_stacks")

    scaling_terms = []
    for m in PERCENT_STAT_RE.finditer(raw):
        percent = float(m.group(1))
        qualifier = (m.group(2) or "").lower().strip()
        stat_word = (m.group(3) or "").lower().strip()
        stat_key = STAT_MAP.get(stat_word, stat_word)
        scaling_terms.append({
            "percent": percent,
            "stat": stat_key,
            "qualifier": qualifier or None,
        })

    effect_type = "stat_modifier"
    if "damage" in lower:
        effect_type = "damage"
    elif "heal" in lower:
        effect_type = "healing"
    elif "shield" in lower:
        effect_type = "shield"
    elif "gold" in lower:
        effect_type = "resource"
    elif status_effects:
        effect_type = "status"
    elif "dash" in lower or "teleport" in lower:
        effect_type = "utility"
    elif "ward" in lower:
        effect_type = "utility"

    confidence = 0.35
    if values:
        confidence += 0.2
    if stat:
        confidence += 0.2
    if trigger:
        confidence += 0.1
    if status_effects:
        confidence += 0.1
    if duration_seconds or cooldown_seconds or interval_seconds:
        confidence += 0.1
    if conditions:
        confidence += 0.05
    confidence = min(confidence, 1.0)

    return {
        "raw": raw,
        "effect_type": effect_type,
        "stat": stat,
        "value": values[0] if values else None,
        "value_range": value_range,
        "unit": unit,
        "duration_seconds": duration_seconds,
        "cooldown_seconds": cooldown_seconds,
        "interval_seconds": interval_seconds,
        "trigger": trigger,
        "applies_to": applies_to,
        "conditions": conditions,
        "status_effects": status_effects,
        "scaling_terms": scaling_terms,
        "numbers_extracted": values,
        "parse_confidence": round(confidence, 2),
    }


def parse_text(text: str):
    effects = []
    for sentence in split_effects(text):
        effects.append(parse_sentence(sentence))
    return effects


def archetype_effects(item_name: str, item):
    # Name-based overrides
    if item_name in ARCHETYPE_OVERRIDES:
        return ARCHETYPE_OVERRIDES[item_name]

    effects = []
    text_blob = " "
    for p in item.get("passives", []):
        text_blob += " " + (p.get("effects") or "")
    for a in item.get("active", []):
        text_blob += " " + (a.get("effects") or "")
    lower = text_blob.lower()

    # Jungle companion
    if any(k in lower for k in JUNGLE_COMPANION_KEYWORDS):
        effects.append({
            "raw": "Jungle companion item with evolve/consume behavior.",
            "effect_type": "utility",
            "stat": "jungle_companion",
            "trigger": "passive",
            "applies_to": "self",
            "conditions": ["jungle_companion"],
            "parse_confidence": 0.8,
        })

    # Support quest / ward items
    if any(k in lower for k in SUPPORT_QUEST_KEYWORDS):
        effects.append({
            "raw": "Support quest and ward charge behavior.",
            "effect_type": "utility",
            "stat": "support_quest",
            "trigger": "passive",
            "applies_to": "self",
            "conditions": ["support_quest"],
            "parse_confidence": 0.8,
        })

    # Tether items
    if any(k in lower for k in TETHER_KEYWORDS):
        effects.append({
            "raw": "Tethered ally effect.",
            "effect_type": "utility",
            "stat": "tether",
            "trigger": "active",
            "applies_to": "ally",
            "conditions": ["tether"],
            "parse_confidence": 0.8,
        })

    # Ward actives
    if any(k in lower for k in WARD_ACTIVE_KEYWORDS):
        effects.append({
            "raw": "Place a ward and grant vision.",
            "effect_type": "utility",
            "stat": "ward",
            "trigger": "active",
            "applies_to": "area",
            "conditions": ["ward_active"],
            "parse_confidence": 0.8,
        })

    return effects


def normalize_item(item):
    effects_structured = []
    # Archetype additions
    archetype = archetype_effects(item.get("name"), item)
    effects_structured.extend(archetype)

    for passive in item.get("passives", []):
        effects_structured.extend(parse_text(passive.get("effects", "")))
    for active in item.get("active", []):
        effects_structured.extend(parse_text(active.get("effects", "")))
    return effects_structured


def main():
    os.makedirs(OUT_DIR, exist_ok=True)
    with open(SCHEMA_PATH, "w", encoding="utf-8") as f:
        json.dump(SCHEMA, f, indent=2, ensure_ascii=False)

    low_conf = []

    for name in os.listdir(ITEMS_DIR):
        if not name.endswith(".json"):
            continue
        path = os.path.join(ITEMS_DIR, name)
        with open(path, "r", encoding="utf-8") as f:
            item = json.load(f)

        effects = normalize_item(item)

        # If any archetype effect exists, raise confidence floor
        if any(e.get("parse_confidence", 0) >= 0.8 for e in effects):
            for e in effects:
                e["parse_confidence"] = max(e.get("parse_confidence", 0), 0.6)

        if any(e.get("parse_confidence", 1) < 0.5 for e in effects):
            low_conf.append(item.get("name"))

        out = {
            "name": item.get("name"),
            "id": item.get("id"),
            "tier": item.get("tier"),
            "rank": item.get("rank"),
            "stats": item.get("stats"),
            "passives": item.get("passives"),
            "active": item.get("active"),
            "effects_structured": effects,
            "shop": item.get("shop"),
            "icon": item.get("icon"),
            "simpleDescription": item.get("simpleDescription"),
            "nicknames": item.get("nicknames"),
            "generated_at": datetime.now(timezone.utc).isoformat(),
        }

        out_path = os.path.join(OUT_DIR, name)
        with open(out_path, "w", encoding="utf-8") as f:
            json.dump(out, f, indent=2, ensure_ascii=False)

    with open(REPORT_PATH, "w", encoding="utf-8") as f:
        json.dump({
            "generated_at": datetime.now(timezone.utc).isoformat(),
            "low_confidence_items": low_conf,
            "low_confidence_count": len(low_conf),
        }, f, indent=2, ensure_ascii=False)

    print(OUT_DIR)
    print(REPORT_PATH)
    print("low_confidence_count", len(low_conf))


if __name__ == "__main__":
    main()
