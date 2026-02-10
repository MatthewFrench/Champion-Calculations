#!/usr/bin/env python3
import argparse
import json
import math
import os
import random
from dataclasses import dataclass, field
from typing import Dict, List, Tuple

ITEMS_DIR = os.path.join(os.path.dirname(__file__), "..", "Items")
GAME_MODE_DIR = os.path.join(os.path.dirname(__file__), "..", "Game Mode")

STAT_KEY_MAP = {
    "abilityPower": "ability_power",
    "health": "health",
    "armor": "armor",
    "magicResist": "magic_resist",
    "attackDamage": "attack_damage",
    "attackSpeed": "attack_speed_percent",
    "movespeed": "move_speed_flat",
    "moveSpeed": "move_speed_flat",
    "movementSpeed": "move_speed_flat",
    "abilityHaste": "ability_haste",
    "critChance": "crit_chance_percent",
}

EXCLUDED_RANKS = {"CONSUMABLE", "TRINKET"}


@dataclass
class Stats:
    ability_power: float = 0.0
    health: float = 0.0
    armor: float = 0.0
    magic_resist: float = 0.0
    attack_damage: float = 0.0
    attack_speed_percent: float = 0.0
    ability_haste: float = 0.0
    move_speed_flat: float = 0.0
    crit_chance_percent: float = 0.0

    def add(self, other: "Stats") -> None:
        for field_name in self.__dataclass_fields__:
            setattr(self, field_name, getattr(self, field_name) + getattr(other, field_name))


@dataclass
class Item:
    name: str
    stats: Stats
    rank: List[str]
    shop_purchasable: bool
    active: List[Dict] = field(default_factory=list)
    passives: List[Dict] = field(default_factory=list)


@dataclass
class ChampionBase:
    name: str
    base_health: float
    base_armor: float
    base_magic_resist: float
    base_attack_damage: float
    base_attack_speed: float
    base_move_speed: float
    is_melee: bool


@dataclass
class EnemyConfig:
    name: str
    base: ChampionBase
    ability_dps_flat: float = 0.0
    ability_dps_ad_ratio: float = 0.0
    ability_dps_ap_ratio: float = 0.0
    stun_interval_seconds: float = 0.0
    stun_duration_seconds: float = 0.0


@dataclass
class SimulationConfig:
    dt: float
    max_time_seconds: float
    vlad_pool_rank: int
    vlad_pool_untargetable_seconds: float
    vlad_pool_cost_percent_current_health: float
    vlad_pool_heal_ratio_of_damage: float
    vlad_pool_base_damage_by_rank: List[float]
    vlad_pool_bonus_health_ratio: float
    zhonya_duration_seconds: float
    zhonya_cooldown_seconds: float
    zhonya_trigger_health_percent: float
    ga_cooldown_seconds: float
    ga_revive_duration_seconds: float
    ga_revive_base_health_ratio: float
    protoplasm_trigger_health_percent: float
    protoplasm_bonus_health: float
    protoplasm_heal_total: float
    protoplasm_duration_seconds: float


@dataclass
class UrfBuffs:
    ability_haste: float
    item_haste: float
    health_cost_multiplier: float
    move_speed_bonus_flat: float
    bonus_attack_speed_multiplier_melee: float
    bonus_attack_speed_multiplier_ranged: float


@dataclass
class BuildResult:
    name: str
    items: List[str]
    time_alive: float


@dataclass
class BuildSearchConfig:
    strategy: str
    beam_width: int
    max_items: int
    random_samples: int
    seed: int


def load_urf_buffs() -> UrfBuffs:
    path = os.path.join(GAME_MODE_DIR, "URF.json")
    with open(path, "r", encoding="utf-8") as f:
        data = json.load(f)
    buffs = data.get("global_buffs", {})
    return UrfBuffs(
        ability_haste=float(buffs.get("ability_haste", 0.0)),
        item_haste=float(buffs.get("item_haste", 0.0)),
        health_cost_multiplier=float(buffs.get("resource_costs", {}).get("health_cost_multiplier", 1.0)),
        move_speed_bonus_flat=float(buffs.get("movement_speed_bonus_flat", 0.0)),
        bonus_attack_speed_multiplier_melee=float(buffs.get("bonus_attack_speed_multiplier", {}).get("melee", 1.0)),
        bonus_attack_speed_multiplier_ranged=float(buffs.get("bonus_attack_speed_multiplier", {}).get("ranged", 1.0)),
    )


