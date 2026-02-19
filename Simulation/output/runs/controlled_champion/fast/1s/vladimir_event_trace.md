# Vladimir Event Trace

## Optimized Build Trace
- 0.000s [state_snapshot] checkpoint 0.0s (captured_at 0.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=4390.5/4390.5 (100.0%) armor=209.5 mr=54.7
  offense: ap=302.4 ah=379.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Umbral Glaive, Unending Despair, Zhonya's Hourglass] | runes [Aftershock, Demolish, Bone Plating, Overgrowth, Triumph, Legend: Haste] | shards [ability_haste, movement_speed, health]
  cooldowns: Stasis item ready
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready; Auto Attack 0.55s
  runtime: cooldowns [none] | stacks [none]
  buffs: none
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=3501.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.37s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: none
  Vayne:
    core: pos=(300.0, 140.0) hp=2641.5/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.16s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: none
  Morgana:
    core: pos=(-300.0, 120.0) hp=3440.5/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.62s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-260.0, -130.0) hp=2413.5/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.54s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=6479.8/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.61s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 0.000s [controlled_champion_cast] Vladimir cast vladimir_hemoplague (impact in 0.25s)
- 0.000s [champion_script] Vayne executed Tumble Empower
- 0.000s [enemy_buff] Vayne empowered next attack
- 0.000s [champion_script] Morgana executed Dark Binding
- 0.000s [damage_in] Morgana Dark Binding -> Vladimir | physical 0.0, magic 1001.5, true 0.0, total 647.4
- 0.000s [champion_script] Morgana executed Soul Shackles
- 0.000s [damage_in] Morgana Soul Shackles -> Vladimir | physical 0.0, magic 678.0, true 0.0, total 438.3
- 0.000s [champion_script] Sona executed Crescendo
- 0.000s [damage_in] Sona Crescendo -> Vladimir | physical 0.0, magic 766.5, true 0.0, total 495.5
- 0.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 0.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 842.8, true 0.0, total 544.8
- 0.159s [attack_start] Vayne begins auto attack
- 0.250s [controlled_champion_ultimate_hit] Vladimir vladimir_hemoplague dealt 1743.0 to 5 enemies in range
- 0.373s [attack_start] Warwick begins auto attack
- 0.500s [champion_script] Vayne executed Tumble Empower
- 0.500s [enemy_buff] Vayne empowered next attack
- 0.525s [damage_in] Vayne Auto Attack -> Vladimir | physical 1138.2, magic 0.0, true 0.0, total 367.8
- 0.525s [attack_hit] Vayne hit Vladimir (phys 1138.2, magic 0.0, true 0.0)
- 0.540s [attack_start] Sona begins auto attack
- 0.612s [attack_start] Dr. Mundo begins auto attack
- 0.613s [damage_in] Warwick Auto Attack -> Vladimir | physical 351.3, magic 81.6, true 0.0, total 166.3
- 0.613s [attack_hit] Warwick hit Vladimir (phys 351.3, magic 81.6, true 0.0)
- 0.620s [attack_start] Morgana begins auto attack
- 0.675s [attack_start] Vayne begins auto attack
- 0.852s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 0.934s [damage_in] Sona Auto Attack -> Vladimir | physical 106.0, magic 0.0, true 0.0, total 34.2
- 0.934s [attack_hit] Sona hit Vladimir (phys 106.0, magic 0.0, true 0.0)
- 0.972s [attack_start] Warwick begins auto attack
- 1.000s [champion_script] Vayne executed Tumble Empower
- 1.000s [enemy_buff] Vayne empowered next attack
- 1.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 1.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 508.9, true 0.0, total 328.9
- 1.022s [damage_in] Morgana Auto Attack -> Vladimir | physical 122.5, magic 0.0, true 0.0, total 39.6
- 1.022s [attack_hit] Morgana hit Vladimir (phys 122.5, magic 0.0, true 0.0)
- 1.041s [damage_in] Vayne Auto Attack -> Vladimir | physical 728.2, magic 0.0, true 0.0, total 235.3
- 1.041s [attack_hit] Vayne hit Vladimir (phys 728.2, magic 0.0, true 0.0)
- 1.183s [attack_start] Vayne begins auto attack
- 1.212s [damage_in] Warwick Auto Attack -> Vladimir | physical 303.0, magic 81.6, true 0.0, total 150.7
- 1.212s [attack_hit] Warwick hit Vladimir (phys 303.0, magic 81.6, true 0.0)
- 1.463s [attack_start] Dr. Mundo begins auto attack
- 1.474s [attack_start] Sona begins auto attack
- 1.500s [champion_script] Vayne executed Tumble Empower
- 1.500s [enemy_buff] Vayne empowered next attack
- 1.549s [damage_in] Vayne Auto Attack -> Vladimir | physical 705.0, magic 0.0, true 905.9, total 1133.7
- 1.549s [controlled_champion_death] Vladimir died
- 1.549s [attack_hit] Vayne hit Vladimir (phys 705.0, magic 0.0, true 905.9)
