#!/usr/bin/env python3
import argparse
import importlib.util
import heapq
import json
import math
import os
import random
from dataclasses import dataclass, field
from typing import Any, Callable, Dict, List, Optional, Tuple

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
    ability_tick_interval_seconds: float = 1.0
    stun_interval_seconds: float = 0.0
    stun_duration_seconds: float = 0.0
    scripts: List[str] = field(default_factory=list)


@dataclass
class SimulationConfig:
    dt: float
    server_tick_rate_hz: float
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
    scripts: List[str] = field(default_factory=list)


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


class VladCombatSimulation:
    def __init__(
        self,
        vlad_base: ChampionBase,
        vlad_build_items: List[Item],
        enemies: List[Tuple[EnemyConfig, List[Item]]],
        sim: SimulationConfig,
        urf: UrfBuffs,
        script_base_dir: Optional[str] = None,
    ) -> None:
        self.vlad_base = vlad_base
        self.vlad_build_items = vlad_build_items
        self.enemies = enemies
        self.sim = sim
        self.urf = urf
        self.script_base_dir = script_base_dir or os.path.dirname(__file__)

        self.tick_seconds = 1.0 / sim.server_tick_rate_hz if sim.server_tick_rate_hz > 0 else sim.dt
        self.time = 0.0
        self.finished = False
        self.death_time: Optional[float] = None

        self.hooks: Dict[str, List[Callable[["VladCombatSimulation", Dict[str, Any]], None]]] = {}
        self._event_queue: List[Tuple[float, int, int, Dict[str, Any]]] = []
        self._event_counter = 0

        vlad_item_stats = Stats()
        for item in vlad_build_items:
            vlad_item_stats.add(item.stats)
        self.vlad_item_stats = vlad_item_stats
        self.vlad_stats = compute_vlad_stats(vlad_base, vlad_item_stats)
        self.max_health = self.vlad_stats.health
        self.health = self.max_health

        self.armor = self.vlad_stats.armor
        self.magic_resist = self.vlad_stats.magic_resist
        self.physical_multiplier = 100.0 / (100.0 + max(0.0, self.armor))
        self.magic_multiplier = 100.0 / (100.0 + max(0.0, self.magic_resist))

        self.ability_haste = vlad_item_stats.ability_haste + urf.ability_haste
        pool_base_cooldown = [28, 25, 22, 19, 16][sim.vlad_pool_rank - 1]
        self.pool_cooldown = cooldown_after_haste(pool_base_cooldown, self.ability_haste)
        self.pool_duration = sim.vlad_pool_untargetable_seconds

        self.zhonya_available = any(item.name == "Zhonya's Hourglass" for item in vlad_build_items)
        self.ga_available = any(item.name == "Guardian Angel" for item in vlad_build_items)
        self.protoplasm_available = any(item.name == "Protoplasm Harness" for item in vlad_build_items)

        self.ga_cooldown = cooldown_after_haste(sim.ga_cooldown_seconds, urf.item_haste)
        self.zhonya_cooldown = cooldown_after_haste(sim.zhonya_cooldown_seconds, urf.item_haste)
        self.protoplasm_cooldown = 120.0

        self.zhonya_cd = 0.0
        self.ga_cd = 0.0
        self.pool_cd = 0.0
        self.protoplasm_cd = 0.0

        self.pool_until = 0.0
        self.stasis_until = 0.0
        self.ga_res_until = 0.0
        self.stunned_until = 0.0

        self.protoplasm_shield = 0.0
        self.pool_heal_rate = 0.0
        self.pool_heal_until = 0.0
        self.protoplasm_hot_rate = 0.0
        self.protoplasm_hot_until = 0.0

        self.enemy_state: List[Dict[str, Any]] = []
        for enemy, build in enemies:
            enemy_stats = Stats()
            for item in build:
                enemy_stats.add(item.stats)
            physical_dps, magic_dps = compute_enemy_dps(enemy, enemy_stats, urf)
            attack_damage = enemy.base.base_attack_damage + enemy_stats.attack_damage
            attack_speed_bonus = enemy_stats.attack_speed_percent / 100.0
            attack_speed = enemy.base.base_attack_speed * (1.0 + attack_speed_bonus)
            attack_speed *= urf.bonus_attack_speed_multiplier_melee if enemy.base.is_melee else urf.bonus_attack_speed_multiplier_ranged
            attack_interval = 1.0 / max(0.001, attack_speed)
            ability_interval = max(0.05, enemy.ability_tick_interval_seconds)
            ability_damage = magic_dps * ability_interval

            state = {
                "enemy": enemy,
                "physical_hit_damage": attack_damage,
                "attack_interval": attack_interval,
                "ability_hit_damage": ability_damage,
                "ability_interval": ability_interval,
            }
            self.enemy_state.append(state)
            self.schedule_event(attack_interval, 30, "enemy_attack", {"enemy_idx": len(self.enemy_state) - 1}, recurring=attack_interval)
            if ability_damage > 0:
                self.schedule_event(ability_interval, 40, "enemy_ability", {"enemy_idx": len(self.enemy_state) - 1}, recurring=ability_interval)
            if enemy.stun_interval_seconds > 0:
                self.schedule_event(enemy.stun_interval_seconds, 20, "enemy_stun", {"enemy_idx": len(self.enemy_state) - 1}, recurring=enemy.stun_interval_seconds)

        self.load_scripts(sim.scripts)
        for enemy in self.enemy_state:
            self.load_scripts(enemy["enemy"].scripts)
        self.run_hooks("on_init", {"time": self.time})

    def register_hook(self, hook_name: str, fn: Callable[["VladCombatSimulation", Dict[str, Any]], None]) -> None:
        self.hooks.setdefault(hook_name, []).append(fn)

    def run_hooks(self, hook_name: str, payload: Dict[str, Any]) -> None:
        for fn in self.hooks.get(hook_name, []):
            fn(self, payload)

    def load_scripts(self, scripts: List[str]) -> None:
        for idx, script_path in enumerate(scripts):
            resolved_path = script_path
            if not os.path.isabs(resolved_path):
                resolved_path = os.path.join(self.script_base_dir, resolved_path)
            spec = importlib.util.spec_from_file_location(f"sim_script_{idx}_{abs(hash(resolved_path))}", resolved_path)
            if spec is None or spec.loader is None:
                raise ValueError(f"Could not load script: {resolved_path}")
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            register = getattr(module, "register", None)
            if not callable(register):
                raise ValueError(f"Script missing register(sim): {resolved_path}")
            register(self)

    def schedule_event(
        self,
        delay_seconds: float,
        priority: int,
        event_type: str,
        payload: Dict[str, Any],
        recurring: Optional[float] = None,
    ) -> None:
        event_time = self.time + max(0.0, delay_seconds)
        self._event_counter += 1
        event = {"type": event_type, "payload": payload, "recurring": recurring}
        heapq.heappush(self._event_queue, (event_time, priority, self._event_counter, event))

    def is_targetable(self) -> bool:
        return self.time >= self.pool_until and self.time >= self.stasis_until and self.time >= self.ga_res_until

    def can_cast(self) -> bool:
        return self.is_targetable() and self.time >= self.stunned_until

    def apply_hot_effects(self, to_time: float) -> None:
        if to_time <= self.time:
            return
        delta = to_time - self.time
        if self.pool_heal_until > self.time:
            active = min(delta, self.pool_heal_until - self.time)
            self.health = min(self.max_health, self.health + self.pool_heal_rate * active)
        if self.protoplasm_hot_until > self.time:
            active = min(delta, self.protoplasm_hot_until - self.time)
            self.health = min(self.max_health, self.health + self.protoplasm_hot_rate * active)
        self.time = to_time

    def apply_damage(self, physical: float = 0.0, magic: float = 0.0, true: float = 0.0) -> None:
        if self.finished or self.health <= 0:
            return
        if not self.is_targetable():
            return
        damage = physical * self.physical_multiplier + magic * self.magic_multiplier + true
        if self.protoplasm_shield > 0 and damage > 0:
            absorbed = min(self.protoplasm_shield, damage)
            self.protoplasm_shield -= absorbed
            damage -= absorbed
        self.health -= damage
        self.run_hooks("on_damage", {"time": self.time, "damage": damage, "health_after": self.health})
        if self.health <= 0:
            self.handle_death()

    def handle_death(self) -> None:
        if self.ga_available and self.time >= self.ga_cd:
            self.ga_cd = self.time + self.ga_cooldown
            self.ga_res_until = self.time + self.sim.ga_revive_duration_seconds
            self.health = max(1.0, self.vlad_base.base_health * self.sim.ga_revive_base_health_ratio)
            self.run_hooks("on_ga_revive", {"time": self.time, "health_after": self.health})
            return
        self.finished = True
        self.death_time = self.time

    def maybe_cast_vlad_defensives(self) -> None:
        if self.finished:
            return
        if self.time >= self.pool_cd and self.can_cast():
            self.pool_cd = self.time + self.pool_cooldown
            self.pool_until = self.time + self.pool_duration
            cost = self.health * self.sim.vlad_pool_cost_percent_current_health * self.urf.health_cost_multiplier
            self.health -= cost
            pool_damage = self.sim.vlad_pool_base_damage_by_rank[self.sim.vlad_pool_rank - 1]
            pool_damage += self.sim.vlad_pool_bonus_health_ratio * (self.vlad_stats.health - self.vlad_base.base_health)
            total_pool_damage = pool_damage * len(self.enemies)
            pool_heal = total_pool_damage * self.sim.vlad_pool_heal_ratio_of_damage
            self.pool_heal_rate = pool_heal / self.pool_duration if self.pool_duration > 0 else 0.0
            self.pool_heal_until = self.time + self.pool_duration
            self.run_hooks("on_cast_pool", {"time": self.time, "health_after_cost": self.health})
            if self.health <= 0:
                self.handle_death()
                return

        if (
            self.zhonya_available
            and self.time >= self.zhonya_cd
            and self.health <= self.max_health * self.sim.zhonya_trigger_health_percent
            and self.time >= self.pool_until
            and self.time >= self.ga_res_until
        ):
            self.zhonya_cd = self.time + self.zhonya_cooldown
            self.stasis_until = self.time + self.sim.zhonya_duration_seconds
            self.run_hooks("on_cast_zhonya", {"time": self.time, "stasis_until": self.stasis_until})

        if self.protoplasm_available and self.time >= self.protoplasm_cd and self.health <= self.max_health * self.sim.protoplasm_trigger_health_percent:
            self.protoplasm_cd = self.time + self.protoplasm_cooldown
            self.protoplasm_shield += self.sim.protoplasm_bonus_health
            self.protoplasm_hot_rate = self.sim.protoplasm_heal_total / max(0.001, self.sim.protoplasm_duration_seconds)
            self.protoplasm_hot_until = self.time + self.sim.protoplasm_duration_seconds
            self.run_hooks("on_trigger_protoplasm", {"time": self.time, "shield": self.protoplasm_shield})

    def process_event(self, event: Dict[str, Any]) -> None:
        event_type = event["type"]
        payload = event["payload"]
        self.run_hooks("on_event_pre", {"time": self.time, "event_type": event_type, "payload": payload})
        if event_type == "enemy_attack":
            state = self.enemy_state[payload["enemy_idx"]]
            self.apply_damage(physical=state["physical_hit_damage"])
        elif event_type == "enemy_ability":
            state = self.enemy_state[payload["enemy_idx"]]
            self.apply_damage(magic=state["ability_hit_damage"])
        elif event_type == "enemy_stun":
            enemy = self.enemy_state[payload["enemy_idx"]]["enemy"]
            if self.is_targetable():
                self.stunned_until = max(self.stunned_until, self.time + enemy.stun_duration_seconds)
        self.run_hooks("on_event_post", {"time": self.time, "event_type": event_type, "payload": payload})

    def step(self, ticks: int = 1) -> bool:
        for _ in range(max(1, ticks)):
            if self.finished or self.time >= self.sim.max_time_seconds:
                self.finished = True
                return False

            target_time = min(self.sim.max_time_seconds, self.time + self.tick_seconds)
            self.run_hooks("on_pre_tick", {"time": self.time, "target_time": target_time})
            self.maybe_cast_vlad_defensives()

            while self._event_queue and self._event_queue[0][0] <= target_time and not self.finished:
                event_time, priority, _, event = heapq.heappop(self._event_queue)
                self.apply_hot_effects(event_time)
                self.process_event(event)
                recurring = event.get("recurring")
                if recurring and recurring > 0 and not self.finished:
                    self._event_counter += 1
                    next_event = {"type": event["type"], "payload": dict(event["payload"]), "recurring": recurring}
                    heapq.heappush(self._event_queue, (event_time + recurring, priority, self._event_counter, next_event))
                self.maybe_cast_vlad_defensives()

            self.apply_hot_effects(target_time)
            self.maybe_cast_vlad_defensives()
            self.run_hooks("on_post_tick", {"time": self.time, "health": self.health})

            if self.health <= 0 and not self.finished:
                self.handle_death()
            if self.finished:
                return False
        return True

    def run_until_end(self) -> float:
        while self.step(1):
            pass
        if self.death_time is not None:
            return self.death_time
        return min(self.time, self.sim.max_time_seconds)