def load_items() -> Dict[str, Item]:
    items = {}
    for filename in os.listdir(ITEMS_DIR):
        if not filename.endswith(".json"):
            continue
        path = os.path.join(ITEMS_DIR, filename)
        with open(path, "r", encoding="utf-8") as f:
            data = json.load(f)
        rank = data.get("rank") or []
        shop = data.get("shop") or {}
        purchasable = bool(shop.get("purchasable", False))
        if not purchasable:
            continue
        if any(r in EXCLUDED_RANKS for r in rank):
            continue
        stats_obj = Stats()
        for raw_key, raw_values in (data.get("stats") or {}).items():
            stat_key = STAT_KEY_MAP.get(raw_key)
            if not stat_key:
                continue
            if "flat" in raw_values:
                value = float(raw_values["flat"])
                setattr(stats_obj, stat_key, getattr(stats_obj, stat_key) + value)
            if "percent" in raw_values:
                value = float(raw_values["percent"])
                if stat_key == "move_speed_flat":
                    stats_obj.move_speed_flat += value
                else:
                    setattr(stats_obj, stat_key, getattr(stats_obj, stat_key) + value)
        item = Item(
            name=data.get("name"),
            stats=stats_obj,
            rank=rank,
            shop_purchasable=purchasable,
            active=data.get("active") or [],
            passives=data.get("passives") or [],
        )
        items[item.name] = item
    return items


def is_boots(item: Item) -> bool:
    return "BOOTS" in (item.rank or [])


def cooldown_after_haste(base_seconds: float, haste: float) -> float:
    return base_seconds * (100.0 / (100.0 + haste))


def compute_vlad_stats(base: ChampionBase, item_stats: Stats) -> Stats:
    # Vladimir passive: AP from bonus health, bonus health from AP.
    ap_items = item_stats.ability_power
    bonus_health_items = item_stats.health
    bonus_health = (bonus_health_items + 1.6 * ap_items) / 0.9472
    ability_power = ap_items + 0.033 * bonus_health

    stats = Stats(
        ability_power=ability_power,
        health=bonus_health,
        armor=item_stats.armor,
        magic_resist=item_stats.magic_resist,
        attack_damage=item_stats.attack_damage,
        attack_speed_percent=item_stats.attack_speed_percent,
        ability_haste=item_stats.ability_haste,
        move_speed_flat=item_stats.move_speed_flat,
        crit_chance_percent=item_stats.crit_chance_percent,
    )
    stats.health += base.base_health
    stats.armor += base.base_armor
    stats.magic_resist += base.base_magic_resist
    return stats


def compute_enemy_dps(enemy: EnemyConfig, item_stats: Stats, urf: UrfBuffs) -> Tuple[float, float]:
    base = enemy.base
    attack_damage = base.base_attack_damage + item_stats.attack_damage
    attack_speed_bonus = item_stats.attack_speed_percent / 100.0
    attack_speed = base.base_attack_speed * (1.0 + attack_speed_bonus)
    attack_speed *= urf.bonus_attack_speed_multiplier_melee if base.is_melee else urf.bonus_attack_speed_multiplier_ranged
    physical_dps = attack_damage * attack_speed

    ability_dps = enemy.ability_dps_flat
    ability_dps += enemy.ability_dps_ad_ratio * attack_damage
    ability_dps += enemy.ability_dps_ap_ratio * item_stats.ability_power
    magic_dps = ability_dps
    return physical_dps, magic_dps


