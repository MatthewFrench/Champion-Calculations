# Vladimir Event Trace

## Optimized Build Trace
- 0.000s [state_snapshot] checkpoint 0.0s (captured_at 0.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=5996.8/5996.8 (100.0%) armor=204.5 mr=54.7
  offense: ap=270.1 ah=330.0
  loadout: items [Abyssal Mask, Bloodletter's Curse, Guardian Angel, Heartsteel, Warmog's Armor, Zhonya's Hourglass] | runes [none] | shards [none]
  cooldowns: Stasis item ready; Revive item ready
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
- 2.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 1799.1, true 0.0, total 1162.9
- 2.000s [controlled_champion_cast] Vladimir cast vladimir_transfusion on Warwick (impact in 0.20s)
- 2.000s [controlled_champion_cast] Vladimir cast vladimir_tides_of_blood (impact in 0.30s)
- 2.000s [controlled_champion_cast] Vladimir cast vladimir_hemoplague (impact in 0.25s)
- 2.130s [attack_start] Warwick begins auto attack
- 2.200s [controlled_champion_primary_hit] Vladimir vladimir_transfusion hit Warwick for 188.4
- 2.250s [controlled_champion_ultimate_hit] Vladimir vladimir_hemoplague dealt 1673.0
- 2.252s [damage_in] Sona Auto Attack -> Vladimir | physical 106.0, magic 0.0, true 0.0, total 34.8
- 2.252s [attack_hit] Sona hit Vladimir (phys 106.0, magic 0.0, true 0.0)
- 2.300s [champion_script] Morgana executed Dark Binding
- 2.300s [damage_in] Morgana Dark Binding -> Vladimir | physical 0.0, magic 908.9, true 0.0, total 587.5
- 2.300s [controlled_champion_secondary_hit] Vladimir vladimir_tides_of_blood dealt 1229.3
- 2.315s [attack_start] Dr. Mundo begins auto attack
- 2.370s [damage_in] Warwick Auto Attack -> Vladimir | physical 493.0, magic 81.6, true 0.0, total 214.7
- 2.370s [attack_hit] Warwick hit Vladimir (phys 493.0, magic 81.6, true 0.0)
- 2.470s [damage_in] Vayne Auto Attack -> Vladimir | physical 891.1, magic 0.0, true 0.0, total 292.7
- 2.470s [attack_hit] Vayne hit Vladimir (phys 891.1, magic 0.0, true 0.0)
- 2.500s [champion_script] Vayne executed Tumble Empower
- 2.500s [enemy_buff] Vayne empowered next attack
- 2.555s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 2.598s [attack_start] Vayne begins auto attack
- 2.692s [attack_start] Warwick begins auto attack
- 2.792s [attack_start] Sona begins auto attack
- 2.932s [damage_in] Warwick Auto Attack -> Vladimir | physical 462.6, magic 81.6, true 0.0, total 204.7
- 2.932s [attack_hit] Warwick hit Vladimir (phys 462.6, magic 81.6, true 0.0)
- 3.000s [champion_script] Vayne executed Tumble Empower
- 3.000s [enemy_buff] Vayne empowered next attack
- 3.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 3.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 1064.0, true 0.0, total 687.8
- 3.069s [damage_in] Vayne Auto Attack -> Vladimir | physical 1173.9, magic 0.0, true 0.0, total 385.5
- 3.069s [attack_hit] Vayne hit Vladimir (phys 1173.9, magic 0.0, true 0.0)
- 3.166s [attack_start] Dr. Mundo begins auto attack
- 3.191s [attack_start] Vayne begins auto attack
- 3.242s [attack_start] Warwick begins auto attack
- 3.378s [damage_in] Sona Auto Attack -> Vladimir | physical 106.0, magic 0.0, true 0.0, total 34.8
- 3.378s [attack_hit] Sona hit Vladimir (phys 106.0, magic 0.0, true 0.0)
- 3.406s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 3.482s [damage_in] Warwick Auto Attack -> Vladimir | physical 383.8, magic 81.6, true 171.9, total 350.7
- 3.482s [attack_hit] Warwick hit Vladimir (phys 383.8, magic 81.6, true 171.9)
- 3.500s [champion_script] Vayne executed Tumble Empower
- 3.500s [enemy_buff] Vayne empowered next attack
- 3.662s [damage_in] Vayne Auto Attack -> Vladimir | physical 773.8, magic 0.0, true 1066.5, total 1320.6
- 3.662s [attack_hit] Vayne hit Vladimir (phys 773.8, magic 0.0, true 1066.5)
- 3.779s [attack_start] Vayne begins auto attack
- 3.783s [attack_start] Warwick begins auto attack
- 3.919s [attack_start] Sona begins auto attack
- 4.000s [champion_script] Vayne executed Tumble Empower
- 4.000s [enemy_buff] Vayne empowered next attack
- 4.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 4.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 280.0, true 0.0, total 181.0
- 4.018s [attack_start] Dr. Mundo begins auto attack
- 4.023s [damage_in] Warwick Auto Attack -> Vladimir | physical 272.7, magic 81.6, true 0.0, total 142.3
- 4.023s [attack_hit] Warwick hit Vladimir (phys 272.7, magic 81.6, true 0.0)
- 4.250s [damage_in] Vayne Auto Attack -> Vladimir | physical 675.1, magic 0.0, true 0.0, total 221.7
- 4.250s [attack_hit] Vayne hit Vladimir (phys 675.1, magic 0.0, true 0.0)
- 4.258s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 4.324s [attack_start] Warwick begins auto attack
- 4.365s [attack_start] Vayne begins auto attack
- 4.500s [champion_script] Vayne executed Tumble Empower
- 4.500s [enemy_buff] Vayne empowered next attack
- 4.504s [damage_in] Sona Auto Attack -> Vladimir | physical 106.0, magic 0.0, true 0.0, total 34.8
- 4.504s [attack_hit] Sona hit Vladimir (phys 106.0, magic 0.0, true 0.0)
- 4.564s [damage_in] Warwick Auto Attack -> Vladimir | physical 248.7, magic 81.6, true 0.0, total 134.5
- 4.564s [attack_hit] Warwick hit Vladimir (phys 248.7, magic 81.6, true 0.0)
- 4.600s [champion_script] Morgana executed Dark Binding
- 4.600s [damage_in] Morgana Dark Binding -> Vladimir | physical 0.0, magic 1065.8, true 0.0, total 688.9
- 4.600s [revive_effect] Revive item restored Vladimir
- 4.836s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 4.865s [attack_start] Warwick begins auto attack
- 4.869s [attack_start] Dr. Mundo begins auto attack
- 4.950s [attack_start] Vayne begins auto attack
- 5.000s [state_snapshot] checkpoint 5.0s (captured_at 5.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1345.0/5996.8 (22.4%) armor=204.5 mr=54.7
  offense: ap=270.1 ah=330.0
  loadout: items [Abyssal Mask, Bloodletter's Curse, Guardian Angel, Heartsteel, Warmog's Armor, Zhonya's Hourglass] | runes [none] | shards [none]
  cooldowns: Stasis item ready; Revive item 74.60s
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague 24.91s
  buffs: Stunned 2.60s; Revive lockout 3.60s; Stun x1 (2.60s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2299.9/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 8]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=1522.4/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 8]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=2321.4/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.58s; Dark Binding 1.88s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.60s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=1294.4/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.04s; Crescendo 18.36s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=5477.4/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: 3.35s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 5.000s [champion_script] Vayne executed Tumble Empower
- 5.000s [enemy_buff] Vayne empowered next attack
- 5.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 5.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 5.045s [attack_start] Sona begins auto attack
- 5.105s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 5.109s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 5.406s [attack_start] Warwick begins auto attack
- 5.420s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 5.500s [champion_script] Vayne executed Tumble Empower
- 5.500s [enemy_buff] Vayne empowered next attack
- 5.534s [attack_start] Vayne begins auto attack
- 5.630s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 5.646s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 5.721s [attack_start] Dr. Mundo begins auto attack
- 5.947s [attack_start] Warwick begins auto attack
- 5.961s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 6.000s [champion_script] Vayne executed Tumble Empower
- 6.000s [enemy_buff] Vayne empowered next attack
- 6.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 6.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 6.005s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 6.119s [attack_start] Vayne begins auto attack
- 6.171s [attack_start] Sona begins auto attack
- 6.187s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 6.487s [attack_start] Warwick begins auto attack
- 6.500s [champion_script] Vayne executed Tumble Empower
- 6.500s [enemy_buff] Vayne empowered next attack
- 6.573s [attack_start] Dr. Mundo begins auto attack
- 6.589s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 6.703s [attack_start] Vayne begins auto attack
- 6.727s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 6.757s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 6.813s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 6.900s [champion_script] Morgana executed Dark Binding
- 6.900s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 7.000s [champion_script] Vayne executed Tumble Empower
- 7.000s [enemy_buff] Vayne empowered next attack
- 7.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 7.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 7.028s [attack_start] Warwick begins auto attack
- 7.174s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 7.268s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 7.288s [attack_start] Vayne begins auto attack
- 7.297s [attack_start] Sona begins auto attack
- 7.424s [attack_start] Dr. Mundo begins auto attack
- 7.500s [champion_script] Vayne executed Tumble Empower
- 7.500s [enemy_buff] Vayne empowered next attack
- 7.569s [attack_start] Warwick begins auto attack
- 7.664s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 7.758s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 7.809s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 7.872s [attack_start] Vayne begins auto attack
- 7.883s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 8.000s [champion_script] Vayne executed Tumble Empower
- 8.000s [enemy_buff] Vayne empowered next attack
- 8.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 8.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 8.110s [attack_start] Warwick begins auto attack
- 8.276s [attack_start] Dr. Mundo begins auto attack
- 8.343s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 8.350s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 8.423s [attack_start] Sona begins auto attack
- 8.456s [attack_start] Vayne begins auto attack
- 8.500s [champion_script] Vayne executed Tumble Empower
- 8.500s [enemy_buff] Vayne empowered next attack
- 8.516s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 8.623s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 8.651s [attack_start] Warwick begins auto attack
- 8.891s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 8.927s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 9.000s [champion_script] Vayne executed Tumble Empower
- 9.000s [enemy_buff] Vayne empowered next attack
- 9.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 9.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 9.009s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 9.041s [attack_start] Vayne begins auto attack
- 9.127s [attack_start] Dr. Mundo begins auto attack
- 9.191s [attack_start] Warwick begins auto attack
- 9.200s [champion_script] Morgana executed Dark Binding
- 9.200s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 9.367s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 9.431s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 9.500s [champion_script] Vayne executed Tumble Empower
- 9.500s [enemy_buff] Vayne empowered next attack
- 9.512s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 9.549s [attack_start] Sona begins auto attack
- 9.625s [attack_start] Vayne begins auto attack
- 9.732s [attack_start] Warwick begins auto attack
- 9.972s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 9.979s [attack_start] Dr. Mundo begins auto attack
- 10.000s [state_snapshot] checkpoint 10.0s (captured_at 10.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1754.4/5996.8 (29.3%) armor=204.5 mr=54.7
  offense: ap=270.1 ah=330.0
  loadout: items [Abyssal Mask, Bloodletter's Curse, Guardian Angel, Heartsteel, Warmog's Armor, Zhonya's Hourglass] | runes [none] | shards [none]
  cooldowns: Stasis item ready; Revive item 69.60s
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 2.34s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague 19.91s
  buffs: Pool untargetable 0.62s; Pool heal-over-time 0.62s; Untargetable x1 (0.62s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1834.2/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.27s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 18]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=1007.8/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.10s to impact); Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 16]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=1806.8/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.54s; Dark Binding 1.48s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.20s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=779.8/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack in-flight (0.13s to impact); Crescendo 13.36s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=5016.4/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: 3.37s (cooldown 4.00s); Heartsteel Colossal Consumption: 6.02s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.10s)
    - Sona Auto Attack -> Vladimir (impact in 0.13s)
  projectile_block_zones: none
  ```
- 10.000s [champion_script] Vayne executed Tumble Empower
- 10.000s [enemy_buff] Vayne empowered next attack
- 10.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 10.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 10.096s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 10.135s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 10.210s [attack_start] Vayne begins auto attack
- 10.219s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 10.273s [attack_start] Warwick begins auto attack
- 10.500s [champion_script] Vayne executed Tumble Empower
- 10.500s [enemy_buff] Vayne empowered next attack
- 10.513s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 10.633s [controlled_champion_cast] Vladimir cast vladimir_transfusion on Warwick (impact in 0.20s)
- 10.633s [controlled_champion_cast] Vladimir cast vladimir_tides_of_blood (impact in 0.30s)
- 10.675s [attack_start] Sona begins auto attack
- 10.680s [damage_in] Vayne Auto Attack -> Vladimir | physical 767.6, magic 0.0, true 1066.5, total 1318.6
- 10.680s [attack_hit] Vayne hit Vladimir (phys 767.6, magic 0.0, true 1066.5)
- 10.680s [controlled_champion_item_active] Vladimir activated stasis item for 2.50s
- 10.794s [attack_start] Vayne begins auto attack
- 10.814s [attack_start] Warwick begins auto attack
- 10.830s [attack_start] Dr. Mundo begins auto attack
- 10.833s [controlled_champion_primary_hit] Vladimir vladimir_transfusion hit Warwick for 188.4
- 10.933s [controlled_champion_secondary_hit] Vladimir vladimir_tides_of_blood dealt 1229.3
- 11.000s [champion_script] Vayne executed Tumble Empower
- 11.000s [enemy_buff] Vayne empowered next attack
- 11.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 11.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 11.054s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 11.070s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 11.261s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 11.265s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 11.355s [attack_start] Warwick begins auto attack
- 11.379s [attack_start] Vayne begins auto attack
- 11.500s [champion_script] Vayne executed Tumble Empower
- 11.500s [enemy_buff] Vayne empowered next attack
- 11.500s [champion_script] Morgana executed Dark Binding
- 11.500s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 11.595s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 11.682s [attack_start] Dr. Mundo begins auto attack
- 11.801s [attack_start] Sona begins auto attack
- 11.849s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 11.895s [attack_start] Warwick begins auto attack
- 11.922s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 11.963s [attack_start] Vayne begins auto attack
- 12.000s [champion_script] Vayne executed Tumble Empower
- 12.000s [enemy_buff] Vayne empowered next attack
- 12.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 12.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 12.135s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 12.387s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 12.434s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 12.436s [attack_start] Warwick begins auto attack
- 12.500s [champion_script] Vayne executed Tumble Empower
- 12.500s [enemy_buff] Vayne empowered next attack
- 12.534s [attack_start] Dr. Mundo begins auto attack
- 12.548s [attack_start] Vayne begins auto attack
- 12.676s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 12.774s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 12.927s [attack_start] Sona begins auto attack
- 12.977s [attack_start] Warwick begins auto attack
- 13.000s [champion_script] Vayne executed Tumble Empower
- 13.000s [enemy_buff] Vayne empowered next attack
- 13.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 13.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 13.018s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 13.132s [attack_start] Vayne begins auto attack
- 13.200s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 13.217s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 13.385s [attack_start] Dr. Mundo begins auto attack
- 13.500s [champion_script] Vayne executed Tumble Empower
- 13.500s [enemy_buff] Vayne empowered next attack
- 13.513s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 13.518s [attack_start] Warwick begins auto attack
- 13.603s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 13.625s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 13.717s [attack_start] Vayne begins auto attack
- 13.758s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 13.800s [champion_script] Morgana executed Dark Binding
- 13.800s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 14.000s [champion_script] Vayne executed Tumble Empower
- 14.000s [enemy_buff] Vayne empowered next attack
- 14.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 14.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 14.053s [attack_start] Sona begins auto attack
- 14.059s [attack_start] Warwick begins auto attack
- 14.187s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 14.237s [attack_start] Dr. Mundo begins auto attack
- 14.299s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 14.301s [attack_start] Vayne begins auto attack
- 14.477s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 14.500s [champion_script] Vayne executed Tumble Empower
- 14.500s [enemy_buff] Vayne empowered next attack
- 14.599s [attack_start] Warwick begins auto attack
- 14.639s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 14.772s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 14.839s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 14.886s [attack_start] Vayne begins auto attack
- 15.000s [state_snapshot] checkpoint 15.0s (captured_at 15.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1327.2/5996.8 (22.1%) armor=204.5 mr=54.7
  offense: ap=270.1 ah=330.0
  loadout: items [Abyssal Mask, Bloodletter's Curse, Guardian Angel, Heartsteel, Warmog's Armor, Zhonya's Hourglass] | runes [none] | shards [none]
  cooldowns: Stasis item 25.68s; Revive item 64.60s
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 1.92s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague 14.91s
  buffs: Pool untargetable 0.20s; Pool heal-over-time 0.20s; Untargetable x1 (0.20s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=948.4/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.14s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 27]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=237.2/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 25]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=1036.2/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.50s; Dark Binding 1.08s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.80s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=9.2/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.18s; Crescendo 8.36s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=4326.1/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.09s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: 2.63s (cooldown 4.00s); Heartsteel Colossal Consumption: 1.02s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 15.000s [champion_script] Vayne executed Tumble Empower
- 15.000s [enemy_buff] Vayne empowered next attack
- 15.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 15.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 15.088s [attack_start] Dr. Mundo begins auto attack
- 15.140s [attack_start] Warwick begins auto attack
- 15.180s [attack_start] Sona begins auto attack
- 15.233s [controlled_champion_cast] Vladimir cast vladimir_transfusion on Warwick (impact in 0.20s)
- 15.233s [controlled_champion_cast] Vladimir cast vladimir_tides_of_blood (impact in 0.30s)
- 15.328s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 15.356s [damage_in] Vayne Auto Attack -> Vladimir | physical 732.6, magic 0.0, true 0.0, total 240.6
- 15.356s [attack_hit] Vayne hit Vladimir (phys 732.6, magic 0.0, true 0.0)
- 15.380s [damage_in] Warwick Auto Attack -> Vladimir | physical 307.1, magic 81.6, true 0.0, total 153.6
- 15.380s [attack_hit] Warwick hit Vladimir (phys 307.1, magic 81.6, true 0.0)
- 15.433s [controlled_champion_primary_hit] Vladimir vladimir_transfusion hit Warwick for 188.4
- 15.470s [attack_start] Vayne begins auto attack
- 15.500s [champion_script] Vayne executed Tumble Empower
- 15.500s [enemy_buff] Vayne empowered next attack
- 15.533s [enemy_death] Vayne died; respawn in 54.5s
- 15.533s [enemy_death] Sona died; respawn in 54.5s
- 15.533s [controlled_champion_secondary_hit] Vladimir vladimir_tides_of_blood dealt 963.5
- 15.681s [attack_start] Warwick begins auto attack
- 15.921s [damage_in] Warwick Auto Attack -> Vladimir | physical 300.7, magic 81.6, true 0.0, total 151.5
- 15.921s [attack_hit] Warwick hit Vladimir (phys 300.7, magic 81.6, true 0.0)
- 15.940s [attack_start] Dr. Mundo begins auto attack
- 16.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 16.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 280.0, true 0.0, total 181.0
- 16.100s [champion_script] Morgana executed Dark Binding
- 16.100s [damage_in] Morgana Dark Binding -> Vladimir | physical 0.0, magic 908.9, true 0.0, total 587.5
- 16.180s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 16.222s [attack_start] Warwick begins auto attack
- 16.462s [damage_in] Warwick Auto Attack -> Vladimir | physical 245.5, magic 81.6, true 171.9, total 305.3
- 16.462s [controlled_champion_death] Vladimir died
- 16.462s [attack_hit] Warwick hit Vladimir (phys 245.5, magic 81.6, true 171.9)
