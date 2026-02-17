# Vladimir Event Trace

## Optimized Build Trace
- 0.000s [state_snapshot] checkpoint 0.0s (captured_at 0.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=4321.7/4321.7 (100.0%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
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
- 0.000s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
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
- 2.000s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 1296.5, true 0.0, total 838.1
- 2.000s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
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
- 4.018s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
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
  core: pos=(0.0, 0.0) hp=3698.8/4321.7 (85.6%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.02s; Pool heal-over-time 1.02s; Untargetable x1 (1.02s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2545.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 8]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=1585.1/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 8]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=2384.1/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.58s; Dark Binding 1.88s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.60s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=1357.1/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.04s; Crescendo 18.36s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=5533.5/6479.8 armor=192.5 mr=72.7
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
- 6.033s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
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
- 8.058s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
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
  core: pos=(0.0, 0.0) hp=4321.7/4321.7 (100.0%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.06s; Pool heal-over-time 0.06s; Untargetable x1 (0.06s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1907.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.27s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 18]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=880.8/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.10s to impact); Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 16]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=1679.8/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.54s; Dark Binding 1.48s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.20s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=652.8/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack in-flight (0.13s to impact); Crescendo 13.36s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=4902.7/6479.8 armor=192.5 mr=72.7
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
- 10.067s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
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
- 12.100s [enemy_death] Sona died; respawn in 54.5s
- 12.100s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 12.135s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 12.434s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 12.436s [attack_start] Warwick begins auto attack
- 12.500s [champion_script] Vayne executed Tumble Empower
- 12.500s [enemy_buff] Vayne empowered next attack
- 12.534s [attack_start] Dr. Mundo begins auto attack
- 12.548s [attack_start] Vayne begins auto attack
- 12.676s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 12.774s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
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
- 14.059s [attack_start] Warwick begins auto attack
- 14.133s [enemy_death] Vayne died; respawn in 54.5s
- 14.133s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 14.237s [attack_start] Dr. Mundo begins auto attack
- 14.299s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 14.477s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 14.599s [attack_start] Warwick begins auto attack
- 14.839s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 15.000s [state_snapshot] checkpoint 15.0s (captured_at 15.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=4148.7/4321.7 (96.0%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.13s; Pool heal-over-time 1.13s; Untargetable x1 (1.13s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=951.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.14s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 27]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.14s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 53.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=623.4/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.50s; Dark Binding 1.08s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.80s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.09s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 51.60s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3956.4/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.09s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: 2.63s (cooldown 4.00s); Heartsteel Colossal Consumption: 1.02s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 15.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 15.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 15.088s [attack_start] Dr. Mundo begins auto attack
- 15.140s [attack_start] Warwick begins auto attack
- 15.328s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 15.380s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 15.681s [attack_start] Warwick begins auto attack
- 15.921s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 15.940s [attack_start] Dr. Mundo begins auto attack
- 16.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 16.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 16.100s [champion_script] Morgana executed Dark Binding
- 16.100s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 16.133s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 16.180s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 16.222s [attack_start] Warwick begins auto attack
- 16.462s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 16.763s [attack_start] Warwick begins auto attack
- 16.791s [attack_start] Dr. Mundo begins auto attack
- 17.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 17.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 17.003s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 17.031s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 17.304s [attack_start] Warwick begins auto attack
- 17.544s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 17.643s [attack_start] Dr. Mundo begins auto attack
- 17.844s [attack_start] Warwick begins auto attack
- 17.883s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 18.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 18.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 18.084s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 18.133s [enemy_death] Morgana died; respawn in 54.5s
- 18.133s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 18.385s [attack_start] Warwick begins auto attack
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
  core: pos=(0.0, 0.0) hp=4224.9/4321.7 (97.8%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.13s; Pool heal-over-time 0.13s; Untargetable x1 (0.13s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=314.4/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.01s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 36]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.09s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 48.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.45s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 52.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.49s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 46.60s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3325.5/6479.8 armor=192.5 mr=72.7
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
- 20.133s [enemy_death] Warwick died; respawn in 54.5s
- 20.133s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 20.198s [attack_start] Dr. Mundo begins auto attack
- 20.438s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 21.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 21.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 21.049s [attack_start] Dr. Mundo begins auto attack
- 21.289s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 21.901s [attack_start] Dr. Mundo begins auto attack
- 22.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 22.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 22.133s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 22.141s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 22.752s [attack_start] Dr. Mundo begins auto attack
- 22.992s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 23.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 23.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 23.604s [attack_start] Dr. Mundo begins auto attack
- 23.844s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 24.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 24.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 24.133s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 24.456s [attack_start] Dr. Mundo begins auto attack
- 24.696s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 25.000s [state_snapshot] checkpoint 25.0s (captured_at 25.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=3648.4/4321.7 (84.4%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.13s; Pool heal-over-time 1.13s; Untargetable x1 (1.13s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.10s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 49.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.03s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 43.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.41s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 47.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.35s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 41.60s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=2379.2/6479.8 armor=192.5 mr=72.7
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
- 25.307s [attack_start] Dr. Mundo begins auto attack
- 25.547s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 26.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 26.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 26.133s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
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
- 28.133s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
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
  core: pos=(0.0, 0.0) hp=3343.4/4321.7 (77.4%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.13s; Pool heal-over-time 0.13s; Untargetable x1 (0.13s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.32s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 44.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.13s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 38.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.37s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 42.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.22s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 36.60s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=1748.3/6479.8 armor=192.5 mr=72.7
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
- 30.133s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 30.417s [attack_start] Dr. Mundo begins auto attack
- 30.657s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 31.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 31.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 31.268s [attack_start] Dr. Mundo begins auto attack
- 31.508s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 32.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 32.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 32.120s [attack_start] Dr. Mundo begins auto attack
- 32.133s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 32.360s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 32.971s [attack_start] Dr. Mundo begins auto attack
- 33.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 33.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 33.211s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 33.823s [attack_start] Dr. Mundo begins auto attack
- 34.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 34.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 34.063s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 34.167s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 34.674s [attack_start] Dr. Mundo begins auto attack
- 34.914s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 35.000s [state_snapshot] checkpoint 35.0s (captured_at 35.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2859.0/4321.7 (66.2%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.17s; Pool heal-over-time 1.17s; Untargetable x1 (1.17s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.17s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 39.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.07s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 33.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.33s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 37.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.08s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 31.60s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=802.0/6479.8 armor=192.5 mr=72.7
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
- 36.184s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 36.378s [attack_start] Dr. Mundo begins auto attack
- 36.618s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 37.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 37.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 37.229s [attack_start] Dr. Mundo begins auto attack
- 37.469s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 38.000s [champion_script] Dr. Mundo executed Infected Bonesaw
- 38.000s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 38.081s [attack_start] Dr. Mundo begins auto attack
- 38.200s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
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
  core: pos=(0.0, 0.0) hp=2666.2/4321.7 (61.7%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.20s; Pool heal-over-time 0.20s; Untargetable x1 (0.20s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.02s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 34.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.01s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 28.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.29s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 32.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.48s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 26.60s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=171.2/6479.8 armor=192.5 mr=72.7
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
- 40.200s [enemy_death] Dr. Mundo died; respawn in 54.5s
- 40.200s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 42.233s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 44.264s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 45.000s [state_snapshot] checkpoint 45.0s (captured_at 45.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2161.6/4321.7 (50.0%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.26s; Pool heal-over-time 1.26s; Untargetable x1 (1.26s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.24s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 29.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.11s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 23.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.25s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 27.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.34s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 21.60s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.53s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: 1.67s (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 49.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 46.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 48.299s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 50.000s [state_snapshot] checkpoint 50.0s (captured_at 50.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1849.5/4321.7 (42.8%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.30s; Pool heal-over-time 0.30s; Untargetable x1 (0.30s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.09s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 24.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.05s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 18.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.21s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 22.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.21s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 16.60s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.42s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 44.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 50.300s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 52.326s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 54.333s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 55.000s [state_snapshot] checkpoint 55.0s (captured_at 55.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1463.8/4321.7 (33.9%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.33s; Pool heal-over-time 1.33s; Untargetable x1 (1.33s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.31s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 19.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.15s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 13.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.16s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 17.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.07s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 11.60s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.31s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 39.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 56.367s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 58.371s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 60.000s [state_snapshot] checkpoint 60.0s (captured_at 60.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1252.5/4321.7 (29.0%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.37s; Pool heal-over-time 0.37s; Untargetable x1 (0.37s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.16s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 14.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.09s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 8.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.12s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 12.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.47s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 6.60s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.21s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 34.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 60.400s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 62.433s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 64.461s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 65.000s [state_snapshot] checkpoint 65.0s (captured_at 65.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=991.3/4321.7 (22.9%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.46s; Pool heal-over-time 1.46s; Untargetable x1 (1.46s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.01s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 9.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.04s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 3.63s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.08s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 7.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.34s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 1.60s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.10s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 29.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 66.467s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 66.630s [enemy_respawn] Sona respawned
- 66.630s [champion_script] Sona executed Crescendo
- 66.630s [impact_nullified] Sona Crescendo on Vladimir was nullified by untargetable or stasis state
- 66.956s [attack_start] Sona begins auto attack
- 67.542s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 68.082s [attack_start] Sona begins auto attack
- 68.500s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 68.667s [enemy_respawn] Vayne respawned
- 68.667s [champion_script] Vayne executed Tumble Empower
- 68.667s [enemy_buff] Vayne empowered next attack
- 68.668s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 68.702s [attack_start] Vayne begins auto attack
- 69.167s [champion_script] Vayne executed Tumble Empower
- 69.167s [enemy_buff] Vayne empowered next attack
- 69.173s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 69.209s [attack_start] Sona begins auto attack
- 69.323s [attack_start] Vayne begins auto attack
- 69.667s [champion_script] Vayne executed Tumble Empower
- 69.667s [enemy_buff] Vayne empowered next attack
- 69.794s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 69.794s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 69.936s [attack_start] Vayne begins auto attack
- 70.000s [state_snapshot] checkpoint 70.0s (captured_at 70.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=927.4/4321.7 (21.5%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.50s; Pool heal-over-time 0.50s; Untargetable x1 (0.50s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.23s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 4.63s
  Vayne:
    core: pos=(520.0, 150.0) hp=2641.5/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=7.026 (interval 0.142s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower 0.17s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 2/6; Guinsoo stacks: 2/8; Attacks landed: 2]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.04s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 2.63s
  Sona:
    core: pos=(-550.0, -180.0) hp=2061.3/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.33s; Crescendo 19.99s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.60s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 24.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 70.167s [champion_script] Vayne executed Tumble Empower
- 70.167s [enemy_buff] Vayne empowered next attack
- 70.335s [attack_start] Sona begins auto attack
- 70.407s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 70.530s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 70.542s [attack_start] Vayne begins auto attack
- 70.667s [champion_script] Vayne executed Tumble Empower
- 70.667s [enemy_buff] Vayne empowered next attack
- 70.920s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 71.012s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 71.141s [attack_start] Vayne begins auto attack
- 71.167s [champion_script] Vayne executed Tumble Empower
- 71.167s [enemy_buff] Vayne empowered next attack
- 71.461s [attack_start] Sona begins auto attack
- 71.611s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 71.667s [champion_script] Vayne executed Tumble Empower
- 71.667s [enemy_buff] Vayne empowered next attack
- 71.734s [attack_start] Vayne begins auto attack
- 72.047s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 72.167s [champion_script] Vayne executed Tumble Empower
- 72.167s [enemy_buff] Vayne empowered next attack
- 72.205s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 72.322s [attack_start] Vayne begins auto attack
- 72.533s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 72.587s [attack_start] Sona begins auto attack
- 72.667s [enemy_respawn] Morgana respawned
- 72.667s [champion_script] Vayne executed Tumble Empower
- 72.667s [enemy_buff] Vayne empowered next attack
- 72.667s [champion_script] Morgana executed Dark Binding
- 72.667s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 72.792s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 72.908s [attack_start] Vayne begins auto attack
- 73.167s [champion_script] Vayne executed Tumble Empower
- 73.167s [enemy_buff] Vayne empowered next attack
- 73.173s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 73.379s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 73.492s [attack_start] Vayne begins auto attack
- 73.667s [champion_script] Vayne executed Tumble Empower
- 73.667s [enemy_buff] Vayne empowered next attack
- 73.713s [attack_start] Sona begins auto attack
- 73.963s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 74.077s [attack_start] Vayne begins auto attack
- 74.167s [champion_script] Vayne executed Tumble Empower
- 74.167s [enemy_buff] Vayne empowered next attack
- 74.299s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 74.547s [damage_in] Vayne Auto Attack -> Vladimir | physical 721.9, magic 0.0, true 0.0, total 344.6
- 74.547s [attack_hit] Vayne hit Vladimir (phys 721.9, magic 0.0, true 0.0)
- 74.547s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 74.661s [enemy_respawn] Warwick respawned
- 74.661s [attack_start] Vayne begins auto attack
- 74.667s [champion_script] Vayne executed Tumble Empower
- 74.667s [enemy_buff] Vayne empowered next attack
- 74.706s [attack_start] Warwick begins auto attack
- 74.839s [attack_start] Sona begins auto attack
- 74.946s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 74.967s [champion_script] Morgana executed Dark Binding
- 74.967s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 75.000s [state_snapshot] checkpoint 75.0s (captured_at 75.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=884.1/4321.7 (20.5%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.55s; Pool heal-over-time 1.55s; Untargetable x1 (1.55s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=3501.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.788 (interval 0.359s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.30s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 1/6; Attacks landed: 1]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=1585.1/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.13s to impact); Tumble Empower 0.17s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 10]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=3088.3/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.62s; Dark Binding 2.25s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 0.67s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=1004.9/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack unavailable; Crescendo 14.99s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.49s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 19.70s
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.13s)
  projectile_block_zones: none
  ```
- 75.132s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 75.167s [champion_script] Vayne executed Tumble Empower
- 75.167s [enemy_buff] Vayne empowered next attack
- 75.246s [attack_start] Vayne begins auto attack
- 75.305s [attack_start] Warwick begins auto attack
- 75.425s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 75.545s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 75.667s [champion_script] Vayne executed Tumble Empower
- 75.667s [enemy_buff] Vayne empowered next attack
- 75.716s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 75.830s [attack_start] Vayne begins auto attack
- 75.890s [attack_start] Warwick begins auto attack
- 75.965s [attack_start] Sona begins auto attack
- 76.130s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 76.167s [champion_script] Vayne executed Tumble Empower
- 76.167s [enemy_buff] Vayne empowered next attack
- 76.301s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 76.415s [attack_start] Vayne begins auto attack
- 76.463s [attack_start] Warwick begins auto attack
- 76.551s [damage_in] Sona Auto Attack -> Vladimir | physical 106.0, magic 0.0, true 0.0, total 50.6
- 76.551s [attack_hit] Sona hit Vladimir (phys 106.0, magic 0.0, true 0.0)
- 76.551s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 76.667s [champion_script] Vayne executed Tumble Empower
- 76.667s [enemy_buff] Vayne empowered next attack
- 76.703s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 76.885s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 76.999s [attack_start] Vayne begins auto attack
- 77.025s [attack_start] Warwick begins auto attack
- 77.091s [attack_start] Sona begins auto attack
- 77.167s [champion_script] Vayne executed Tumble Empower
- 77.167s [enemy_buff] Vayne empowered next attack
- 77.265s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 77.267s [champion_script] Morgana executed Dark Binding
- 77.267s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 77.470s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 77.576s [attack_start] Warwick begins auto attack
- 77.584s [attack_start] Vayne begins auto attack
- 77.667s [champion_script] Vayne executed Tumble Empower
- 77.667s [enemy_buff] Vayne empowered next attack
- 77.677s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 77.816s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 78.054s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 78.116s [attack_start] Warwick begins auto attack
- 78.167s [champion_script] Vayne executed Tumble Empower
- 78.167s [enemy_buff] Vayne empowered next attack
- 78.168s [attack_start] Vayne begins auto attack
- 78.217s [attack_start] Sona begins auto attack
- 78.356s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 78.553s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 78.639s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 78.657s [attack_start] Warwick begins auto attack
- 78.667s [champion_script] Vayne executed Tumble Empower
- 78.667s [enemy_buff] Vayne empowered next attack
- 78.753s [attack_start] Vayne begins auto attack
- 78.803s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 78.897s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 79.167s [champion_script] Vayne executed Tumble Empower
- 79.167s [enemy_buff] Vayne empowered next attack
- 79.198s [attack_start] Warwick begins auto attack
- 79.223s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 79.337s [attack_start] Vayne begins auto attack
- 79.343s [attack_start] Sona begins auto attack
- 79.438s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 79.567s [champion_script] Morgana executed Dark Binding
- 79.567s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 79.667s [champion_script] Vayne executed Tumble Empower
- 79.667s [enemy_buff] Vayne empowered next attack
- 79.739s [attack_start] Warwick begins auto attack
- 79.808s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 79.921s [attack_start] Vayne begins auto attack
- 79.929s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 79.979s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 80.000s [state_snapshot] checkpoint 80.0s (captured_at 80.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1603.1/4321.7 (37.1%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.55s; Pool heal-over-time 0.55s; Untargetable x1 (0.55s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2863.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.28s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 10]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=880.8/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower 0.17s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 19]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=2384.1/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.58s; Dark Binding 1.85s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 0.27s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=300.7/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.47s; Crescendo 9.99s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.39s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 14.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 80.167s [champion_script] Vayne executed Tumble Empower
- 80.167s [enemy_buff] Vayne empowered next attack
- 80.280s [attack_start] Warwick begins auto attack
- 80.392s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 80.470s [attack_start] Sona begins auto attack
- 80.506s [attack_start] Vayne begins auto attack
- 80.520s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 80.561s [enemy_death] Sona died; respawn in 54.5s
- 80.561s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 80.667s [champion_script] Vayne executed Tumble Empower
- 80.667s [enemy_buff] Vayne empowered next attack
- 80.820s [attack_start] Warwick begins auto attack
- 80.977s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 81.060s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 81.090s [attack_start] Vayne begins auto attack
- 81.167s [champion_script] Vayne executed Tumble Empower
- 81.167s [enemy_buff] Vayne empowered next attack
- 81.361s [attack_start] Warwick begins auto attack
- 81.561s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 81.601s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 81.667s [champion_script] Vayne executed Tumble Empower
- 81.667s [enemy_buff] Vayne empowered next attack
- 81.675s [attack_start] Vayne begins auto attack
- 81.867s [champion_script] Morgana executed Dark Binding
- 81.867s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 81.902s [attack_start] Warwick begins auto attack
- 82.142s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 82.145s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 82.167s [champion_script] Vayne executed Tumble Empower
- 82.167s [enemy_buff] Vayne empowered next attack
- 82.259s [attack_start] Vayne begins auto attack
- 82.443s [attack_start] Warwick begins auto attack
- 82.567s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 82.667s [champion_script] Vayne executed Tumble Empower
- 82.667s [enemy_buff] Vayne empowered next attack
- 82.683s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 82.730s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 82.844s [attack_start] Vayne begins auto attack
- 82.984s [attack_start] Warwick begins auto attack
- 83.167s [champion_script] Vayne executed Tumble Empower
- 83.167s [enemy_buff] Vayne empowered next attack
- 83.224s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 83.314s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 83.428s [attack_start] Vayne begins auto attack
- 83.524s [attack_start] Warwick begins auto attack
- 83.667s [champion_script] Vayne executed Tumble Empower
- 83.667s [enemy_buff] Vayne empowered next attack
- 83.764s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 83.899s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 84.013s [attack_start] Vayne begins auto attack
- 84.065s [attack_start] Warwick begins auto attack
- 84.167s [champion_script] Vayne executed Tumble Empower
- 84.167s [enemy_buff] Vayne empowered next attack
- 84.167s [champion_script] Morgana executed Dark Binding
- 84.167s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 84.305s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 84.483s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 84.597s [attack_start] Vayne begins auto attack
- 84.597s [enemy_death] Vayne died; respawn in 54.5s
- 84.597s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 84.606s [attack_start] Warwick begins auto attack
- 84.846s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 85.000s [state_snapshot] checkpoint 85.0s (captured_at 85.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2033.8/4321.7 (47.1%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.60s; Pool heal-over-time 1.60s; Untargetable x1 (1.60s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1907.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.15s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 19]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.12s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 54.10s
  Morgana:
    core: pos=(-650.0, 120.0) hp=1327.7/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.53s; Dark Binding 1.45s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.53s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 50.06s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.28s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 9.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 85.147s [attack_start] Warwick begins auto attack
- 85.387s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 85.688s [attack_start] Warwick begins auto attack
- 85.928s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 86.228s [attack_start] Warwick begins auto attack
- 86.467s [champion_script] Morgana executed Dark Binding
- 86.467s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 86.468s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 86.600s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 86.769s [attack_start] Warwick begins auto attack
- 87.009s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 87.310s [attack_start] Warwick begins auto attack
- 87.550s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 87.851s [attack_start] Warwick begins auto attack
- 88.091s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 88.392s [attack_start] Warwick begins auto attack
- 88.623s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 88.632s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 88.767s [champion_script] Morgana executed Dark Binding
- 88.767s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 88.933s [attack_start] Warwick begins auto attack
- 89.173s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 89.473s [attack_start] Warwick begins auto attack
- 89.713s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 90.000s [state_snapshot] checkpoint 90.0s (captured_at 90.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2238.6/4321.7 (51.8%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.62s; Pool heal-over-time 0.62s; Untargetable x1 (0.62s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1270.4/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.01s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 28]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.06s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 49.10s
  Morgana:
    core: pos=(-650.0, 120.0) hp=623.4/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.49s; Dark Binding 1.05s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.39s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 45.06s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.17s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 4.70s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 90.014s [attack_start] Warwick begins auto attack
- 90.254s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 90.555s [attack_start] Warwick begins auto attack
- 90.633s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 90.795s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 91.067s [champion_script] Morgana executed Dark Binding
- 91.067s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 91.096s [attack_start] Warwick begins auto attack
- 91.336s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 91.637s [attack_start] Warwick begins auto attack
- 91.877s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 92.177s [attack_start] Warwick begins auto attack
- 92.417s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 92.661s [enemy_death] Morgana died; respawn in 54.5s
- 92.661s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 92.718s [attack_start] Warwick begins auto attack
- 92.958s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 93.259s [attack_start] Warwick begins auto attack
- 93.499s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 93.800s [attack_start] Warwick begins auto attack
- 94.040s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 94.341s [attack_start] Warwick begins auto attack
- 94.581s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 94.667s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 94.717s [enemy_respawn] Dr. Mundo respawned
- 94.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 94.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 94.881s [attack_start] Warwick begins auto attack
- 95.000s [state_snapshot] checkpoint 95.0s (captured_at 95.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2173.1/4321.7 (50.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.67s; Pool heal-over-time 1.67s; Untargetable x1 (1.67s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=314.4/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 37]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.16s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 44.10s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.45s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 52.16s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.26s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 40.06s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=6479.8/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.07s; Infected Bonesaw 0.67s
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 95.065s [attack_start] Dr. Mundo begins auto attack
- 95.121s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 95.305s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 95.422s [attack_start] Warwick begins auto attack
- 95.662s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 95.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 95.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 95.917s [attack_start] Dr. Mundo begins auto attack
- 95.963s [attack_start] Warwick begins auto attack
- 96.157s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 96.203s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 96.504s [attack_start] Warwick begins auto attack
- 96.692s [enemy_death] Warwick died; respawn in 54.5s
- 96.692s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 96.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 96.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 96.768s [attack_start] Dr. Mundo begins auto attack
- 97.008s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 97.620s [attack_start] Dr. Mundo begins auto attack
- 97.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 97.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 97.860s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 98.471s [attack_start] Dr. Mundo begins auto attack
- 98.700s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 98.711s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 98.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 98.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 99.323s [attack_start] Dr. Mundo begins auto attack
- 99.563s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 99.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 99.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 100.000s [state_snapshot] checkpoint 100.0s (captured_at 100.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2163.8/4321.7 (50.1%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.70s; Pool heal-over-time 0.70s; Untargetable x1 (0.70s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.10s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 51.19s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.10s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 39.10s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.41s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 47.16s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.12s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 35.06s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=5848.9/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.17s; Infected Bonesaw 0.67s
    runtime: cooldowns [Grasp of the Undying: 3.56s (cooldown 4.00s); Heartsteel Colossal Consumption: 2.81s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 100.175s [attack_start] Dr. Mundo begins auto attack
- 100.415s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 100.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 100.717s [damage_in] Dr. Mundo Infected Bonesaw -> Vladimir | physical 0.0, magic 659.1, true 0.0, total 426.0
- 100.717s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 101.026s [attack_start] Dr. Mundo begins auto attack
- 101.266s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 101.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 101.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 101.878s [attack_start] Dr. Mundo begins auto attack
- 102.118s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 102.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 102.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 102.729s [attack_start] Dr. Mundo begins auto attack
- 102.729s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 102.969s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 103.581s [attack_start] Dr. Mundo begins auto attack
- 103.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 103.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 103.821s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 104.432s [attack_start] Dr. Mundo begins auto attack
- 104.672s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 104.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 104.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 104.733s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 105.000s [state_snapshot] checkpoint 105.0s (captured_at 105.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1582.7/4321.7 (36.6%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.73s; Pool heal-over-time 1.73s; Untargetable x1 (1.73s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.32s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 46.19s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.04s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 34.10s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.37s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 42.16s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.52s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 30.06s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=4902.7/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.28s; Infected Bonesaw 0.67s
    runtime: cooldowns [Grasp of the Undying: 2.82s (cooldown 4.00s); Heartsteel Colossal Consumption: 5.47s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 105.284s [attack_start] Dr. Mundo begins auto attack
- 105.524s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 105.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 105.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 106.136s [attack_start] Dr. Mundo begins auto attack
- 106.376s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 106.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 106.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 106.767s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 106.987s [attack_start] Dr. Mundo begins auto attack
- 107.227s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 107.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 107.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 107.839s [attack_start] Dr. Mundo begins auto attack
- 108.079s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 108.690s [attack_start] Dr. Mundo begins auto attack
- 108.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 108.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 108.800s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 108.930s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 109.542s [attack_start] Dr. Mundo begins auto attack
- 109.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 109.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 109.782s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 110.000s [state_snapshot] checkpoint 110.0s (captured_at 110.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1568.7/4321.7 (36.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.80s; Pool heal-over-time 0.80s; Untargetable x1 (0.80s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.17s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 41.19s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.14s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 29.10s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.33s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 37.16s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.39s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 25.06s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=4271.8/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.39s; Infected Bonesaw 0.67s
    runtime: cooldowns [Grasp of the Undying: 2.08s (cooldown 4.00s); Heartsteel Colossal Consumption: 0.47s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 110.393s [attack_start] Dr. Mundo begins auto attack
- 110.633s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 110.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 110.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 110.817s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 111.245s [attack_start] Dr. Mundo begins auto attack
- 111.485s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 111.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 111.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 112.097s [attack_start] Dr. Mundo begins auto attack
- 112.337s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 112.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 112.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 112.833s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 112.948s [attack_start] Dr. Mundo begins auto attack
- 113.188s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 113.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 113.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 113.800s [attack_start] Dr. Mundo begins auto attack
- 114.040s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 114.651s [attack_start] Dr. Mundo begins auto attack
- 114.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 114.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 114.867s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 114.891s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 115.000s [state_snapshot] checkpoint 115.0s (captured_at 115.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1446.3/4321.7 (33.5%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.13s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.87s; Pool heal-over-time 1.87s; Untargetable x1 (1.87s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.02s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 36.19s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.08s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 24.10s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.29s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 32.16s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.25s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 20.06s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3325.5/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.50s; Infected Bonesaw 0.67s
    runtime: cooldowns [Grasp of the Undying: 1.34s (cooldown 4.00s); Heartsteel Colossal Consumption: 3.13s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 115.503s [attack_start] Dr. Mundo begins auto attack
- 115.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 115.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 115.743s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 116.354s [attack_start] Dr. Mundo begins auto attack
- 116.594s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 116.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 116.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 116.869s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 117.206s [attack_start] Dr. Mundo begins auto attack
- 117.446s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 117.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 117.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 118.058s [attack_start] Dr. Mundo begins auto attack
- 118.298s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 118.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 118.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 118.900s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 118.909s [attack_start] Dr. Mundo begins auto attack
- 119.149s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 119.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 119.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 119.761s [attack_start] Dr. Mundo begins auto attack
- 120.000s [state_snapshot] checkpoint 120.0s (captured_at 120.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1452.7/4321.7 (33.6%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.90s; Pool heal-over-time 0.90s; Untargetable x1 (0.90s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.24s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 31.19s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.02s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 19.10s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.24s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 27.16s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.11s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 15.06s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=2694.6/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.67s
    runtime: cooldowns [Grasp of the Undying: 0.59s (cooldown 4.00s); Heartsteel Colossal Consumption: 5.80s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 120.001s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 120.612s [attack_start] Dr. Mundo begins auto attack
- 120.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 120.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 120.852s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 120.917s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 121.464s [attack_start] Dr. Mundo begins auto attack
- 121.704s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 121.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 121.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 122.315s [attack_start] Dr. Mundo begins auto attack
- 122.555s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 122.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 122.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 122.933s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 123.167s [attack_start] Dr. Mundo begins auto attack
- 123.407s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 123.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 123.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 124.019s [attack_start] Dr. Mundo begins auto attack
- 124.259s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 124.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 124.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 124.870s [attack_start] Dr. Mundo begins auto attack
- 124.966s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 125.000s [state_snapshot] checkpoint 125.0s (captured_at 125.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1353.5/4321.7 (31.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.23s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.97s; Pool heal-over-time 1.97s; Untargetable x1 (1.97s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.09s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 26.19s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.13s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 14.10s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.20s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 22.16s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.51s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 10.06s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=1748.3/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.67s
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: 0.80s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 125.110s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 125.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 125.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 125.722s [attack_start] Dr. Mundo begins auto attack
- 125.962s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 126.573s [attack_start] Dr. Mundo begins auto attack
- 126.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 126.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 126.813s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 126.967s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 127.425s [attack_start] Dr. Mundo begins auto attack
- 127.665s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 127.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 127.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 128.276s [attack_start] Dr. Mundo begins auto attack
- 128.516s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 128.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 128.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 129.000s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 129.128s [attack_start] Dr. Mundo begins auto attack
- 129.368s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 129.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 129.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 129.980s [attack_start] Dr. Mundo begins auto attack
- 130.000s [state_snapshot] checkpoint 130.0s (captured_at 130.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1372.5/4321.7 (31.8%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.00s; Pool heal-over-time 1.00s; Untargetable x1 (1.00s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.31s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 21.19s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.07s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 9.10s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.16s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 17.16s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.38s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 5.06s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=1117.5/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.67s
    runtime: cooldowns [Grasp of the Undying: 3.37s (cooldown 4.00s); Heartsteel Colossal Consumption: 3.46s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 130.220s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 130.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 130.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 130.831s [attack_start] Dr. Mundo begins auto attack
- 131.017s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 131.071s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 131.683s [attack_start] Dr. Mundo begins auto attack
- 131.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 131.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 131.923s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 132.534s [attack_start] Dr. Mundo begins auto attack
- 132.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 132.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 132.774s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 133.033s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 133.386s [attack_start] Dr. Mundo begins auto attack
- 133.626s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 133.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 133.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 134.237s [attack_start] Dr. Mundo begins auto attack
- 134.477s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 134.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 134.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 135.000s [state_snapshot] checkpoint 135.0s (captured_at 135.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1395.5/4321.7 (32.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.03s; Pool heal-over-time 0.03s; Untargetable x1 (0.03s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.16s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 16.19s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.01s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 4.10s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.12s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 12.16s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.24s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 0.06s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=486.6/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.09s; Infected Bonesaw 0.67s
    runtime: cooldowns [Grasp of the Undying: 2.63s (cooldown 4.00s); Heartsteel Colossal Consumption: 6.13s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 135.067s [enemy_respawn] Sona respawned
- 135.067s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 135.067s [champion_script] Sona executed Crescendo
- 135.067s [impact_nullified] Sona Crescendo on Vladimir was nullified by untargetable or stasis state
- 135.089s [attack_start] Dr. Mundo begins auto attack
- 135.239s [attack_start] Sona begins auto attack
- 135.329s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 135.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 135.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 135.825s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 135.941s [attack_start] Dr. Mundo begins auto attack
- 136.181s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 136.365s [attack_start] Sona begins auto attack
- 136.717s [champion_script] Dr. Mundo executed Infected Bonesaw
- 136.717s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 136.792s [attack_start] Dr. Mundo begins auto attack
- 136.951s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 137.032s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 137.080s [enemy_death] Dr. Mundo died; respawn in 54.5s
- 137.080s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 137.491s [attack_start] Sona begins auto attack
- 138.077s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 138.617s [attack_start] Sona begins auto attack
- 139.100s [enemy_respawn] Vayne respawned
- 139.100s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 139.100s [champion_script] Vayne executed Tumble Empower
- 139.100s [enemy_buff] Vayne empowered next attack
- 139.153s [attack_start] Vayne begins auto attack
- 139.203s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 139.600s [champion_script] Vayne executed Tumble Empower
- 139.600s [enemy_buff] Vayne empowered next attack
- 139.623s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 139.743s [attack_start] Sona begins auto attack
- 139.774s [attack_start] Vayne begins auto attack
- 140.000s [state_snapshot] checkpoint 140.0s (captured_at 140.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1517.3/4321.7 (35.1%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.10s; Pool heal-over-time 1.10s; Untargetable x1 (1.10s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.01s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 11.19s
  Vayne:
    core: pos=(520.0, 150.0) hp=2289.3/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.650 (interval 0.150s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.24s to impact); Tumble Empower 0.10s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 1/6; Guinsoo stacks: 1/8; Attacks landed: 1]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.08s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 7.16s
  Sona:
    core: pos=(-550.0, -180.0) hp=1357.1/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack in-flight (0.33s to impact); Crescendo 18.43s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.09s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: 1.13s (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 51.58s
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.24s)
    - Sona Auto Attack -> Vladimir (impact in 0.33s)
  projectile_block_zones: none
  ```
- 140.100s [champion_script] Vayne executed Tumble Empower
- 140.100s [enemy_buff] Vayne empowered next attack
- 140.244s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 140.329s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 140.387s [attack_start] Vayne begins auto attack
- 140.600s [champion_script] Vayne executed Tumble Empower
- 140.600s [enemy_buff] Vayne empowered next attack
- 140.857s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 140.869s [attack_start] Sona begins auto attack
- 140.992s [attack_start] Vayne begins auto attack
- 141.100s [champion_script] Vayne executed Tumble Empower
- 141.100s [enemy_buff] Vayne empowered next attack
- 141.131s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 141.455s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 141.463s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 141.591s [attack_start] Vayne begins auto attack
- 141.600s [champion_script] Vayne executed Tumble Empower
- 141.600s [enemy_buff] Vayne empowered next attack
- 141.995s [attack_start] Sona begins auto attack
- 142.062s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 142.100s [champion_script] Vayne executed Tumble Empower
- 142.100s [enemy_buff] Vayne empowered next attack
- 142.185s [attack_start] Vayne begins auto attack
- 142.581s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 142.600s [champion_script] Vayne executed Tumble Empower
- 142.600s [enemy_buff] Vayne empowered next attack
- 142.655s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 142.772s [attack_start] Vayne begins auto attack
- 143.100s [champion_script] Vayne executed Tumble Empower
- 143.100s [enemy_buff] Vayne empowered next attack
- 143.122s [attack_start] Sona begins auto attack
- 143.133s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 143.243s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 143.358s [attack_start] Vayne begins auto attack
- 143.600s [champion_script] Vayne executed Tumble Empower
- 143.600s [enemy_buff] Vayne empowered next attack
- 143.707s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 143.829s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 143.943s [attack_start] Vayne begins auto attack
- 144.100s [champion_script] Vayne executed Tumble Empower
- 144.100s [enemy_buff] Vayne empowered next attack
- 144.248s [attack_start] Sona begins auto attack
- 144.414s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 144.527s [attack_start] Vayne begins auto attack
- 144.600s [champion_script] Vayne executed Tumble Empower
- 144.600s [enemy_buff] Vayne empowered next attack
- 144.833s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 144.998s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 145.000s [state_snapshot] checkpoint 145.0s (captured_at 145.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1790.3/4321.7 (41.4%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.13s; Pool heal-over-time 0.13s; Untargetable x1 (0.13s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.23s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 6.19s
  Vayne:
    core: pos=(520.0, 150.0) hp=1585.1/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.11s; Tumble Empower 0.10s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 10]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.04s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 2.16s
  Sona:
    core: pos=(-550.0, -180.0) hp=652.8/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.37s; Crescendo 13.43s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.59s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 46.58s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 145.100s [champion_script] Vayne executed Tumble Empower
- 145.100s [enemy_buff] Vayne empowered next attack
- 145.112s [attack_start] Vayne begins auto attack
- 145.167s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 145.374s [attack_start] Sona begins auto attack
- 145.582s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 145.600s [champion_script] Vayne executed Tumble Empower
- 145.600s [enemy_buff] Vayne empowered next attack
- 145.696s [attack_start] Vayne begins auto attack
- 145.960s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 146.100s [champion_script] Vayne executed Tumble Empower
- 146.100s [enemy_buff] Vayne empowered next attack
- 146.167s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 146.281s [attack_start] Vayne begins auto attack
- 146.500s [attack_start] Sona begins auto attack
- 146.600s [champion_script] Vayne executed Tumble Empower
- 146.600s [enemy_buff] Vayne empowered next attack
- 146.751s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 146.865s [attack_start] Vayne begins auto attack
- 147.086s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 147.100s [champion_script] Vayne executed Tumble Empower
- 147.100s [enemy_buff] Vayne empowered next attack
- 147.167s [enemy_respawn] Morgana respawned
- 147.167s [champion_script] Morgana executed Dark Binding
- 147.167s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 147.200s [enemy_death] Sona died; respawn in 54.5s
- 147.200s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 147.336s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 147.450s [attack_start] Vayne begins auto attack
- 147.600s [champion_script] Vayne executed Tumble Empower
- 147.600s [enemy_buff] Vayne empowered next attack
- 147.920s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 148.034s [attack_start] Vayne begins auto attack
- 148.100s [champion_script] Vayne executed Tumble Empower
- 148.100s [enemy_buff] Vayne empowered next attack
- 148.505s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 148.600s [champion_script] Vayne executed Tumble Empower
- 148.600s [enemy_buff] Vayne empowered next attack
- 148.619s [attack_start] Vayne begins auto attack
- 149.089s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 149.100s [champion_script] Vayne executed Tumble Empower
- 149.100s [enemy_buff] Vayne empowered next attack
- 149.203s [attack_start] Vayne begins auto attack
- 149.203s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 149.467s [champion_script] Morgana executed Dark Binding
- 149.467s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 149.600s [champion_script] Vayne executed Tumble Empower
- 149.600s [enemy_buff] Vayne empowered next attack
- 149.674s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 149.788s [attack_start] Vayne begins auto attack
- 150.000s [state_snapshot] checkpoint 150.0s (captured_at 150.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1971.9/4321.7 (45.6%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.20s; Pool heal-over-time 1.20s; Untargetable x1 (1.20s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.08s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 1.19s
  Vayne:
    core: pos=(520.0, 150.0) hp=528.7/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.26s to impact); Tumble Empower 0.10s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 18]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=2736.2/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.62s; Dark Binding 1.75s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 0.17s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.33s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 51.70s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.49s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 41.58s
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.26s)
  projectile_block_zones: none
  ```
- 150.100s [champion_script] Vayne executed Tumble Empower
- 150.100s [enemy_buff] Vayne empowered next attack
- 150.258s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 150.372s [attack_start] Vayne begins auto attack
- 150.600s [champion_script] Vayne executed Tumble Empower
- 150.600s [enemy_buff] Vayne empowered next attack
- 150.843s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 150.956s [attack_start] Vayne begins auto attack
- 151.100s [champion_script] Vayne executed Tumble Empower
- 151.100s [enemy_buff] Vayne empowered next attack
- 151.200s [enemy_respawn] Warwick respawned
- 151.202s [attack_start] Warwick begins auto attack
- 151.233s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 151.427s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 151.442s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 151.541s [attack_start] Vayne begins auto attack
- 151.600s [champion_script] Vayne executed Tumble Empower
- 151.600s [enemy_buff] Vayne empowered next attack
- 151.767s [champion_script] Morgana executed Dark Binding
- 151.767s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 151.801s [attack_start] Warwick begins auto attack
- 152.011s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 152.041s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 152.100s [champion_script] Vayne executed Tumble Empower
- 152.100s [enemy_buff] Vayne empowered next attack
- 152.125s [attack_start] Vayne begins auto attack
- 152.386s [attack_start] Warwick begins auto attack
- 152.596s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 152.600s [champion_script] Vayne executed Tumble Empower
- 152.600s [enemy_buff] Vayne empowered next attack
- 152.626s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 152.710s [attack_start] Vayne begins auto attack
- 152.959s [attack_start] Warwick begins auto attack
- 153.100s [champion_script] Vayne executed Tumble Empower
- 153.100s [enemy_buff] Vayne empowered next attack
- 153.180s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 153.199s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 153.267s [enemy_death] Vayne died; respawn in 54.5s
- 153.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 153.521s [attack_start] Warwick begins auto attack
- 153.761s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 154.067s [champion_script] Morgana executed Dark Binding
- 154.067s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 154.072s [attack_start] Warwick begins auto attack
- 154.312s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 154.613s [attack_start] Warwick begins auto attack
- 154.853s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 155.000s [state_snapshot] checkpoint 155.0s (captured_at 155.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2300.2/4321.7 (53.2%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2863.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.15s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 7]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.05s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 52.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=2031.9/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.57s; Dark Binding 1.35s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.19s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 46.70s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.38s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 36.58s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 155.153s [attack_start] Warwick begins auto attack
- 155.300s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 155.393s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 155.694s [attack_start] Warwick begins auto attack
- 155.934s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 156.235s [attack_start] Warwick begins auto attack
- 156.367s [champion_script] Morgana executed Dark Binding
- 156.367s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 156.475s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 156.776s [attack_start] Warwick begins auto attack
- 157.016s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 157.317s [attack_start] Warwick begins auto attack
- 157.317s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 157.557s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 157.857s [attack_start] Warwick begins auto attack
- 158.097s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 158.398s [attack_start] Warwick begins auto attack
- 158.638s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 158.667s [champion_script] Morgana executed Dark Binding
- 158.667s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 158.939s [attack_start] Warwick begins auto attack
- 159.179s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 159.333s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 159.480s [attack_start] Warwick begins auto attack
- 159.720s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 160.000s [state_snapshot] checkpoint 160.0s (captured_at 160.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2272.7/4321.7 (52.6%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.33s; Pool heal-over-time 1.33s; Untargetable x1 (1.33s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1907.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.02s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 16]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.15s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 47.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=975.5/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.53s; Dark Binding 0.95s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.05s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 41.70s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.27s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 31.58s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 160.021s [attack_start] Warwick begins auto attack
- 160.261s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 160.562s [attack_start] Warwick begins auto attack
- 160.802s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 160.967s [champion_script] Morgana executed Dark Binding
- 160.967s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 161.102s [attack_start] Warwick begins auto attack
- 161.342s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 161.342s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 161.643s [attack_start] Warwick begins auto attack
- 161.883s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 162.184s [attack_start] Warwick begins auto attack
- 162.424s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 162.725s [attack_start] Warwick begins auto attack
- 162.965s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 163.266s [attack_start] Warwick begins auto attack
- 163.267s [champion_script] Morgana executed Dark Binding
- 163.267s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 163.367s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 163.506s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 163.806s [attack_start] Warwick begins auto attack
- 164.046s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 164.347s [attack_start] Warwick begins auto attack
- 164.587s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 164.888s [attack_start] Warwick begins auto attack
- 165.000s [state_snapshot] checkpoint 165.0s (captured_at 165.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2409.9/4321.7 (55.8%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.37s; Pool heal-over-time 0.37s; Untargetable x1 (0.37s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1270.4/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 25]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.09s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 42.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=271.3/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.49s; Dark Binding 0.55s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.46s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 36.70s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.16s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 26.58s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 165.128s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 165.400s [enemy_death] Morgana died; respawn in 54.5s
- 165.400s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 165.429s [attack_start] Warwick begins auto attack
- 165.669s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 165.970s [attack_start] Warwick begins auto attack
- 166.210s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 166.510s [attack_start] Warwick begins auto attack
- 166.750s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 167.051s [attack_start] Warwick begins auto attack
- 167.291s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 167.433s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 167.592s [attack_start] Warwick begins auto attack
- 167.832s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 168.133s [attack_start] Warwick begins auto attack
- 168.373s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 168.674s [attack_start] Warwick begins auto attack
- 168.914s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 169.214s [attack_start] Warwick begins auto attack
- 169.445s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 169.454s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 169.755s [attack_start] Warwick begins auto attack
- 169.995s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 170.000s [state_snapshot] checkpoint 170.0s (captured_at 170.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2202.9/4321.7 (51.0%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.45s; Pool heal-over-time 1.45s; Untargetable x1 (1.45s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=314.4/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.30s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 35]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.03s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 37.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.45s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 49.90s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.32s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 31.70s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.06s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 21.58s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 170.296s [attack_start] Warwick begins auto attack
- 170.536s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 170.837s [attack_start] Warwick begins auto attack
- 171.077s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 171.378s [attack_start] Warwick begins auto attack
- 171.466s [enemy_death] Warwick died; respawn in 54.5s
- 171.466s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 173.467s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 175.000s [state_snapshot] checkpoint 175.0s (captured_at 175.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2031.2/4321.7 (47.0%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.47s; Pool heal-over-time 0.47s; Untargetable x1 (0.47s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.35s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 50.97s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.13s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 32.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.41s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 44.90s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.18s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 26.70s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.56s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 16.58s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 175.500s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 177.523s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 179.533s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 180.000s [state_snapshot] checkpoint 180.0s (captured_at 180.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1607.6/4321.7 (37.2%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.53s; Pool heal-over-time 1.53s; Untargetable x1 (1.53s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.20s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 45.97s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.07s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 27.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.37s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 39.90s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.04s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 21.70s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.45s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 11.58s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 181.567s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 183.580s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 185.000s [state_snapshot] checkpoint 185.0s (captured_at 185.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1375.5/4321.7 (31.8%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.58s; Pool heal-over-time 0.58s; Untargetable x1 (0.58s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.05s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 40.97s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.01s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 22.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.33s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 34.90s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.45s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 16.70s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.35s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 6.58s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 185.600s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 187.607s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 189.627s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 190.000s [state_snapshot] checkpoint 190.0s (captured_at 190.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1088.6/4321.7 (25.2%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.63s; Pool heal-over-time 1.63s; Untargetable x1 (1.63s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.27s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 35.97s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.12s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 17.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.28s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 29.90s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.31s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 11.70s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.24s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 1.58s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 191.600s [enemy_respawn] Dr. Mundo respawned
- 191.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 191.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 191.633s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 192.073s [attack_start] Dr. Mundo begins auto attack
- 192.313s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 192.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 192.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 192.925s [attack_start] Dr. Mundo begins auto attack
- 193.165s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 193.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 193.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 193.667s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 193.777s [attack_start] Dr. Mundo begins auto attack
- 194.017s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 194.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 194.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 194.628s [attack_start] Dr. Mundo begins auto attack
- 194.868s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 195.000s [state_snapshot] checkpoint 195.0s (captured_at 195.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1082.1/4321.7 (25.0%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.67s; Pool heal-over-time 0.67s; Untargetable x1 (0.67s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.12s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 30.97s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.06s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 12.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.24s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 24.90s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.17s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 6.70s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=5848.9/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.48s; Infected Bonesaw 0.56s
    runtime: cooldowns [Grasp of the Undying: 1.31s (cooldown 4.00s); Heartsteel Colossal Consumption: 4.81s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 195.480s [attack_start] Dr. Mundo begins auto attack
- 195.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 195.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 195.695s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 195.720s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 196.331s [attack_start] Dr. Mundo begins auto attack
- 196.571s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 196.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 196.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 197.183s [attack_start] Dr. Mundo begins auto attack
- 197.423s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 197.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 197.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 197.700s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 198.034s [attack_start] Dr. Mundo begins auto attack
- 198.274s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 198.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 198.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 198.886s [attack_start] Dr. Mundo begins auto attack
- 199.126s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 199.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 199.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 199.733s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 199.738s [attack_start] Dr. Mundo begins auto attack
- 199.978s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 200.000s [state_snapshot] checkpoint 200.0s (captured_at 200.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1062.5/4321.7 (24.6%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.73s; Pool heal-over-time 1.73s; Untargetable x1 (1.73s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.34s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 25.97s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.16s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 7.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.20s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 19.90s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.03s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 1.70s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=4902.7/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.59s; Infected Bonesaw 0.56s
    runtime: cooldowns [Grasp of the Undying: 0.57s (cooldown 4.00s); Heartsteel Colossal Consumption: 7.48s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 200.589s [attack_start] Dr. Mundo begins auto attack
- 200.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 200.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 200.829s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 201.441s [attack_start] Dr. Mundo begins auto attack
- 201.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 201.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 201.681s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 201.733s [enemy_respawn] Sona respawned
- 201.733s [champion_script] Sona executed Crescendo
- 201.733s [impact_nullified] Sona Crescendo on Vladimir was nullified by untargetable or stasis state
- 201.752s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 202.195s [attack_start] Sona begins auto attack
- 202.292s [attack_start] Dr. Mundo begins auto attack
- 202.532s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 202.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 202.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 202.781s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 203.144s [attack_start] Dr. Mundo begins auto attack
- 203.321s [attack_start] Sona begins auto attack
- 203.384s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 203.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 203.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 203.767s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 203.907s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 203.995s [attack_start] Dr. Mundo begins auto attack
- 204.235s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 204.447s [attack_start] Sona begins auto attack
- 204.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 204.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 204.847s [attack_start] Dr. Mundo begins auto attack
- 205.000s [state_snapshot] checkpoint 205.0s (captured_at 205.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1288.0/4321.7 (29.8%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.77s; Pool heal-over-time 0.77s; Untargetable x1 (0.77s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.19s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 20.97s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.10s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 2.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.16s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 14.90s
  Sona:
    core: pos=(-550.0, -180.0) hp=1709.2/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack in-flight (0.03s to impact); Crescendo 20.10s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=4271.8/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.56s
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: 2.48s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles:
    - Sona Auto Attack -> Vladimir (impact in 0.03s)
  projectile_block_zones: none
  ```
- 205.033s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 205.087s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 205.574s [attack_start] Sona begins auto attack
- 205.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 205.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 205.699s [attack_start] Dr. Mundo begins auto attack
- 205.774s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 205.939s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 206.159s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 206.550s [attack_start] Dr. Mundo begins auto attack
- 206.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 206.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 206.700s [attack_start] Sona begins auto attack
- 206.790s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 207.285s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 207.402s [attack_start] Dr. Mundo begins auto attack
- 207.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 207.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 207.642s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 207.799s [enemy_respawn] Vayne respawned
- 207.799s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 207.799s [champion_script] Vayne executed Tumble Empower
- 207.799s [enemy_buff] Vayne empowered next attack
- 207.809s [attack_start] Vayne begins auto attack
- 207.826s [attack_start] Sona begins auto attack
- 208.253s [attack_start] Dr. Mundo begins auto attack
- 208.280s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 208.299s [champion_script] Vayne executed Tumble Empower
- 208.299s [enemy_buff] Vayne empowered next attack
- 208.412s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 208.430s [attack_start] Vayne begins auto attack
- 208.493s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 208.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 208.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 208.799s [champion_script] Vayne executed Tumble Empower
- 208.799s [enemy_buff] Vayne empowered next attack
- 208.901s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 208.952s [attack_start] Sona begins auto attack
- 209.043s [attack_start] Vayne begins auto attack
- 209.105s [attack_start] Dr. Mundo begins auto attack
- 209.299s [champion_script] Vayne executed Tumble Empower
- 209.299s [enemy_buff] Vayne empowered next attack
- 209.345s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 209.514s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 209.538s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 209.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 209.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 209.649s [attack_start] Vayne begins auto attack
- 209.799s [champion_script] Vayne executed Tumble Empower
- 209.799s [enemy_buff] Vayne empowered next attack
- 209.800s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 209.956s [attack_start] Dr. Mundo begins auto attack
- 210.000s [state_snapshot] checkpoint 210.0s (captured_at 210.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1565.1/4321.7 (36.2%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.06s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.80s; Pool heal-over-time 1.80s; Untargetable x1 (1.80s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.04s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 15.97s
  Vayne:
    core: pos=(520.0, 150.0) hp=1937.2/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=7.403 (interval 0.135s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.12s to impact); Tumble Empower 0.30s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 3/6; Guinsoo stacks: 3/8; Attacks landed: 3]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.12s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 9.90s
  Sona:
    core: pos=(-550.0, -180.0) hp=652.8/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.08s; Crescendo 15.10s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3325.5/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.56s
    runtime: cooldowns [Grasp of the Undying: 3.34s (cooldown 4.00s); Heartsteel Colossal Consumption: 5.14s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.12s)
  projectile_block_zones: none
  ```
- 210.078s [attack_start] Sona begins auto attack
- 210.119s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 210.196s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 210.248s [attack_start] Vayne begins auto attack
- 210.299s [champion_script] Vayne executed Tumble Empower
- 210.299s [enemy_buff] Vayne empowered next attack
- 210.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 210.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 210.664s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 210.718s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 210.799s [champion_script] Vayne executed Tumble Empower
- 210.799s [enemy_buff] Vayne empowered next attack
- 210.808s [attack_start] Dr. Mundo begins auto attack
- 210.841s [attack_start] Vayne begins auto attack
- 211.048s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 211.204s [attack_start] Sona begins auto attack
- 211.299s [champion_script] Vayne executed Tumble Empower
- 211.299s [enemy_buff] Vayne empowered next attack
- 211.312s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 211.429s [attack_start] Vayne begins auto attack
- 211.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 211.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 211.660s [attack_start] Dr. Mundo begins auto attack
- 211.790s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 211.799s [champion_script] Vayne executed Tumble Empower
- 211.799s [enemy_buff] Vayne empowered next attack
- 211.833s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 211.900s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 211.900s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 212.015s [attack_start] Vayne begins auto attack
- 212.299s [champion_script] Vayne executed Tumble Empower
- 212.299s [enemy_buff] Vayne empowered next attack
- 212.330s [attack_start] Sona begins auto attack
- 212.486s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 212.511s [attack_start] Dr. Mundo begins auto attack
- 212.599s [attack_start] Vayne begins auto attack
- 212.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 212.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 212.751s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 212.799s [champion_script] Vayne executed Tumble Empower
- 212.799s [enemy_buff] Vayne empowered next attack
- 212.916s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 213.070s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 213.184s [attack_start] Vayne begins auto attack
- 213.299s [champion_script] Vayne executed Tumble Empower
- 213.299s [enemy_buff] Vayne empowered next attack
- 213.363s [attack_start] Dr. Mundo begins auto attack
- 213.456s [attack_start] Sona begins auto attack
- 213.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 213.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 213.603s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 213.655s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 213.768s [attack_start] Vayne begins auto attack
- 213.799s [champion_script] Vayne executed Tumble Empower
- 213.799s [enemy_buff] Vayne empowered next attack
- 213.837s [enemy_death] Sona died; respawn in 54.5s
- 213.837s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 214.214s [attack_start] Dr. Mundo begins auto attack
- 214.239s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 214.299s [champion_script] Vayne executed Tumble Empower
- 214.299s [enemy_buff] Vayne empowered next attack
- 214.353s [attack_start] Vayne begins auto attack
- 214.454s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 214.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 214.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 214.799s [champion_script] Vayne executed Tumble Empower
- 214.799s [enemy_buff] Vayne empowered next attack
- 214.823s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 214.937s [attack_start] Vayne begins auto attack
- 215.000s [state_snapshot] checkpoint 215.0s (captured_at 215.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2026.5/4321.7 (46.9%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.84s; Pool heal-over-time 0.84s; Untargetable x1 (0.84s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.26s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 10.97s
  Vayne:
    core: pos=(520.0, 150.0) hp=1232.9/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower 0.30s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 12]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.08s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 4.90s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.12s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 53.34s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=2694.6/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.07s; Infected Bonesaw 0.56s
    runtime: cooldowns [Grasp of the Undying: 2.60s (cooldown 4.00s); Heartsteel Colossal Consumption: 0.14s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 215.066s [attack_start] Dr. Mundo begins auto attack
- 215.299s [champion_script] Vayne executed Tumble Empower
- 215.299s [enemy_buff] Vayne empowered next attack
- 215.306s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 215.408s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 215.522s [attack_start] Vayne begins auto attack
- 215.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 215.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 215.799s [champion_script] Vayne executed Tumble Empower
- 215.799s [enemy_buff] Vayne empowered next attack
- 215.867s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 215.917s [attack_start] Dr. Mundo begins auto attack
- 215.992s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 216.106s [attack_start] Vayne begins auto attack
- 216.157s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 216.299s [champion_script] Vayne executed Tumble Empower
- 216.299s [enemy_buff] Vayne empowered next attack
- 216.577s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 216.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 216.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 216.691s [attack_start] Vayne begins auto attack
- 216.769s [attack_start] Dr. Mundo begins auto attack
- 216.799s [champion_script] Vayne executed Tumble Empower
- 216.799s [enemy_buff] Vayne empowered next attack
- 217.009s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 217.161s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 217.275s [attack_start] Vayne begins auto attack
- 217.299s [champion_script] Vayne executed Tumble Empower
- 217.299s [enemy_buff] Vayne empowered next attack
- 217.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 217.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 217.621s [attack_start] Dr. Mundo begins auto attack
- 217.746s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 217.799s [champion_script] Vayne executed Tumble Empower
- 217.799s [enemy_buff] Vayne empowered next attack
- 217.860s [attack_start] Vayne begins auto attack
- 217.861s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 217.870s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 218.299s [champion_script] Vayne executed Tumble Empower
- 218.299s [enemy_buff] Vayne empowered next attack
- 218.330s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 218.444s [attack_start] Vayne begins auto attack
- 218.472s [attack_start] Dr. Mundo begins auto attack
- 218.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 218.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 218.712s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 218.799s [champion_script] Vayne executed Tumble Empower
- 218.799s [enemy_buff] Vayne empowered next attack
- 218.915s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 219.029s [attack_start] Vayne begins auto attack
- 219.299s [champion_script] Vayne executed Tumble Empower
- 219.299s [enemy_buff] Vayne empowered next attack
- 219.324s [attack_start] Dr. Mundo begins auto attack
- 219.499s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 219.564s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 219.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 219.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 219.613s [attack_start] Vayne begins auto attack
- 219.799s [champion_script] Vayne executed Tumble Empower
- 219.799s [enemy_buff] Vayne empowered next attack
- 219.899s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 219.933s [enemy_respawn] Morgana respawned
- 219.933s [champion_script] Morgana executed Dark Binding
- 219.933s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 220.000s [state_snapshot] checkpoint 220.0s (captured_at 220.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2066.9/4321.7 (47.8%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool 0.16s; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.90s; Pool heal-over-time 1.90s; Untargetable x1 (1.90s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.11s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 5.97s
  Vayne:
    core: pos=(520.0, 150.0) hp=176.5/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.08s to impact); Tumble Empower 0.30s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 20]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=3440.5/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.04s; Dark Binding 2.22s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.93s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.53s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 48.34s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=1748.3/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.18s; Infected Bonesaw 0.56s
    runtime: cooldowns [Grasp of the Undying: 1.86s (cooldown 4.00s); Heartsteel Colossal Consumption: 2.81s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.08s)
  projectile_block_zones: none
  ```
- 220.084s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 220.175s [attack_start] Dr. Mundo begins auto attack
- 220.197s [attack_start] Vayne begins auto attack
- 220.299s [champion_script] Vayne executed Tumble Empower
- 220.299s [enemy_buff] Vayne empowered next attack
- 220.415s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 220.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 220.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 220.668s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 220.782s [attack_start] Vayne begins auto attack
- 220.799s [champion_script] Vayne executed Tumble Empower
- 220.799s [enemy_buff] Vayne empowered next attack
- 221.027s [attack_start] Dr. Mundo begins auto attack
- 221.252s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 221.267s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 221.299s [champion_script] Vayne executed Tumble Empower
- 221.299s [enemy_buff] Vayne empowered next attack
- 221.366s [attack_start] Vayne begins auto attack
- 221.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 221.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 221.799s [champion_script] Vayne executed Tumble Empower
- 221.799s [enemy_buff] Vayne empowered next attack
- 221.837s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 221.878s [attack_start] Dr. Mundo begins auto attack
- 221.900s [enemy_death] Vayne died; respawn in 54.5s
- 221.900s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 222.118s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 222.233s [champion_script] Morgana executed Dark Binding
- 222.233s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 222.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 222.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 222.730s [attack_start] Dr. Mundo begins auto attack
- 222.970s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 223.582s [attack_start] Dr. Mundo begins auto attack
- 223.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 223.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 223.822s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 223.933s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 224.433s [attack_start] Dr. Mundo begins auto attack
- 224.533s [champion_script] Morgana executed Dark Binding
- 224.533s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 224.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 224.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 224.673s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 225.000s [state_snapshot] checkpoint 225.0s (captured_at 225.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2272.2/4321.7 (52.6%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.93s; Pool heal-over-time 0.93s; Untargetable x1 (0.93s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.33s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 0.97s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.14s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 51.40s
  Morgana:
    core: pos=(-650.0, 120.0) hp=2736.2/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.61s; Dark Binding 1.82s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.53s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.39s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 43.34s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=1117.5/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.28s; Infected Bonesaw 0.56s
    runtime: cooldowns [Grasp of the Undying: 1.12s (cooldown 4.00s); Heartsteel Colossal Consumption: 5.47s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 225.285s [attack_start] Dr. Mundo begins auto attack
- 225.525s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 225.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 225.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 225.936s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 225.967s [enemy_respawn] Warwick respawned
- 226.076s [attack_start] Warwick begins auto attack
- 226.136s [attack_start] Dr. Mundo begins auto attack
- 226.316s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 226.376s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 226.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 226.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 226.675s [attack_start] Warwick begins auto attack
- 226.833s [champion_script] Morgana executed Dark Binding
- 226.833s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 226.915s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 226.988s [attack_start] Dr. Mundo begins auto attack
- 227.228s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 227.260s [attack_start] Warwick begins auto attack
- 227.500s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 227.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 227.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 227.833s [attack_start] Warwick begins auto attack
- 227.839s [attack_start] Dr. Mundo begins auto attack
- 227.967s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 228.073s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 228.079s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 228.395s [attack_start] Warwick begins auto attack
- 228.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 228.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 228.635s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 228.691s [attack_start] Dr. Mundo begins auto attack
- 228.931s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 228.946s [attack_start] Warwick begins auto attack
- 229.133s [champion_script] Morgana executed Dark Binding
- 229.133s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 229.186s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 229.487s [attack_start] Warwick begins auto attack
- 229.542s [attack_start] Dr. Mundo begins auto attack
- 229.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 229.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 229.727s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 229.782s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 230.000s [state_snapshot] checkpoint 230.0s (captured_at 230.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2505.3/4321.7 (58.0%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: none
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=3182.3/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.03s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 7]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.08s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 46.40s
  Morgana:
    core: pos=(-650.0, 120.0) hp=2031.9/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.57s; Dark Binding 1.42s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.13s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.25s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 38.34s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=486.6/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.39s; Infected Bonesaw 0.56s
    runtime: cooldowns [Grasp of the Undying: 0.38s (cooldown 4.00s); Heartsteel Colossal Consumption: 0.47s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 230.000s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 230.027s [attack_start] Warwick begins auto attack
- 230.267s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 230.394s [attack_start] Dr. Mundo begins auto attack
- 230.568s [attack_start] Warwick begins auto attack
- 230.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 230.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 230.634s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 230.808s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 231.109s [attack_start] Warwick begins auto attack
- 231.246s [attack_start] Dr. Mundo begins auto attack
- 231.349s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 231.433s [champion_script] Morgana executed Dark Binding
- 231.433s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 231.486s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 231.600s [champion_script] Dr. Mundo executed Infected Bonesaw
- 231.600s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 231.650s [attack_start] Warwick begins auto attack
- 231.890s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 232.033s [enemy_death] Dr. Mundo died; respawn in 54.5s
- 232.033s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 232.191s [attack_start] Warwick begins auto attack
- 232.431s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 232.731s [attack_start] Warwick begins auto attack
- 232.971s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 233.272s [attack_start] Warwick begins auto attack
- 233.512s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 233.733s [champion_script] Morgana executed Dark Binding
- 233.733s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 233.813s [attack_start] Warwick begins auto attack
- 234.053s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 234.053s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 234.354s [attack_start] Warwick begins auto attack
- 234.594s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 234.895s [attack_start] Warwick begins auto attack
- 235.000s [state_snapshot] checkpoint 235.0s (captured_at 235.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2564.9/4321.7 (59.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.05s; Pool heal-over-time 1.05s; Untargetable x1 (1.05s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2226.4/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 16]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.02s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 41.40s
  Morgana:
    core: pos=(-650.0, 120.0) hp=975.5/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.53s; Dark Binding 1.02s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.73s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.11s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 33.34s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.16s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: 3.13s (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 51.53s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 235.135s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 235.435s [attack_start] Warwick begins auto attack
- 235.675s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 235.976s [attack_start] Warwick begins auto attack
- 236.033s [champion_script] Morgana executed Dark Binding
- 236.033s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 236.067s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 236.216s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 236.517s [attack_start] Warwick begins auto attack
- 236.757s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 237.058s [attack_start] Warwick begins auto attack
- 237.298s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 237.599s [attack_start] Warwick begins auto attack
- 237.839s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 238.100s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 238.139s [attack_start] Warwick begins auto attack
- 238.333s [champion_script] Morgana executed Dark Binding
- 238.333s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 238.379s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 238.680s [attack_start] Warwick begins auto attack
- 238.920s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 239.221s [attack_start] Warwick begins auto attack
- 239.461s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 239.762s [attack_start] Warwick begins auto attack
- 240.000s [state_snapshot] checkpoint 240.0s (captured_at 240.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2662.5/4321.7 (61.6%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.10s; Pool heal-over-time 0.10s; Untargetable x1 (0.10s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1589.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 25]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.12s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 36.40s
  Morgana:
    core: pos=(-650.0, 120.0) hp=271.3/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.49s; Dark Binding 0.62s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.33s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.52s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 28.34s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.05s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 46.53s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 240.002s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 240.122s [enemy_death] Morgana died; respawn in 54.5s
- 240.122s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 240.303s [attack_start] Warwick begins auto attack
- 240.543s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 240.844s [attack_start] Warwick begins auto attack
- 241.084s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 241.384s [attack_start] Warwick begins auto attack
- 241.624s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 241.925s [attack_start] Warwick begins auto attack
- 242.133s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 242.165s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 242.466s [attack_start] Warwick begins auto attack
- 242.706s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 243.007s [attack_start] Warwick begins auto attack
- 243.247s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 243.548s [attack_start] Warwick begins auto attack
- 243.788s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 244.088s [attack_start] Warwick begins auto attack
- 244.167s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 244.328s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 244.629s [attack_start] Warwick begins auto attack
- 244.869s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 245.000s [state_snapshot] checkpoint 245.0s (captured_at 245.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2394.9/4321.7 (55.4%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.17s; Pool heal-over-time 1.17s; Untargetable x1 (1.17s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=633.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.17s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 35]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.06s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 31.40s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.45s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 49.62s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.38s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 23.34s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.55s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 41.53s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 245.170s [attack_start] Warwick begins auto attack
- 245.410s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 245.711s [attack_start] Warwick begins auto attack
- 245.951s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 246.180s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 246.252s [attack_start] Warwick begins auto attack
- 246.492s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 246.792s [attack_start] Warwick begins auto attack
- 247.032s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 247.333s [attack_start] Warwick begins auto attack
- 247.573s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 247.874s [attack_start] Warwick begins auto attack
- 248.114s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 248.200s [enemy_death] Warwick died; respawn in 54.5s
- 248.200s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 250.000s [state_snapshot] checkpoint 250.0s (captured_at 250.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2270.2/4321.7 (52.5%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.20s; Pool heal-over-time 0.20s; Untargetable x1 (0.20s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.28s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 52.70s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.01s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 26.40s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.41s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 44.62s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.24s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 18.34s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.44s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 36.53s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 250.233s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 252.237s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 254.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 255.000s [state_snapshot] checkpoint 255.0s (captured_at 255.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1804.2/4321.7 (41.7%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.13s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 47.70s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.11s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 21.40s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.37s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 39.62s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.10s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 13.34s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.34s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 31.53s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 256.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 258.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 260.000s [state_snapshot] checkpoint 260.0s (captured_at 260.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1543.7/4321.7 (35.7%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.35s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 42.70s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.05s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 16.40s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.32s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 34.62s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.51s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 8.34s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.23s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 26.53s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 260.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 262.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 264.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 265.000s [state_snapshot] checkpoint 265.0s (captured_at 265.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1221.8/4321.7 (28.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.20s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 37.70s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.15s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 11.40s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.28s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 29.62s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.37s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 3.34s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.12s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 21.53s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 266.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 268.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 268.367s [enemy_respawn] Sona respawned
- 268.367s [champion_script] Sona executed Crescendo
- 268.367s [impact_nullified] Sona Crescendo on Vladimir was nullified by untargetable or stasis state
- 268.611s [attack_start] Sona begins auto attack
- 269.197s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 269.737s [attack_start] Sona begins auto attack
- 270.000s [state_snapshot] checkpoint 270.0s (captured_at 270.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1045.4/4321.7 (24.2%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.05s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 32.70s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.09s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 6.40s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.24s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 24.62s
  Sona:
    core: pos=(-550.0, -180.0) hp=2413.5/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack in-flight (0.32s to impact); Crescendo 21.73s
    runtime: cooldowns [Luden's Echo: 1.37s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.01s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 16.53s
field:
  projectiles:
    - Sona Auto Attack -> Vladimir (impact in 0.32s)
  projectile_block_zones: none
  ```
- 270.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 270.323s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 270.864s [attack_start] Sona begins auto attack
- 271.449s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 271.990s [attack_start] Sona begins auto attack
- 272.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 272.575s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 273.116s [attack_start] Sona begins auto attack
- 273.702s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 274.242s [attack_start] Sona begins auto attack
- 274.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 274.828s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 275.000s [state_snapshot] checkpoint 275.0s (captured_at 275.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1054.2/4321.7 (24.4%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.27s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 27.70s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.03s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 1.40s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.20s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 19.62s
  Sona:
    core: pos=(-550.0, -180.0) hp=1357.1/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.37s; Crescendo 16.73s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.52s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 11.53s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 275.368s [attack_start] Sona begins auto attack
- 275.954s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 276.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 276.400s [enemy_respawn] Vayne respawned
- 276.400s [champion_script] Vayne executed Tumble Empower
- 276.400s [enemy_buff] Vayne empowered next attack
- 276.466s [attack_start] Vayne begins auto attack
- 276.494s [attack_start] Sona begins auto attack
- 276.900s [champion_script] Vayne executed Tumble Empower
- 276.900s [enemy_buff] Vayne empowered next attack
- 276.936s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 277.080s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 277.087s [attack_start] Vayne begins auto attack
- 277.400s [champion_script] Vayne executed Tumble Empower
- 277.400s [enemy_buff] Vayne empowered next attack
- 277.557s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 277.620s [attack_start] Sona begins auto attack
- 277.700s [attack_start] Vayne begins auto attack
- 277.900s [champion_script] Vayne executed Tumble Empower
- 277.900s [enemy_buff] Vayne empowered next attack
- 278.170s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 278.206s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 278.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 278.305s [attack_start] Vayne begins auto attack
- 278.400s [champion_script] Vayne executed Tumble Empower
- 278.400s [enemy_buff] Vayne empowered next attack
- 278.746s [attack_start] Sona begins auto attack
- 278.776s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 278.900s [champion_script] Vayne executed Tumble Empower
- 278.900s [enemy_buff] Vayne empowered next attack
- 278.904s [attack_start] Vayne begins auto attack
- 279.332s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 279.375s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 279.400s [champion_script] Vayne executed Tumble Empower
- 279.400s [enemy_buff] Vayne empowered next attack
- 279.498s [attack_start] Vayne begins auto attack
- 279.872s [attack_start] Sona begins auto attack
- 279.900s [champion_script] Vayne executed Tumble Empower
- 279.900s [enemy_buff] Vayne empowered next attack
- 279.968s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 280.000s [state_snapshot] checkpoint 280.0s (captured_at 280.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1240.1/4321.7 (28.7%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.12s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 22.70s
  Vayne:
    core: pos=(520.0, 150.0) hp=2289.3/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.532 (interval 0.117s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.09s; Tumble Empower 0.40s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 6/8; Attacks landed: 6]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.16s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 14.62s
  Sona:
    core: pos=(-550.0, -180.0) hp=652.8/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack unavailable; Crescendo 11.73s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.41s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 6.53s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 280.085s [attack_start] Vayne begins auto attack
- 280.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 280.400s [champion_script] Vayne executed Tumble Empower
- 280.400s [enemy_buff] Vayne empowered next attack
- 280.458s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 280.556s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 280.672s [attack_start] Vayne begins auto attack
- 280.900s [champion_script] Vayne executed Tumble Empower
- 280.900s [enemy_buff] Vayne empowered next attack
- 280.998s [attack_start] Sona begins auto attack
- 281.142s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 281.256s [attack_start] Vayne begins auto attack
- 281.400s [champion_script] Vayne executed Tumble Empower
- 281.400s [enemy_buff] Vayne empowered next attack
- 281.584s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 281.727s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 281.840s [attack_start] Vayne begins auto attack
- 281.900s [champion_script] Vayne executed Tumble Empower
- 281.900s [enemy_buff] Vayne empowered next attack
- 282.124s [attack_start] Sona begins auto attack
- 282.267s [enemy_death] Sona died; respawn in 54.5s
- 282.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 282.311s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 282.400s [champion_script] Vayne executed Tumble Empower
- 282.400s [enemy_buff] Vayne empowered next attack
- 282.425s [attack_start] Vayne begins auto attack
- 282.896s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 282.900s [champion_script] Vayne executed Tumble Empower
- 282.900s [enemy_buff] Vayne empowered next attack
- 283.009s [attack_start] Vayne begins auto attack
- 283.400s [champion_script] Vayne executed Tumble Empower
- 283.400s [enemy_buff] Vayne empowered next attack
- 283.480s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 283.594s [attack_start] Vayne begins auto attack
- 283.900s [champion_script] Vayne executed Tumble Empower
- 283.900s [enemy_buff] Vayne empowered next attack
- 284.064s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 284.178s [attack_start] Vayne begins auto attack
- 284.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 284.400s [champion_script] Vayne executed Tumble Empower
- 284.400s [enemy_buff] Vayne empowered next attack
- 284.649s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 284.763s [attack_start] Vayne begins auto attack
- 284.900s [champion_script] Vayne executed Tumble Empower
- 284.900s [enemy_buff] Vayne empowered next attack
- 285.000s [state_snapshot] checkpoint 285.0s (captured_at 285.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1404.4/4321.7 (32.5%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.34s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 17.70s
  Vayne:
    core: pos=(520.0, 150.0) hp=1232.9/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.23s to impact); Tumble Empower 0.40s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 14]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.12s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 9.62s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.03s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 51.77s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.30s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 1.53s
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.23s)
  projectile_block_zones: none
  ```
- 285.233s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 285.347s [attack_start] Vayne begins auto attack
- 285.400s [champion_script] Vayne executed Tumble Empower
- 285.400s [enemy_buff] Vayne empowered next attack
- 285.818s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 285.900s [champion_script] Vayne executed Tumble Empower
- 285.900s [enemy_buff] Vayne empowered next attack
- 285.932s [attack_start] Vayne begins auto attack
- 286.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 286.400s [champion_script] Vayne executed Tumble Empower
- 286.400s [enemy_buff] Vayne empowered next attack
- 286.402s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 286.516s [attack_start] Vayne begins auto attack
- 286.533s [enemy_respawn] Dr. Mundo respawned
- 286.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 286.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 286.900s [champion_script] Vayne executed Tumble Empower
- 286.900s [enemy_buff] Vayne empowered next attack
- 286.987s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 287.101s [attack_start] Vayne begins auto attack
- 287.139s [attack_start] Dr. Mundo begins auto attack
- 287.379s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 287.400s [champion_script] Vayne executed Tumble Empower
- 287.400s [enemy_buff] Vayne empowered next attack
- 287.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 287.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 287.571s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 287.685s [attack_start] Vayne begins auto attack
- 287.900s [champion_script] Vayne executed Tumble Empower
- 287.900s [enemy_buff] Vayne empowered next attack
- 287.990s [attack_start] Dr. Mundo begins auto attack
- 288.156s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 288.230s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 288.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 288.270s [attack_start] Vayne begins auto attack
- 288.400s [champion_script] Vayne executed Tumble Empower
- 288.400s [enemy_buff] Vayne empowered next attack
- 288.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 288.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 288.740s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 288.842s [attack_start] Dr. Mundo begins auto attack
- 288.854s [attack_start] Vayne begins auto attack
- 288.900s [champion_script] Vayne executed Tumble Empower
- 288.900s [enemy_buff] Vayne empowered next attack
- 289.082s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 289.325s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 289.400s [champion_script] Vayne executed Tumble Empower
- 289.400s [enemy_buff] Vayne empowered next attack
- 289.438s [attack_start] Vayne begins auto attack
- 289.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 289.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 289.693s [attack_start] Dr. Mundo begins auto attack
- 289.900s [champion_script] Vayne executed Tumble Empower
- 289.900s [enemy_buff] Vayne empowered next attack
- 289.909s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 289.933s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 290.000s [state_snapshot] checkpoint 290.0s (captured_at 290.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1530.2/4321.7 (35.4%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.19s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 12.70s
  Vayne:
    core: pos=(520.0, 150.0) hp=528.7/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.02s; Tumble Empower 0.40s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 23]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.08s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 4.62s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.43s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 46.77s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=6164.4/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.54s; Infected Bonesaw 0.49s
    runtime: cooldowns [Grasp of the Undying: 1.38s (cooldown 4.00s); Heartsteel Colossal Consumption: 4.88s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 290.023s [attack_start] Vayne begins auto attack
- 290.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 290.400s [champion_script] Vayne executed Tumble Empower
- 290.400s [enemy_buff] Vayne empowered next attack
- 290.493s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 290.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 290.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 290.545s [attack_start] Dr. Mundo begins auto attack
- 290.607s [attack_start] Vayne begins auto attack
- 290.785s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 290.900s [champion_script] Vayne executed Tumble Empower
- 290.900s [enemy_buff] Vayne empowered next attack
- 291.078s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 291.192s [attack_start] Vayne begins auto attack
- 291.396s [attack_start] Dr. Mundo begins auto attack
- 291.400s [champion_script] Vayne executed Tumble Empower
- 291.400s [enemy_buff] Vayne empowered next attack
- 291.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 291.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 291.636s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 291.662s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 291.776s [attack_start] Vayne begins auto attack
- 291.900s [champion_script] Vayne executed Tumble Empower
- 291.900s [enemy_buff] Vayne empowered next attack
- 292.247s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 292.248s [attack_start] Dr. Mundo begins auto attack
- 292.267s [enemy_death] Vayne died; respawn in 54.5s
- 292.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 292.488s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 292.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 292.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 293.100s [attack_start] Dr. Mundo begins auto attack
- 293.340s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 293.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 293.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 293.951s [attack_start] Dr. Mundo begins auto attack
- 294.191s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 294.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 294.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 294.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 294.633s [enemy_respawn] Morgana respawned
- 294.633s [champion_script] Morgana executed Dark Binding
- 294.633s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 294.803s [attack_start] Dr. Mundo begins auto attack
- 295.000s [state_snapshot] checkpoint 295.0s (captured_at 295.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1574.8/4321.7 (36.4%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.04s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 7.70s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.07s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 51.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=3440.5/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.03s; Dark Binding 1.92s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.63s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.29s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 41.77s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=5218.1/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.49s
    runtime: cooldowns [Grasp of the Undying: 0.64s (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 295.043s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 295.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 295.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 295.654s [attack_start] Dr. Mundo begins auto attack
- 295.894s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 296.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 296.506s [attack_start] Dr. Mundo begins auto attack
- 296.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 296.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 296.746s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 296.933s [champion_script] Morgana executed Dark Binding
- 296.933s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 297.357s [attack_start] Dr. Mundo begins auto attack
- 297.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 297.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 297.597s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 298.209s [attack_start] Dr. Mundo begins auto attack
- 298.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 298.449s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 298.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 298.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 299.061s [attack_start] Dr. Mundo begins auto attack
- 299.233s [champion_script] Morgana executed Dark Binding
- 299.233s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 299.301s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 299.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 299.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 299.912s [attack_start] Dr. Mundo begins auto attack
- 300.000s [state_snapshot] checkpoint 300.0s (captured_at 300.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1757.5/4321.7 (40.7%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.26s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 2.70s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.01s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 46.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=2736.2/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.61s; Dark Binding 1.52s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 2.23s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.15s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 36.77s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=4587.2/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.49s
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: 2.54s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 300.152s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 300.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 300.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 300.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 300.764s [attack_start] Dr. Mundo begins auto attack
- 301.004s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 301.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 301.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 301.533s [champion_script] Morgana executed Dark Binding
- 301.533s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 301.615s [attack_start] Dr. Mundo begins auto attack
- 301.855s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 302.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 302.467s [attack_start] Dr. Mundo begins auto attack
- 302.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 302.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 302.700s [enemy_respawn] Warwick respawned
- 302.707s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 302.873s [attack_start] Warwick begins auto attack
- 303.113s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 303.318s [attack_start] Dr. Mundo begins auto attack
- 303.472s [attack_start] Warwick begins auto attack
- 303.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 303.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 303.558s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 303.712s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 303.833s [champion_script] Morgana executed Dark Binding
- 303.833s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 304.057s [attack_start] Warwick begins auto attack
- 304.170s [attack_start] Dr. Mundo begins auto attack
- 304.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 304.297s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 304.410s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 304.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 304.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 304.630s [attack_start] Warwick begins auto attack
- 304.870s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 305.000s [state_snapshot] checkpoint 305.0s (captured_at 305.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1877.2/4321.7 (43.4%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=3182.3/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.110 (interval 0.322s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.19s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 4/6; Attacks landed: 4]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.11s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 41.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=1679.8/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.57s; Dark Binding 1.12s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.83s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.02s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 31.77s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3640.9/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.02s; Infected Bonesaw 0.49s
    runtime: cooldowns [Grasp of the Undying: 3.41s (cooldown 4.00s); Heartsteel Colossal Consumption: 5.21s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 305.022s [attack_start] Dr. Mundo begins auto attack
- 305.192s [attack_start] Warwick begins auto attack
- 305.262s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 305.432s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 305.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 305.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 305.743s [attack_start] Warwick begins auto attack
- 305.873s [attack_start] Dr. Mundo begins auto attack
- 305.983s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 306.113s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 306.133s [champion_script] Morgana executed Dark Binding
- 306.133s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 306.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 306.284s [attack_start] Warwick begins auto attack
- 306.524s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 306.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 306.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 306.725s [attack_start] Dr. Mundo begins auto attack
- 306.824s [attack_start] Warwick begins auto attack
- 306.965s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 307.064s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 307.365s [attack_start] Warwick begins auto attack
- 307.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 307.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 307.576s [attack_start] Dr. Mundo begins auto attack
- 307.605s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 307.816s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 307.906s [attack_start] Warwick begins auto attack
- 308.146s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 308.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 308.428s [attack_start] Dr. Mundo begins auto attack
- 308.433s [champion_script] Morgana executed Dark Binding
- 308.433s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 308.447s [attack_start] Warwick begins auto attack
- 308.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 308.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 308.668s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 308.687s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 308.988s [attack_start] Warwick begins auto attack
- 309.228s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 309.279s [attack_start] Dr. Mundo begins auto attack
- 309.519s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 309.528s [attack_start] Warwick begins auto attack
- 309.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 309.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 309.768s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 310.000s [state_snapshot] checkpoint 310.0s (captured_at 310.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2296.6/4321.7 (53.1%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2545.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.07s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 13]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.05s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 36.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=975.5/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.53s; Dark Binding 0.72s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.43s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.42s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 26.77s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3010.1/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.13s; Infected Bonesaw 0.49s
    runtime: cooldowns [Grasp of the Undying: 2.67s (cooldown 4.00s); Heartsteel Colossal Consumption: 0.21s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 310.069s [attack_start] Warwick begins auto attack
- 310.131s [attack_start] Dr. Mundo begins auto attack
- 310.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 310.309s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 310.371s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 310.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 310.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 310.610s [attack_start] Warwick begins auto attack
- 310.733s [champion_script] Morgana executed Dark Binding
- 310.733s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 310.850s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 310.983s [attack_start] Dr. Mundo begins auto attack
- 311.151s [attack_start] Warwick begins auto attack
- 311.223s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 311.391s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 311.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 311.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 311.692s [attack_start] Warwick begins auto attack
- 311.834s [attack_start] Dr. Mundo begins auto attack
- 311.932s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 312.074s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 312.233s [attack_start] Warwick begins auto attack
- 312.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 312.473s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 312.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 312.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 312.686s [attack_start] Dr. Mundo begins auto attack
- 312.773s [attack_start] Warwick begins auto attack
- 312.926s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 313.013s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 313.033s [champion_script] Morgana executed Dark Binding
- 313.033s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 313.314s [attack_start] Warwick begins auto attack
- 313.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 313.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 313.537s [attack_start] Dr. Mundo begins auto attack
- 313.554s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 313.777s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 313.855s [attack_start] Warwick begins auto attack
- 314.095s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 314.267s [enemy_death] Morgana died; respawn in 54.6s
- 314.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 314.389s [attack_start] Dr. Mundo begins auto attack
- 314.396s [attack_start] Warwick begins auto attack
- 314.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 314.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 314.629s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 314.636s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 314.937s [attack_start] Warwick begins auto attack
- 315.000s [state_snapshot] checkpoint 315.0s (captured_at 315.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2475.3/4321.7 (57.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1589.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 22]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.15s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 31.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.49s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.03s (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 53.86s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.28s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 21.77s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=2063.8/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.24s; Infected Bonesaw 0.49s
    runtime: cooldowns [Grasp of the Undying: 1.93s (cooldown 4.00s); Heartsteel Colossal Consumption: 2.87s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 315.177s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 315.240s [attack_start] Dr. Mundo begins auto attack
- 315.477s [attack_start] Warwick begins auto attack
- 315.480s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 315.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 315.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 315.717s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 316.018s [attack_start] Warwick begins auto attack
- 316.092s [attack_start] Dr. Mundo begins auto attack
- 316.258s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 316.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 316.332s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 316.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 316.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 316.559s [attack_start] Warwick begins auto attack
- 316.799s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 316.943s [attack_start] Dr. Mundo begins auto attack
- 317.100s [attack_start] Warwick begins auto attack
- 317.183s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 317.340s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 317.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 317.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 317.641s [attack_start] Warwick begins auto attack
- 317.795s [attack_start] Dr. Mundo begins auto attack
- 317.881s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 318.035s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 318.181s [attack_start] Warwick begins auto attack
- 318.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 318.421s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 318.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 318.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 318.647s [attack_start] Dr. Mundo begins auto attack
- 318.722s [attack_start] Warwick begins auto attack
- 318.887s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 318.962s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 319.263s [attack_start] Warwick begins auto attack
- 319.498s [attack_start] Dr. Mundo begins auto attack
- 319.503s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 319.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 319.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 319.738s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 319.804s [attack_start] Warwick begins auto attack
- 320.000s [state_snapshot] checkpoint 320.0s (captured_at 320.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2606.0/4321.7 (60.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=951.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 31]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.10s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 26.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.45s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 48.86s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.14s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 16.77s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=1432.9/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.35s; Infected Bonesaw 0.49s
    runtime: cooldowns [Grasp of the Undying: 1.18s (cooldown 4.00s); Heartsteel Colossal Consumption: 5.54s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 320.044s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 320.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 320.345s [attack_start] Warwick begins auto attack
- 320.350s [attack_start] Dr. Mundo begins auto attack
- 320.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 320.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 320.585s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 320.590s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 320.885s [attack_start] Warwick begins auto attack
- 321.125s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 321.201s [attack_start] Dr. Mundo begins auto attack
- 321.426s [attack_start] Warwick begins auto attack
- 321.441s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 321.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 321.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 321.666s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 321.967s [attack_start] Warwick begins auto attack
- 322.053s [attack_start] Dr. Mundo begins auto attack
- 322.207s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 322.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 322.293s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 322.508s [attack_start] Warwick begins auto attack
- 322.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 322.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 322.748s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 322.904s [attack_start] Dr. Mundo begins auto attack
- 323.049s [attack_start] Warwick begins auto attack
- 323.144s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 323.289s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 323.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 323.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 323.590s [attack_start] Warwick begins auto attack
- 323.756s [attack_start] Dr. Mundo begins auto attack
- 323.830s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 323.996s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 324.130s [attack_start] Warwick begins auto attack
- 324.267s [enemy_death] Warwick died; respawn in 54.7s
- 324.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 324.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 324.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 324.608s [attack_start] Dr. Mundo begins auto attack
- 324.848s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 325.000s [state_snapshot] checkpoint 325.0s (captured_at 325.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2490.6/4321.7 (57.6%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.12s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 53.93s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.04s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 21.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.40s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 43.86s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.01s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 11.77s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=486.6/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.46s; Infected Bonesaw 0.49s
    runtime: cooldowns [Grasp of the Undying: 0.44s (cooldown 4.00s); Heartsteel Colossal Consumption: 0.54s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 325.459s [attack_start] Dr. Mundo begins auto attack
- 325.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 325.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 325.699s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 326.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 326.311s [attack_start] Dr. Mundo begins auto attack
- 326.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 326.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 326.551s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 327.162s [attack_start] Dr. Mundo begins auto attack
- 327.402s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 327.533s [champion_script] Dr. Mundo executed Infected Bonesaw
- 327.533s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 328.014s [attack_start] Dr. Mundo begins auto attack
- 328.254s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 328.267s [enemy_death] Dr. Mundo died; respawn in 54.7s
- 328.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 330.000s [state_snapshot] checkpoint 330.0s (captured_at 330.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2365.4/4321.7 (54.7%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.34s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 48.93s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.14s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 16.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.36s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 38.86s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.41s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 6.77s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.09s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: 3.20s (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 52.96s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 330.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 332.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 334.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 335.000s [state_snapshot] checkpoint 335.0s (captured_at 335.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1877.5/4321.7 (43.4%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.19s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 43.93s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.08s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 11.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.32s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 33.86s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.27s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 1.77s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.59s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 47.96s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 336.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 336.767s [enemy_respawn] Sona respawned
- 336.767s [champion_script] Sona executed Crescendo
- 336.767s [impact_nullified] Sona Crescendo on Vladimir was nullified by untargetable or stasis state
- 336.894s [attack_start] Sona begins auto attack
- 337.480s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 338.020s [attack_start] Sona begins auto attack
- 338.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 338.606s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 339.146s [attack_start] Sona begins auto attack
- 339.732s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 340.000s [state_snapshot] checkpoint 340.0s (captured_at 340.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1698.0/4321.7 (39.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.04s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 38.93s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.02s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 6.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.28s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 28.86s
  Sona:
    core: pos=(-550.0, -180.0) hp=2061.3/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.27s; Crescendo 20.13s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.49s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 42.96s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 340.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 340.272s [attack_start] Sona begins auto attack
- 340.858s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 341.398s [attack_start] Sona begins auto attack
- 341.984s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 342.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 342.524s [attack_start] Sona begins auto attack
- 343.110s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 343.650s [attack_start] Sona begins auto attack
- 344.236s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 344.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 344.776s [attack_start] Sona begins auto attack
- 345.000s [state_snapshot] checkpoint 345.0s (captured_at 345.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1581.9/4321.7 (36.6%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.26s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 33.93s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.12s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 1.77s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.24s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 23.86s
  Sona:
    core: pos=(-550.0, -180.0) hp=1004.9/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack in-flight (0.36s to impact); Crescendo 15.13s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.38s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 37.96s
field:
  projectiles:
    - Sona Auto Attack -> Vladimir (impact in 0.36s)
  projectile_block_zones: none
  ```
- 345.362s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 345.903s [attack_start] Sona begins auto attack
- 346.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 346.488s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 346.767s [enemy_respawn] Vayne respawned
- 346.767s [champion_script] Vayne executed Tumble Empower
- 346.767s [enemy_buff] Vayne empowered next attack
- 346.876s [attack_start] Vayne begins auto attack
- 347.029s [attack_start] Sona begins auto attack
- 347.267s [champion_script] Vayne executed Tumble Empower
- 347.267s [enemy_buff] Vayne empowered next attack
- 347.346s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 347.497s [attack_start] Vayne begins auto attack
- 347.614s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 347.767s [champion_script] Vayne executed Tumble Empower
- 347.767s [enemy_buff] Vayne empowered next attack
- 347.967s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 348.110s [attack_start] Vayne begins auto attack
- 348.155s [attack_start] Sona begins auto attack
- 348.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 348.267s [champion_script] Vayne executed Tumble Empower
- 348.267s [enemy_buff] Vayne empowered next attack
- 348.580s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 348.715s [attack_start] Vayne begins auto attack
- 348.741s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 348.767s [champion_script] Vayne executed Tumble Empower
- 348.767s [enemy_buff] Vayne empowered next attack
- 349.186s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 349.267s [champion_script] Vayne executed Tumble Empower
- 349.267s [enemy_buff] Vayne empowered next attack
- 349.281s [attack_start] Sona begins auto attack
- 349.314s [attack_start] Vayne begins auto attack
- 349.767s [champion_script] Vayne executed Tumble Empower
- 349.767s [enemy_buff] Vayne empowered next attack
- 349.785s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 349.867s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 349.908s [attack_start] Vayne begins auto attack
- 350.000s [state_snapshot] checkpoint 350.0s (captured_at 350.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1691.6/4321.7 (39.1%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.11s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 28.93s
  Vayne:
    core: pos=(520.0, 150.0) hp=2289.3/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.156 (interval 0.123s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower 0.27s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 5/6; Guinsoo stacks: 5/8; Attacks landed: 5]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.20s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 18.86s
  Sona:
    core: pos=(-550.0, -180.0) hp=300.7/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.41s; Crescendo 10.13s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.27s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 32.96s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 350.267s [enemy_death] Sona died; respawn in 54.8s
- 350.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 350.267s [champion_script] Vayne executed Tumble Empower
- 350.267s [enemy_buff] Vayne empowered next attack
- 350.378s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 350.495s [attack_start] Vayne begins auto attack
- 350.767s [champion_script] Vayne executed Tumble Empower
- 350.767s [enemy_buff] Vayne empowered next attack
- 350.966s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 351.081s [attack_start] Vayne begins auto attack
- 351.267s [champion_script] Vayne executed Tumble Empower
- 351.267s [enemy_buff] Vayne empowered next attack
- 351.552s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 351.666s [attack_start] Vayne begins auto attack
- 351.767s [champion_script] Vayne executed Tumble Empower
- 351.767s [enemy_buff] Vayne empowered next attack
- 352.137s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 352.250s [attack_start] Vayne begins auto attack
- 352.267s [champion_script] Vayne executed Tumble Empower
- 352.267s [enemy_buff] Vayne empowered next attack
- 352.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 352.721s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 352.767s [champion_script] Vayne executed Tumble Empower
- 352.767s [enemy_buff] Vayne empowered next attack
- 352.835s [attack_start] Vayne begins auto attack
- 353.267s [champion_script] Vayne executed Tumble Empower
- 353.267s [enemy_buff] Vayne empowered next attack
- 353.305s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 353.419s [attack_start] Vayne begins auto attack
- 353.767s [champion_script] Vayne executed Tumble Empower
- 353.767s [enemy_buff] Vayne empowered next attack
- 353.890s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 354.004s [attack_start] Vayne begins auto attack
- 354.267s [champion_script] Vayne executed Tumble Empower
- 354.267s [enemy_buff] Vayne empowered next attack
- 354.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 354.474s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 354.588s [attack_start] Vayne begins auto attack
- 354.767s [champion_script] Vayne executed Tumble Empower
- 354.767s [enemy_buff] Vayne empowered next attack
- 355.000s [state_snapshot] checkpoint 355.0s (captured_at 355.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1665.1/4321.7 (38.5%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.33s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 23.93s
  Vayne:
    core: pos=(520.0, 150.0) hp=1232.9/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.06s to impact); Tumble Empower 0.27s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 13]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.16s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 13.86s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.27s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 50.10s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.16s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 27.96s
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.06s)
  projectile_block_zones: none
  ```
- 355.059s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 355.173s [attack_start] Vayne begins auto attack
- 355.267s [champion_script] Vayne executed Tumble Empower
- 355.267s [enemy_buff] Vayne empowered next attack
- 355.643s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 355.757s [attack_start] Vayne begins auto attack
- 355.767s [champion_script] Vayne executed Tumble Empower
- 355.767s [enemy_buff] Vayne empowered next attack
- 356.228s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 356.267s [champion_script] Vayne executed Tumble Empower
- 356.267s [enemy_buff] Vayne empowered next attack
- 356.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 356.342s [attack_start] Vayne begins auto attack
- 356.767s [champion_script] Vayne executed Tumble Empower
- 356.767s [enemy_buff] Vayne empowered next attack
- 356.812s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 356.926s [attack_start] Vayne begins auto attack
- 357.267s [champion_script] Vayne executed Tumble Empower
- 357.267s [enemy_buff] Vayne empowered next attack
- 357.397s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 357.511s [attack_start] Vayne begins auto attack
- 357.767s [champion_script] Vayne executed Tumble Empower
- 357.767s [enemy_buff] Vayne empowered next attack
- 357.981s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 358.095s [attack_start] Vayne begins auto attack
- 358.267s [champion_script] Vayne executed Tumble Empower
- 358.267s [enemy_buff] Vayne empowered next attack
- 358.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 358.566s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 358.679s [attack_start] Vayne begins auto attack
- 358.767s [champion_script] Vayne executed Tumble Empower
- 358.767s [enemy_buff] Vayne empowered next attack
- 359.150s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 359.264s [attack_start] Vayne begins auto attack
- 359.267s [champion_script] Vayne executed Tumble Empower
- 359.267s [enemy_buff] Vayne empowered next attack
- 359.734s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 359.767s [champion_script] Vayne executed Tumble Empower
- 359.767s [enemy_buff] Vayne empowered next attack
- 359.848s [attack_start] Vayne begins auto attack
- 360.000s [state_snapshot] checkpoint 360.0s (captured_at 360.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1671.2/4321.7 (38.7%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.18s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 18.93s
  Vayne:
    core: pos=(520.0, 150.0) hp=528.7/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower 0.27s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 22]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.11s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 8.86s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.13s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 45.10s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.06s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 22.96s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 360.267s [champion_script] Vayne executed Tumble Empower
- 360.267s [enemy_buff] Vayne empowered next attack
- 360.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 360.319s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 360.433s [attack_start] Vayne begins auto attack
- 360.767s [champion_script] Vayne executed Tumble Empower
- 360.767s [enemy_buff] Vayne empowered next attack
- 360.903s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 361.017s [attack_start] Vayne begins auto attack
- 361.267s [champion_script] Vayne executed Tumble Empower
- 361.267s [enemy_buff] Vayne empowered next attack
- 361.488s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 361.602s [attack_start] Vayne begins auto attack
- 361.767s [champion_script] Vayne executed Tumble Empower
- 361.767s [enemy_buff] Vayne empowered next attack
- 362.072s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 362.186s [attack_start] Vayne begins auto attack
- 362.267s [champion_script] Vayne executed Tumble Empower
- 362.267s [enemy_buff] Vayne empowered next attack
- 362.267s [enemy_death] Vayne died; respawn in 54.9s
- 362.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 364.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 365.000s [state_snapshot] checkpoint 365.0s (captured_at 365.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1473.2/4321.7 (34.1%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.03s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 13.93s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.10s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 52.18s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.07s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 3.86s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.54s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 40.10s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.56s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 17.96s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 366.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 368.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 368.867s [enemy_respawn] Morgana respawned
- 368.867s [champion_script] Morgana executed Dark Binding
- 368.867s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 370.000s [state_snapshot] checkpoint 370.0s (captured_at 370.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1260.5/4321.7 (29.2%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.25s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 8.93s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.04s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 47.18s
  Morgana:
    core: pos=(-650.0, 120.0) hp=3440.5/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.03s; Dark Binding 1.15s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.87s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.40s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 35.10s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.45s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 12.96s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 370.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 371.167s [champion_script] Morgana executed Dark Binding
- 371.167s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 372.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 373.467s [champion_script] Morgana executed Dark Binding
- 373.467s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 374.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 375.000s [state_snapshot] checkpoint 375.0s (captured_at 375.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1224.5/4321.7 (28.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.10s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 3.93s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.14s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 42.18s
  Morgana:
    core: pos=(-650.0, 120.0) hp=2384.1/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.61s; Dark Binding 0.75s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.47s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.26s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 30.10s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.34s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 7.96s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 375.767s [champion_script] Morgana executed Dark Binding
- 375.767s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 376.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 378.067s [champion_script] Morgana executed Dark Binding
- 378.067s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 378.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 378.933s [enemy_respawn] Warwick respawned
- 379.202s [attack_start] Warwick begins auto attack
- 379.442s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 379.801s [attack_start] Warwick begins auto attack
- 380.000s [state_snapshot] checkpoint 380.0s (captured_at 380.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1294.2/4321.7 (29.9%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=3501.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.788 (interval 0.359s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 1/6; Attacks landed: 1]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.08s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 37.18s
  Morgana:
    core: pos=(-650.0, 120.0) hp=1679.8/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.57s; Dark Binding 0.35s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.07s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.12s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 25.10s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.24s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 2.96s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 380.041s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 380.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 380.367s [champion_script] Morgana executed Dark Binding
- 380.367s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 380.386s [attack_start] Warwick begins auto attack
- 380.626s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 380.959s [attack_start] Warwick begins auto attack
- 381.199s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 381.520s [attack_start] Warwick begins auto attack
- 381.760s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 382.071s [attack_start] Warwick begins auto attack
- 382.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 382.311s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 382.612s [attack_start] Warwick begins auto attack
- 382.667s [champion_script] Morgana executed Dark Binding
- 382.667s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 382.852s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 382.967s [enemy_respawn] Dr. Mundo respawned
- 382.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 382.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 383.153s [attack_start] Warwick begins auto attack
- 383.295s [attack_start] Dr. Mundo begins auto attack
- 383.393s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 383.535s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 383.694s [attack_start] Warwick begins auto attack
- 383.934s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 383.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 383.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 384.147s [attack_start] Dr. Mundo begins auto attack
- 384.235s [attack_start] Warwick begins auto attack
- 384.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 384.387s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 384.475s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 384.775s [attack_start] Warwick begins auto attack
- 384.967s [champion_script] Morgana executed Dark Binding
- 384.967s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 384.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 384.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 384.998s [attack_start] Dr. Mundo begins auto attack
- 385.000s [state_snapshot] checkpoint 385.0s (captured_at 385.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1502.3/4321.7 (34.8%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=2545.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 10]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.02s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 32.18s
  Morgana:
    core: pos=(-650.0, 120.0) hp=623.4/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.53s; Dark Binding 2.25s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 0.67s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.53s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 20.10s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=6164.4/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.92s
    runtime: cooldowns [Grasp of the Undying: 2.54s (cooldown 4.00s); Heartsteel Colossal Consumption: 6.04s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 385.015s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 385.238s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 385.316s [attack_start] Warwick begins auto attack
- 385.556s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 385.850s [attack_start] Dr. Mundo begins auto attack
- 385.857s [attack_start] Warwick begins auto attack
- 385.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 385.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 386.090s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 386.097s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 386.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 386.398s [attack_start] Warwick begins auto attack
- 386.638s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 386.702s [attack_start] Dr. Mundo begins auto attack
- 386.939s [attack_start] Warwick begins auto attack
- 386.942s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 386.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 386.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 387.179s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 387.267s [champion_script] Morgana executed Dark Binding
- 387.267s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 387.479s [attack_start] Warwick begins auto attack
- 387.553s [attack_start] Dr. Mundo begins auto attack
- 387.719s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 387.793s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 387.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 387.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 388.020s [attack_start] Warwick begins auto attack
- 388.260s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 388.267s [enemy_death] Morgana died; respawn in 55.1s
- 388.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 388.405s [attack_start] Dr. Mundo begins auto attack
- 388.561s [attack_start] Warwick begins auto attack
- 388.645s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 388.801s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 388.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 388.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 389.102s [attack_start] Warwick begins auto attack
- 389.256s [attack_start] Dr. Mundo begins auto attack
- 389.342s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 389.496s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 389.643s [attack_start] Warwick begins auto attack
- 389.883s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 389.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 389.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 390.000s [state_snapshot] checkpoint 390.0s (captured_at 390.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1954.8/4321.7 (45.2%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=1907.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.18s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 20]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.12s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 27.18s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.49s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 0.27s (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 53.36s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.39s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 15.10s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=5533.5/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.11s; Infected Bonesaw 0.92s
    runtime: cooldowns [Grasp of the Undying: 1.79s (cooldown 4.00s); Heartsteel Colossal Consumption: 1.04s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 390.108s [attack_start] Dr. Mundo begins auto attack
- 390.183s [attack_start] Warwick begins auto attack
- 390.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 390.348s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 390.423s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 390.724s [attack_start] Warwick begins auto attack
- 390.959s [attack_start] Dr. Mundo begins auto attack
- 390.964s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 390.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 390.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 391.199s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 391.265s [attack_start] Warwick begins auto attack
- 391.505s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 391.806s [attack_start] Warwick begins auto attack
- 391.811s [attack_start] Dr. Mundo begins auto attack
- 391.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 391.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 392.046s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 392.051s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 392.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 392.347s [attack_start] Warwick begins auto attack
- 392.587s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 392.663s [attack_start] Dr. Mundo begins auto attack
- 392.887s [attack_start] Warwick begins auto attack
- 392.903s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 392.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 392.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 393.127s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 393.428s [attack_start] Warwick begins auto attack
- 393.514s [attack_start] Dr. Mundo begins auto attack
- 393.668s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 393.754s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 393.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 393.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 393.969s [attack_start] Warwick begins auto attack
- 394.209s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 394.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 394.366s [attack_start] Dr. Mundo begins auto attack
- 394.510s [attack_start] Warwick begins auto attack
- 394.606s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 394.750s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 394.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 394.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 395.000s [state_snapshot] checkpoint 395.0s (captured_at 395.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1984.3/4321.7 (45.9%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=951.7/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.05s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 29]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.06s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 22.18s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.44s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 48.36s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.25s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 10.10s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=4587.2/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.22s; Infected Bonesaw 0.92s
    runtime: cooldowns [Grasp of the Undying: 1.05s (cooldown 4.00s); Heartsteel Colossal Consumption: 3.70s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 395.051s [attack_start] Warwick begins auto attack
- 395.217s [attack_start] Dr. Mundo begins auto attack
- 395.291s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 395.457s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 395.592s [attack_start] Warwick begins auto attack
- 395.832s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 395.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 395.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 396.069s [attack_start] Dr. Mundo begins auto attack
- 396.132s [attack_start] Warwick begins auto attack
- 396.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 396.309s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 396.372s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 396.673s [attack_start] Warwick begins auto attack
- 396.913s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 396.920s [attack_start] Dr. Mundo begins auto attack
- 396.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 396.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 397.160s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 397.214s [attack_start] Warwick begins auto attack
- 397.454s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 397.755s [attack_start] Warwick begins auto attack
- 397.772s [attack_start] Dr. Mundo begins auto attack
- 397.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 397.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 397.995s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 398.012s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 398.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 398.296s [attack_start] Warwick begins auto attack
- 398.536s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 398.623s [attack_start] Dr. Mundo begins auto attack
- 398.836s [attack_start] Warwick begins auto attack
- 398.863s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 398.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 398.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 399.076s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 399.377s [attack_start] Warwick begins auto attack
- 399.475s [attack_start] Dr. Mundo begins auto attack
- 399.617s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 399.715s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 399.918s [attack_start] Warwick begins auto attack
- 399.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 399.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 400.000s [state_snapshot] checkpoint 400.0s (captured_at 400.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2141.7/4321.7 (49.6%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=314.4/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=3.324 (interval 0.301s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack unavailable; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Attacks landed: 38]
    buffs: none
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.00s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 17.18s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.40s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 43.36s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.11s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 5.10s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3956.4/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.33s; Infected Bonesaw 0.92s
    runtime: cooldowns [Grasp of the Undying: 0.31s (cooldown 4.00s); Heartsteel Colossal Consumption: 6.36s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 400.158s [impact_nullified] Warwick auto attack on Vladimir was nullified by untargetable or stasis state
- 400.267s [enemy_death] Warwick died; respawn in 55.2s
- 400.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 400.327s [attack_start] Dr. Mundo begins auto attack
- 400.567s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 400.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 400.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 401.178s [attack_start] Dr. Mundo begins auto attack
- 401.418s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 401.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 401.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 402.030s [attack_start] Dr. Mundo begins auto attack
- 402.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 402.270s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 402.881s [attack_start] Dr. Mundo begins auto attack
- 402.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 402.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 403.121s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 403.733s [attack_start] Dr. Mundo begins auto attack
- 403.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 403.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 403.973s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 404.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 404.584s [attack_start] Dr. Mundo begins auto attack
- 404.824s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 404.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 404.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 405.000s [state_snapshot] checkpoint 405.0s (captured_at 405.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1999.0/4321.7 (46.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.31s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 50.44s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.11s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 12.18s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.36s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 38.36s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.52s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 0.10s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=3010.1/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.44s; Infected Bonesaw 0.92s
    runtime: cooldowns [Grasp of the Undying: 3.82s (cooldown 4.00s); Heartsteel Colossal Consumption: 1.36s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 405.105s [enemy_respawn] Sona respawned
- 405.105s [champion_script] Sona executed Crescendo
- 405.105s [impact_nullified] Sona Crescendo on Vladimir was nullified by untargetable or stasis state
- 405.436s [attack_start] Dr. Mundo begins auto attack
- 405.517s [attack_start] Sona begins auto attack
- 405.676s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 405.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 405.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 406.102s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 406.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 406.288s [attack_start] Dr. Mundo begins auto attack
- 406.528s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 406.643s [attack_start] Sona begins auto attack
- 406.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 406.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 407.139s [attack_start] Dr. Mundo begins auto attack
- 407.228s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 407.379s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 407.769s [attack_start] Sona begins auto attack
- 407.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 407.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 407.991s [attack_start] Dr. Mundo begins auto attack
- 408.231s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 408.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 408.355s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 408.842s [attack_start] Dr. Mundo begins auto attack
- 408.895s [attack_start] Sona begins auto attack
- 408.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 408.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 409.082s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 409.481s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 409.694s [attack_start] Dr. Mundo begins auto attack
- 409.934s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 409.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 409.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 410.000s [state_snapshot] checkpoint 410.0s (captured_at 410.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2120.5/4321.7 (49.1%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.16s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 45.44s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.05s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 7.18s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.32s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 33.36s
  Sona:
    core: pos=(-550.0, -180.0) hp=1709.2/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.02s; Crescendo 18.47s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=2379.2/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.55s; Infected Bonesaw 0.92s
    runtime: cooldowns [Grasp of the Undying: 3.08s (cooldown 4.00s); Heartsteel Colossal Consumption: 4.03s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 410.021s [attack_start] Sona begins auto attack
- 410.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 410.545s [attack_start] Dr. Mundo begins auto attack
- 410.607s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 410.785s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 410.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 410.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 411.147s [attack_start] Sona begins auto attack
- 411.397s [attack_start] Dr. Mundo begins auto attack
- 411.637s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 411.733s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 411.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 411.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 412.249s [attack_start] Dr. Mundo begins auto attack
- 412.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 412.273s [attack_start] Sona begins auto attack
- 412.489s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 412.859s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 412.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 412.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 413.100s [attack_start] Dr. Mundo begins auto attack
- 413.340s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 413.399s [attack_start] Sona begins auto attack
- 413.952s [attack_start] Dr. Mundo begins auto attack
- 413.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 413.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 413.985s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 414.192s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 414.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 414.525s [attack_start] Sona begins auto attack
- 414.803s [attack_start] Dr. Mundo begins auto attack
- 414.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 414.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 415.000s [state_snapshot] checkpoint 415.0s (captured_at 415.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2129.5/4321.7 (49.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.01s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 40.44s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.15s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 2.18s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.28s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 28.36s
  Sona:
    core: pos=(-550.0, -180.0) hp=652.8/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack in-flight (0.11s to impact); Crescendo 13.47s
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: none
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=1432.9/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.92s
    runtime: cooldowns [Grasp of the Undying: 2.34s (cooldown 4.00s); Heartsteel Colossal Consumption: 6.69s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles:
    - Sona Auto Attack -> Vladimir (impact in 0.11s)
  projectile_block_zones: none
  ```
- 415.043s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 415.111s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 415.651s [attack_start] Sona begins auto attack
- 415.655s [attack_start] Dr. Mundo begins auto attack
- 415.895s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 415.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 415.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 416.237s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 416.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 416.506s [attack_start] Dr. Mundo begins auto attack
- 416.746s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 416.778s [attack_start] Sona begins auto attack
- 416.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 416.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 417.200s [enemy_respawn] Vayne respawned
- 417.200s [champion_script] Vayne executed Tumble Empower
- 417.200s [enemy_buff] Vayne empowered next attack
- 417.220s [attack_start] Vayne begins auto attack
- 417.358s [attack_start] Dr. Mundo begins auto attack
- 417.363s [impact_nullified] Sona auto attack on Vladimir was nullified by untargetable or stasis state
- 417.598s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 417.690s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 417.700s [champion_script] Vayne executed Tumble Empower
- 417.700s [enemy_buff] Vayne empowered next attack
- 417.841s [attack_start] Vayne begins auto attack
- 417.904s [attack_start] Sona begins auto attack
- 417.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 417.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 418.200s [champion_script] Vayne executed Tumble Empower
- 418.200s [enemy_buff] Vayne empowered next attack
- 418.210s [attack_start] Dr. Mundo begins auto attack
- 418.267s [enemy_death] Sona died; respawn in 55.3s
- 418.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 418.311s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 418.450s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 418.454s [attack_start] Vayne begins auto attack
- 418.700s [champion_script] Vayne executed Tumble Empower
- 418.700s [enemy_buff] Vayne empowered next attack
- 418.924s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 418.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 418.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 419.059s [attack_start] Vayne begins auto attack
- 419.061s [attack_start] Dr. Mundo begins auto attack
- 419.200s [champion_script] Vayne executed Tumble Empower
- 419.200s [enemy_buff] Vayne empowered next attack
- 419.301s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 419.530s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 419.659s [attack_start] Vayne begins auto attack
- 419.700s [champion_script] Vayne executed Tumble Empower
- 419.700s [enemy_buff] Vayne empowered next attack
- 419.913s [attack_start] Dr. Mundo begins auto attack
- 419.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 419.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 420.000s [state_snapshot] checkpoint 420.0s (captured_at 420.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2367.5/4321.7 (54.8%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.23s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 35.44s
  Vayne:
    core: pos=(520.0, 150.0) hp=2289.3/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=7.779 (interval 0.129s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.13s to impact); Tumble Empower 0.20s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 4/6; Guinsoo stacks: 4/8; Attacks landed: 4]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.24s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 23.36s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.11s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 53.56s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=802.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack unavailable; Infected Bonesaw 0.92s
    runtime: cooldowns [Grasp of the Undying: 1.60s (cooldown 4.00s); Heartsteel Colossal Consumption: 1.69s (cooldown 7.50s)] | stacks [none]
    buffs: none
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.13s)
  projectile_block_zones: none
  ```
- 420.129s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 420.153s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 420.200s [champion_script] Vayne executed Tumble Empower
- 420.200s [enemy_buff] Vayne empowered next attack
- 420.252s [attack_start] Vayne begins auto attack
- 420.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 420.700s [champion_script] Vayne executed Tumble Empower
- 420.700s [enemy_buff] Vayne empowered next attack
- 420.722s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 420.764s [attack_start] Dr. Mundo begins auto attack
- 420.840s [attack_start] Vayne begins auto attack
- 420.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 420.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 421.004s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 421.200s [champion_script] Vayne executed Tumble Empower
- 421.200s [enemy_buff] Vayne empowered next attack
- 421.310s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 421.426s [attack_start] Vayne begins auto attack
- 421.616s [attack_start] Dr. Mundo begins auto attack
- 421.700s [champion_script] Vayne executed Tumble Empower
- 421.700s [enemy_buff] Vayne empowered next attack
- 421.856s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 421.896s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 421.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 421.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 422.010s [attack_start] Vayne begins auto attack
- 422.200s [champion_script] Vayne executed Tumble Empower
- 422.200s [enemy_buff] Vayne empowered next attack
- 422.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 422.467s [attack_start] Dr. Mundo begins auto attack
- 422.481s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 422.595s [attack_start] Vayne begins auto attack
- 422.700s [champion_script] Vayne executed Tumble Empower
- 422.700s [enemy_buff] Vayne empowered next attack
- 422.707s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 422.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 422.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 423.065s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 423.179s [attack_start] Vayne begins auto attack
- 423.200s [champion_script] Vayne executed Tumble Empower
- 423.200s [enemy_buff] Vayne empowered next attack
- 423.319s [attack_start] Dr. Mundo begins auto attack
- 423.559s [attack_missed] Dr. Mundo auto attack missed Vladimir (target outside hitbox path (distance 216.3 > reach 145.0))
- 423.650s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 423.700s [champion_script] Vayne executed Tumble Empower
- 423.700s [enemy_buff] Vayne empowered next attack
- 423.764s [attack_start] Vayne begins auto attack
- 423.967s [champion_script] Dr. Mundo executed Infected Bonesaw
- 423.967s [impact_nullified] Dr. Mundo Infected Bonesaw on Vladimir was nullified by untargetable or stasis state
- 424.171s [attack_start] Dr. Mundo begins auto attack
- 424.200s [champion_script] Vayne executed Tumble Empower
- 424.200s [enemy_buff] Vayne empowered next attack
- 424.234s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 424.267s [enemy_death] Dr. Mundo died; respawn in 55.3s
- 424.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 424.348s [attack_start] Vayne begins auto attack
- 424.700s [champion_script] Vayne executed Tumble Empower
- 424.700s [enemy_buff] Vayne empowered next attack
- 424.819s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 424.932s [attack_start] Vayne begins auto attack
- 425.000s [state_snapshot] checkpoint 425.0s (captured_at 425.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2318.6/4321.7 (53.7%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.08s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 30.44s
  Vayne:
    core: pos=(520.0, 150.0) hp=1232.9/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack unavailable; Tumble Empower 0.20s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 13]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.20s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 18.36s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.51s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 48.56s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.02s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: 0.86s (cooldown 4.00s); Heartsteel Colossal Consumption: 4.36s (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 54.60s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 425.200s [champion_script] Vayne executed Tumble Empower
- 425.200s [enemy_buff] Vayne empowered next attack
- 425.403s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 425.517s [attack_start] Vayne begins auto attack
- 425.700s [champion_script] Vayne executed Tumble Empower
- 425.700s [enemy_buff] Vayne empowered next attack
- 425.988s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 426.101s [attack_start] Vayne begins auto attack
- 426.200s [champion_script] Vayne executed Tumble Empower
- 426.200s [enemy_buff] Vayne empowered next attack
- 426.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 426.572s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 426.686s [attack_start] Vayne begins auto attack
- 426.700s [champion_script] Vayne executed Tumble Empower
- 426.700s [enemy_buff] Vayne empowered next attack
- 427.156s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 427.200s [champion_script] Vayne executed Tumble Empower
- 427.200s [enemy_buff] Vayne empowered next attack
- 427.270s [attack_start] Vayne begins auto attack
- 427.700s [champion_script] Vayne executed Tumble Empower
- 427.700s [enemy_buff] Vayne empowered next attack
- 427.741s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 427.855s [attack_start] Vayne begins auto attack
- 428.200s [champion_script] Vayne executed Tumble Empower
- 428.200s [enemy_buff] Vayne empowered next attack
- 428.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 428.325s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 428.439s [attack_start] Vayne begins auto attack
- 428.700s [champion_script] Vayne executed Tumble Empower
- 428.700s [enemy_buff] Vayne empowered next attack
- 428.910s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 429.024s [attack_start] Vayne begins auto attack
- 429.200s [champion_script] Vayne executed Tumble Empower
- 429.200s [enemy_buff] Vayne empowered next attack
- 429.494s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 429.608s [attack_start] Vayne begins auto attack
- 429.700s [champion_script] Vayne executed Tumble Empower
- 429.700s [enemy_buff] Vayne empowered next attack
- 430.000s [state_snapshot] checkpoint 430.0s (captured_at 430.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=2258.2/4321.7 (52.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.30s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 25.44s
  Vayne:
    core: pos=(520.0, 150.0) hp=528.7/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=8.783 (interval 0.114s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack in-flight (0.08s to impact); Tumble Empower 0.20s
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 6/6; Guinsoo stacks: 8/8; Attacks landed: 21]
    buffs: none
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.15s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 13.36s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.38s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 43.56s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.53s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 49.60s
field:
  projectiles:
    - Vayne Auto Attack -> Vladimir (impact in 0.08s)
  projectile_block_zones: none
  ```
- 430.079s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 430.193s [attack_start] Vayne begins auto attack
- 430.200s [champion_script] Vayne executed Tumble Empower
- 430.200s [enemy_buff] Vayne empowered next attack
- 430.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 430.663s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 430.700s [champion_script] Vayne executed Tumble Empower
- 430.700s [enemy_buff] Vayne empowered next attack
- 430.777s [attack_start] Vayne begins auto attack
- 431.200s [champion_script] Vayne executed Tumble Empower
- 431.200s [enemy_buff] Vayne empowered next attack
- 431.248s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 431.362s [attack_start] Vayne begins auto attack
- 431.700s [champion_script] Vayne executed Tumble Empower
- 431.700s [enemy_buff] Vayne empowered next attack
- 431.832s [impact_nullified] Vayne auto attack on Vladimir was nullified by untargetable or stasis state
- 431.946s [attack_start] Vayne begins auto attack
- 432.200s [champion_script] Vayne executed Tumble Empower
- 432.200s [enemy_buff] Vayne empowered next attack
- 432.267s [enemy_death] Vayne died; respawn in 55.4s
- 432.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 434.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 435.000s [state_snapshot] checkpoint 435.0s (captured_at 435.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1937.8/4321.7 (44.8%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.15s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 20.44s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.13s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 52.65s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.11s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 8.36s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.24s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 38.56s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.42s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 44.60s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 436.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 438.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 440.000s [state_snapshot] checkpoint 440.0s (captured_at 440.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1658.0/4321.7 (38.4%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.37s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 15.44s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.07s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 47.65s
  Morgana:
    core: pos=(-650.0, 120.0) hp=0.0/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.07s; Dark Binding ready; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 3.36s
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.10s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 33.56s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.31s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 39.60s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 440.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 442.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 443.367s [enemy_respawn] Morgana respawned
- 443.367s [champion_script] Morgana executed Dark Binding
- 443.367s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 444.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 445.000s [state_snapshot] checkpoint 445.0s (captured_at 445.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1351.0/4321.7 (31.3%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 1.27s; Pool heal-over-time 1.27s; Untargetable x1 (1.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.22s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 10.44s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.01s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 42.65s
  Morgana:
    core: pos=(-650.0, 120.0) hp=3088.3/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.03s; Dark Binding 0.65s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 1.37s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.50s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 28.56s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.20s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 34.60s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 445.667s [champion_script] Morgana executed Dark Binding
- 445.667s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 446.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 447.967s [champion_script] Morgana executed Dark Binding
- 447.967s [impact_nullified] Morgana Dark Binding on Vladimir was nullified by untargetable or stasis state
- 448.267s [controlled_champion_cast] Vladimir cast Sanguine Pool (untargetable 2.00s)
- 450.000s [state_snapshot] checkpoint 450.0s (captured_at 450.000s)
  ```text
controlled_champion:
  identity: Vladimir
  core: pos=(0.0, 0.0) hp=1402.5/4321.7 (32.5%) armor=109.5 mr=54.7
  offense: ap=377.8 ah=6041.0
  loadout: items [Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff] | runes [Unsealed Spellbook, Hextech Flashtraption, Biscuit Delivery, Jack Of All Trades, Presence of Mind, Legend: Haste] | shards [attack_speed, health, tenacity]
  cooldowns: none
  abilities: Q:vladimir_transfusion ready; W:vladimir_sanguine_pool ready; E:vladimir_tides_of_blood ready; R:vladimir_hemoplague ready
  buffs: Pool untargetable 0.27s; Pool heal-over-time 0.27s; Untargetable x1 (0.27s)
enemies:
  Warwick:
    core: pos=(140.0, 0.0) hp=0.0/3501.0 armor=191.6 mr=70.9
    combat: ad=237.5 ap=0.0 as=2.681 (interval 0.373s) ah=310.0
    loadout: items [Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking]
    abilities: Auto Attack 0.07s; Infinite Duress ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Attacks landed: 0]
    buffs: Respawning in 5.44s
  Vayne:
    core: pos=(520.0, 150.0) hp=0.0/2641.5 armor=110.4 mr=54.7
    combat: ad=294.6 ap=30.0 as=6.274 (interval 0.159s) ah=300.0
    loadout: items [Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge] | runes [Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth]
    abilities: Auto Attack 0.11s; Tumble Empower ready
    runtime: cooldowns [none] | stacks [Lethal Tempo stacks: 0/6; Guinsoo stacks: 0/8; Attacks landed: 0]
    buffs: Respawning in 37.65s
  Morgana:
    core: pos=(-650.0, 120.0) hp=2384.1/3440.5 armor=154.8 mr=54.7
    combat: ad=122.5 ap=410.0 as=1.613 (interval 0.620s) ah=338.0
    loadout: items [Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo] | runes [Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter]
    abilities: Auto Attack 0.61s; Dark Binding 0.25s; Soul Shackles ready
    runtime: cooldowns [Luden's Echo: 0.97s (cooldown 3.00s)] | stacks [none]
    buffs: none
  Sona:
    core: pos=(-550.0, -180.0) hp=0.0/2413.5 armor=105.8 mr=54.7
    combat: ad=106.0 ap=530.0 as=1.851 (interval 0.540s) ah=328.0
    loadout: items [Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap] | runes [Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize]
    abilities: Auto Attack 0.37s; Crescendo ready
    runtime: cooldowns [Luden's Echo: ready (cooldown 3.00s)] | stacks [none]
    buffs: Respawning in 23.56s
  Dr. Mundo:
    core: pos=(180.0, -120.0) hp=0.0/6479.8 armor=192.5 mr=72.7
    combat: ad=148.5 ap=0.0 as=1.635 (interval 0.612s) ah=318.0
    loadout: items [Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra] | runes [Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight]
    abilities: Auto Attack 0.10s; Infected Bonesaw ready
    runtime: cooldowns [Grasp of the Undying: ready (cooldown 4.00s); Heartsteel Colossal Consumption: ready (cooldown 7.50s)] | stacks [none]
    buffs: Respawning in 29.60s
field:
  projectiles: none
  projectile_block_zones: none
  ```
- 450.267s [champion_script] Morgana executed Dark Binding
- 450.267s [damage_in] Morgana Dark Binding -> Vladimir | physical 0.0, magic 841.9, true 0.0, total 544.2
- 452.567s [champion_script] Morgana executed Dark Binding
- 452.567s [damage_in] Morgana Dark Binding -> Vladimir | physical 0.0, magic 998.8, true 0.0, total 645.6
- 454.867s [champion_script] Morgana executed Dark Binding
- 454.867s [damage_in] Morgana Dark Binding -> Vladimir | physical 0.0, magic 841.9, true 0.0, total 544.2
- 454.867s [controlled_champion_death] Vladimir died
