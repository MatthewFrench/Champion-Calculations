#!/usr/bin/env python
# Validates character JSON files for consistent field formats and data types.
# Python 2.7 compatible.

import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))

REQUIRED_TOP_LEVEL = [
    "name",
    "data_version",
    "role_tags",
    "base_stats",
    "abilities",
    "sources",
    "notes",
    "schema_notes",
]

BASE_STATS_KEYS = [
    "health",
    "resource",
    "move_speed",
    "armor",
    "magic_resist",
    "attack_range",
    "health_regeneration",
    "resource_regeneration",
    "crit_chance",
    "attack_damage",
    "attack_speed",
]

ABILITY_KEYS = ["passive", "basic_ability_1", "basic_ability_2", "basic_ability_3", "ultimate"]

NUMERIC_TYPES = (int, float)


def is_number(v):
    return isinstance(v, NUMERIC_TYPES)


def error(errors, path, msg):
    errors.append("%s: %s" % (path, msg))


def require_keys(errors, obj, keys, path):
    for k in keys:
        if k not in obj:
            error(errors, path, "missing key '%s'" % k)


def check_number(errors, value, path):
    if not is_number(value):
        error(errors, path, "expected number, got %s" % type(value).__name__)


def check_number_list(errors, value, path):
    if not isinstance(value, list):
        error(errors, path, "expected list of numbers, got %s" % type(value).__name__)
        return
    for i, v in enumerate(value):
        if v is None:
            error(errors, "%s[%d]" % (path, i), "value is null")
        elif not is_number(v):
            error(errors, "%s[%d]" % (path, i), "expected number, got %s" % type(v).__name__)


def check_string(errors, value, path):
    if not isinstance(value, str):
        error(errors, path, "expected string, got %s" % type(value).__name__)


def check_string_list(errors, value, path):
    if not isinstance(value, list):
        error(errors, path, "expected list of strings, got %s" % type(value).__name__)
        return
    for i, v in enumerate(value):
        if not isinstance(v, str):
            error(errors, "%s[%d]" % (path, i), "expected string, got %s" % type(v).__name__)


def check_base_stat_obj(errors, obj, path):
    if not isinstance(obj, dict):
        error(errors, path, "expected object")
        return
    if "base" not in obj:
        error(errors, path, "missing key 'base'")
        return
    check_number(errors, obj["base"], path + ".base")
    # per_level or per_level_percent
    if "per_level" in obj:
        check_number(errors, obj["per_level"], path + ".per_level")
    if "per_level_percent" in obj:
        check_number(errors, obj["per_level_percent"], path + ".per_level_percent")


def check_formula(errors, formula, path):
    if not isinstance(formula, dict):
        error(errors, path, "expected object")
        return
    for k in ["op", "output_stat", "terms"]:
        if k not in formula:
            error(errors, path, "missing key '%s'" % k)
    if "op" in formula:
        check_string(errors, formula["op"], path + ".op")
    if "output_stat" in formula:
        check_string(errors, formula["output_stat"], path + ".output_stat")
    if "terms" in formula:
        if not isinstance(formula["terms"], list):
            error(errors, path + ".terms", "expected list")
        else:
            for i, term in enumerate(formula["terms"]):
                tpath = "%s.terms[%d]" % (path, i)
                if not isinstance(term, dict):
                    error(errors, tpath, "expected object")
                    continue
                # source or input_stat required
                if "source" not in term and "input_stat" not in term:
                    error(errors, tpath, "expected 'source' or 'input_stat'")
                if "source" in term:
                    check_string(errors, term["source"], tpath + ".source")
                if "input_stat" in term:
                    check_string(errors, term["input_stat"], tpath + ".input_stat")
                if "coefficient" in term:
                    check_number(errors, term["coefficient"], tpath + ".coefficient")
                if "coefficient_type" in term:
                    check_string(errors, term["coefficient_type"], tpath + ".coefficient_type")
    if "contextual_multipliers" in formula:
        if not isinstance(formula["contextual_multipliers"], dict):
            error(errors, path + ".contextual_multipliers", "expected object")


def check_effect(errors, effect, path):
    if not isinstance(effect, dict):
        error(errors, path, "expected object")
        return
    if "id" not in effect:
        error(errors, path, "missing key 'id'")
    else:
        check_string(errors, effect["id"], path + ".id")
    if "description" in effect:
        check_string(errors, effect["description"], path + ".description")
    if "base_by_rank" in effect:
        check_number_list(errors, effect["base_by_rank"], path + ".base_by_rank")
    if "base_by_champion_level" in effect:
        check_number_list(errors, effect["base_by_champion_level"], path + ".base_by_champion_level")
    if "value_seconds" in effect:
        check_number(errors, effect["value_seconds"], path + ".value_seconds")
    if "value_percent" in effect:
        check_number(errors, effect["value_percent"], path + ".value_percent")
    if "duration_seconds" in effect:
        check_number(errors, effect["duration_seconds"], path + ".duration_seconds")
    if "tick_interval_seconds" in effect:
        check_number(errors, effect["tick_interval_seconds"], path + ".tick_interval_seconds")
    if "formula" in effect:
        check_formula(errors, effect["formula"], path + ".formula")
    if "context_notes" in effect:
        check_string(errors, effect["context_notes"], path + ".context_notes")