def simulate_vlad_survival(
    vlad_base: ChampionBase,
    vlad_build_items: List[Item],
    enemies: List[Tuple[EnemyConfig, List[Item]]],
    sim: SimulationConfig,
    urf: UrfBuffs,
) -> float:
    vlad_item_stats = Stats()
    for item in vlad_build_items:
        vlad_item_stats.add(item.stats)

    vlad_stats = compute_vlad_stats(vlad_base, vlad_item_stats)
    max_health = vlad_stats.health
    armor = vlad_stats.armor
    magic_resist = vlad_stats.magic_resist

    ability_haste = vlad_item_stats.ability_haste + urf.ability_haste
    pool_base_cooldown = [28, 25, 22, 19, 16][sim.vlad_pool_rank - 1]
    pool_cooldown = cooldown_after_haste(pool_base_cooldown, ability_haste)
    pool_duration = sim.vlad_pool_untargetable_seconds

    zhonya_available = any(item.name == "Zhonya's Hourglass" for item in vlad_build_items)
    ga_available = any(item.name == "Guardian Angel" for item in vlad_build_items)
    protoplasm_available = any(item.name == "Protoplasm Harness" for item in vlad_build_items)

    zhonya_cd = 0.0
    ga_cd = 0.0
    pool_cd = 0.0

    protoplasm_cd = 0.0
    protoplasm_shield = 0.0
    protoplasm_hot_remaining = 0.0

    stunned_until = 0.0
    pool_until = 0.0
    stasis_until = 0.0
    ga_res_until = 0.0

    time = 0.0
    health = max_health

    enemy_state = []
    for enemy, build in enemies:
        enemy_stats = Stats()
        for item in build:
            enemy_stats.add(item.stats)
        physical_dps, magic_dps = compute_enemy_dps(enemy, enemy_stats, urf)
        enemy_state.append({
            "enemy": enemy,
            "physical_dps": physical_dps,
            "magic_dps": magic_dps,
            "next_stun": enemy.stun_interval_seconds if enemy.stun_interval_seconds > 0 else None,
        })

    def total_dps():
        physical = sum(e["physical_dps"] for e in enemy_state)
        magic = sum(e["magic_dps"] for e in enemy_state)
        return physical, magic

    pool_heal_per_second = 0.0
    pool_heal_remaining = 0.0

    physical_dps, magic_dps = total_dps()
    physical_multiplier = 100.0 / (100.0 + max(0.0, armor))
    magic_multiplier = 100.0 / (100.0 + max(0.0, magic_resist))

    ga_cooldown = cooldown_after_haste(sim.ga_cooldown_seconds, urf.item_haste)
    zhonya_cooldown = cooldown_after_haste(sim.zhonya_cooldown_seconds, urf.item_haste)

    while time < sim.max_time_seconds:
        targetable = time >= pool_until and time >= stasis_until and time >= ga_res_until
        stunned = time < stunned_until

        if time >= pool_cd and not stunned and time >= pool_until and time >= stasis_until and time >= ga_res_until:
            pool_cd = time + pool_cooldown
            pool_until = time + pool_duration
            cost = health * sim.vlad_pool_cost_percent_current_health * urf.health_cost_multiplier
            health -= cost
            pool_damage = sim.vlad_pool_base_damage_by_rank[sim.vlad_pool_rank - 1] + sim.vlad_pool_bonus_health_ratio * (vlad_stats.health - vlad_base.base_health)
            total_pool_damage = pool_damage * len(enemies)
            pool_heal = total_pool_damage * sim.vlad_pool_heal_ratio_of_damage
            pool_heal_per_second = pool_heal / pool_duration if pool_duration > 0 else 0.0
            pool_heal_remaining = pool_duration

        if zhonya_available and time >= zhonya_cd and health <= max_health * sim.zhonya_trigger_health_percent and time >= pool_until and time >= ga_res_until:
            zhonya_cd = time + zhonya_cooldown
            stasis_until = time + sim.zhonya_duration_seconds

        if protoplasm_available and time >= protoplasm_cd and health <= max_health * sim.protoplasm_trigger_health_percent:
            protoplasm_cd = time + 120.0
            protoplasm_shield += sim.protoplasm_bonus_health
            protoplasm_hot_remaining = sim.protoplasm_duration_seconds

        for state in enemy_state:
            next_stun = state["next_stun"]
            enemy = state["enemy"]
            if next_stun is not None and time >= next_stun:
                stunned_until = max(stunned_until, time + enemy.stun_duration_seconds)
                state["next_stun"] = time + enemy.stun_interval_seconds

        if targetable and health > 0:
            damage = (physical_dps * physical_multiplier + magic_dps * magic_multiplier) * sim.dt
            if protoplasm_shield > 0:
                absorbed = min(protoplasm_shield, damage)
                protoplasm_shield -= absorbed
                damage -= absorbed
            health -= damage

        if pool_heal_remaining > 0:
            heal = pool_heal_per_second * sim.dt
            health = min(max_health, health + heal)
            pool_heal_remaining = max(0.0, pool_heal_remaining - sim.dt)

        if protoplasm_hot_remaining > 0:
            heal = (sim.protoplasm_heal_total / sim.protoplasm_duration_seconds) * sim.dt
            health = min(max_health, health + heal)
            protoplasm_hot_remaining = max(0.0, protoplasm_hot_remaining - sim.dt)

        if health <= 0:
            if ga_available and time >= ga_cd:
                ga_cd = time + ga_cooldown
                ga_res_until = time + sim.ga_revive_duration_seconds
                health = max(1.0, vlad_base.base_health * sim.ga_revive_base_health_ratio)
            else:
                return time

        time += sim.dt

    return sim.max_time_seconds


def build_item_stats(items: List[Item]) -> Stats:
    stats = Stats()
    for item in items:
        stats.add(item.stats)
    return stats