def simulate_vlad_survival(
    vlad_base: ChampionBase,
    vlad_build_items: List[Item],
    enemies: List[Tuple[EnemyConfig, List[Item]]],
    sim: SimulationConfig,
    urf: UrfBuffs,
    script_base_dir: Optional[str] = None,
) -> float:
    runner = VladCombatSimulation(vlad_base, vlad_build_items, enemies, sim, urf, script_base_dir=script_base_dir)
    return runner.run_until_end()


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
    server_tick_rate_hz = float(data.get("server_tick_rate_hz", 30.0))
    dt = float(data.get("dt", 1.0 / server_tick_rate_hz if server_tick_rate_hz > 0 else 0.05))
    return SimulationConfig(
        dt=dt,
        server_tick_rate_hz=server_tick_rate_hz,
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
        scripts=list(data.get("scripts", [])),
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
        ability_tick_interval_seconds=float(data.get("ability_tick_interval_seconds", 1.0)),
        stun_interval_seconds=float(data.get("stun_interval_seconds", 0.0)),
        stun_duration_seconds=float(data.get("stun_duration_seconds", 0.0)),
        scripts=list(data.get("scripts", [])),
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
    scenario_dir = os.path.dirname(os.path.abspath(scenario_path))

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
        return simulate_vlad_survival(vlad_base, build_items, enemy_builds, sim, urf, script_base_dir=scenario_dir)

    vlad_best_build = build_search(item_pool, max_items, search_cfg, vlad_score)

    baseline_fixed_time = simulate_vlad_survival(vlad_base, baseline_fixed_build, enemy_builds, sim, urf, script_base_dir=scenario_dir)
    vlad_best_time = simulate_vlad_survival(vlad_base, vlad_best_build, enemy_builds, sim, urf, script_base_dir=scenario_dir)

    print("Enemy builds (optimized for DPS):")
    for enemy, build in enemy_builds:
        print(f"- {enemy.name}: {', '.join(i.name for i in build)}")

    print("\nVladimir baseline build (fixed):")
    print(f"- Items: {', '.join(i.name for i in baseline_fixed_build)}")
    print(f"- Time alive: {baseline_fixed_time:.2f}s")

    print("\nVladimir best build (optimized for survival):")
    print(f"- Items: {', '.join(i.name for i in vlad_best_build)}")
    print(f"- Time alive: {vlad_best_time:.2f}s")


def run_vlad_stepper(scenario_path: str, ticks: int) -> None:
    items = load_items()
    urf = load_urf_buffs()
    scenario = load_scenario(scenario_path)
    scenario_dir = os.path.dirname(os.path.abspath(scenario_path))

    sim_cfg = parse_simulation_config(scenario["simulation"])
    vlad_base = parse_champion_base(scenario["vladimir_base"])
    enemy_configs = [parse_enemy_config(e) for e in scenario["enemies"]]

    search_cfg = parse_build_search(scenario["search"])
    item_pool = default_item_pool(items)

    enemy_builds = []
    for enemy in enemy_configs:
        def enemy_score(build_items: List[Item]) -> float:
            stats = build_item_stats(build_items)
            physical_dps, magic_dps = compute_enemy_dps(enemy, stats, urf)
            return physical_dps + magic_dps

        build = build_search(item_pool, search_cfg.max_items, search_cfg, enemy_score)
        enemy_builds.append((enemy, build))

    baseline_fixed_build = item_pool_from_names(items, scenario["vladimir_baseline_fixed"])
    sim = VladCombatSimulation(vlad_base, baseline_fixed_build, enemy_builds, sim_cfg, urf, script_base_dir=scenario_dir)

    print(f"Server tick rate: {sim_cfg.server_tick_rate_hz:.2f} Hz ({sim.tick_seconds:.5f}s/tick)")
    for tick in range(max(1, ticks)):
        alive = sim.step(1)
        status = "alive" if alive else "finished"
        print(
            f"tick={tick + 1} time={sim.time:.3f}s health={sim.health:.2f} "
            f"targetable={sim.is_targetable()} can_cast={sim.can_cast()} status={status}"
        )
        if not alive:
            break


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
    parser.add_argument("--mode", choices=["vlad", "vlad_step", "taric_as", "hecarim_ms"], default="vlad")
    parser.add_argument("--ticks", type=int, default=30, help="Ticks to run in vlad_step mode")
    args = parser.parse_args()

    if args.mode == "vlad":
        run_vlad_scenario(args.scenario)
    elif args.mode == "vlad_step":
        run_vlad_stepper(args.scenario, args.ticks)
    elif args.mode == "taric_as":
        run_stat_optimization("attack_speed_percent", args.scenario, "attack speed")
    elif args.mode == "hecarim_ms":
        run_stat_optimization("move_speed_flat", args.scenario, "move speed")


if __name__ == "__main__":
    main()
