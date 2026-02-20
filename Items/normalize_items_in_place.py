import json
import os
import re
from datetime import datetime, timezone

ITEMS_DIR = os.path.dirname(os.path.abspath(__file__))
REPORT_PATH = os.path.join(ITEMS_DIR, "items_quality_report.json")

NUMBER_RE = re.compile(r"-?\d+\.?\d*")
RANGE_RE = re.compile(r"(\d+\.?\d*)\s*[\-–]\s*(\d+\.?\d*)")
DURATION_RE = re.compile(r"for\s+(\d+\.?\d*)\s*(seconds|second|s)", re.I)
COOLDOWN_RE = re.compile(r"cooldown:?\s*(\d+\.?\d*)\s*(seconds|second|s)", re.I)
INTERVAL_RE = re.compile(r"every\s+(\d+\.?\d*)\s*(seconds|second|s)", re.I)
WITHIN_UNITS_RE = re.compile(r"within\s+(\d+\.?\d*)\s*units", re.I)
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


def normalize_item(item):
    effects_structured = []
    passives = item.get("passives") or []
    active = item.get("active") or []
    for passive in passives:
        effects_structured.extend(parse_text(passive.get("effects", "")))
    for act in active:
        effects_structured.extend(parse_text(act.get("effects", "")))
    return effects_structured


def main():
    low_conf = []
    updated = 0
    for name in os.listdir(ITEMS_DIR):
        if not name.endswith(".json"):
            continue
        path = os.path.join(ITEMS_DIR, name)
        with open(path, "r", encoding="utf-8") as f:
            item = json.load(f)

        effects = normalize_item(item)
        if any(e.get("parse_confidence", 1) < 0.5 for e in effects):
            low_conf.append(item.get("name"))

        item["effects_structured"] = effects
        item.setdefault("schema_notes", {})
        item["schema_notes"]["effects_structured"] = "Parsed from passive/active text for simulation use."
        item["schema_notes"]["effects_structured_generated_at"] = datetime.now(timezone.utc).isoformat()

        with open(path, "w", encoding="utf-8") as f:
            json.dump(item, f, indent=2, ensure_ascii=False)
        updated += 1

    with open(REPORT_PATH, "w", encoding="utf-8") as f:
        json.dump({
            "generated_at": datetime.now(timezone.utc).isoformat(),
            "low_confidence_items": low_conf,
            "low_confidence_count": len(low_conf),
        }, f, indent=2, ensure_ascii=False)

    print(f"Updated {updated} items in place")
    print(REPORT_PATH)
    print("low_confidence_count", len(low_conf))


if __name__ == "__main__":
    main()