def check_cost(errors, cost, path):
    if not isinstance(cost, dict):
        error(errors, path, "expected object")
        return
    if "resource" not in cost:
        error(errors, path, "missing key 'resource'")
    else:
        check_string(errors, cost["resource"], path + ".resource")
    if "value_type" in cost:
        check_string(errors, cost["value_type"], path + ".value_type")
    if "value_by_rank" in cost:
        check_number_list(errors, cost["value_by_rank"], path + ".value_by_rank")
    if "ratio_by_charge" in cost:
        check_number_list(errors, cost["ratio_by_charge"], path + ".ratio_by_charge")
    if "ratio_type" in cost:
        check_string(errors, cost["ratio_type"], path + ".ratio_type")
    if "context_notes" in cost:
        check_string(errors, cost["context_notes"], path + ".context_notes")


def check_ability(errors, ability, path, require_slot):
    if not isinstance(ability, dict):
        error(errors, path, "expected object")
        return
    if require_slot:
        if "slot" not in ability:
            error(errors, path, "missing key 'slot'")
        else:
            check_string(errors, ability["slot"], path + ".slot")
        if "default_keybinding" not in ability:
            error(errors, path, "missing key 'default_keybinding'")
        else:
            check_string(errors, ability["default_keybinding"], path + ".default_keybinding")
    if "name" in ability:
        check_string(errors, ability["name"], path + ".name")
    if "type" in ability:
        check_string(errors, ability["type"], path + ".type")
    if "range" in ability and ability["range"] is not None:
        check_number(errors, ability["range"], path + ".range")
    if "cooldown_seconds_by_rank" in ability:
        check_number_list(errors, ability["cooldown_seconds_by_rank"], path + ".cooldown_seconds_by_rank")
    if "cost" in ability:
        check_cost(errors, ability["cost"], path + ".cost")
    if "effects" in ability:
        if not isinstance(ability["effects"], list):
            error(errors, path + ".effects", "expected list")
        else:
            for i, effect in enumerate(ability["effects"]):
                check_effect(errors, effect, "%s.effects[%d]" % (path, i))
    if "description" in ability:
        check_string(errors, ability["description"], path + ".description")
    if "context_notes" in ability:
        check_string(errors, ability["context_notes"], path + ".context_notes")


def validate_file(path):
    errors = []
    try:
        with open(path) as f:
            data = json.load(f)
    except Exception as e:
        return ["%s: failed to load JSON (%s)" % (path, e)]

    for k in REQUIRED_TOP_LEVEL:
        if k not in data:
            error(errors, path, "missing top-level key '%s'" % k)

    if "name" in data:
        check_string(errors, data["name"], path + ".name")
    if "data_version" in data:
        check_string(errors, data["data_version"], path + ".data_version")
    if "role_tags" in data:
        check_string_list(errors, data["role_tags"], path + ".role_tags")

    if "base_stats" in data:
        if not isinstance(data["base_stats"], dict):
            error(errors, path + ".base_stats", "expected object")
        else:
            require_keys(errors, data["base_stats"], BASE_STATS_KEYS, path + ".base_stats")
            if "resource" in data["base_stats"]:
                res = data["base_stats"]["resource"]
                if isinstance(res, dict):
                    if "type" not in res:
                        error(errors, path + ".base_stats.resource", "missing key 'type'")
                    else:
                        check_string(errors, res["type"], path + ".base_stats.resource.type")
                    if "base" in res:
                        check_number(errors, res["base"], path + ".base_stats.resource.base")
                    if "per_level" in res:
                        check_number(errors, res["per_level"], path + ".base_stats.resource.per_level")
                else:
                    error(errors, path + ".base_stats.resource", "expected object")
            for stat in [
                "health",
                "armor",
                "magic_resist",
                "health_regeneration",
                "resource_regeneration",
                "crit_chance",
                "attack_damage",
                "attack_speed",
            ]:
                if stat in data["base_stats"]:
                    check_base_stat_obj(errors, data["base_stats"][stat], path + ".base_stats." + stat)
            for stat in ["move_speed", "attack_range"]:
                if stat in data["base_stats"]:
                    check_number(errors, data["base_stats"][stat], path + ".base_stats." + stat)

    if "abilities" in data:
        if not isinstance(data["abilities"], dict):
            error(errors, path + ".abilities", "expected object")
        else:
            for k in ABILITY_KEYS:
                if k not in data["abilities"]:
                    error(errors, path + ".abilities", "missing key '%s'" % k)
            if "passive" in data["abilities"]:
                check_ability(errors, data["abilities"]["passive"], path + ".abilities.passive", False)
            for k in ["basic_ability_1", "basic_ability_2", "basic_ability_3", "ultimate"]:
                if k in data["abilities"]:
                    check_ability(errors, data["abilities"][k], path + ".abilities." + k, True)

    if "sources" in data:
        if not isinstance(data["sources"], list):
            error(errors, path + ".sources", "expected list")

    if "notes" in data:
        check_string_list(errors, data["notes"], path + ".notes")

    if "schema_notes" in data:
        if not isinstance(data["schema_notes"], dict):
            error(errors, path + ".schema_notes", "expected object")

    return errors


def main():
    files = [f for f in os.listdir(HERE) if f.endswith('.json')]
    if not files:
        print("No JSON files found in %s" % HERE)
        return 2
    all_errors = []
    for f in sorted(files):
        path = os.path.join(HERE, f)
        errs = validate_file(path)
        if errs:
            all_errors.extend(errs)
    if all_errors:
        print("Validation errors:")
        for e in all_errors:
            print("- %s" % e)
        return 1
    print("All character JSON files passed validation.")
    return 0

if __name__ == '__main__':
    sys.exit(main())
