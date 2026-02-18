# Vladimir Event Trace

## Optimized Build Trace
- 0.000s [state_snapshot] checkpoint 0.0s (captured_at 0.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=3688.5/3688.5 (100.0%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
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
- 2.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 1106.5, true 0.0, total 715.3
- 2.000s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 2.130s [attack_start] Warwick begins auto attack
- 2.252s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 2.300s [champion_script] Morgana executed Dark Binding
- 2.300s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 2.315s [attack_start] Dr. Mundo begins auto attack
- 2.370s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 2.470s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 2.500s [champion_script] Vayne executed Tumble Empower
- 2.500s [enemy_buff] Vayne empowered next attack
- 2.555s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 2.598s [attack_start] Vayne begins auto attack
- 2.692s [attack_start] Warwick begins auto attack
- 2.792s [attack_start] Sona begins auto attack
- 2.932s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 3.000s [champion_script] Vayne executed Tumble Empower
- 3.000s [enemy_buff] Vayne empowered next attack
- 3.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 3.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 3.069s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 3.166s [attack_start] Dr. Mundo begins auto attack
- 3.191s [attack_start] Vayne begins auto attack
- 3.242s [attack_start] Warwick begins auto attack
- 3.378s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 3.406s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 3.482s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 3.500s [champion_script] Vayne executed Tumble Empower
- 3.500s [enemy_buff] Vayne empowered next attack
- 3.662s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 3.779s [attack_start] Vayne begins auto attack
- 3.783s [attack_start] Warwick begins auto attack
- 3.919s [attack_start] Sona begins auto attack
- 4.000s [champion_script] Vayne executed Tumble Empower
- 4.000s [enemy_buff] Vayne empowered next attack
- 4.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 4.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 4.018s [attack_start] Dr. Mundo begins auto attack
- 4.018s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 4.023s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 4.250s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 4.258s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 4.324s [attack_start] Warwick begins auto attack
- 4.365s [attack_start] Vayne begins auto attack
- 4.500s [champion_script] Vayne executed Tumble Empower
- 4.500s [enemy_buff] Vayne empowered next attack
- 4.504s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 4.564s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 4.600s [champion_script] Morgana executed Dark Binding
- 4.600s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 4.836s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 4.865s [attack_start] Warwick begins auto attack
- 4.869s [attack_start] Dr. Mundo begins auto attack
- 4.950s [attack_start] Vayne begins auto attack
- 5.000s [state_snapshot] checkpoint 5.0s (captured_at 5.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=3136.9/3688.5 (85.0%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.02s; Pool heal-over-time 1.02s; Untargetable x1 (1.02s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2711.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 8]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=1769.3/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 8]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=2568.3/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.58s; Dark Binding 1.88s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.60s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=1541.3/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.04s; Crescendo 18.36s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=5698.5/6479.8 armor=192.5 mr=72.7
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
- 6.033s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
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
- 8.058s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 8.110s [attack_start] Warwick begins auto attack
- 8.276s [attack_start] Dr. Mundo begins auto attack
- 8.343s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 8.350s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 8.423s [attack_start] Sona begins auto attack
- 8.456s [attack_start] Vayne begins auto attack
- 8.500s [champion_script] Vayne executed Tumble Empower
- 8.500s [enemy_buff] Vayne empowered next attack
- 8.516s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
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
  core: pos=(0.0, 0.0) hp=3660.3/3688.5 (99.2%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.06s; Pool heal-over-time 0.06s; Untargetable x1 (0.06s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2185.5/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.27s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 18]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=1187.8/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.10s to impact); Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 16]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=1986.8/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.54s; Dark Binding 1.48s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.20s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=959.8/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack in-flight (0.13s to impact); Crescendo 13.36s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=5177.6/6479.8 armor=192.5 mr=72.7
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
- 10.067s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 10.096s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 10.135s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 10.210s [attack_start] Vayne begins auto attack
- 10.219s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 10.273s [attack_start] Warwick begins auto attack
- 10.500s [champion_script] Vayne executed Tumble Empower
- 10.500s [enemy_buff] Vayne empowered next attack
- 10.513s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 10.675s [attack_start] Sona begins auto attack
- 10.680s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 10.794s [attack_start] Vayne begins auto attack
- 10.814s [attack_start] Warwick begins auto attack
- 10.830s [attack_start] Dr. Mundo begins auto attack
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
- 12.100s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
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
- 14.133s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
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
  core: pos=(0.0, 0.0) hp=3593.3/3688.5 (97.4%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.13s; Pool heal-over-time 1.13s; Untargetable x1 (1.13s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1396.2/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.14s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 27]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=315.6/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 25]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=1114.6/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.50s; Dark Binding 1.08s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.80s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=87.6/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.18s; Crescendo 8.36s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=4396.3/6479.8 armor=192.5 mr=72.7
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
- 15.328s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 15.356s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 15.380s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 15.470s [attack_start] Vayne begins auto attack
- 15.500s [champion_script] Vayne executed Tumble Empower
- 15.500s [enemy_buff] Vayne empowered next attack
- 15.681s [attack_start] Warwick begins auto attack
- 15.765s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 15.921s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 15.940s [attack_start] Dr. Mundo begins auto attack
- 15.941s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 16.000s [champion_script] Vayne executed Tumble Empower
- 16.000s [enemy_buff] Vayne empowered next attack
- 16.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 16.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 16.054s [attack_start] Vayne begins auto attack
- 16.100s [champion_script] Morgana executed Dark Binding
- 16.100s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 16.133s [enemy_death] Sona died; respawn in 54.5s
- 16.133s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 16.180s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 16.222s [attack_start] Warwick begins auto attack
- 16.462s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 16.500s [champion_script] Vayne executed Tumble Empower
- 16.500s [enemy_buff] Vayne empowered next attack
- 16.525s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 16.639s [attack_start] Vayne begins auto attack
- 16.763s [attack_start] Warwick begins auto attack
- 16.791s [attack_start] Dr. Mundo begins auto attack
- 17.000s [champion_script] Vayne executed Tumble Empower
- 17.000s [enemy_buff] Vayne empowered next attack
- 17.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 17.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 17.003s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 17.031s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 17.109s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 17.223s [attack_start] Vayne begins auto attack
- 17.304s [attack_start] Warwick begins auto attack
- 17.500s [champion_script] Vayne executed Tumble Empower
- 17.500s [enemy_buff] Vayne empowered next attack
- 17.544s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 17.643s [attack_start] Dr. Mundo begins auto attack
- 17.694s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 17.808s [attack_start] Vayne begins auto attack
- 17.844s [attack_start] Warwick begins auto attack
- 17.883s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 18.000s [champion_script] Vayne executed Tumble Empower
- 18.000s [enemy_buff] Vayne empowered next attack
- 18.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 18.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 18.084s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 18.133s [enemy_death] Vayne died; respawn in 54.5s
- 18.133s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 18.385s [attack_start] Warwick begins auto attack
- 18.400s [champion_script] Morgana executed Dark Binding
- 18.400s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 18.495s [attack_start] Dr. Mundo begins auto attack
- 18.625s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 18.735s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 18.926s [attack_start] Warwick begins auto attack
- 19.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 19.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 19.166s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 19.346s [attack_start] Dr. Mundo begins auto attack
- 19.467s [attack_start] Warwick begins auto attack
- 19.586s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 19.707s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 20.000s [state_snapshot] checkpoint 20.0s (captured_at 20.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=3646.8/3688.5 (98.9%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.13s; Pool heal-over-time 0.13s; Untargetable x1 (0.13s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=870.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.01s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 36]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.03s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 52.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=533.1/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.45s; Dark Binding 0.68s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.40s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.09s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 50.63s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3875.5/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.20s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: 1.88s (cooldown 4.00s); Heartsteel Colossal Consumption: 3.68s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 20.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 20.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 20.008s [attack_start] Warwick begins auto attack
- 20.133s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 20.198s [attack_start] Dr. Mundo begins auto attack
- 20.248s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 20.438s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 20.548s [attack_start] Warwick begins auto attack
- 20.700s [champion_script] Morgana executed Dark Binding
- 20.700s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 20.788s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 21.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 21.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 21.049s [attack_start] Dr. Mundo begins auto attack
- 21.089s [attack_start] Warwick begins auto attack
- 21.289s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 21.329s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 21.630s [attack_start] Warwick begins auto attack
- 21.870s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 21.901s [attack_start] Dr. Mundo begins auto attack
- 22.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 22.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 22.133s [enemy_death] Morgana died; respawn in 54.5s
- 22.133s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 22.141s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 22.171s [attack_start] Warwick begins auto attack
- 22.411s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 22.712s [attack_start] Warwick begins auto attack
- 22.752s [attack_start] Dr. Mundo begins auto attack
- 22.952s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 22.992s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 23.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 23.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 23.252s [attack_start] Warwick begins auto attack
- 23.492s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 23.604s [attack_start] Dr. Mundo begins auto attack
- 23.793s [attack_start] Warwick begins auto attack
- 23.844s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 24.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 24.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 24.033s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 24.133s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 24.334s [attack_start] Warwick begins auto attack
- 24.456s [attack_start] Dr. Mundo begins auto attack
- 24.574s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 24.696s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 24.875s [attack_start] Warwick begins auto attack
- 25.000s [state_snapshot] checkpoint 25.0s (captured_at 25.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=3389.2/3688.5 (91.9%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.13s; Pool heal-over-time 1.13s; Untargetable x1 (1.13s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=80.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 45]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.13s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 47.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.41s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 51.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.49s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 45.63s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3094.2/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.31s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: 1.14s (cooldown 4.00s); Heartsteel Colossal Consumption: 6.34s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 25.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 25.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 25.115s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 25.307s [attack_start] Dr. Mundo begins auto attack
- 25.416s [attack_start] Warwick begins auto attack
- 25.547s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 25.656s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 25.956s [attack_start] Warwick begins auto attack
- 26.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 26.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 26.133s [enemy_death] Warwick died; respawn in 54.5s
- 26.133s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 26.159s [attack_start] Dr. Mundo begins auto attack
- 26.399s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 27.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 27.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 27.010s [attack_start] Dr. Mundo begins auto attack
- 27.250s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 27.862s [attack_start] Dr. Mundo begins auto attack
- 28.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 28.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 28.102s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 28.133s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 28.713s [attack_start] Dr. Mundo begins auto attack
- 28.953s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 29.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 29.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 29.565s [attack_start] Dr. Mundo begins auto attack
- 29.805s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 30.000s [state_snapshot] checkpoint 30.0s (captured_at 30.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=3143.6/3688.5 (85.2%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.13s; Pool heal-over-time 0.13s; Untargetable x1 (0.13s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.30s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 50.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.07s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 42.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.37s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 46.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.35s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 40.63s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=2573.3/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.42s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: 0.40s (cooldown 4.00s); Heartsteel Colossal Consumption: 1.34s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 30.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 30.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 30.133s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 30.417s [attack_start] Dr. Mundo begins auto attack
- 30.657s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 31.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 31.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 31.268s [attack_start] Dr. Mundo begins auto attack
- 31.508s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 32.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 32.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 32.120s [attack_start] Dr. Mundo begins auto attack
- 32.133s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 32.360s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 32.971s [attack_start] Dr. Mundo begins auto attack
- 33.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 33.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 33.211s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 33.823s [attack_start] Dr. Mundo begins auto attack
- 34.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 34.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 34.063s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 34.135s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 34.674s [attack_start] Dr. Mundo begins auto attack
- 34.914s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 35.000s [state_snapshot] checkpoint 35.0s (captured_at 35.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2665.0/3688.5 (72.3%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.14s; Pool heal-over-time 1.14s; Untargetable x1 (1.14s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.15s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 45.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.02s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 37.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.33s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 41.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.22s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 35.63s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=1792.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.53s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: 3.91s (cooldown 4.00s); Heartsteel Colossal Consumption: 4.01s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 35.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 35.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 35.526s [attack_start] Dr. Mundo begins auto attack
- 35.766s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 36.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 36.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 36.167s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 36.378s [attack_start] Dr. Mundo begins auto attack
- 36.618s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 37.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 37.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 37.229s [attack_start] Dr. Mundo begins auto attack
- 37.469s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 38.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 38.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 38.081s [attack_start] Dr. Mundo begins auto attack
- 38.200s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 38.321s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 38.932s [attack_start] Dr. Mundo begins auto attack
- 39.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 39.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 39.172s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 39.784s [attack_start] Dr. Mundo begins auto attack
- 40.000s [state_snapshot] checkpoint 40.0s (captured_at 40.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2460.8/3688.5 (66.7%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.20s; Pool heal-over-time 0.20s; Untargetable x1 (0.20s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.37s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 40.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.12s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 32.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.29s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 36.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.08s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 30.63s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=1271.1/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: 3.17s (cooldown 4.00s); Heartsteel Colossal Consumption: 6.67s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 40.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 40.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 40.024s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 40.200s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 40.635s [attack_start] Dr. Mundo begins auto attack
- 40.875s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 41.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 41.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 41.487s [attack_start] Dr. Mundo begins auto attack
- 41.727s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 42.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 42.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 42.200s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 42.339s [attack_start] Dr. Mundo begins auto attack
- 42.579s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 43.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 43.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 43.190s [attack_start] Dr. Mundo begins auto attack
- 43.430s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 44.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 44.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 44.042s [attack_start] Dr. Mundo begins auto attack
- 44.200s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 44.282s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 44.893s [attack_start] Dr. Mundo begins auto attack
- 45.000s [state_snapshot] checkpoint 45.0s (captured_at 45.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2124.2/3688.5 (57.6%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.20s; Pool heal-over-time 1.20s; Untargetable x1 (1.20s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.22s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 35.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.06s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 27.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.25s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 31.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.48s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 25.63s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=489.8/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: 2.43s (cooldown 4.00s); Heartsteel Colossal Consumption: 1.67s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 45.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 45.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 45.133s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 45.745s [attack_start] Dr. Mundo begins auto attack
- 45.985s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 46.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 46.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 46.200s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 46.596s [attack_start] Dr. Mundo begins auto attack
- 46.836s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 47.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 47.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 47.448s [attack_start] Dr. Mundo begins auto attack
- 47.688s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 48.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 48.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 48.200s [enemy_death] Dr. Mundo died; respawn in 54.5s
- 48.200s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 50.000s [state_snapshot] checkpoint 50.0s (captured_at 50.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1991.8/3688.5 (54.0%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.20s; Pool heal-over-time 0.20s; Untargetable x1 (0.20s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.07s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 30.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.16s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 22.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.21s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 26.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.34s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 20.63s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.13s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: 1.69s (cooldown 4.00s); Heartsteel Colossal Consumption: 4.34s (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 52.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 50.205s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 52.231s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 54.233s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 55.000s [state_snapshot] checkpoint 55.0s (captured_at 55.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1581.9/3688.5 (42.9%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.23s; Pool heal-over-time 1.23s; Untargetable x1 (1.23s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.29s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 25.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.10s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 17.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.16s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 21.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.21s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 15.63s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.03s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 47.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 56.250s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 58.263s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 60.000s [state_snapshot] checkpoint 60.0s (captured_at 60.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1353.5/3688.5 (36.7%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.26s; Pool heal-over-time 0.26s; Untargetable x1 (0.26s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.14s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 20.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.04s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 12.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.12s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 16.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.07s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 10.63s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.53s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 42.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 60.267s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 62.273s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 64.300s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 65.000s [state_snapshot] checkpoint 65.0s (captured_at 65.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1071.2/3688.5 (29.0%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.30s; Pool heal-over-time 1.30s; Untargetable x1 (1.30s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.36s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 15.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.14s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 7.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.08s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 11.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.47s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 5.63s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.42s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 37.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 66.321s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 68.330s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 70.000s [state_snapshot] checkpoint 70.0s (captured_at 70.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=916.6/3688.5 (24.8%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.33s; Pool heal-over-time 0.33s; Untargetable x1 (0.33s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.21s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 10.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.08s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 2.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.04s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 6.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.33s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 0.63s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.32s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 32.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 70.333s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 70.659s [enemy_respawn] Sona respawned
- 70.659s [champion_script] Sona executed Crescendo
- 70.659s [impact_nullified] Sona Crescendo on Vladimir was nullified by untargetable or stasis state
- 70.875s [attack_start] Sona begins auto attack
- 71.461s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 72.001s [attack_start] Sona begins auto attack
- 72.359s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 72.587s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 72.634s [enemy_respawn] Vayne respawned
- 72.634s [attack_start] Vayne begins auto attack
- 72.634s [champion_script] Vayne executed Tumble Empower
- 72.634s [enemy_buff] Vayne empowered next attack
- 73.104s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 73.127s [attack_start] Sona begins auto attack
- 73.134s [champion_script] Vayne executed Tumble Empower
- 73.134s [enemy_buff] Vayne empowered next attack
- 73.255s [attack_start] Vayne begins auto attack
- 73.634s [champion_script] Vayne executed Tumble Empower
- 73.634s [enemy_buff] Vayne empowered next attack
- 73.713s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 73.725s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 73.868s [attack_start] Vayne begins auto attack
- 74.134s [champion_script] Vayne executed Tumble Empower
- 74.134s [enemy_buff] Vayne empowered next attack
- 74.253s [attack_start] Sona begins auto attack
- 74.338s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 74.367s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 74.473s [attack_start] Vayne begins auto attack
- 74.634s [champion_script] Vayne executed Tumble Empower
- 74.634s [enemy_buff] Vayne empowered next attack
- 74.839s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 74.944s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 75.000s [state_snapshot] checkpoint 75.0s (captured_at 75.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=861.3/3688.5 (23.4%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.37s; Pool heal-over-time 1.37s; Untargetable x1 (1.37s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.06s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 5.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=2350.7/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=7.779 (interval 0.129s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.07s; Tumble Empower 0.13s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 4/6; Guinsoo stacks: 4/8; Attacks landed: 4]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.62s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 1.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=1832.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.38s; Crescendo 19.02s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.21s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 27.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 75.073s [attack_start] Vayne begins auto attack
- 75.134s [champion_script] Vayne executed Tumble Empower
- 75.134s [enemy_buff] Vayne empowered next attack
- 75.379s [attack_start] Sona begins auto attack
- 75.543s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 75.634s [champion_script] Vayne executed Tumble Empower
- 75.634s [enemy_buff] Vayne empowered next attack
- 75.666s [attack_start] Vayne begins auto attack
- 75.965s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 76.134s [champion_script] Vayne executed Tumble Empower
- 76.134s [enemy_buff] Vayne empowered next attack
- 76.136s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 76.254s [attack_start] Vayne begins auto attack
- 76.400s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 76.505s [attack_start] Sona begins auto attack
- 76.634s [enemy_respawn] Morgana respawned
- 76.634s [champion_script] Vayne executed Tumble Empower
- 76.634s [enemy_buff] Vayne empowered next attack
- 76.634s [champion_script] Morgana executed Dark Binding
- 76.634s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 76.724s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 76.840s [attack_start] Vayne begins auto attack
- 77.091s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 77.134s [champion_script] Vayne executed Tumble Empower
- 77.134s [enemy_buff] Vayne empowered next attack
- 77.310s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 77.424s [attack_start] Vayne begins auto attack
- 77.632s [attack_start] Sona begins auto attack
- 77.634s [champion_script] Vayne executed Tumble Empower
- 77.634s [enemy_buff] Vayne empowered next attack
- 77.895s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 78.009s [attack_start] Vayne begins auto attack
- 78.134s [champion_script] Vayne executed Tumble Empower
- 78.134s [enemy_buff] Vayne empowered next attack
- 78.217s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 78.417s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 78.479s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 78.593s [attack_start] Vayne begins auto attack
- 78.634s [champion_script] Vayne executed Tumble Empower
- 78.634s [enemy_buff] Vayne empowered next attack
- 78.758s [attack_start] Sona begins auto attack
- 78.934s [champion_script] Morgana executed Dark Binding
- 78.934s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 79.064s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 79.134s [champion_script] Vayne executed Tumble Empower
- 79.134s [enemy_buff] Vayne empowered next attack
- 79.178s [attack_start] Vayne begins auto attack
- 79.343s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 79.634s [champion_script] Vayne executed Tumble Empower
- 79.634s [enemy_buff] Vayne empowered next attack
- 79.648s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 79.762s [attack_start] Vayne begins auto attack
- 79.884s [attack_start] Sona begins auto attack
- 80.000s [state_snapshot] checkpoint 80.0s (captured_at 80.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1207.4/3688.5 (32.7%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.42s; Pool heal-over-time 0.42s; Untargetable x1 (0.42s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.28s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 0.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=1769.3/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.23s to impact); Tumble Empower 0.13s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 12]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=3149.7/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.58s; Dark Binding 1.22s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=1250.5/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack unavailable; Crescendo 14.02s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.10s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 22.70s
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.23s)
  projectile_block_zones: none
  ```
- 80.134s [champion_script] Vayne executed Tumble Empower
- 80.134s [enemy_buff] Vayne empowered next attack
- 80.233s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 80.347s [attack_start] Vayne begins auto attack
- 80.433s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 80.470s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 80.634s [enemy_respawn] Warwick respawned
- 80.634s [champion_script] Vayne executed Tumble Empower
- 80.634s [enemy_buff] Vayne empowered next attack
- 80.655s [attack_start] Warwick begins auto attack
- 80.817s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 80.895s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 80.931s [attack_start] Vayne begins auto attack
- 81.010s [attack_start] Sona begins auto attack
- 81.134s [champion_script] Vayne executed Tumble Empower
- 81.134s [enemy_buff] Vayne empowered next attack
- 81.234s [champion_script] Morgana executed Dark Binding
- 81.234s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 81.254s [attack_start] Warwick begins auto attack
- 81.402s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 81.494s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 81.515s [attack_start] Vayne begins auto attack
- 81.596s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 81.634s [champion_script] Vayne executed Tumble Empower
- 81.634s [enemy_buff] Vayne empowered next attack
- 81.839s [attack_start] Warwick begins auto attack
- 81.986s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 82.079s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 82.100s [attack_start] Vayne begins auto attack
- 82.134s [champion_script] Vayne executed Tumble Empower
- 82.134s [enemy_buff] Vayne empowered next attack
- 82.136s [attack_start] Sona begins auto attack
- 82.412s [attack_start] Warwick begins auto attack
- 82.434s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 82.570s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 82.634s [champion_script] Vayne executed Tumble Empower
- 82.634s [enemy_buff] Vayne empowered next attack
- 82.652s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 82.684s [attack_start] Vayne begins auto attack
- 82.722s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 82.974s [attack_start] Warwick begins auto attack
- 83.134s [champion_script] Vayne executed Tumble Empower
- 83.134s [enemy_buff] Vayne empowered next attack
- 83.155s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 83.214s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 83.262s [attack_start] Sona begins auto attack
- 83.269s [attack_start] Vayne begins auto attack
- 83.524s [attack_start] Warwick begins auto attack
- 83.534s [champion_script] Morgana executed Dark Binding
- 83.534s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 83.634s [champion_script] Vayne executed Tumble Empower
- 83.634s [enemy_buff] Vayne empowered next attack
- 83.739s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 83.764s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 83.848s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 83.853s [attack_start] Vayne begins auto attack
- 84.065s [attack_start] Warwick begins auto attack
- 84.134s [champion_script] Vayne executed Tumble Empower
- 84.134s [enemy_buff] Vayne empowered next attack
- 84.305s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 84.324s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 84.388s [attack_start] Sona begins auto attack
- 84.438s [attack_start] Vayne begins auto attack
- 84.438s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 84.606s [attack_start] Warwick begins auto attack
- 84.634s [champion_script] Vayne executed Tumble Empower
- 84.634s [enemy_buff] Vayne empowered next attack
- 84.846s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 84.908s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 84.974s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 85.000s [state_snapshot] checkpoint 85.0s (captured_at 85.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1633.5/3688.5 (44.3%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.44s; Pool heal-over-time 1.44s; Untargetable x1 (1.44s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2974.8/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.15s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 8]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=897.1/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.02s; Tumble Empower 0.13s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 21]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=2277.5/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.53s; Dark Binding 0.82s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=378.3/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.51s; Crescendo 9.02s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.61s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 17.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 85.022s [attack_start] Vayne begins auto attack
- 85.134s [champion_script] Vayne executed Tumble Empower
- 85.134s [enemy_buff] Vayne empowered next attack
- 85.147s [attack_start] Warwick begins auto attack
- 85.387s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 85.493s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 85.514s [attack_start] Sona begins auto attack
- 85.607s [attack_start] Vayne begins auto attack
- 85.634s [champion_script] Vayne executed Tumble Empower
- 85.634s [enemy_buff] Vayne empowered next attack
- 85.688s [attack_start] Warwick begins auto attack
- 85.834s [champion_script] Morgana executed Dark Binding
- 85.834s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 85.928s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 86.077s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 86.100s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 86.134s [champion_script] Vayne executed Tumble Empower
- 86.134s [enemy_buff] Vayne empowered next attack
- 86.191s [attack_start] Vayne begins auto attack
- 86.228s [attack_start] Warwick begins auto attack
- 86.459s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 86.468s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 86.634s [champion_script] Vayne executed Tumble Empower
- 86.634s [enemy_buff] Vayne empowered next attack
- 86.640s [attack_start] Sona begins auto attack
- 86.662s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 86.769s [attack_start] Warwick begins auto attack
- 86.776s [attack_start] Vayne begins auto attack
- 87.009s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 87.134s [champion_script] Vayne executed Tumble Empower
- 87.134s [enemy_buff] Vayne empowered next attack
- 87.226s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 87.246s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 87.310s [attack_start] Warwick begins auto attack
- 87.360s [attack_start] Vayne begins auto attack
- 87.550s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 87.634s [champion_script] Vayne executed Tumble Empower
- 87.634s [enemy_buff] Vayne empowered next attack
- 87.766s [attack_start] Sona begins auto attack
- 87.831s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 87.851s [attack_start] Warwick begins auto attack
- 87.944s [attack_start] Vayne begins auto attack
- 88.091s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 88.134s [champion_script] Vayne executed Tumble Empower
- 88.134s [enemy_buff] Vayne empowered next attack
- 88.134s [champion_script] Morgana executed Dark Binding
- 88.134s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 88.352s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 88.392s [attack_start] Warwick begins auto attack
- 88.415s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 88.467s [enemy_death] Sona died; respawn in 54.5s
- 88.467s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 88.529s [attack_start] Vayne begins auto attack
- 88.632s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 88.634s [champion_script] Vayne executed Tumble Empower
- 88.634s [enemy_buff] Vayne empowered next attack
- 88.933s [attack_start] Warwick begins auto attack
- 89.000s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 89.113s [attack_start] Vayne begins auto attack
- 89.134s [champion_script] Vayne executed Tumble Empower
- 89.134s [enemy_buff] Vayne empowered next attack
- 89.173s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 89.473s [attack_start] Warwick begins auto attack
- 89.584s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 89.634s [champion_script] Vayne executed Tumble Empower
- 89.634s [enemy_buff] Vayne empowered next attack
- 89.698s [attack_start] Vayne begins auto attack
- 89.713s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 90.000s [state_snapshot] checkpoint 90.0s (captured_at 90.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2136.6/3688.5 (57.9%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.47s; Pool heal-over-time 0.47s; Untargetable x1 (0.47s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2448.6/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.01s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 17]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=315.6/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.17s to impact); Tumble Empower 0.13s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 29]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=1696.1/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.49s; Dark Binding 0.42s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.51s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 52.97s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.50s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 12.70s
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.17s)
  projectile_block_zones: none
  ```
- 90.014s [attack_start] Warwick begins auto attack
- 90.134s [champion_script] Vayne executed Tumble Empower
- 90.134s [enemy_buff] Vayne empowered next attack
- 90.168s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 90.254s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 90.282s [attack_start] Vayne begins auto attack
- 90.434s [champion_script] Morgana executed Dark Binding
- 90.434s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 90.482s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 90.555s [attack_start] Warwick begins auto attack
- 90.634s [champion_script] Vayne executed Tumble Empower
- 90.634s [enemy_buff] Vayne empowered next attack
- 90.753s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 90.795s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 90.867s [attack_start] Vayne begins auto attack
- 91.096s [attack_start] Warwick begins auto attack
- 91.134s [champion_script] Vayne executed Tumble Empower
- 91.134s [enemy_buff] Vayne empowered next attack
- 91.336s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 91.337s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 91.451s [attack_start] Vayne begins auto attack
- 91.634s [champion_script] Vayne executed Tumble Empower
- 91.634s [enemy_buff] Vayne empowered next attack
- 91.637s [attack_start] Warwick begins auto attack
- 91.877s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 91.922s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 92.036s [attack_start] Vayne begins auto attack
- 92.134s [champion_script] Vayne executed Tumble Empower
- 92.134s [enemy_buff] Vayne empowered next attack
- 92.177s [attack_start] Warwick begins auto attack
- 92.417s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 92.500s [enemy_death] Vayne died; respawn in 54.5s
- 92.500s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 92.718s [attack_start] Warwick begins auto attack
- 92.734s [champion_script] Morgana executed Dark Binding
- 92.734s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 92.958s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 93.259s [attack_start] Warwick begins auto attack
- 93.499s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 93.800s [attack_start] Warwick begins auto attack
- 94.040s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 94.341s [attack_start] Warwick begins auto attack
- 94.533s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 94.581s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 94.881s [attack_start] Warwick begins auto attack
- 95.000s [state_snapshot] checkpoint 95.0s (captured_at 95.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2158.8/3688.5 (58.5%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.53s; Pool heal-over-time 1.53s; Untargetable x1 (1.53s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1659.3/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 26]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.06s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 52.00s
  Morgana:
    core: pos=(-650.0, 120.0) hp=823.8/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.45s; Dark Binding 0.02s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.38s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 47.97s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.39s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 7.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 95.034s [champion_script] Morgana executed Dark Binding
- 95.034s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 95.121s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 95.422s [attack_start] Warwick begins auto attack
- 95.662s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 95.963s [attack_start] Warwick begins auto attack
- 96.203s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 96.504s [attack_start] Warwick begins auto attack
- 96.534s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 96.744s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 97.045s [attack_start] Warwick begins auto attack
- 97.285s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 97.334s [champion_script] Morgana executed Dark Binding
- 97.334s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 97.585s [attack_start] Warwick begins auto attack
- 97.825s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 98.126s [attack_start] Warwick begins auto attack
- 98.366s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 98.551s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 98.667s [attack_start] Warwick begins auto attack
- 98.907s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 99.208s [attack_start] Warwick begins auto attack
- 99.448s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 99.634s [champion_script] Morgana executed Dark Binding
- 99.634s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 99.749s [attack_start] Warwick begins auto attack
- 99.989s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 100.000s [state_snapshot] checkpoint 100.0s (captured_at 100.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2230.2/3688.5 (60.5%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.55s; Pool heal-over-time 0.55s; Untargetable x1 (0.55s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1133.1/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.29s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 36]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.16s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 47.00s
  Morgana:
    core: pos=(-650.0, 120.0) hp=242.4/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.41s; Dark Binding 1.92s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.63s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.24s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 42.97s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.28s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 2.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 100.290s [attack_start] Warwick begins auto attack
- 100.530s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 100.567s [enemy_death] Morgana died; respawn in 54.5s
- 100.567s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 100.830s [attack_start] Warwick begins auto attack
- 101.070s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 101.371s [attack_start] Warwick begins auto attack
- 101.611s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 101.912s [attack_start] Warwick begins auto attack
- 102.152s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 102.453s [attack_start] Warwick begins auto attack
- 102.600s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 102.693s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 102.708s [enemy_respawn] Dr. Mundo respawned
- 102.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 102.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 102.729s [attack_start] Dr. Mundo begins auto attack
- 102.969s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 102.994s [attack_start] Warwick begins auto attack
- 103.234s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 103.534s [attack_start] Warwick begins auto attack
- 103.581s [attack_start] Dr. Mundo begins auto attack
- 103.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 103.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 103.774s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 103.821s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 104.075s [attack_start] Warwick begins auto attack
- 104.315s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 104.432s [attack_start] Dr. Mundo begins auto attack
- 104.608s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 104.616s [attack_start] Warwick begins auto attack
- 104.672s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 104.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 104.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 104.856s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 105.000s [state_snapshot] checkpoint 105.0s (captured_at 105.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2034.9/3688.5 (55.2%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.61s; Pool heal-over-time 1.61s; Untargetable x1 (1.61s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=343.8/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.16s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 45]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.10s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 42.00s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.37s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 50.07s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.10s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 37.97s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=6219.4/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.28s; Infected Bonesaw 0.66s
    runtime: cooldowns [Grasp of the Undying: 1.97s (cooldown 4.00s); Heartsteel Colossal Consumption: 5.47s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 105.157s [attack_start] Warwick begins auto attack
- 105.284s [attack_start] Dr. Mundo begins auto attack
- 105.397s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 105.524s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 105.698s [attack_start] Warwick begins auto attack
- 105.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 105.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 105.938s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 106.136s [attack_start] Dr. Mundo begins auto attack
- 106.238s [attack_start] Warwick begins auto attack
- 106.376s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 106.478s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 106.609s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 106.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 106.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 106.779s [attack_start] Warwick begins auto attack
- 106.987s [attack_start] Dr. Mundo begins auto attack
- 107.019s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 107.227s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 107.320s [attack_start] Warwick begins auto attack
- 107.560s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 107.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 107.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 107.839s [attack_start] Dr. Mundo begins auto attack
- 107.861s [attack_start] Warwick begins auto attack
- 108.079s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 108.101s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 108.402s [attack_start] Warwick begins auto attack
- 108.633s [enemy_death] Warwick died; respawn in 54.5s
- 108.633s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 108.690s [attack_start] Dr. Mundo begins auto attack
- 108.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 108.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 108.930s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 109.542s [attack_start] Dr. Mundo begins auto attack
- 109.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 109.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 109.782s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 110.000s [state_snapshot] checkpoint 110.0s (captured_at 110.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2064.3/3688.5 (56.0%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.63s; Pool heal-over-time 0.63s; Untargetable x1 (0.63s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.13s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 53.13s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.04s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 37.00s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.33s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 45.07s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.50s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 32.97s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=5698.5/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.39s; Infected Bonesaw 0.66s
    runtime: cooldowns [Grasp of the Undying: 1.23s (cooldown 4.00s); Heartsteel Colossal Consumption: 0.47s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 110.393s [attack_start] Dr. Mundo begins auto attack
- 110.633s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 110.633s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 110.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 110.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 111.245s [attack_start] Dr. Mundo begins auto attack
- 111.485s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 111.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 111.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 112.097s [attack_start] Dr. Mundo begins auto attack
- 112.337s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 112.665s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 112.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 112.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 112.948s [attack_start] Dr. Mundo begins auto attack
- 113.188s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 113.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 113.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 113.800s [attack_start] Dr. Mundo begins auto attack
- 114.040s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 114.651s [attack_start] Dr. Mundo begins auto attack
- 114.666s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 114.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 114.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 114.891s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 115.000s [state_snapshot] checkpoint 115.0s (captured_at 115.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1811.6/3688.5 (49.1%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.04s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.67s; Pool heal-over-time 1.67s; Untargetable x1 (1.67s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.36s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 48.13s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.14s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 32.00s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.29s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 40.07s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.37s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 27.97s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=4917.2/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.50s; Infected Bonesaw 0.66s
    runtime: cooldowns [Grasp of the Undying: 0.48s (cooldown 4.00s); Heartsteel Colossal Consumption: 3.13s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 115.503s [attack_start] Dr. Mundo begins auto attack
- 115.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 115.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 115.743s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 116.354s [attack_start] Dr. Mundo begins auto attack
- 116.594s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 116.667s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 116.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 116.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 117.206s [attack_start] Dr. Mundo begins auto attack
- 117.446s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 117.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 117.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 118.058s [attack_start] Dr. Mundo begins auto attack
- 118.298s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 118.700s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 118.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 118.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 118.909s [attack_start] Dr. Mundo begins auto attack
- 119.149s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 119.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 119.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 119.761s [attack_start] Dr. Mundo begins auto attack
- 120.000s [state_snapshot] checkpoint 120.0s (captured_at 120.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1728.8/3688.5 (46.9%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.70s; Pool heal-over-time 0.70s; Untargetable x1 (0.70s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.20s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 43.13s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.08s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 27.00s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.24s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 35.07s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.23s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 22.97s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=4396.3/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.66s
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: 5.80s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 120.001s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 120.612s [attack_start] Dr. Mundo begins auto attack
- 120.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 120.708s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 526.9, true 0.0, total 340.6
- 120.708s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 120.852s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 121.464s [attack_start] Dr. Mundo begins auto attack
- 121.704s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 121.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 121.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 122.315s [attack_start] Dr. Mundo begins auto attack
- 122.555s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 122.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 122.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 122.724s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 123.167s [attack_start] Dr. Mundo begins auto attack
- 123.407s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 123.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 123.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 124.019s [attack_start] Dr. Mundo begins auto attack
- 124.259s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 124.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 124.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 124.733s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 124.870s [attack_start] Dr. Mundo begins auto attack
- 125.000s [state_snapshot] checkpoint 125.0s (captured_at 125.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1269.9/3688.5 (34.4%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.11s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.73s; Pool heal-over-time 1.73s; Untargetable x1 (1.73s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.05s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 38.13s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.02s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 22.00s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.20s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 30.07s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.09s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 17.97s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3615.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.66s
    runtime: cooldowns [Grasp of the Undying: 3.26s (cooldown 4.00s); Heartsteel Colossal Consumption: 0.80s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 125.110s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 125.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 125.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 125.722s [attack_start] Dr. Mundo begins auto attack
- 125.962s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 126.573s [attack_start] Dr. Mundo begins auto attack
- 126.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 126.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 126.767s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 126.813s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 127.425s [attack_start] Dr. Mundo begins auto attack
- 127.665s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 127.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 127.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 128.276s [attack_start] Dr. Mundo begins auto attack
- 128.516s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 128.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 128.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 128.784s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 129.128s [attack_start] Dr. Mundo begins auto attack
- 129.368s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 129.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 129.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 129.980s [attack_start] Dr. Mundo begins auto attack
- 130.000s [state_snapshot] checkpoint 130.0s (captured_at 130.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1264.3/3688.5 (34.3%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.78s; Pool heal-over-time 0.78s; Untargetable x1 (0.78s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.28s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 33.13s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.12s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 17.00s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.16s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 25.07s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.49s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 12.97s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3094.2/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.66s
    runtime: cooldowns [Grasp of the Undying: 2.52s (cooldown 4.00s); Heartsteel Colossal Consumption: 3.46s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 130.220s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 130.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 130.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 130.800s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 130.831s [attack_start] Dr. Mundo begins auto attack
- 131.071s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 131.683s [attack_start] Dr. Mundo begins auto attack
- 131.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 131.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 131.923s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 132.534s [attack_start] Dr. Mundo begins auto attack
- 132.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 132.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 132.774s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 132.808s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 133.386s [attack_start] Dr. Mundo begins auto attack
- 133.626s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 133.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 133.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 134.237s [attack_start] Dr. Mundo begins auto attack
- 134.477s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 134.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 134.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 134.817s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 135.000s [state_snapshot] checkpoint 135.0s (captured_at 135.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1171.1/3688.5 (31.8%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.19s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.82s; Pool heal-over-time 1.82s; Untargetable x1 (1.82s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.12s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 28.13s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.07s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 12.00s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.12s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 20.07s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.36s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 7.97s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=2312.8/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.09s; Infected Bonesaw 0.66s
    runtime: cooldowns [Grasp of the Undying: 1.77s (cooldown 4.00s); Heartsteel Colossal Consumption: 6.13s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 135.089s [attack_start] Dr. Mundo begins auto attack
- 135.329s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 135.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 135.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 135.941s [attack_start] Dr. Mundo begins auto attack
- 136.181s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 136.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 136.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 136.792s [attack_start] Dr. Mundo begins auto attack
- 136.820s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 137.032s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 137.644s [attack_start] Dr. Mundo begins auto attack
- 137.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 137.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 137.884s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 138.495s [attack_start] Dr. Mundo begins auto attack
- 138.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 138.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 138.735s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 138.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 139.347s [attack_start] Dr. Mundo begins auto attack
- 139.587s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 139.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 139.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 140.000s [state_snapshot] checkpoint 140.0s (captured_at 140.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1180.6/3688.5 (32.0%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.83s; Pool heal-over-time 0.83s; Untargetable x1 (0.83s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.35s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 23.13s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.01s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 7.00s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.08s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 15.07s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.22s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 2.97s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=1792.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.20s; Infected Bonesaw 0.66s
    runtime: cooldowns [Grasp of the Undying: 1.03s (cooldown 4.00s); Heartsteel Colossal Consumption: 1.13s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 140.198s [attack_start] Dr. Mundo begins auto attack
- 140.438s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 140.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 140.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 140.867s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 141.050s [attack_start] Dr. Mundo begins auto attack
- 141.290s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 141.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 141.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 141.901s [attack_start] Dr. Mundo begins auto attack
- 142.141s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 142.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 142.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 142.753s [attack_start] Dr. Mundo begins auto attack
- 142.877s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 142.993s [enemy_respawn] Sona respawned
- 142.993s [champion_script] Sona executed Crescendo
- 142.993s [impact_nullified] Sona Crescendo on Vladimir was nullified by untargetable or stasis state
- 142.993s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 143.462s [attack_start] Sona begins auto attack
- 143.605s [attack_start] Dr. Mundo begins auto attack
- 143.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 143.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 143.845s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 144.048s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 144.456s [attack_start] Dr. Mundo begins auto attack
- 144.588s [attack_start] Sona begins auto attack
- 144.696s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 144.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 144.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 144.893s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 145.000s [state_snapshot] checkpoint 145.0s (captured_at 145.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1108.1/3688.5 (30.0%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.27s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.89s; Pool heal-over-time 1.89s; Untargetable x1 (1.89s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.20s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 18.13s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.11s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 2.00s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.04s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 10.07s
  Sona:
    core: pos=(-550.0, -180.0) hp=2122.7/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack in-flight (0.17s to impact); Crescendo 21.36s
    runtime: cooldowns [Luden's Echo: 0.99s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=1010.7/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.31s; Infected Bonesaw 0.66s
    runtime: cooldowns [Grasp of the Undying: 0.29s (cooldown 4.00s); Heartsteel Colossal Consumption: 3.79s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles:
    - Sona Auto Attack -> Vladimir (impact in 0.17s)
  projectile_block_zones: none
  ```
- 145.174s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 145.308s [attack_start] Dr. Mundo begins auto attack
- 145.548s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 145.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 145.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 145.714s [attack_start] Sona begins auto attack
- 146.159s [attack_start] Dr. Mundo begins auto attack
- 146.300s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 146.399s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 146.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 146.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 146.840s [attack_start] Sona begins auto attack
- 146.897s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 147.008s [enemy_respawn] Vayne respawned
- 147.008s [champion_script] Vayne executed Tumble Empower
- 147.008s [enemy_buff] Vayne empowered next attack
- 147.011s [attack_start] Dr. Mundo begins auto attack
- 147.021s [attack_start] Vayne begins auto attack
- 147.251s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 147.426s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 147.492s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 147.508s [champion_script] Vayne executed Tumble Empower
- 147.508s [enemy_buff] Vayne empowered next attack
- 147.642s [attack_start] Vayne begins auto attack
- 147.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 147.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 147.862s [attack_start] Dr. Mundo begins auto attack
- 147.966s [attack_start] Sona begins auto attack
- 148.008s [champion_script] Vayne executed Tumble Empower
- 148.008s [enemy_buff] Vayne empowered next attack
- 148.102s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 148.113s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 148.255s [attack_start] Vayne begins auto attack
- 148.508s [champion_script] Vayne executed Tumble Empower
- 148.508s [enemy_buff] Vayne empowered next attack
- 148.552s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 148.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 148.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 148.714s [attack_start] Dr. Mundo begins auto attack
- 148.726s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 148.861s [attack_start] Vayne begins auto attack
- 148.900s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 148.954s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 149.008s [champion_script] Vayne executed Tumble Empower
- 149.008s [enemy_buff] Vayne empowered next attack
- 149.092s [attack_start] Sona begins auto attack
- 149.331s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 149.460s [attack_start] Vayne begins auto attack
- 149.508s [champion_script] Vayne executed Tumble Empower
- 149.508s [enemy_buff] Vayne empowered next attack
- 149.566s [attack_start] Dr. Mundo begins auto attack
- 149.678s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 149.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 149.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 149.806s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 149.931s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 150.000s [state_snapshot] checkpoint 150.0s (captured_at 150.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1373.9/3688.5 (37.2%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.90s; Pool heal-over-time 0.90s; Untargetable x1 (0.90s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.05s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 13.13s
  Vayne:
    core: pos=(520.0, 150.0) hp=2350.7/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.156 (interval 0.123s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.05s; Tumble Empower 0.01s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 5/6; Guinsoo stacks: 5/8; Attacks landed: 5]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.62s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 5.07s
  Sona:
    core: pos=(-550.0, -180.0) hp=1541.3/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.22s; Crescendo 16.36s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=489.8/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.42s; Infected Bonesaw 0.66s
    runtime: cooldowns [Grasp of the Undying: 3.81s (cooldown 4.00s); Heartsteel Colossal Consumption: 6.45s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 150.008s [champion_script] Vayne executed Tumble Empower
- 150.008s [enemy_buff] Vayne empowered next attack
- 150.053s [attack_start] Vayne begins auto attack
- 150.218s [attack_start] Sona begins auto attack
- 150.417s [attack_start] Dr. Mundo begins auto attack
- 150.508s [champion_script] Vayne executed Tumble Empower
- 150.508s [enemy_buff] Vayne empowered next attack
- 150.524s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 150.641s [attack_start] Vayne begins auto attack
- 150.657s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 150.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 150.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 150.804s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 150.908s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 151.008s [champion_script] Vayne executed Tumble Empower
- 151.008s [enemy_buff] Vayne empowered next attack
- 151.112s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 151.227s [attack_start] Vayne begins auto attack
- 151.269s [attack_start] Dr. Mundo begins auto attack
- 151.344s [attack_start] Sona begins auto attack
- 151.508s [champion_script] Vayne executed Tumble Empower
- 151.508s [enemy_buff] Vayne empowered next attack
- 151.509s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 151.698s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 151.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 151.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 151.811s [attack_start] Vayne begins auto attack
- 151.930s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 152.008s [champion_script] Vayne executed Tumble Empower
- 152.008s [enemy_buff] Vayne empowered next attack
- 152.120s [attack_start] Dr. Mundo begins auto attack
- 152.282s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 152.360s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 152.396s [attack_start] Vayne begins auto attack
- 152.471s [attack_start] Sona begins auto attack
- 152.508s [champion_script] Vayne executed Tumble Empower
- 152.508s [enemy_buff] Vayne empowered next attack
- 152.708s [champion_script] Dr. Mundo executed Infected Bonesaw
- 152.708s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 152.867s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 152.933s [enemy_death] Dr. Mundo died; respawn in 54.5s
- 152.933s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 152.980s [attack_start] Vayne begins auto attack
- 153.008s [champion_script] Vayne executed Tumble Empower
- 153.008s [enemy_buff] Vayne empowered next attack
- 153.056s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 153.451s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 153.508s [champion_script] Vayne executed Tumble Empower
- 153.508s [enemy_buff] Vayne empowered next attack
- 153.565s [attack_start] Vayne begins auto attack
- 153.597s [attack_start] Sona begins auto attack
- 154.008s [champion_script] Vayne executed Tumble Empower
- 154.008s [enemy_buff] Vayne empowered next attack
- 154.035s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 154.149s [attack_start] Vayne begins auto attack
- 154.182s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 154.508s [champion_script] Vayne executed Tumble Empower
- 154.508s [enemy_buff] Vayne empowered next attack
- 154.620s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 154.723s [attack_start] Sona begins auto attack
- 154.734s [attack_start] Vayne begins auto attack
- 154.934s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 155.000s [state_snapshot] checkpoint 155.0s (captured_at 155.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1624.3/3688.5 (44.0%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.31s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.93s; Pool heal-over-time 1.93s; Untargetable x1 (1.93s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.27s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 8.13s
  Vayne:
    core: pos=(520.0, 150.0) hp=1478.5/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.20s to impact); Tumble Empower 0.01s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 13]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.57s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 0.07s
  Sona:
    core: pos=(-550.0, -180.0) hp=669.1/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack in-flight (0.31s to impact); Crescendo 11.36s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.42s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: 1.45s (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 52.43s
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.20s)
    - Sona Auto Attack -> Vladimir (impact in 0.31s)
  projectile_block_zones: none
  ```
- 155.008s [champion_script] Vayne executed Tumble Empower
- 155.008s [enemy_buff] Vayne empowered next attack
- 155.093s [enemy_respawn] Morgana respawned
- 155.093s [champion_script] Morgana executed Dark Binding
- 155.093s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 155.204s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 155.309s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 155.318s [attack_start] Vayne begins auto attack
- 155.508s [champion_script] Vayne executed Tumble Empower
- 155.508s [enemy_buff] Vayne empowered next attack
- 155.789s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 155.849s [attack_start] Sona begins auto attack
- 155.903s [attack_start] Vayne begins auto attack
- 156.008s [champion_script] Vayne executed Tumble Empower
- 156.008s [enemy_buff] Vayne empowered next attack
- 156.373s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 156.435s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 156.487s [attack_start] Vayne begins auto attack
- 156.508s [champion_script] Vayne executed Tumble Empower
- 156.508s [enemy_buff] Vayne empowered next attack
- 156.958s [damage_in] Vayne Auto Attack -> Vladimir | physical 756.1, magic 0.0, true 0.0, total 291.4
- 156.958s [attack_hit] Vayne hit Vladimir (phys 756.1, magic 0.0, true 0.0)
- 156.958s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 156.975s [attack_start] Sona begins auto attack
- 157.008s [champion_script] Vayne executed Tumble Empower
- 157.008s [enemy_buff] Vayne empowered next attack
- 157.072s [attack_start] Vayne begins auto attack
- 157.393s [champion_script] Morgana executed Dark Binding
- 157.393s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 157.508s [champion_script] Vayne executed Tumble Empower
- 157.508s [enemy_buff] Vayne empowered next attack
- 157.542s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 157.561s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 157.656s [attack_start] Vayne begins auto attack
- 158.008s [champion_script] Vayne executed Tumble Empower
- 158.008s [enemy_buff] Vayne empowered next attack
- 158.101s [attack_start] Sona begins auto attack
- 158.127s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 158.241s [attack_start] Vayne begins auto attack
- 158.508s [champion_script] Vayne executed Tumble Empower
- 158.508s [enemy_buff] Vayne empowered next attack
- 158.687s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 158.711s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 158.825s [attack_start] Vayne begins auto attack
- 158.967s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 159.008s [champion_script] Vayne executed Tumble Empower
- 159.008s [enemy_buff] Vayne empowered next attack
- 159.227s [attack_start] Sona begins auto attack
- 159.296s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 159.409s [attack_start] Vayne begins auto attack
- 159.508s [champion_script] Vayne executed Tumble Empower
- 159.508s [enemy_buff] Vayne empowered next attack
- 159.693s [champion_script] Morgana executed Dark Binding
- 159.693s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 159.813s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 159.880s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 159.994s [attack_start] Vayne begins auto attack
- 160.000s [state_snapshot] checkpoint 160.0s (captured_at 160.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1662.0/3688.5 (45.1%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.97s; Pool heal-over-time 0.97s; Untargetable x1 (0.97s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.12s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 3.13s
  Vayne:
    core: pos=(520.0, 150.0) hp=897.1/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower 0.01s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 22]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=2859.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.53s; Dark Binding 1.98s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.69s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=87.6/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.35s; Crescendo 6.36s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.31s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 47.43s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 160.008s [champion_script] Vayne executed Tumble Empower
- 160.008s [enemy_buff] Vayne empowered next attack
- 160.353s [attack_start] Sona begins auto attack
- 160.465s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 160.508s [champion_script] Vayne executed Tumble Empower
- 160.508s [enemy_buff] Vayne empowered next attack
- 160.578s [attack_start] Vayne begins auto attack
- 160.939s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 160.993s [enemy_death] Sona died; respawn in 54.5s
- 160.993s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 161.008s [champion_script] Vayne executed Tumble Empower
- 161.008s [enemy_buff] Vayne empowered next attack
- 161.049s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 161.163s [attack_start] Vayne begins auto attack
- 161.508s [champion_script] Vayne executed Tumble Empower
- 161.508s [enemy_buff] Vayne empowered next attack
- 161.633s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 161.747s [attack_start] Vayne begins auto attack
- 161.993s [champion_script] Morgana executed Dark Binding
- 161.993s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 162.008s [champion_script] Vayne executed Tumble Empower
- 162.008s [enemy_buff] Vayne empowered next attack
- 162.218s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 162.332s [attack_start] Vayne begins auto attack
- 162.508s [champion_script] Vayne executed Tumble Empower
- 162.508s [enemy_buff] Vayne empowered next attack
- 162.802s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 162.916s [attack_start] Vayne begins auto attack
- 163.000s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 163.008s [champion_script] Vayne executed Tumble Empower
- 163.008s [enemy_buff] Vayne empowered next attack
- 163.167s [enemy_respawn] Warwick respawned
- 163.387s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 163.473s [attack_start] Warwick begins auto attack
- 163.501s [attack_start] Vayne begins auto attack
- 163.508s [champion_script] Vayne executed Tumble Empower
- 163.508s [enemy_buff] Vayne empowered next attack
- 163.713s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 163.971s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 164.008s [champion_script] Vayne executed Tumble Empower
- 164.008s [enemy_buff] Vayne empowered next attack
- 164.072s [attack_start] Warwick begins auto attack
- 164.085s [attack_start] Vayne begins auto attack
- 164.293s [champion_script] Morgana executed Dark Binding
- 164.293s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 164.312s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 164.508s [champion_script] Vayne executed Tumble Empower
- 164.508s [enemy_buff] Vayne empowered next attack
- 164.556s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 164.657s [attack_start] Warwick begins auto attack
- 164.670s [attack_start] Vayne begins auto attack
- 164.897s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 165.000s [state_snapshot] checkpoint 165.0s (captured_at 165.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1890.4/3688.5 (51.3%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.00s; Pool heal-over-time 0.00s; Untargetable x1 (expired)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=3501.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.003 (interval 0.333s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.23s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 3/6; Attacks landed: 3]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=315.6/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.14s to impact); Tumble Empower 0.01s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 30]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=2277.5/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.49s; Dark Binding 1.58s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.29s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.26s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 50.49s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.20s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 42.43s
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.14s)
  projectile_block_zones: none
  ```
- 165.008s [champion_script] Vayne executed Tumble Empower
- 165.008s [enemy_buff] Vayne empowered next attack
- 165.008s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 165.140s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 165.230s [attack_start] Warwick begins auto attack
- 165.254s [attack_start] Vayne begins auto attack
- 165.470s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 165.508s [champion_script] Vayne executed Tumble Empower
- 165.508s [enemy_buff] Vayne empowered next attack
- 165.725s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 165.792s [attack_start] Warwick begins auto attack
- 165.838s [attack_start] Vayne begins auto attack
- 166.008s [champion_script] Vayne executed Tumble Empower
- 166.008s [enemy_buff] Vayne empowered next attack
- 166.032s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 166.309s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 166.343s [attack_start] Warwick begins auto attack
- 166.423s [attack_start] Vayne begins auto attack
- 166.508s [champion_script] Vayne executed Tumble Empower
- 166.508s [enemy_buff] Vayne empowered next attack
- 166.583s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 166.593s [champion_script] Morgana executed Dark Binding
- 166.593s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 166.883s [attack_start] Warwick begins auto attack
- 166.894s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 167.007s [attack_start] Vayne begins auto attack
- 167.008s [champion_script] Vayne executed Tumble Empower
- 167.008s [enemy_buff] Vayne empowered next attack
- 167.033s [enemy_death] Vayne died; respawn in 54.5s
- 167.033s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 167.123s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 167.424s [attack_start] Warwick begins auto attack
- 167.664s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 167.965s [attack_start] Warwick begins auto attack
- 168.205s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 168.506s [attack_start] Warwick begins auto attack
- 168.746s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 168.893s [champion_script] Morgana executed Dark Binding
- 168.893s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 169.043s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 169.047s [attack_start] Warwick begins auto attack
- 169.287s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 169.587s [attack_start] Warwick begins auto attack
- 169.827s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 170.000s [state_snapshot] checkpoint 170.0s (captured_at 170.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1953.0/3688.5 (52.9%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.04s; Pool heal-over-time 1.04s; Untargetable x1 (1.04s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2711.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.13s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 12]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.08s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 51.53s
  Morgana:
    core: pos=(-650.0, 120.0) hp=1405.3/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.45s; Dark Binding 1.18s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.89s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.12s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 45.49s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.10s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 37.43s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 170.128s [attack_start] Warwick begins auto attack
- 170.368s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 170.669s [attack_start] Warwick begins auto attack
- 170.909s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 171.067s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 171.193s [champion_script] Morgana executed Dark Binding
- 171.193s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 171.210s [attack_start] Warwick begins auto attack
- 171.450s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 171.751s [attack_start] Warwick begins auto attack
- 171.991s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 172.292s [attack_start] Warwick begins auto attack
- 172.532s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 172.832s [attack_start] Warwick begins auto attack
- 173.072s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 173.072s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 173.373s [attack_start] Warwick begins auto attack
- 173.493s [champion_script] Morgana executed Dark Binding
- 173.493s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 173.613s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 173.914s [attack_start] Warwick begins auto attack
- 174.154s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 174.455s [attack_start] Warwick begins auto attack
- 174.695s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 174.996s [attack_start] Warwick begins auto attack
- 175.000s [state_snapshot] checkpoint 175.0s (captured_at 175.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2059.0/3688.5 (55.8%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.07s; Pool heal-over-time 0.07s; Untargetable x1 (0.07s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2185.5/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 21]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.02s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 46.53s
  Morgana:
    core: pos=(-650.0, 120.0) hp=823.8/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.41s; Dark Binding 0.78s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.49s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.53s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 40.49s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.60s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 32.43s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 175.093s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 175.236s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 175.536s [attack_start] Warwick begins auto attack
- 175.776s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 175.793s [champion_script] Morgana executed Dark Binding
- 175.793s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 176.077s [attack_start] Warwick begins auto attack
- 176.317s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 176.618s [attack_start] Warwick begins auto attack
- 176.858s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 177.100s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 177.159s [attack_start] Warwick begins auto attack
- 177.399s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 177.700s [attack_start] Warwick begins auto attack
- 177.940s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 178.093s [champion_script] Morgana executed Dark Binding
- 178.093s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 178.240s [attack_start] Warwick begins auto attack
- 178.480s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 178.781s [attack_start] Warwick begins auto attack
- 179.021s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 179.128s [enemy_death] Morgana died; respawn in 54.5s
- 179.128s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 179.322s [attack_start] Warwick begins auto attack
- 179.562s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 179.863s [attack_start] Warwick begins auto attack
- 180.000s [state_snapshot] checkpoint 180.0s (captured_at 180.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1996.4/3688.5 (54.1%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.13s; Pool heal-over-time 1.13s; Untargetable x1 (1.13s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1396.2/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 30]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.12s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 41.53s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.37s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.09s (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 53.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.39s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 35.49s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.49s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 27.43s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 180.103s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 180.404s [attack_start] Warwick begins auto attack
- 180.644s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 180.944s [attack_start] Warwick begins auto attack
- 181.133s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 181.184s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 181.485s [attack_start] Warwick begins auto attack
- 181.725s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 182.026s [attack_start] Warwick begins auto attack
- 182.266s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 182.567s [attack_start] Warwick begins auto attack
- 182.807s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 183.108s [attack_start] Warwick begins auto attack
- 183.147s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 183.348s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 183.649s [attack_start] Warwick begins auto attack
- 183.889s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 184.189s [attack_start] Warwick begins auto attack
- 184.429s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 184.730s [attack_start] Warwick begins auto attack
- 184.970s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 185.000s [state_snapshot] checkpoint 185.0s (captured_at 185.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1927.4/3688.5 (52.3%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.15s; Pool heal-over-time 0.15s; Untargetable x1 (0.15s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=870.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.27s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 40]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.06s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 36.53s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.33s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 48.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.25s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 30.49s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.39s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 22.43s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 185.167s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 185.271s [attack_start] Warwick begins auto attack
- 185.511s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 185.812s [attack_start] Warwick begins auto attack
- 186.052s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 186.353s [attack_start] Warwick begins auto attack
- 186.593s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 186.893s [attack_start] Warwick begins auto attack
- 187.133s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 187.185s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 187.434s [attack_start] Warwick begins auto attack
- 187.674s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 187.975s [attack_start] Warwick begins auto attack
- 188.215s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 188.516s [attack_start] Warwick begins auto attack
- 188.756s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 189.057s [attack_start] Warwick begins auto attack
- 189.200s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 189.297s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 189.597s [attack_start] Warwick begins auto attack
- 189.837s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 190.000s [state_snapshot] checkpoint 190.0s (captured_at 190.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1702.2/3688.5 (46.1%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.20s; Pool heal-over-time 1.20s; Untargetable x1 (1.20s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=80.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.14s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 49]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.00s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 31.53s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.28s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 43.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.11s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 25.49s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.28s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 17.43s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 190.138s [attack_start] Warwick begins auto attack
- 190.378s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 190.679s [attack_start] Warwick begins auto attack
- 190.919s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 191.220s [attack_start] Warwick begins auto attack
- 191.220s [enemy_death] Warwick died; respawn in 54.5s
- 191.220s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 193.233s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 195.000s [state_snapshot] checkpoint 195.0s (captured_at 195.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1519.3/3688.5 (41.2%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.23s; Pool heal-over-time 0.23s; Untargetable x1 (0.23s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.19s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 50.72s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.10s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 26.53s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.24s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 38.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.52s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 20.49s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.17s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 12.43s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 195.243s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 197.267s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 199.293s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 200.000s [state_snapshot] checkpoint 200.0s (captured_at 200.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1202.5/3688.5 (32.6%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.29s; Pool heal-over-time 1.29s; Untargetable x1 (1.29s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.04s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 45.72s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.04s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 21.53s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.20s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 33.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.38s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 15.49s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.06s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 7.43s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 201.300s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 203.301s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 205.000s [state_snapshot] checkpoint 205.0s (captured_at 205.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1028.9/3688.5 (27.9%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.30s; Pool heal-over-time 0.30s; Untargetable x1 (0.30s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.26s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 40.72s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.14s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 16.53s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.16s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 28.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.24s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 10.49s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.57s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 2.43s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 205.304s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 207.333s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 207.467s [enemy_respawn] Dr. Mundo respawned
- 207.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 207.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 208.013s [attack_start] Dr. Mundo begins auto attack
- 208.253s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 208.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 208.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 208.865s [attack_start] Dr. Mundo begins auto attack
- 209.105s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 209.364s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 209.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 209.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 209.716s [attack_start] Dr. Mundo begins auto attack
- 209.956s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 210.000s [state_snapshot] checkpoint 210.0s (captured_at 210.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=839.2/3688.5 (22.8%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.36s; Pool heal-over-time 1.36s; Untargetable x1 (1.36s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.11s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 35.72s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.09s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 11.53s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.12s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 23.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.11s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 5.49s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=6219.4/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.57s; Infected Bonesaw 0.42s
    runtime: cooldowns [Grasp of the Undying: 2.25s (cooldown 4.00s); Heartsteel Colossal Consumption: 5.75s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 210.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 210.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 210.568s [attack_start] Dr. Mundo begins auto attack
- 210.808s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 211.367s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 211.420s [attack_start] Dr. Mundo begins auto attack
- 211.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 211.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 211.660s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 212.271s [attack_start] Dr. Mundo begins auto attack
- 212.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 212.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 212.511s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 213.123s [attack_start] Dr. Mundo begins auto attack
- 213.363s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 213.400s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 213.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 213.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 213.974s [attack_start] Dr. Mundo begins auto attack
- 214.214s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 214.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 214.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 214.826s [attack_start] Dr. Mundo begins auto attack
- 215.000s [state_snapshot] checkpoint 215.0s (captured_at 215.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=898.4/3688.5 (24.4%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.40s; Pool heal-over-time 0.40s; Untargetable x1 (0.40s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.33s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 30.72s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.03s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 6.53s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.08s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 18.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.51s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 0.49s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=5698.5/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.42s
    runtime: cooldowns [Grasp of the Undying: 1.51s (cooldown 4.00s); Heartsteel Colossal Consumption: 0.75s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 215.066s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 215.433s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 215.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 215.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 215.500s [enemy_respawn] Sona respawned
- 215.500s [champion_script] Sona executed Crescendo
- 215.500s [impact_nullified] Sona Crescendo on Vladimir was nullified by untargetable or stasis state
- 215.508s [attack_start] Sona begins auto attack
- 215.677s [attack_start] Dr. Mundo begins auto attack
- 215.917s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 216.094s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 216.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 216.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 216.529s [attack_start] Dr. Mundo begins auto attack
- 216.634s [attack_start] Sona begins auto attack
- 216.769s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 217.220s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 217.381s [attack_start] Dr. Mundo begins auto attack
- 217.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 217.467s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 280.0, true 0.0, total 181.0
- 217.467s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 217.621s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 217.761s [attack_start] Sona begins auto attack
- 218.232s [attack_start] Dr. Mundo begins auto attack
- 218.346s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 218.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 218.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 218.472s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 218.887s [attack_start] Sona begins auto attack
- 219.084s [attack_start] Dr. Mundo begins auto attack
- 219.324s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 219.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 219.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 219.472s [damage_in] Sona Auto Attack -> Vladimir | physical 106.0, magic 0.0, true 0.0, total 40.8
- 219.472s [attack_hit] Sona hit Vladimir (phys 106.0, magic 0.0, true 0.0)
- 219.472s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 219.935s [attack_start] Dr. Mundo begins auto attack
- 220.000s [state_snapshot] checkpoint 220.0s (captured_at 220.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=794.2/3688.5 (21.5%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.47s; Pool heal-over-time 1.47s; Untargetable x1 (1.47s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.18s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 25.72s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.13s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 1.53s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.04s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 13.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=1832.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.01s; Crescendo 18.86s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=4917.2/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.42s
    runtime: cooldowns [Grasp of the Undying: 0.77s (cooldown 4.00s); Heartsteel Colossal Consumption: 3.42s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 220.013s [attack_start] Sona begins auto attack
- 220.175s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 220.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 220.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 220.599s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 220.787s [attack_start] Dr. Mundo begins auto attack
- 221.027s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 221.139s [attack_start] Sona begins auto attack
- 221.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 221.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 221.500s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 221.563s [enemy_respawn] Vayne respawned
- 221.563s [attack_start] Vayne begins auto attack
- 221.563s [champion_script] Vayne executed Tumble Empower
- 221.563s [enemy_buff] Vayne empowered next attack
- 221.638s [attack_start] Dr. Mundo begins auto attack
- 221.725s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 221.878s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 222.033s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 222.063s [champion_script] Vayne executed Tumble Empower
- 222.063s [enemy_buff] Vayne empowered next attack
- 222.184s [attack_start] Vayne begins auto attack
- 222.265s [attack_start] Sona begins auto attack
- 222.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 222.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 222.490s [attack_start] Dr. Mundo begins auto attack
- 222.563s [champion_script] Vayne executed Tumble Empower
- 222.563s [enemy_buff] Vayne empowered next attack
- 222.654s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 222.730s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 222.797s [attack_start] Vayne begins auto attack
- 222.851s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 223.063s [champion_script] Vayne executed Tumble Empower
- 223.063s [enemy_buff] Vayne empowered next attack
- 223.267s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 223.342s [attack_start] Dr. Mundo begins auto attack
- 223.391s [attack_start] Sona begins auto attack
- 223.402s [attack_start] Vayne begins auto attack
- 223.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 223.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 223.533s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 223.563s [champion_script] Vayne executed Tumble Empower
- 223.563s [enemy_buff] Vayne empowered next attack
- 223.582s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 223.873s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 223.977s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 224.002s [attack_start] Vayne begins auto attack
- 224.063s [champion_script] Vayne executed Tumble Empower
- 224.063s [enemy_buff] Vayne empowered next attack
- 224.193s [attack_start] Dr. Mundo begins auto attack
- 224.433s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 224.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 224.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 224.472s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 224.517s [attack_start] Sona begins auto attack
- 224.563s [champion_script] Vayne executed Tumble Empower
- 224.563s [enemy_buff] Vayne empowered next attack
- 224.595s [attack_start] Vayne begins auto attack
- 225.000s [state_snapshot] checkpoint 225.0s (captured_at 225.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1121.8/3688.5 (30.4%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.53s; Pool heal-over-time 0.53s; Untargetable x1 (0.53s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.03s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 20.72s
  Vayne:
    core: pos=(520.0, 150.0) hp=2350.7/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.156 (interval 0.123s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.07s to impact); Tumble Empower 0.06s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 5/6; Guinsoo stacks: 5/8; Attacks landed: 5]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.61s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 8.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=1250.5/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack in-flight (0.10s to impact); Crescendo 13.86s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=4396.3/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.04s; Infected Bonesaw 0.42s
    runtime: cooldowns [Grasp of the Undying: 0.03s (cooldown 4.00s); Heartsteel Colossal Consumption: 6.08s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.07s)
    - Sona Auto Attack -> Vladimir (impact in 0.10s)
  projectile_block_zones: none
  ```
- 225.045s [attack_start] Dr. Mundo begins auto attack
- 225.063s [champion_script] Vayne executed Tumble Empower
- 225.063s [enemy_buff] Vayne empowered next attack
- 225.065s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 225.103s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 225.183s [attack_start] Vayne begins auto attack
- 225.285s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 225.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 225.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 225.563s [champion_script] Vayne executed Tumble Empower
- 225.563s [enemy_buff] Vayne empowered next attack
- 225.563s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 225.643s [attack_start] Sona begins auto attack
- 225.653s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 225.769s [attack_start] Vayne begins auto attack
- 225.896s [attack_start] Dr. Mundo begins auto attack
- 226.063s [champion_script] Vayne executed Tumble Empower
- 226.063s [enemy_buff] Vayne empowered next attack
- 226.136s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 226.229s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 226.239s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 226.353s [attack_start] Vayne begins auto attack
- 226.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 226.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 226.563s [champion_script] Vayne executed Tumble Empower
- 226.563s [enemy_buff] Vayne empowered next attack
- 226.748s [attack_start] Dr. Mundo begins auto attack
- 226.769s [attack_start] Sona begins auto attack
- 226.824s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 226.938s [attack_start] Vayne begins auto attack
- 226.988s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 227.063s [champion_script] Vayne executed Tumble Empower
- 227.063s [enemy_buff] Vayne empowered next attack
- 227.355s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 227.408s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 227.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 227.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 227.522s [attack_start] Vayne begins auto attack
- 227.563s [champion_script] Vayne executed Tumble Empower
- 227.563s [enemy_buff] Vayne empowered next attack
- 227.567s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 227.599s [attack_start] Dr. Mundo begins auto attack
- 227.839s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 227.895s [attack_start] Sona begins auto attack
- 227.993s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 228.063s [champion_script] Vayne executed Tumble Empower
- 228.063s [enemy_buff] Vayne empowered next attack
- 228.107s [attack_start] Vayne begins auto attack
- 228.451s [attack_start] Dr. Mundo begins auto attack
- 228.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 228.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 228.481s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 228.563s [champion_script] Vayne executed Tumble Empower
- 228.563s [enemy_buff] Vayne empowered next attack
- 228.577s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 228.691s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 228.691s [attack_start] Vayne begins auto attack
- 229.021s [attack_start] Sona begins auto attack
- 229.063s [champion_script] Vayne executed Tumble Empower
- 229.063s [enemy_buff] Vayne empowered next attack
- 229.162s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 229.276s [attack_start] Vayne begins auto attack
- 229.302s [attack_start] Dr. Mundo begins auto attack
- 229.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 229.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 229.542s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 229.563s [champion_script] Vayne executed Tumble Empower
- 229.563s [enemy_buff] Vayne empowered next attack
- 229.600s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 229.607s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 229.746s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 229.860s [attack_start] Vayne begins auto attack
- 230.000s [state_snapshot] checkpoint 230.0s (captured_at 230.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1441.4/3688.5 (39.1%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.60s; Pool heal-over-time 1.60s; Untargetable x1 (1.60s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.25s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 15.72s
  Vayne:
    core: pos=(520.0, 150.0) hp=1478.5/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower 0.06s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 14]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.57s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 3.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=378.3/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.15s; Crescendo 8.86s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3615.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.15s; Infected Bonesaw 0.42s
    runtime: cooldowns [Grasp of the Undying: 3.54s (cooldown 4.00s); Heartsteel Colossal Consumption: 1.08s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 230.063s [champion_script] Vayne executed Tumble Empower
- 230.063s [enemy_buff] Vayne empowered next attack
- 230.148s [attack_start] Sona begins auto attack
- 230.154s [attack_start] Dr. Mundo begins auto attack
- 230.331s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 230.394s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 230.444s [attack_start] Vayne begins auto attack
- 230.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 230.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 230.563s [champion_script] Vayne executed Tumble Empower
- 230.563s [enemy_buff] Vayne empowered next attack
- 230.733s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 230.915s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 231.006s [attack_start] Dr. Mundo begins auto attack
- 231.029s [attack_start] Vayne begins auto attack
- 231.063s [champion_script] Vayne executed Tumble Empower
- 231.063s [enemy_buff] Vayne empowered next attack
- 231.246s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 231.274s [attack_start] Sona begins auto attack
- 231.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 231.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 231.499s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 231.563s [champion_script] Vayne executed Tumble Empower
- 231.563s [enemy_buff] Vayne empowered next attack
- 231.613s [attack_start] Vayne begins auto attack
- 231.613s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 231.857s [attack_start] Dr. Mundo begins auto attack
- 231.859s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 232.063s [champion_script] Vayne executed Tumble Empower
- 232.063s [enemy_buff] Vayne empowered next attack
- 232.084s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 232.097s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 232.198s [attack_start] Vayne begins auto attack
- 232.400s [attack_start] Sona begins auto attack
- 232.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 232.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 232.563s [champion_script] Vayne executed Tumble Empower
- 232.563s [enemy_buff] Vayne empowered next attack
- 232.668s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 232.709s [attack_start] Dr. Mundo begins auto attack
- 232.782s [attack_start] Vayne begins auto attack
- 232.949s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 232.986s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 233.063s [champion_script] Vayne executed Tumble Empower
- 233.063s [enemy_buff] Vayne empowered next attack
- 233.253s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 233.367s [attack_start] Vayne begins auto attack
- 233.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 233.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 233.526s [attack_start] Sona begins auto attack
- 233.560s [attack_start] Dr. Mundo begins auto attack
- 233.563s [champion_script] Vayne executed Tumble Empower
- 233.563s [enemy_buff] Vayne empowered next attack
- 233.633s [enemy_respawn] Morgana respawned
- 233.633s [enemy_death] Sona died; respawn in 54.5s
- 233.633s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 233.633s [champion_script] Morgana executed Dark Binding
- 233.633s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 233.800s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 233.837s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 233.951s [attack_start] Vayne begins auto attack
- 234.063s [champion_script] Vayne executed Tumble Empower
- 234.063s [enemy_buff] Vayne empowered next attack
- 234.412s [attack_start] Dr. Mundo begins auto attack
- 234.422s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 234.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 234.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 234.536s [attack_start] Vayne begins auto attack
- 234.563s [champion_script] Vayne executed Tumble Empower
- 234.563s [enemy_buff] Vayne empowered next attack
- 234.652s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 235.000s [state_snapshot] checkpoint 235.0s (captured_at 235.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1830.4/3688.5 (49.6%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.63s; Pool heal-over-time 0.63s; Untargetable x1 (0.63s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.10s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 10.72s
  Vayne:
    core: pos=(520.0, 150.0) hp=897.1/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.01s to impact); Tumble Empower 0.06s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 22]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=3149.7/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.53s; Dark Binding 0.92s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.63s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.35s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 53.13s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3094.2/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.26s; Infected Bonesaw 0.42s
    runtime: cooldowns [Grasp of the Undying: 2.80s (cooldown 4.00s); Heartsteel Colossal Consumption: 3.75s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.01s)
  projectile_block_zones: none
  ```
- 235.006s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 235.063s [champion_script] Vayne executed Tumble Empower
- 235.063s [enemy_buff] Vayne empowered next attack
- 235.120s [attack_start] Vayne begins auto attack
- 235.263s [attack_start] Dr. Mundo begins auto attack
- 235.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 235.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 235.503s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 235.563s [champion_script] Vayne executed Tumble Empower
- 235.563s [enemy_buff] Vayne empowered next attack
- 235.591s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 235.663s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 235.705s [attack_start] Vayne begins auto attack
- 235.933s [champion_script] Morgana executed Dark Binding
- 235.933s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 236.063s [champion_script] Vayne executed Tumble Empower
- 236.063s [enemy_buff] Vayne empowered next attack
- 236.115s [attack_start] Dr. Mundo begins auto attack
- 236.175s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 236.289s [attack_start] Vayne begins auto attack
- 236.355s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 236.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 236.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 236.563s [champion_script] Vayne executed Tumble Empower
- 236.563s [enemy_buff] Vayne empowered next attack
- 236.760s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 236.873s [attack_start] Vayne begins auto attack
- 236.967s [attack_start] Dr. Mundo begins auto attack
- 237.063s [champion_script] Vayne executed Tumble Empower
- 237.063s [enemy_buff] Vayne empowered next attack
- 237.207s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 237.344s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 237.458s [attack_start] Vayne begins auto attack
- 237.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 237.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 237.563s [champion_script] Vayne executed Tumble Empower
- 237.563s [enemy_buff] Vayne empowered next attack
- 237.667s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 237.818s [attack_start] Dr. Mundo begins auto attack
- 237.929s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 238.042s [attack_start] Vayne begins auto attack
- 238.058s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 238.063s [champion_script] Vayne executed Tumble Empower
- 238.063s [enemy_buff] Vayne empowered next attack
- 238.233s [champion_script] Morgana executed Dark Binding
- 238.233s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 238.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 238.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 238.513s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 238.563s [champion_script] Vayne executed Tumble Empower
- 238.563s [enemy_buff] Vayne empowered next attack
- 238.627s [attack_start] Vayne begins auto attack
- 238.670s [attack_start] Dr. Mundo begins auto attack
- 238.910s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 239.063s [champion_script] Vayne executed Tumble Empower
- 239.063s [enemy_buff] Vayne empowered next attack
- 239.097s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 239.211s [attack_start] Vayne begins auto attack
- 239.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 239.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 239.521s [attack_start] Dr. Mundo begins auto attack
- 239.563s [champion_script] Vayne executed Tumble Empower
- 239.563s [enemy_buff] Vayne empowered next attack
- 239.669s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 239.682s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 239.761s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 239.796s [attack_start] Vayne begins auto attack
- 240.000s [state_snapshot] checkpoint 240.0s (captured_at 240.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2010.1/3688.5 (54.5%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.04s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.67s; Pool heal-over-time 1.67s; Untargetable x1 (1.67s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.32s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 5.72s
  Vayne:
    core: pos=(520.0, 150.0) hp=24.8/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.27s to impact); Tumble Empower 0.06s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 31]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=2277.5/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.49s; Dark Binding 0.52s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.23s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.21s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 48.13s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=2312.8/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.37s; Infected Bonesaw 0.42s
    runtime: cooldowns [Grasp of the Undying: 2.06s (cooldown 4.00s); Heartsteel Colossal Consumption: 6.41s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.27s)
  projectile_block_zones: none
  ```
- 240.063s [champion_script] Vayne executed Tumble Empower
- 240.063s [enemy_buff] Vayne empowered next attack
- 240.266s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 240.373s [attack_start] Dr. Mundo begins auto attack
- 240.380s [attack_start] Vayne begins auto attack
- 240.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 240.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 240.533s [champion_script] Morgana executed Dark Binding
- 240.533s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 240.563s [champion_script] Vayne executed Tumble Empower
- 240.563s [enemy_buff] Vayne empowered next attack
- 240.613s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 240.851s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 240.965s [attack_start] Vayne begins auto attack
- 241.063s [champion_script] Vayne executed Tumble Empower
- 241.063s [enemy_buff] Vayne empowered next attack
- 241.224s [attack_start] Dr. Mundo begins auto attack
- 241.435s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 241.464s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 241.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 241.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 241.549s [attack_start] Vayne begins auto attack
- 241.563s [champion_script] Vayne executed Tumble Empower
- 241.563s [enemy_buff] Vayne empowered next attack
- 241.700s [enemy_death] Vayne died; respawn in 54.5s
- 241.700s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 242.076s [attack_start] Dr. Mundo begins auto attack
- 242.316s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 242.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 242.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 242.833s [champion_script] Morgana executed Dark Binding
- 242.833s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 242.928s [attack_start] Dr. Mundo begins auto attack
- 243.168s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 243.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 243.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 243.733s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 243.779s [attack_start] Dr. Mundo begins auto attack
- 244.019s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 244.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 244.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 244.631s [attack_start] Dr. Mundo begins auto attack
- 244.871s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 245.000s [state_snapshot] checkpoint 245.0s (captured_at 245.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2164.8/3688.5 (58.7%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.73s; Pool heal-over-time 0.73s; Untargetable x1 (0.73s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.17s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 0.72s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.10s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 51.20s
  Morgana:
    core: pos=(-650.0, 120.0) hp=1696.1/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.45s; Dark Binding 0.12s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 0.83s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.07s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 43.13s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=1792.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.48s; Infected Bonesaw 0.42s
    runtime: cooldowns [Grasp of the Undying: 1.32s (cooldown 4.00s); Heartsteel Colossal Consumption: 1.41s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 245.133s [champion_script] Morgana executed Dark Binding
- 245.133s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 245.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 245.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 245.482s [attack_start] Dr. Mundo begins auto attack
- 245.722s [enemy_respawn] Warwick respawned
- 245.722s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 245.734s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 245.918s [attack_start] Warwick begins auto attack
- 246.158s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 246.334s [attack_start] Dr. Mundo begins auto attack
- 246.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 246.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 246.517s [attack_start] Warwick begins auto attack
- 246.574s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 246.757s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 247.102s [attack_start] Warwick begins auto attack
- 247.185s [attack_start] Dr. Mundo begins auto attack
- 247.342s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 247.425s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 247.433s [champion_script] Morgana executed Dark Binding
- 247.433s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 247.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 247.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 247.675s [attack_start] Warwick begins auto attack
- 247.767s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 247.915s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 248.037s [attack_start] Dr. Mundo begins auto attack
- 248.237s [attack_start] Warwick begins auto attack
- 248.277s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 248.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 248.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 248.477s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 248.788s [attack_start] Warwick begins auto attack
- 248.889s [attack_start] Dr. Mundo begins auto attack
- 249.028s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 249.129s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 249.329s [attack_start] Warwick begins auto attack
- 249.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 249.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 249.569s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 249.733s [champion_script] Morgana executed Dark Binding
- 249.733s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 249.740s [attack_start] Dr. Mundo begins auto attack
- 249.787s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 249.869s [attack_start] Warwick begins auto attack
- 249.980s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 250.000s [state_snapshot] checkpoint 250.0s (captured_at 250.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2222.3/3688.5 (60.3%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.16s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.79s; Pool heal-over-time 1.79s; Untargetable x1 (1.79s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2711.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 7]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.04s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 46.20s
  Morgana:
    core: pos=(-650.0, 120.0) hp=823.8/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.41s; Dark Binding 2.02s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 0.43s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.47s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 38.13s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=1010.7/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.59s; Infected Bonesaw 0.42s
    runtime: cooldowns [Grasp of the Undying: 0.57s (cooldown 4.00s); Heartsteel Colossal Consumption: 4.07s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 250.109s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 250.410s [attack_start] Warwick begins auto attack
- 250.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 250.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 250.592s [attack_start] Dr. Mundo begins auto attack
- 250.650s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 250.832s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 250.951s [attack_start] Warwick begins auto attack
- 251.191s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 251.443s [attack_start] Dr. Mundo begins auto attack
- 251.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 251.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 251.492s [attack_start] Warwick begins auto attack
- 251.683s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 251.732s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 251.791s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 252.033s [attack_start] Warwick begins auto attack
- 252.033s [champion_script] Morgana executed Dark Binding
- 252.033s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 252.273s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 252.295s [attack_start] Dr. Mundo begins auto attack
- 252.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 252.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 252.535s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 252.574s [attack_start] Warwick begins auto attack
- 252.814s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 253.114s [attack_start] Warwick begins auto attack
- 253.146s [attack_start] Dr. Mundo begins auto attack
- 253.354s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 253.386s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 253.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 253.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 253.655s [attack_start] Warwick begins auto attack
- 253.800s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 253.895s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 253.998s [attack_start] Dr. Mundo begins auto attack
- 254.196s [attack_start] Warwick begins auto attack
- 254.238s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 254.333s [champion_script] Morgana executed Dark Binding
- 254.333s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 254.436s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 254.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 254.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 254.737s [attack_start] Warwick begins auto attack
- 254.850s [attack_start] Dr. Mundo begins auto attack
- 254.977s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 255.000s [state_snapshot] checkpoint 255.0s (captured_at 255.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2460.8/3688.5 (66.7%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.80s; Pool heal-over-time 0.80s; Untargetable x1 (0.80s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2185.5/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.28s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 17]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.14s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 41.20s
  Morgana:
    core: pos=(-650.0, 120.0) hp=242.4/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.37s; Dark Binding 1.62s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 0.03s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.34s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 33.13s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=489.8/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.42s
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: 6.74s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 255.090s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 255.278s [attack_start] Warwick begins auto attack
- 255.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 255.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 255.518s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 255.701s [attack_start] Dr. Mundo begins auto attack
- 255.818s [attack_start] Warwick begins auto attack
- 255.818s [enemy_death] Morgana died; respawn in 54.5s
- 255.818s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 255.941s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 256.058s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 256.359s [attack_start] Warwick begins auto attack
- 256.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 256.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 256.553s [attack_start] Dr. Mundo begins auto attack
- 256.599s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 256.793s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 256.900s [attack_start] Warwick begins auto attack
- 257.140s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 257.404s [attack_start] Dr. Mundo begins auto attack
- 257.441s [attack_start] Warwick begins auto attack
- 257.467s [champion_script] Dr. Mundo executed Infected Bonesaw
- 257.467s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 257.644s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 257.681s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 257.822s [enemy_death] Dr. Mundo died; respawn in 54.5s
- 257.822s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 257.982s [attack_start] Warwick begins auto attack
- 258.222s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 258.522s [attack_start] Warwick begins auto attack
- 258.762s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 259.063s [attack_start] Warwick begins auto attack
- 259.303s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 259.604s [attack_start] Warwick begins auto attack
- 259.822s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 259.844s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 260.000s [state_snapshot] checkpoint 260.0s (captured_at 260.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2365.2/3688.5 (64.1%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.20s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.82s; Pool heal-over-time 1.82s; Untargetable x1 (1.82s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1396.2/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.14s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 26]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.08s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 36.20s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.32s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 50.32s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.20s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 28.13s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.09s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: 1.74s (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 52.32s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 260.145s [attack_start] Warwick begins auto attack
- 260.385s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 260.686s [attack_start] Warwick begins auto attack
- 260.926s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 261.226s [attack_start] Warwick begins auto attack
- 261.466s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 261.767s [attack_start] Warwick begins auto attack
- 261.822s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 262.007s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 262.308s [attack_start] Warwick begins auto attack
- 262.548s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 262.849s [attack_start] Warwick begins auto attack
- 263.089s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 263.390s [attack_start] Warwick begins auto attack
- 263.630s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 263.822s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 263.931s [attack_start] Warwick begins auto attack
- 264.171s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 264.471s [attack_start] Warwick begins auto attack
- 264.711s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 265.000s [state_snapshot] checkpoint 265.0s (captured_at 265.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2204.7/3688.5 (59.8%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.82s; Pool heal-over-time 0.82s; Untargetable x1 (0.82s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=870.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.01s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 35]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.02s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 31.20s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.28s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 45.32s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.06s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 23.13s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.59s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 47.32s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 265.012s [attack_start] Warwick begins auto attack
- 265.252s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 265.553s [attack_start] Warwick begins auto attack
- 265.793s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 265.822s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 266.094s [attack_start] Warwick begins auto attack
- 266.334s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 266.635s [attack_start] Warwick begins auto attack
- 266.875s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 267.175s [attack_start] Warwick begins auto attack
- 267.415s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 267.716s [attack_start] Warwick begins auto attack
- 267.822s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 267.956s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 268.257s [attack_start] Warwick begins auto attack
- 268.497s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 268.798s [attack_start] Warwick begins auto attack
- 269.038s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 269.339s [attack_start] Warwick begins auto attack
- 269.579s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 269.822s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 269.879s [attack_start] Warwick begins auto attack
- 270.000s [state_snapshot] checkpoint 270.0s (captured_at 270.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1918.2/3688.5 (52.0%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.20s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.82s; Pool heal-over-time 1.82s; Untargetable x1 (1.82s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=80.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 44]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.12s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 26.20s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.24s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 40.32s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.47s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 18.13s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.49s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 42.32s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 270.119s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 270.420s [attack_start] Warwick begins auto attack
- 270.660s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 270.961s [attack_start] Warwick begins auto attack
- 271.201s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 271.502s [attack_start] Warwick begins auto attack
- 271.742s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 271.822s [enemy_death] Warwick died; respawn in 54.5s
- 271.822s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 273.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 275.000s [state_snapshot] checkpoint 275.0s (captured_at 275.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1725.2/3688.5 (46.8%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.83s; Pool heal-over-time 0.83s; Untargetable x1 (0.83s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.03s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 51.32s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.06s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 21.20s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.20s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 35.32s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.33s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 13.13s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.38s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 37.32s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 275.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 277.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 279.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 280.000s [state_snapshot] checkpoint 280.0s (captured_at 280.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1365.4/3688.5 (37.0%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.21s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.83s; Pool heal-over-time 1.83s; Untargetable x1 (1.83s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.25s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 46.32s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.01s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 16.20s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.16s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 30.32s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.19s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 8.13s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.27s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 32.32s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 281.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 283.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 285.000s [state_snapshot] checkpoint 285.0s (captured_at 285.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1168.3/3688.5 (31.7%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.83s; Pool heal-over-time 0.83s; Untargetable x1 (0.83s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.10s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 41.32s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.11s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 11.20s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.12s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 25.32s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.05s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 3.13s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.16s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 27.32s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 285.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 287.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 288.133s [enemy_respawn] Sona respawned
- 288.133s [champion_script] Sona executed Crescendo
- 288.133s [impact_nullified] Sona Crescendo on Vladimir was nullified by untargetable or stasis state
- 288.295s [attack_start] Sona begins auto attack
- 288.881s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 289.421s [attack_start] Sona begins auto attack
- 289.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 290.000s [state_snapshot] checkpoint 290.0s (captured_at 290.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=931.9/3688.5 (25.3%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.21s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.83s; Pool heal-over-time 1.83s; Untargetable x1 (1.83s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.32s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 36.32s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.05s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 6.20s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.08s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 20.32s
  Sona:
    core: pos=(-550.0, -180.0) hp=2122.7/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack in-flight (0.01s to impact); Crescendo 21.50s
    runtime: cooldowns [Luden's Echo: 1.13s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.06s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 22.32s
field:
  projectiles:
    - Sona Auto Attack -> Vladimir (impact in 0.01s)
  projectile_block_zones: none
  ```
- 290.007s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 290.547s [attack_start] Sona begins auto attack
- 291.133s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 291.674s [attack_start] Sona begins auto attack
- 291.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 292.259s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 292.800s [attack_start] Sona begins auto attack
- 293.385s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 293.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 293.926s [attack_start] Sona begins auto attack
- 294.511s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 295.000s [state_snapshot] checkpoint 295.0s (captured_at 295.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=997.3/3688.5 (27.0%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.83s; Pool heal-over-time 0.83s; Untargetable x1 (0.83s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.17s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 31.32s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.15s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 1.20s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.03s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 15.32s
  Sona:
    core: pos=(-550.0, -180.0) hp=1541.3/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.05s; Crescendo 16.50s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.56s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 17.32s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 295.052s [attack_start] Sona begins auto attack
- 295.638s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 295.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 296.178s [attack_start] Sona begins auto attack
- 296.200s [enemy_respawn] Vayne respawned
- 296.200s [champion_script] Vayne executed Tumble Empower
- 296.200s [enemy_buff] Vayne empowered next attack
- 296.264s [attack_start] Vayne begins auto attack
- 296.700s [champion_script] Vayne executed Tumble Empower
- 296.700s [enemy_buff] Vayne empowered next attack
- 296.735s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 296.764s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 296.885s [attack_start] Vayne begins auto attack
- 297.200s [champion_script] Vayne executed Tumble Empower
- 297.200s [enemy_buff] Vayne empowered next attack
- 297.304s [attack_start] Sona begins auto attack
- 297.356s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 297.498s [attack_start] Vayne begins auto attack
- 297.700s [champion_script] Vayne executed Tumble Empower
- 297.700s [enemy_buff] Vayne empowered next attack
- 297.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 297.890s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 297.969s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 298.104s [attack_start] Vayne begins auto attack
- 298.200s [champion_script] Vayne executed Tumble Empower
- 298.200s [enemy_buff] Vayne empowered next attack
- 298.430s [attack_start] Sona begins auto attack
- 298.574s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 298.700s [champion_script] Vayne executed Tumble Empower
- 298.700s [enemy_buff] Vayne empowered next attack
- 298.703s [attack_start] Vayne begins auto attack
- 299.016s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 299.173s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 299.200s [champion_script] Vayne executed Tumble Empower
- 299.200s [enemy_buff] Vayne empowered next attack
- 299.296s [attack_start] Vayne begins auto attack
- 299.556s [attack_start] Sona begins auto attack
- 299.700s [champion_script] Vayne executed Tumble Empower
- 299.700s [enemy_buff] Vayne empowered next attack
- 299.767s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 299.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 299.884s [attack_start] Vayne begins auto attack
- 300.000s [state_snapshot] checkpoint 300.0s (captured_at 300.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1068.6/3688.5 (29.0%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.21s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.83s; Pool heal-over-time 1.83s; Untargetable x1 (1.83s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.02s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 26.32s
  Vayne:
    core: pos=(520.0, 150.0) hp=2060.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.532 (interval 0.117s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower 0.20s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 6/8; Attacks landed: 6]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.61s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 10.32s
  Sona:
    core: pos=(-550.0, -180.0) hp=669.1/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack in-flight (0.14s to impact); Crescendo 11.50s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.45s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 12.32s
field:
  projectiles:
    - Sona Auto Attack -> Vladimir (impact in 0.14s)
  projectile_block_zones: none
  ```
- 300.142s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 300.200s [champion_script] Vayne executed Tumble Empower
- 300.200s [enemy_buff] Vayne empowered next attack
- 300.354s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 300.470s [attack_start] Vayne begins auto attack
- 300.682s [attack_start] Sona begins auto attack
- 300.700s [champion_script] Vayne executed Tumble Empower
- 300.700s [enemy_buff] Vayne empowered next attack
- 300.940s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 301.054s [attack_start] Vayne begins auto attack
- 301.200s [champion_script] Vayne executed Tumble Empower
- 301.200s [enemy_buff] Vayne empowered next attack
- 301.268s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 301.525s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 301.639s [attack_start] Vayne begins auto attack
- 301.700s [champion_script] Vayne executed Tumble Empower
- 301.700s [enemy_buff] Vayne empowered next attack
- 301.808s [attack_start] Sona begins auto attack
- 301.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 302.109s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 302.200s [champion_script] Vayne executed Tumble Empower
- 302.200s [enemy_buff] Vayne empowered next attack
- 302.223s [attack_start] Vayne begins auto attack
- 302.394s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 302.694s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 302.700s [champion_script] Vayne executed Tumble Empower
- 302.700s [enemy_buff] Vayne empowered next attack
- 302.808s [attack_start] Vayne begins auto attack
- 302.934s [attack_start] Sona begins auto attack
- 303.200s [champion_script] Vayne executed Tumble Empower
- 303.200s [enemy_buff] Vayne empowered next attack
- 303.278s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 303.392s [attack_start] Vayne begins auto attack
- 303.520s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 303.700s [champion_script] Vayne executed Tumble Empower
- 303.700s [enemy_buff] Vayne empowered next attack
- 303.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 303.863s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 303.977s [attack_start] Vayne begins auto attack
- 304.061s [attack_start] Sona begins auto attack
- 304.200s [champion_script] Vayne executed Tumble Empower
- 304.200s [enemy_buff] Vayne empowered next attack
- 304.447s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 304.561s [attack_start] Vayne begins auto attack
- 304.646s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 304.700s [champion_script] Vayne executed Tumble Empower
- 304.700s [enemy_buff] Vayne empowered next attack
- 305.000s [state_snapshot] checkpoint 305.0s (captured_at 305.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1314.3/3688.5 (35.6%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.83s; Pool heal-over-time 0.83s; Untargetable x1 (0.83s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.24s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 21.32s
  Vayne:
    core: pos=(520.0, 150.0) hp=1478.5/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.03s to impact); Tumble Empower 0.20s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 14]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.57s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 5.32s
  Sona:
    core: pos=(-550.0, -180.0) hp=87.6/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.19s; Crescendo 6.50s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.35s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 7.32s
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.03s)
  projectile_block_zones: none
  ```
- 305.032s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 305.146s [attack_start] Vayne begins auto attack
- 305.187s [attack_start] Sona begins auto attack
- 305.200s [champion_script] Vayne executed Tumble Empower
- 305.200s [enemy_buff] Vayne empowered next attack
- 305.616s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 305.700s [champion_script] Vayne executed Tumble Empower
- 305.700s [enemy_buff] Vayne empowered next attack
- 305.730s [attack_start] Vayne begins auto attack
- 305.772s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 305.833s [enemy_death] Sona died; respawn in 54.5s
- 305.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 306.200s [champion_script] Vayne executed Tumble Empower
- 306.200s [enemy_buff] Vayne empowered next attack
- 306.201s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 306.314s [attack_start] Vayne begins auto attack
- 306.700s [champion_script] Vayne executed Tumble Empower
- 306.700s [enemy_buff] Vayne empowered next attack
- 306.785s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 306.899s [attack_start] Vayne begins auto attack
- 307.200s [champion_script] Vayne executed Tumble Empower
- 307.200s [enemy_buff] Vayne empowered next attack
- 307.370s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 307.483s [attack_start] Vayne begins auto attack
- 307.700s [champion_script] Vayne executed Tumble Empower
- 307.700s [enemy_buff] Vayne empowered next attack
- 307.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 307.954s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 308.068s [attack_start] Vayne begins auto attack
- 308.200s [champion_script] Vayne executed Tumble Empower
- 308.200s [enemy_buff] Vayne empowered next attack
- 308.538s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 308.652s [attack_start] Vayne begins auto attack
- 308.700s [champion_script] Vayne executed Tumble Empower
- 308.700s [enemy_buff] Vayne empowered next attack
- 309.123s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 309.200s [champion_script] Vayne executed Tumble Empower
- 309.200s [enemy_buff] Vayne empowered next attack
- 309.237s [attack_start] Vayne begins auto attack
- 309.700s [champion_script] Vayne executed Tumble Empower
- 309.700s [enemy_buff] Vayne empowered next attack
- 309.707s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 309.821s [attack_start] Vayne begins auto attack
- 309.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 310.000s [state_snapshot] checkpoint 310.0s (captured_at 310.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1282.8/3688.5 (34.8%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.21s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.83s; Pool heal-over-time 1.83s; Untargetable x1 (1.83s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.09s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 16.32s
  Vayne:
    core: pos=(520.0, 150.0) hp=606.3/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower 0.20s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 23]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.53s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 0.32s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.09s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 50.37s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.24s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 2.32s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 310.200s [champion_script] Vayne executed Tumble Empower
- 310.200s [enemy_buff] Vayne empowered next attack
- 310.292s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 310.333s [enemy_respawn] Morgana respawned
- 310.333s [champion_script] Morgana executed Dark Binding
- 310.333s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 310.406s [attack_start] Vayne begins auto attack
- 310.700s [champion_script] Vayne executed Tumble Empower
- 310.700s [enemy_buff] Vayne empowered next attack
- 310.876s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 310.990s [attack_start] Vayne begins auto attack
- 311.200s [champion_script] Vayne executed Tumble Empower
- 311.200s [enemy_buff] Vayne empowered next attack
- 311.461s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 311.575s [attack_start] Vayne begins auto attack
- 311.700s [champion_script] Vayne executed Tumble Empower
- 311.700s [enemy_buff] Vayne empowered next attack
- 311.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 312.045s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 312.159s [attack_start] Vayne begins auto attack
- 312.200s [champion_script] Vayne executed Tumble Empower
- 312.200s [enemy_buff] Vayne empowered next attack
- 312.327s [enemy_respawn] Dr. Mundo respawned
- 312.327s [champion_script] Dr. Mundo executed Infected Bonesaw
- 312.327s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 312.630s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 312.633s [champion_script] Morgana executed Dark Binding
- 312.633s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 312.686s [attack_start] Dr. Mundo begins auto attack
- 312.700s [champion_script] Vayne executed Tumble Empower
- 312.700s [enemy_buff] Vayne empowered next attack
- 312.744s [attack_start] Vayne begins auto attack
- 312.926s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 313.200s [champion_script] Vayne executed Tumble Empower
- 313.200s [enemy_buff] Vayne empowered next attack
- 313.214s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 313.327s [champion_script] Dr. Mundo executed Infected Bonesaw
- 313.327s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 313.328s [attack_start] Vayne begins auto attack
- 313.537s [attack_start] Dr. Mundo begins auto attack
- 313.700s [champion_script] Vayne executed Tumble Empower
- 313.700s [enemy_buff] Vayne empowered next attack
- 313.777s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 313.799s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 313.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 313.912s [attack_start] Vayne begins auto attack
- 314.200s [champion_script] Vayne executed Tumble Empower
- 314.200s [enemy_buff] Vayne empowered next attack
- 314.327s [champion_script] Dr. Mundo executed Infected Bonesaw
- 314.327s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 314.383s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 314.389s [attack_start] Dr. Mundo begins auto attack
- 314.497s [attack_start] Vayne begins auto attack
- 314.629s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 314.700s [champion_script] Vayne executed Tumble Empower
- 314.700s [enemy_buff] Vayne empowered next attack
- 314.933s [champion_script] Morgana executed Dark Binding
- 314.933s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 314.967s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 315.000s [state_snapshot] checkpoint 315.0s (captured_at 315.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1474.7/3688.5 (40.0%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.83s; Pool heal-over-time 0.83s; Untargetable x1 (0.83s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.31s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 11.32s
  Vayne:
    core: pos=(520.0, 150.0) hp=24.8/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.08s; Tumble Empower 0.20s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 32]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=2859.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.49s; Dark Binding 2.22s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.93s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.50s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 45.37s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=6219.4/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.24s; Infected Bonesaw 0.28s
    runtime: cooldowns [Grasp of the Undying: 1.93s (cooldown 4.00s); Heartsteel Colossal Consumption: 5.43s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 315.081s [attack_start] Vayne begins auto attack
- 315.200s [champion_script] Vayne executed Tumble Empower
- 315.200s [enemy_buff] Vayne empowered next attack
- 315.240s [attack_start] Dr. Mundo begins auto attack
- 315.327s [champion_script] Dr. Mundo executed Infected Bonesaw
- 315.327s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 315.480s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 315.552s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 315.666s [attack_start] Vayne begins auto attack
- 315.700s [champion_script] Vayne executed Tumble Empower
- 315.700s [enemy_buff] Vayne empowered next attack
- 315.833s [enemy_death] Vayne died; respawn in 54.6s
- 315.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 316.092s [attack_start] Dr. Mundo begins auto attack
- 316.327s [champion_script] Dr. Mundo executed Infected Bonesaw
- 316.327s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 316.332s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 316.943s [attack_start] Dr. Mundo begins auto attack
- 317.183s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 317.233s [champion_script] Morgana executed Dark Binding
- 317.233s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 317.327s [champion_script] Dr. Mundo executed Infected Bonesaw
- 317.327s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 317.795s [attack_start] Dr. Mundo begins auto attack
- 317.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 318.035s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 318.327s [champion_script] Dr. Mundo executed Infected Bonesaw
- 318.327s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 318.647s [attack_start] Dr. Mundo begins auto attack
- 318.887s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 319.327s [champion_script] Dr. Mundo executed Infected Bonesaw
- 319.327s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 319.498s [attack_start] Dr. Mundo begins auto attack
- 319.533s [champion_script] Morgana executed Dark Binding
- 319.533s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 319.738s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 319.833s [controlled_champion_cast] Vladimir cast vladimir_sanguine_pool (untargetable 2.00s)
- 320.000s [state_snapshot] checkpoint 320.0s (captured_at 320.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1565.0/3688.5 (42.4%) armor=159.5 mr=54.7
  offense: ap=189.2 ah=4173.0
  loadout: items [Abyssal Mask, Actualizer, Death's Dance, Edge of Night, Horizon Focus, Runaan's Hurricane] | runes [Unsealed Spellbook, Cash Back, Time Warp Tonic, Cosmic Insight, Cheap Shot, Sixth Sense] | shards [ability_haste, movement_speed, health]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.21s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.83s; Pool heal-over-time 1.83s; Untargetable x1 (1.83s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.16s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 6.32s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.01s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 50.44s
  Morgana:
    core: pos=(-650.0, 120.0) hp=1986.8/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.45s; Dark Binding 1.82s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.53s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.36s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 40.37s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=5438.1/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.35s; Infected Bonesaw 0.28s
    runtime: cooldowns [Grasp of the Undying: 1.18s (cooldown 4.00s); Heartsteel Colossal Consumption: 0.43s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 320.327s [champion_script] Dr. Mundo executed Infected Bonesaw
- 320.327s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 320.350s [attack_start] Dr. Mundo begins auto attack
- 320.590s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 321.201s [attack_start] Dr. Mundo begins auto attack
- 321.327s [champion_script] Dr. Mundo executed Infected Bonesaw
- 321.327s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 321.441s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 321.833s [champion_script] Morgana executed Dark Binding
- 321.833s [damage_in] Morgana Dark Binding -> Vladimir | physical 0.0, magic 816.5, true 0.0, total 527.8
- 322.053s [attack_start] Dr. Mundo begins auto attack
- 322.293s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 322.327s [champion_script] Dr. Mundo executed Infected Bonesaw
- 322.327s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 356.6, true 0.0, total 230.5
- 322.904s [attack_start] Dr. Mundo begins auto attack
- 323.144s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 323.327s [champion_script] Dr. Mundo executed Infected Bonesaw
- 323.327s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 287.5, true 0.0, total 185.8
- 323.756s [attack_start] Dr. Mundo begins auto attack
- 323.996s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 324.133s [champion_script] Morgana executed Dark Binding
- 324.133s [damage_in] Morgana Dark Binding -> Vladimir | physical 0.0, magic 973.4, true 0.0, total 629.2
- 324.327s [champion_script] Dr. Mundo executed Infected Bonesaw
- 324.327s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 280.0, true 0.0, total 181.0
- 324.327s [controlled_champion_death] Vladimir died
