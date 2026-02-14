import json
import os
from datetime import datetime, timezone


ITEMS_DIR = "/Users/matthewfrench/Documents/League of Legends/Vladimir/Items"
REPORT_PATH = os.path.join(ITEMS_DIR, "items_schema_validation_report.json")


TOP_LEVEL_REQUIRED = [
    "name",
    "id",
    "tier",
    "rank",
    "stats",
    "passives",
    "active",
    "effects_structured",
    "shop",
    "icon",
    "simpleDescription",
    "nicknames",
    "generated_at",
    "schema_notes",
]


def is_item_payload(data):
    return isinstance(data, dict) and "name" in data and "id" in data


def error(errors, path, message):
    errors.append({"level": "error", "path": path, "message": message})


def warn(errors, path, message):
    errors.append({"level": "warning", "path": path, "message": message})


def validate_top_level(data, path, errors):
    for key in TOP_LEVEL_REQUIRED:
        if key not in data:
            error(errors, f"{path}.{key}", "missing required field")
    if not isinstance(data.get("name"), str):
        error(errors, f"{path}.name", "expected string")
    if not isinstance(data.get("id"), int):
        error(errors, f"{path}.id", "expected integer")
    if not isinstance(data.get("tier"), int):
        error(errors, f"{path}.tier", "expected integer")
    if not isinstance(data.get("rank"), list):
        error(errors, f"{path}.rank", "expected array of strings")
    if not isinstance(data.get("stats"), dict):
        error(errors, f"{path}.stats", "expected object (no null)")
    if not isinstance(data.get("passives"), list):
        error(errors, f"{path}.passives", "expected array (no null)")
    active = data.get("active")
    if not isinstance(active, (list, dict)):
        error(errors, f"{path}.active", "expected array or object (no null)")
    if not isinstance(data.get("effects_structured"), list):
        error(errors, f"{path}.effects_structured", "expected array (no null)")
    if not isinstance(data.get("shop"), dict):
        error(errors, f"{path}.shop", "expected object (no null)")
    if not isinstance(data.get("nicknames"), list):
        error(errors, f"{path}.nicknames", "expected array (no null)")
    if not isinstance(data.get("schema_notes"), dict):
        error(errors, f"{path}.schema_notes", "expected object")


def validate_effect(effect, path, errors):
    if not isinstance(effect, dict):
        error(errors, path, "effect must be an object")
        return

    if "raw" not in effect or not isinstance(effect.get("raw"), str) or not effect.get("raw"):
        error(errors, f"{path}.raw", "missing human-readable raw description")

    if "effect_type" not in effect or not isinstance(effect.get("effect_type"), str):
        error(errors, f"{path}.effect_type", "missing effect_type")

    if "id" not in effect or not isinstance(effect.get("id"), str) or not effect.get("id"):
        warn(errors, f"{path}.id", "missing or empty effect id (recommended for simulation)")

    # Simulation-focused checks by effect type
    effect_type = effect.get("effect_type")
    if effect_type == "damage":
        if "damage_type" not in effect and "stat" not in effect:
            warn(errors, path, "damage effect missing damage_type or stat")
        if "value" not in effect and not effect.get("scaling_terms"):
            warn(errors, path, "damage effect missing value or scaling_terms")
    elif effect_type in ("heal", "healing", "health_regeneration"):
        if "value" not in effect and not effect.get("scaling_terms"):
            warn(errors, path, "healing effect missing value or scaling_terms")
    elif effect_type == "shield":
        if "value" not in effect and not effect.get("scaling_terms"):
            warn(errors, path, "shield effect missing value or scaling_terms")
    elif effect_type == "stat_modifier":
        if "stat" not in effect and not effect.get("scaling_terms"):
            warn(errors, path, "stat_modifier missing stat or scaling_terms")
    elif effect_type == "status":
        if not effect.get("status_effects") and not effect.get("stat_modifiers"):
            warn(errors, path, "status effect missing status_effects or stat_modifiers")
    elif effect_type == "stack_gain":
        if "stack_name" not in effect:
            warn(errors, path, "stack_gain missing stack_name")
    elif effect_type == "resource":
        if "resource" not in effect and "stat" not in effect:
            warn(errors, path, "resource effect missing resource/stat")

    # Shared checks
    if "trigger" in effect and not isinstance(effect.get("trigger"), (dict, str)):
        error(errors, f"{path}.trigger", "trigger must be object or string")
    if "applies_to" in effect and not isinstance(effect.get("applies_to"), str):
        error(errors, f"{path}.applies_to", "applies_to must be string")


def validate_item(data, path):
    errors = []
    validate_top_level(data, path, errors)

    effects = data.get("effects_structured", [])
    if isinstance(effects, list):
        for i, effect in enumerate(effects):
            validate_effect(effect, f"{path}.effects_structured[{i}]", errors)

    return errors


def main():
    report = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "items_checked": 0,
        "errors": [],
        "warnings": [],
        "files": {},
    }

    for name in sorted(os.listdir(ITEMS_DIR)):
        if not name.endswith(".json"):
            continue
        if name in {"items_quality_report.json", "items_schema_validation_report.json"}:
            continue
        path = os.path.join(ITEMS_DIR, name)
        try:
            with open(path, "r", encoding="utf-8") as f:
                data = json.load(f)
        except Exception as exc:
            report["errors"].append({"file": name, "message": f"failed to read json: {exc}"})
            continue

        if not is_item_payload(data):
            continue

        report["items_checked"] += 1
        item_errors = validate_item(data, name)
        if item_errors:
            report["files"][name] = item_errors
            for entry in item_errors:
                if entry["level"] == "error":
                    report["errors"].append({"file": name, "path": entry["path"], "message": entry["message"]})
                else:
                    report["warnings"].append({"file": name, "path": entry["path"], "message": entry["message"]})

    with open(REPORT_PATH, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2, sort_keys=True)

    print(f"Validated {report['items_checked']} items.")
    print(f"Errors: {len(report['errors'])}, Warnings: {len(report['warnings'])}")
    print(f"Report: {REPORT_PATH}")


if __name__ == "__main__":
    main()
