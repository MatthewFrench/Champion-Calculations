# Vladimir Event Trace

## Optimized Build Trace
- 0.000s [state_snapshot] checkpoint 0.0s (captured_at 0.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=4168.5/4168.5 (100.0%) armor=109.5 mr=54.7
  offense: ap=134.0 ah=325.0
  loadout: items [Abyssal Mask, Actualizer, Endless Hunger, Kaenic Rookern, Spear of Shojin, Youmuu's Ghostblade] | runes [Arcane Comet, Nimbus Cloak, Transcendence, Gathering Storm, Shield Bash, Overgrowth] | shards [attack_speed, health, tenacity]
  cooldowns: Arcane Comet: ready
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
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
    core: pos=(520.0, 150.0) hp=2641.5/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.16s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=3440.5/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.62s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=2413.5/2413.5 armor=105.8 mr=54.7
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
- 0.000s [controlled_champion_cast] Vladimir cast vladimir_transfusion on Warwick (impact in 0.20s)
- 0.000s [controlled_champion_cast] Vladimir cast vladimir_tides_of_blood (impact in 0.30s)
- 0.000s [controlled_champion_cast] Vladimir cast vladimir_hemoplague (impact in 0.25s)
- 0.000s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 0.000s [champion_script] Vayne executed Tumble Empower
- 0.000s [enemy_buff] Vayne empowered next attack
- 0.000s [champion_script] Morgana executed Dark Binding
- 0.000s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 0.000s [champion_script] Sona executed Crescendo
- 0.000s [impact_nullified] Sona Crescendo on Vladimir was nullified by untargetable or stasis state
- 0.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 0.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 0.159s [attack_start] Vayne begins auto attack
- 0.200s [controlled_champion_primary_hit] Vladimir vladimir_transfusion hit Warwick for 173.9
- 0.250s [controlled_champion_ultimate_hit] Vladimir vladimir_hemoplague dealt 1377.3
- 0.300s [controlled_champion_secondary_hit] Vladimir vladimir_tides_of_blood dealt 891.3
- 0.373s [attack_start] Warwick begins auto attack
- 0.500s [champion_script] Vayne executed Tumble Empower
- 0.500s [enemy_buff] Vayne empowered next attack
- 0.540s [attack_start] Sona begins auto attack
- 0.612s [attack_start] Dr. Mundo begins auto attack
- 0.613s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 0.630s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 0.780s [attack_start] Vayne begins auto attack
- 0.852s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 0.972s [attack_start] Warwick begins auto attack
- 1.000s [champion_script] Vayne executed Tumble Empower
- 1.000s [enemy_buff] Vayne empowered next attack
- 1.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 1.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 1.126s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 1.212s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 1.251s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 1.393s [attack_start] Vayne begins auto attack
- 1.463s [attack_start] Dr. Mundo begins auto attack
- 1.500s [champion_script] Vayne executed Tumble Empower
- 1.500s [enemy_buff] Vayne empowered next attack
- 1.557s [attack_start] Warwick begins auto attack
- 1.666s [attack_start] Sona begins auto attack
- 1.703s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 1.797s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 1.864s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 1.999s [attack_start] Vayne begins auto attack
- 2.000s [champion_script] Vayne executed Tumble Empower
- 2.000s [enemy_buff] Vayne empowered next attack
- 2.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 2.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 1250.5, true 0.0, total 808.4
- 2.000s [controlled_champion_cast] Vladimir cast vladimir_transfusion on Warwick (impact in 0.20s)
- 2.000s [controlled_champion_cast] Vladimir cast vladimir_tides_of_blood (impact in 0.30s)
- 2.130s [attack_start] Warwick begins auto attack
- 2.200s [controlled_champion_primary_hit] Vladimir vladimir_transfusion hit Warwick for 140.6
- 2.252s [damage_in] Sona Auto Attack -> Vladimir | physical 106.0, magic 0.0, true 0.0, total 50.6
- 2.252s [attack_hit] Sona hit Vladimir (phys 106.0, magic 0.0, true 0.0)
- 2.300s [champion_script] Morgana executed Dark Binding
- 2.300s [damage_in] Morgana Dark Binding -> Vladimir | physical 0.0, magic 835.7, true 0.0, total 540.2
- 2.300s [controlled_champion_secondary_hit] Vladimir vladimir_tides_of_blood dealt 891.3
- 2.315s [attack_start] Dr. Mundo begins auto attack
- 2.370s [damage_in] Warwick Auto Attack -> Vladimir | physical 405.8, magic 81.6, true 0.0, total 246.4
- 2.370s [attack_hit] Warwick hit Vladimir (phys 405.8, magic 81.6, true 0.0)
- 2.470s [damage_in] Vayne Auto Attack -> Vladimir | physical 802.0, magic 0.0, true 0.0, total 382.8
- 2.470s [attack_hit] Vayne hit Vladimir (phys 802.0, magic 0.0, true 0.0)
- 2.500s [champion_script] Vayne executed Tumble Empower
- 2.500s [enemy_buff] Vayne empowered next attack
- 2.555s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 2.598s [attack_start] Vayne begins auto attack
- 2.692s [attack_start] Warwick begins auto attack
- 2.792s [attack_start] Sona begins auto attack
- 2.932s [damage_in] Warwick Auto Attack -> Vladimir | physical 368.0, magic 81.6, true 0.0, total 228.4
- 2.932s [attack_hit] Warwick hit Vladimir (phys 368.0, magic 81.6, true 0.0)
- 3.000s [champion_script] Vayne executed Tumble Empower
- 3.000s [enemy_buff] Vayne empowered next attack
- 3.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 3.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 584.0, true 0.0, total 377.5
- 3.069s [damage_in] Vayne Auto Attack -> Vladimir | physical 1096.5, magic 0.0, true 0.0, total 523.4
- 3.069s [attack_hit] Vayne hit Vladimir (phys 1096.5, magic 0.0, true 0.0)
- 3.166s [attack_start] Dr. Mundo begins auto attack
- 3.191s [attack_start] Vayne begins auto attack
- 3.242s [attack_start] Warwick begins auto attack
- 3.378s [damage_in] Sona Auto Attack -> Vladimir | physical 106.0, magic 0.0, true 0.0, total 50.6
- 3.378s [attack_hit] Sona hit Vladimir (phys 106.0, magic 0.0, true 0.0)
- 3.406s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 3.482s [damage_in] Warwick Auto Attack -> Vladimir | physical 297.2, magic 81.6, true 171.9, total 366.5
- 3.482s [attack_hit] Warwick hit Vladimir (phys 297.2, magic 81.6, true 171.9)
- 3.500s [champion_script] Vayne executed Tumble Empower
- 3.500s [enemy_buff] Vayne empowered next attack
- 3.662s [damage_in] Vayne Auto Attack -> Vladimir | physical 686.2, magic 0.0, true 883.7, total 1211.2
- 3.662s [controlled_champion_death] Vladimir died
- 3.662s [attack_hit] Vayne hit Vladimir (phys 686.2, magic 0.0, true 883.7)