def choose_best_build_by_stat(item_pool: List[Item], stat_key: str, max_items: int, beam_width: int) -> List[Item]:
    def stat_value(items: List[Item]) -> float:
        stats = build_item_stats(items)
        return getattr(stats, stat_key)

    candidates = [[]]
    for _ in range(max_items):
        next_candidates = []
        for build in candidates:
            has_boots = any(is_boots(i) for i in build)
            for item in item_pool:
                if item in build:
                    continue
                if is_boots(item) and has_boots:
                    continue
                next_candidates.append(build + [item])
        next_candidates.sort(key=stat_value, reverse=True)
        candidates = next_candidates[:beam_width]
    return candidates[0] if candidates else []


def build_search(
    item_pool: List[Item],
    max_items: int,
    search: BuildSearchConfig,
    score_fn,
) -> List[Item]:
    if search.strategy == "greedy":
        build = []
        for _ in range(max_items):
            best = None
            best_score = -math.inf
            for item in item_pool:
                if item in build:
                    continue
                if is_boots(item) and any(is_boots(i) for i in build):
                    continue
                candidate = build + [item]
                score = score_fn(candidate)
                if score > best_score:
                    best_score = score
                    best = item
            if best is None:
                break
            build.append(best)
        return build

    if search.strategy == "beam":
        candidates = [[]]
        for _ in range(max_items):
            next_candidates = []
            for build in candidates:
                has_boots = any(is_boots(i) for i in build)
                for item in item_pool:
                    if item in build:
                        continue
                    if is_boots(item) and has_boots:
                        continue
                    next_candidates.append(build + [item])
            next_candidates.sort(key=score_fn, reverse=True)
            candidates = next_candidates[:search.beam_width]
        return candidates[0] if candidates else []

    if search.strategy == "random":
        rng = random.Random(search.seed)
        best_build = []
        best_score = -math.inf
        pool = list(item_pool)
        for _ in range(search.random_samples):
            rng.shuffle(pool)
            build = []
            for item in pool:
                if len(build) >= max_items:
                    break
                if item in build:
                    continue
                if is_boots(item) and any(is_boots(i) for i in build):
                    continue
                build.append(item)
            score = score_fn(build)
            if score > best_score:
                best_score = score
                best_build = build
        return best_build

    raise ValueError(f"Unknown search strategy: {search.strategy}")


def parse_simulation_config(data: Dict) -> SimulationConfig:
    return SimulationConfig(
        dt=float(data["dt"]),
        max_time_seconds=float(data["max_time_seconds"]),
        vlad_pool_rank=int(data["vlad_pool_rank"]),
        vlad_pool_untargetable_seconds=float(data["vlad_pool_untargetable_seconds"]),
        vlad_pool_cost_percent_current_health=float(data["vlad_pool_cost_percent_current_health"]),
        vlad_pool_heal_ratio_of_damage=float(data["vlad_pool_heal_ratio_of_damage"]),
        vlad_pool_base_damage_by_rank=list(data["vlad_pool_base_damage_by_rank"]),
        vlad_pool_bonus_health_ratio=float(data["vlad_pool_bonus_health_ratio"]),
        zhonya_duration_seconds=float(data["zhonya_duration_seconds"]),
        zhonya_cooldown_seconds=float(data["zhonya_cooldown_seconds"]),
        zhonya_trigger_health_percent=float(data["zhonya_trigger_health_percent"]),
        ga_cooldown_seconds=float(data["ga_cooldown_seconds"]),
        ga_revive_duration_seconds=float(data["ga_revive_duration_seconds"]),
        ga_revive_base_health_ratio=float(data["ga_revive_base_health_ratio"]),
        protoplasm_trigger_health_percent=float(data["protoplasm_trigger_health_percent"]),
        protoplasm_bonus_health=float(data["protoplasm_bonus_health"]),
        protoplasm_heal_total=float(data["protoplasm_heal_total"]),
        protoplasm_duration_seconds=float(data["protoplasm_duration_seconds"]),
    )


def parse_champion_base(data: Dict) -> ChampionBase:
    return ChampionBase(
        name=data["name"],
        base_health=float(data["base_health"]),
        base_armor=float(data["base_armor"]),
        base_magic_resist=float(data["base_magic_resist"]),
        base_attack_damage=float(data["base_attack_damage"]),
        base_attack_speed=float(data["base_attack_speed"]),
        base_move_speed=float(data["base_move_speed"]),
        is_melee=bool(data["is_melee"]),
    )


