# Vladimir Event Trace

## Rune Proc Telemetry
- Arcane Comet: procs `2` / attempts `12` / eligible `2` (proc/attempt 16.7%, proc/eligible 100.0%), bonus damage `317.74` (4.18% share), bonus healing `0.00` (0.00% share)
  - sources: ability (procs 2, attempts 12, eligible 2, proc/attempt 16.7%, proc/eligible 100.0%, damage 317.74, healing 0.00)

## Optimized Build Trace
- 0.000s [state_snapshot] checkpoint 0.0s (captured_at 0.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=4134.7/4134.7 (100.0%) armor=204.5 mr=54.7
  offense: ap=177.4 ah=308.0
  loadout: items [Blade of the Ruined King, Guardian Angel, Guinsoo's Rageblade, Heartsteel, Infinity Edge, Zhonya's Hourglass] | runes [Arcane Comet, Nimbus Cloak, Celerity, Gathering Storm, Magical Footwear, Jack Of All Trades] | shards [ability_haste, movement_speed, health]
  cooldowns: Stasis item ready; Revive item ready
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready; Auto Attack 0.37s
  runtime: cooldowns [Heartsteel Colossal Consumption: ready (cooldown 7.50s); Arcane Comet: ready] | stacks [Guinsoo stacks: 0/8; Attacks landed: 0]
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
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s); Arcane Comet: ready] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-260.0, -130.0) hp=2413.5/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.54s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s); Summon Aery: ready] | stacks [none]
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
- 0.000s [damage_in] Morgana Dark Binding -> Vladimir | physical 0.0, magic 1141.8, true 0.0, total 738.1
- 0.000s [champion_script] Morgana executed Soul Shackles
- 0.000s [damage_in] Morgana Soul Shackles -> Vladimir | physical 0.0, magic 678.0, true 0.0, total 438.3
- 0.000s [champion_script] Sona executed Crescendo
- 0.000s [damage_in] Sona Crescendo -> Vladimir | physical 0.0, magic 869.5, true 0.0, total 562.1
- 0.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 0.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 718.9, true 0.0, total 464.7
- 0.159s [attack_start] Vayne begins auto attack
- 0.250s [controlled_champion_ultimate_hit] Vladimir vladimir_hemoplague dealt 1564.5 to 5 enemies in range
- 0.373s [attack_start] Warwick begins auto attack
- 0.500s [champion_script] Vayne executed Tumble Empower
- 0.500s [enemy_buff] Vayne empowered next attack
- 0.525s [damage_in] Vayne Auto Attack -> Vladimir | physical 1118.2, magic 0.0, true 0.0, total 367.2
- 0.525s [attack_hit] Vayne hit Vladimir (phys 1118.2, magic 0.0, true 0.0)
- 0.540s [attack_start] Sona begins auto attack
- 0.612s [attack_start] Dr. Mundo begins auto attack
- 0.613s [damage_in] Warwick Auto Attack -> Vladimir | physical 331.4, magic 81.6, true 0.0, total 161.6
- 0.613s [attack_hit] Warwick hit Vladimir (phys 331.4, magic 81.6, true 0.0)
- 0.620s [attack_start] Morgana begins auto attack
- 0.675s [attack_start] Vayne begins auto attack
- 0.852s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 0.934s [damage_in] Sona Auto Attack -> Vladimir | physical 106.0, magic 0.0, true 0.0, total 34.8
- 0.934s [attack_hit] Sona hit Vladimir (phys 106.0, magic 0.0, true 0.0)
- 0.972s [attack_start] Warwick begins auto attack
- 1.000s [champion_script] Vayne executed Tumble Empower
- 1.000s [enemy_buff] Vayne empowered next attack
- 1.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 1.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 410.4, true 0.0, total 265.3
- 1.022s [damage_in] Morgana Auto Attack -> Vladimir | physical 122.5, magic 0.0, true 0.0, total 40.2
- 1.022s [attack_hit] Morgana hit Vladimir (phys 122.5, magic 0.0, true 0.0)
- 1.041s [damage_in] Vayne Auto Attack -> Vladimir | physical 712.2, magic 0.0, true 0.0, total 233.9
- 1.041s [attack_hit] Vayne hit Vladimir (phys 712.2, magic 0.0, true 0.0)
- 1.183s [attack_start] Vayne begins auto attack
- 1.212s [damage_in] Warwick Auto Attack -> Vladimir | physical 287.2, magic 81.6, true 0.0, total 147.1
- 1.212s [attack_hit] Warwick hit Vladimir (phys 287.2, magic 81.6, true 0.0)
- 1.463s [attack_start] Dr. Mundo begins auto attack
- 1.474s [attack_start] Sona begins auto attack
- 1.500s [champion_script] Vayne executed Tumble Empower
- 1.500s [enemy_buff] Vayne empowered next attack
- 1.549s [damage_in] Vayne Auto Attack -> Vladimir | physical 689.4, magic 0.0, true 880.3, total 1106.7
- 1.549s [revive_effect] Revive item restored Vladimir
- 1.549s [attack_hit] Vayne hit Vladimir (phys 689.4, magic 0.0, true 880.3)
- 1.557s [attack_start] Warwick begins auto attack
- 1.642s [attack_start] Morgana begins auto attack
- 1.684s [attack_start] Vayne begins auto attack
- 1.703s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 1.797s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 1.868s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 2.000s [champion_script] Vayne executed Tumble Empower
- 2.000s [enemy_buff] Vayne empowered next attack
- 2.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 2.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 2.044s [impact_nullified] Morgana auto attack on Vladimir was nullified by untargetable or stasis state
- 2.049s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 2.130s [attack_start] Warwick begins auto attack
- 2.178s [attack_start] Vayne begins auto attack
- 2.300s [champion_script] Morgana executed Dark Binding
- 2.300s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 2.315s [attack_start] Dr. Mundo begins auto attack
- 2.370s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 2.408s [attack_start] Sona begins auto attack
- 2.500s [champion_script] Vayne executed Tumble Empower
- 2.500s [enemy_buff] Vayne empowered next attack
- 2.543s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 2.555s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 2.663s [attack_start] Morgana begins auto attack
- 2.666s [attack_start] Vayne begins auto attack
- 2.692s [attack_start] Warwick begins auto attack
- 2.802s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 2.932s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 3.000s [champion_script] Morgana executed Soul Shackles Detonate
- 3.000s [impact_nullified] Morgana Soul Shackles Detonate on Vladimir was nullified by untargetable or stasis state
- 3.000s [champion_script] Vayne executed Tumble Empower
- 3.000s [enemy_buff] Vayne empowered next attack
- 3.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 3.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 3.032s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 3.065s [impact_nullified] Morgana auto attack on Vladimir was nullified by untargetable or stasis state
- 3.149s [attack_start] Vayne begins auto attack
- 3.166s [attack_start] Dr. Mundo begins auto attack
- 3.242s [attack_start] Warwick begins auto attack
- 3.343s [attack_start] Sona begins auto attack
- 3.406s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 3.482s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 3.500s [champion_script] Vayne executed Tumble Empower
- 3.500s [enemy_buff] Vayne empowered next attack
- 3.514s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 3.630s [attack_start] Vayne begins auto attack
- 3.685s [attack_start] Morgana begins auto attack
- 3.736s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 3.783s [attack_start] Warwick begins auto attack
- 3.995s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 4.000s [champion_script] Vayne executed Tumble Empower
- 4.000s [enemy_buff] Vayne empowered next attack
- 4.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 4.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 4.018s [attack_start] Dr. Mundo begins auto attack
- 4.023s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 4.087s [impact_nullified] Morgana auto attack on Vladimir was nullified by untargetable or stasis state
- 4.109s [attack_start] Vayne begins auto attack
- 4.258s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 4.277s [attack_start] Sona begins auto attack
- 4.324s [attack_start] Warwick begins auto attack
- 4.475s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 4.500s [champion_script] Vayne executed Tumble Empower
- 4.500s [enemy_buff] Vayne empowered next attack
- 4.564s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 4.589s [attack_start] Vayne begins auto attack
- 4.600s [champion_script] Morgana executed Dark Binding
- 4.600s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 4.670s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 4.707s [attack_start] Morgana begins auto attack
- 4.865s [attack_start] Warwick begins auto attack
- 4.869s [attack_start] Dr. Mundo begins auto attack
- 4.954s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 5.000s [state_snapshot] checkpoint 5.0s (captured_at 5.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1345.0/4134.7 (32.5%) armor=204.5 mr=54.7
  offense: ap=177.4 ah=308.0
  loadout: items [Blade of the Ruined King, Guardian Angel, Guinsoo's Rageblade, Heartsteel, Infinity Edge, Zhonya's Hourglass] | runes [Arcane Comet, Nimbus Cloak, Celerity, Gathering Storm, Magical Footwear, Jack Of All Trades] | shards [ability_haste, movement_speed, health]
  cooldowns: Stasis item ready; Revive item 71.55s
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague 24.41s; Auto Attack 0.14s
  runtime: cooldowns [Heartsteel Colossal Consumption: ready (cooldown 7.50s); Arcane Comet: 3.25s] | stacks [Guinsoo stacks: 0/8; Attacks landed: 0]
  buffs: Revive lockout 0.55s
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=3130.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 8]
    buffs: none
  Vayne:
    core: pos=(300.0, 140.0) hp=2334.9/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.07s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 10]
    buffs: none
  Morgana:
    core: pos=(-300.0, 120.0) hp=3133.9/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack in-flight (0.11s to impact); Dark Binding 1.88s; Soul Shackles 17.83s
    runtime: cooldowns [Luden's Echo: 2.60s (cooldown 3.00s); Arcane Comet: 3.00s] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-260.0, -130.0) hp=2106.9/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.21s; Crescendo 18.36s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s); Summon Aery: ready] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=6205.2/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: 3.35s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles:
    - Morgana Auto Attack -> Vladimir (impact in 0.11s)
  projectile_block_zones: none
  ```
- 5.000s [champion_script] Vayne executed Tumble Empower
- 5.000s [enemy_buff] Vayne empowered next attack
- 5.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 5.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 5.068s [attack_start] Vayne begins auto attack
- 5.105s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 5.109s [impact_nullified] Morgana auto attack on Vladimir was nullified by untargetable or stasis state
- 5.109s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 5.211s [attack_start] Sona begins auto attack
- 5.406s [attack_start] Warwick begins auto attack
- 5.433s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 5.500s [champion_script] Vayne executed Tumble Empower
- 5.500s [enemy_buff] Vayne empowered next attack
- 5.547s [attack_start] Vayne begins auto attack
- 5.567s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s, damage tick 0.50s)
- 5.604s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 5.646s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 5.721s [attack_start] Dr. Mundo begins auto attack
- 5.729s [attack_start] Morgana begins auto attack
- 5.913s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 5.947s [attack_start] Warwick begins auto attack
- 5.961s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 6.000s [champion_script] Vayne executed Tumble Empower
- 6.000s [enemy_buff] Vayne empowered next attack
- 6.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 6.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 6.027s [attack_start] Vayne begins auto attack
- 6.033s [controlled_champion_pool_tick] Vladimir vladimir_sanguine_pool tick dealt 400.9 to 5 enemies in range
- 6.131s [impact_nullified] Morgana auto attack on Vladimir was nullified by untargetable or stasis state
- 6.145s [attack_start] Sona begins auto attack
- 6.187s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 6.392s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 6.487s [attack_start] Warwick begins auto attack
- 6.500s [champion_script] Vayne executed Tumble Empower
- 6.500s [enemy_buff] Vayne empowered next attack
- 6.506s [attack_start] Vayne begins auto attack
- 6.539s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 6.539s [controlled_champion_pool_tick] Vladimir vladimir_sanguine_pool tick dealt 400.9 to 5 enemies in range
- 6.573s [attack_start] Dr. Mundo begins auto attack
- 6.727s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 6.750s [attack_start] Morgana begins auto attack
- 6.813s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 6.872s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 6.900s [champion_script] Morgana executed Dark Binding
- 6.900s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 6.985s [attack_start] Vayne begins auto attack
- 7.000s [champion_script] Vayne executed Tumble Empower
- 7.000s [enemy_buff] Vayne empowered next attack
- 7.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 7.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 7.028s [attack_start] Warwick begins auto attack
- 7.033s [controlled_champion_pool_tick] Vladimir vladimir_sanguine_pool tick dealt 400.9 to 5 enemies in range
- 7.079s [attack_start] Sona begins auto attack
- 7.152s [impact_nullified] Morgana auto attack on Vladimir was nullified by untargetable or stasis state
- 7.268s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 7.351s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 7.424s [attack_start] Dr. Mundo begins auto attack
- 7.465s [attack_start] Vayne begins auto attack
- 7.473s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 7.500s [champion_script] Vayne executed Tumble Empower
- 7.500s [enemy_buff] Vayne empowered next attack
- 7.533s [controlled_champion_pool_tick] Vladimir vladimir_sanguine_pool tick dealt 400.9 to 5 enemies in range
- 7.569s [attack_start] Warwick begins auto attack
- 7.569s [controlled_champion_cast] Vladimir cast vladimir_transfusion on Warwick (impact in 0.20s)
- 7.664s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 7.769s [controlled_champion_primary_hit] Vladimir vladimir_transfusion hit Warwick for 155.9
- 7.769s [controlled_champion_cast] Vladimir cast vladimir_tides_of_blood (impact in 0.30s)
- 7.772s [attack_start] Morgana begins auto attack
- 7.809s [damage_in] Warwick Auto Attack -> Vladimir | physical 343.3, magic 81.6, true 0.0, total 165.5
- 7.809s [attack_hit] Warwick hit Vladimir (phys 343.3, magic 81.6, true 0.0)
- 7.830s [damage_in] Vayne Auto Attack -> Vladimir | physical 744.4, magic 0.0, true 0.0, total 244.5
- 7.830s [attack_hit] Vayne hit Vladimir (phys 744.4, magic 0.0, true 0.0)
- 7.944s [attack_start] Vayne begins auto attack
- 8.000s [champion_script] Vayne executed Tumble Empower
- 8.000s [enemy_buff] Vayne empowered next attack
- 8.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 8.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 406.2, true 0.0, total 262.6
- 8.013s [attack_start] Sona begins auto attack
- 8.069s [controlled_champion_secondary_hit] Vladimir vladimir_tides_of_blood dealt 999.1 to 5 enemies in range
- 8.076s [controlled_champion_attack_start] Vladimir begins auto attack on Warwick
- 8.110s [attack_start] Warwick begins auto attack
- 8.174s [damage_in] Morgana Auto Attack -> Vladimir | physical 122.5, magic 0.0, true 0.0, total 40.2
- 8.174s [attack_hit] Morgana hit Vladimir (phys 122.5, magic 0.0, true 0.0)
- 8.276s [attack_start] Dr. Mundo begins auto attack
- 8.310s [damage_in] Vayne Auto Attack -> Vladimir | physical 711.6, magic 0.0, true 0.0, total 233.7
- 8.310s [attack_hit] Vayne hit Vladimir (phys 711.6, magic 0.0, true 0.0)
- 8.350s [damage_in] Warwick Auto Attack -> Vladimir | physical 286.6, magic 81.6, true 171.9, total 318.7
- 8.350s [attack_hit] Warwick hit Vladimir (phys 286.6, magic 81.6, true 171.9)
- 8.350s [controlled_champion_item_active] Vladimir activated stasis item for 2.50s
- 8.364s [controlled_champion_attack_hit] Vladimir auto attacked Warwick (phys 779.1, magic 0.0, true 0.0, dealt 267.2)
- 8.407s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 8.424s [attack_start] Vayne begins auto attack
- 8.500s [champion_script] Vayne executed Tumble Empower
- 8.500s [enemy_buff] Vayne empowered next attack
- 8.516s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 8.651s [attack_start] Warwick begins auto attack
- 8.789s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 8.794s [attack_start] Morgana begins auto attack
- 8.891s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 8.903s [attack_start] Vayne begins auto attack
- 8.947s [attack_start] Sona begins auto attack
- 9.000s [champion_script] Vayne executed Tumble Empower
- 9.000s [enemy_buff] Vayne empowered next attack
- 9.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 9.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 9.127s [attack_start] Dr. Mundo begins auto attack
- 9.191s [attack_start] Warwick begins auto attack
- 9.196s [impact_nullified] Morgana auto attack on Vladimir was nullified by untargetable or stasis state
- 9.200s [champion_script] Morgana executed Dark Binding
- 9.200s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 9.269s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 9.341s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 9.367s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 9.382s [attack_start] Vayne begins auto attack
- 9.431s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 9.500s [champion_script] Vayne executed Tumble Empower
- 9.500s [enemy_buff] Vayne empowered next attack
- 9.732s [attack_start] Warwick begins auto attack
- 9.748s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 9.816s [attack_start] Morgana begins auto attack
- 9.862s [attack_start] Vayne begins auto attack
- 9.881s [attack_start] Sona begins auto attack
- 9.972s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 9.979s [attack_start] Dr. Mundo begins auto attack
- 10.000s [state_snapshot] checkpoint 10.0s (captured_at 10.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=498.9/4134.7 (12.1%) armor=204.5 mr=54.7
  offense: ap=177.4 ah=308.0
  loadout: items [Blade of the Ruined King, Guardian Angel, Guinsoo's Rageblade, Heartsteel, Infinity Edge, Zhonya's Hourglass] | runes [Arcane Comet, Nimbus Cloak, Celerity, Gathering Storm, Magical Footwear, Jack Of All Trades] | shards [ability_haste, movement_speed, health]
  cooldowns: Stasis item 28.35s; Revive item 66.55s
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague 19.41s; Auto Attack 0.16s
  runtime: cooldowns [Heartsteel Colossal Consumption: 5.86s (cooldown 7.50s); Arcane Comet: ready] | stacks [Guinsoo stacks: 1/8; Attacks landed: 1]
  buffs: Stasis 0.85s; Stasis x1 (0.85s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2217.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.27s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 18]
    buffs: none
  Vayne:
    core: pos=(300.0, 140.0) hp=1792.8/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 20]
    buffs: none
  Morgana:
    core: pos=(-300.0, 120.0) hp=2591.8/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack unavailable; Dark Binding 1.48s; Soul Shackles 12.83s
    runtime: cooldowns [Luden's Echo: 2.20s (cooldown 3.00s); Arcane Comet: 7.20s] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-260.0, -130.0) hp=1564.8/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack unavailable; Crescendo 13.36s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s); Summon Aery: 0.41s] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=5719.6/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: 3.37s (cooldown 4.00s); Heartsteel Colossal Consumption: 6.02s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 10.000s [champion_script] Vayne executed Tumble Empower
- 10.000s [enemy_buff] Vayne empowered next attack
- 10.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 10.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 10.218s [impact_nullified] Morgana auto attack on Vladimir was nullified by untargetable or stasis state
- 10.219s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 10.227s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 10.273s [attack_start] Warwick begins auto attack
- 10.275s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 10.341s [attack_start] Vayne begins auto attack
- 10.500s [champion_script] Vayne executed Tumble Empower
- 10.500s [enemy_buff] Vayne empowered next attack
- 10.513s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 10.707s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 10.814s [attack_start] Warwick begins auto attack
- 10.815s [attack_start] Sona begins auto attack
- 10.821s [attack_start] Vayne begins auto attack
- 10.830s [attack_start] Dr. Mundo begins auto attack
- 10.837s [attack_start] Morgana begins auto attack
- 10.867s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s, damage tick 0.50s)
- 11.000s [champion_script] Vayne executed Tumble Empower
- 11.000s [enemy_buff] Vayne empowered next attack
- 11.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 11.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 11.054s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 11.070s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 11.186s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 11.209s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 11.239s [impact_nullified] Morgana auto attack on Vladimir was nullified by untargetable or stasis state
- 11.300s [attack_start] Vayne begins auto attack
- 11.355s [attack_start] Warwick begins auto attack
- 11.355s [controlled_champion_pool_tick] Vladimir vladimir_sanguine_pool tick dealt 569.0 to 5 enemies in range
- 11.500s [champion_script] Vayne executed Tumble Empower
- 11.500s [enemy_buff] Vayne empowered next attack
- 11.500s [champion_script] Morgana executed Dark Binding
- 11.500s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 11.595s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 11.665s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 11.682s [attack_start] Dr. Mundo begins auto attack
- 11.749s [attack_start] Sona begins auto attack
- 11.779s [attack_start] Vayne begins auto attack
- 11.859s [attack_start] Morgana begins auto attack
- 11.859s [controlled_champion_pool_tick] Vladimir vladimir_sanguine_pool tick dealt 569.0 to 5 enemies in range
- 11.895s [attack_start] Warwick begins auto attack
- 11.922s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 12.000s [champion_script] Vayne executed Tumble Empower
- 12.000s [enemy_buff] Vayne empowered next attack
- 12.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 12.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 12.135s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 12.143s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 12.145s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 12.259s [attack_start] Vayne begins auto attack
- 12.261s [impact_nullified] Morgana auto attack on Vladimir was nullified by untargetable or stasis state
- 12.333s [controlled_champion_pool_tick] Vladimir vladimir_sanguine_pool tick dealt 569.0 to 5 enemies in range
- 12.436s [attack_start] Warwick begins auto attack
- 12.500s [champion_script] Vayne executed Tumble Empower
- 12.500s [enemy_buff] Vayne empowered next attack
- 12.534s [attack_start] Dr. Mundo begins auto attack
- 12.624s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 12.676s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 12.683s [attack_start] Sona begins auto attack
- 12.738s [attack_start] Vayne begins auto attack
- 12.774s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 12.833s [controlled_champion_pool_tick] Vladimir vladimir_sanguine_pool tick dealt 569.0 to 5 enemies in range
- 12.881s [attack_start] Morgana begins auto attack
- 12.881s [controlled_champion_cast] Vladimir cast vladimir_transfusion on Warwick (impact in 0.20s)
- 12.977s [attack_start] Warwick begins auto attack
- 13.000s [champion_script] Vayne executed Tumble Empower
- 13.000s [enemy_buff] Vayne empowered next attack
- 13.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 13.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 343.3, true 0.0, total 221.9
- 13.077s [damage_in] Sona Auto Attack -> Vladimir | physical 106.0, magic 0.0, true 0.0, total 34.8
- 13.077s [attack_hit] Sona hit Vladimir (phys 106.0, magic 0.0, true 0.0)
- 13.081s [controlled_champion_primary_hit] Vladimir vladimir_transfusion hit Warwick for 248.8
- 13.100s [controlled_champion_cast] Vladimir cast vladimir_tides_of_blood (impact in 0.30s)
- 13.104s [damage_in] Vayne Auto Attack -> Vladimir | physical 705.5, magic 0.0, true 880.3, total 1112.0
- 13.104s [controlled_champion_death] Vladimir died
- 13.104s [attack_hit] Vayne hit Vladimir (phys 705.5, magic 0.0, true 880.3)