def parse_enemy_config(data: Dict) -> EnemyConfig:
    base = parse_champion_base(data["base"])
    return EnemyConfig(
        name=data["name"],
        base=base,
        ability_dps_flat=float(data.get("ability_dps_flat", 0.0)),
        ability_dps_ad_ratio=float(data.get("ability_dps_ad_ratio", 0.0)),
        ability_dps_ap_ratio=float(data.get("ability_dps_ap_ratio", 0.0)),
        stun_interval_seconds=float(data.get("stun_interval_seconds", 0.0)),
        stun_duration_seconds=float(data.get("stun_duration_seconds", 0.0)),
    )


def parse_build_search(data: Dict) -> BuildSearchConfig:
    return BuildSearchConfig(
        strategy=data["strategy"],
        beam_width=int(data.get("beam_width", 20)),
        max_items=int(data.get("max_items", 6)),
        random_samples=int(data.get("random_samples", 200)),
        seed=int(data.get("seed", 1337)),
    )


def load_scenario(path: str) -> Dict:
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def item_pool_from_names(items: Dict[str, Item], names: List[str]) -> List[Item]:
    pool = []
    for name in names:
        item = items.get(name)
        if not item:
            raise ValueError(f"Item not found: {name}")
        pool.append(item)
    return pool


def default_item_pool(items: Dict[str, Item]) -> List[Item]:
    pool = []
    for item in items.values():
        if item.shop_purchasable:
            pool.append(item)
    pool.sort(key=lambda i: i.name)
    return pool


def run_vlad_scenario(scenario_path: str) -> None:
    items = load_items()
    urf = load_urf_buffs()
    scenario = load_scenario(scenario_path)

    sim = parse_simulation_config(scenario["simulation"])
    vlad_base = parse_champion_base(scenario["vladimir_base"])
    enemy_configs = [parse_enemy_config(e) for e in scenario["enemies"]]

    search_cfg = parse_build_search(scenario["search"])
    max_items = search_cfg.max_items

    item_pool = default_item_pool(items)

    baseline_fixed_names = scenario["vladimir_baseline_fixed"]
    baseline_fixed_build = item_pool_from_names(items, baseline_fixed_names)

    enemy_builds = []
    for enemy in enemy_configs:
        def enemy_score(build_items: List[Item]) -> float:
            stats = build_item_stats(build_items)
            physical_dps, magic_dps = compute_enemy_dps(enemy, stats, urf)
            return physical_dps + magic_dps

        build = build_search(item_pool, max_items, search_cfg, enemy_score)
        enemy_builds.append((enemy, build))

    def vlad_score(build_items: List[Item]) -> float:
        return simulate_vlad_survival(vlad_base, build_items, enemy_builds, sim, urf)

    vlad_best_build = build_search(item_pool, max_items, search_cfg, vlad_score)

    baseline_fixed_time = simulate_vlad_survival(vlad_base, baseline_fixed_build, enemy_builds, sim, urf)
    vlad_best_time = simulate_vlad_survival(vlad_base, vlad_best_build, enemy_builds, sim, urf)

    print("Enemy builds (optimized for DPS):")
    for enemy, build in enemy_builds:
        print(f"- {enemy.name}: {', '.join(i.name for i in build)}")

    print("\nVladimir baseline build (fixed):")
    print(f"- Items: {', '.join(i.name for i in baseline_fixed_build)}")
    print(f"- Time alive: {baseline_fixed_time:.2f}s")

    print("\nVladimir best build (optimized for survival):")
    print(f"- Items: {', '.join(i.name for i in vlad_best_build)}")
    print(f"- Time alive: {vlad_best_time:.2f}s")


def run_stat_optimization(stat_key: str, scenario_path: str, label: str) -> None:
    items = load_items()
    scenario = load_scenario(scenario_path)
    search_cfg = parse_build_search(scenario["search"])
    item_pool = default_item_pool(items)

    build = choose_best_build_by_stat(item_pool, stat_key, search_cfg.max_items, search_cfg.beam_width)
    stats = build_item_stats(build)
    value = getattr(stats, stat_key)

    print(f"Best build for {label}:")
    print(f"- Items: {', '.join(i.name for i in build)}")
    print(f"- Total {label}: {value:.2f}")


def main() -> None:
    parser = argparse.ArgumentParser(description="URF Vladimir survival simulator")
    parser.add_argument("--scenario", required=True, help="Path to scenario JSON")
    parser.add_argument("--mode", choices=["vlad", "taric_as", "hecarim_ms"], default="vlad")
    args = parser.parse_args()

    if args.mode == "vlad":
        run_vlad_scenario(args.scenario)
    elif args.mode == "taric_as":
        run_stat_optimization("attack_speed_percent", args.scenario, "attack speed")
    elif args.mode == "hecarim_ms":
        run_stat_optimization("move_speed_flat", args.scenario, "move speed")


if __name__ == "__main__":
    main()
