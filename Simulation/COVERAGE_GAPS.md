# Coverage Gaps Tracker (2026-02-19)

This file tracks known fidelity and coverage gaps against game-accurate behavior.

## Coverage Snapshot
- Champion data files in repo: `6` (`DrMundo`, `Morgana`, `Sona`, `Vayne`, `Vladimir`, `Warwick`)
- Controlled champion script coverage: `1/6` (`Vladimir` only)
- Enemy scripted-event champion coverage: `5/6` (`Doctor Mundo`, `Morgana`, `Sona`, `Vayne`, `Warwick`; `Vladimir` missing as enemy script actor)
- Legal URF legendary item pool used by search: `114`
- Legal pool items with passive/active/structured effect payload: `112`
- Legal pool item effect scripts modeled in runtime: `9`
- Legal pool item effect payloads unmodeled in runtime: `103`
- Rune count in `RunesReforged`: `61`
- Dynamic combat-time rune effects modeled: `16`
- Deterministic static rune effects modeled: `6`
- Runes with no modeled deterministic effect and no modeled combat-time effect: `39`

## Champion And Ability Gaps
- Controlled champion generic script support:
  - Only `Vladimir` has a controlled champion script implementation.
  - Missing controlled champion scripts: `DrMundo`, `Morgana`, `Sona`, `Vayne`, `Warwick`.
- Enemy scripted ability depth:
  - `Warwick`: only passive profile + `Infinite Duress` script event.
  - `Vayne`: only `Tumble` and `Silver Bolts`-style periodic true-hit behavior.
  - `Morgana`: only `Dark Binding` and `Soul Shackles` (including detonation follow-up).
  - `Sona`: only `Crescendo`.
  - `Doctor Mundo`: only `Infected Bonesaw`.
  - `Vladimir`: no enemy-scripted spell events.
- Vladimir fidelity is first-pass:
  - Scripted `Q/E/R/W` loop exists, but not every live conditional nuance is modeled.
- Slot/remap architecture:
  - Foundations exist, but full actor-wide slot-agnostic remap/steal behavior is not complete.

## Item Effect Gaps
Modeled runtime item effect items in legal URF pool (`9`):
- `Blade of the Ruined King`
- `Guardian Angel`
- `Guinsoo's Rageblade`
- `Heartsteel`
- `Kraken Slayer`
- `Liandry's Torment`
- `Luden's Echo`
- `Protoplasm Harness`
- `Zhonya's Hourglass`

Legal URF pool items with effect payload but currently unmodeled runtime effects (`103`):
- `Abyssal Mask`
- `Actualizer`
- `Ardent Censer`
- `Axiom Arc`
- `Bandlepipes`
- `Banshee's Veil`
- `Bastionbreaker`
- `Black Cleaver`
- `Blackfire Torch`
- `Bloodletter's Curse`
- `Bloodsong`
- `Bloodthirster`
- `Celestial Opposition`
- `Chempunk Chainsword`
- `Cosmic Drive`
- `Cryptbloom`
- `Dawncore`
- `Dead Man's Plate`
- `Death's Dance`
- `Dream Maker`
- `Dusk and Dawn`
- `Echoes of Helia`
- `Eclipse`
- `Edge of Night`
- `Endless Hunger`
- `Essence Reaver`
- `Experimental Hexplate`
- `Fiendhunter Bolts`
- `Force of Nature`
- `Frozen Heart`
- `Hexoptics C44`
- `Hextech Gunblade`
- `Hextech Rocketbelt`
- `Hollow Radiance`
- `Horizon Focus`
- `Hubris`
- `Hullbreaker`
- `Iceborn Gauntlet`
- `Immortal Shieldbow`
- `Imperial Mandate`
- `Jak'Sho, The Protean`
- `Kaenic Rookern`
- `Knight's Vow`
- `Lich Bane`
- `Locket of the Iron Solari`
- `Lord Dominik's Regards`
- `Malignance`
- `Maw of Malmortius`
- `Mejai's Soulstealer`
- `Mercurial Scimitar`
- `Mikael's Blessing`
- `Moonstone Renewer`
- `Morellonomicon`
- `Mortal Reminder`
- `Muramana`
- `Nashor's Tooth`
- `Navori Flickerblade`
- `Opportunity`
- `Overlord's Bloodmail`
- `Phantom Dancer`
- `Profane Hydra`
- `Rabadon's Deathcap`
- `Randuin's Omen`
- `Rapid Firecannon`
- `Ravenous Hydra`
- `Redemption`
- `Riftmaker`
- `Rod of Ages`
- `Runaan's Hurricane`
- `Rylai's Crystal Scepter`
- `Seraph's Embrace`
- `Serpent's Fang`
- `Serylda's Grudge`
- `Shadowflame`
- `Shurelya's Battlesong`
- `Solstice Sleigh`
- `Spear of Shojin`
- `Spirit Visage`
- `Staff of Flowing Water`
- `Statikk Shiv`
- `Sterak's Gage`
- `Stormsurge`
- `Stridebreaker`
- `Sundered Sky`
- `Sunfire Aegis`
- `Terminus`
- `The Collector`
- `Thornmail`
- `Titanic Hydra`
- `Trailblazer`
- `Trinity Force`
- `Umbral Glaive`
- `Unending Despair`
- `Voltaic Cyclosword`
- `Warmog's Armor`
- `Whispering Circlet`
- `Winter's Approach`
- `Wit's End`
- `Youmuu's Ghostblade`
- `Yun Tal Wildarrows`
- `Zaz'Zak's Realmspike`
- `Zeke's Convergence`
- `Zephyr`

Known deterministic stat-model gaps:
- Structured item passives are not globally applied during deterministic stat resolution; only selected paths are modeled.
- Example impact class: ratio/stat-scaling passives (for example AP amplification) are not generally represented unless explicitly scripted.

## Rune And Mastery Gaps
Dynamic combat-time runes modeled (`16`):
- `Aftershock`
- `Arcane Comet`
- `Conqueror`
- `Dark Harvest`
- `Electrocute`
- `First Strike`
- `Fleet Footwork`
- `Gathering Storm`
- `Grasp of the Undying`
- `Hail of Blades`
- `Lethal Tempo`
- `Phase Rush`
- `Press the Attack`
- `Second Wind`
- `Summon Aery`
- `Triumph`

Deterministic static runes modeled (`6`):
- `Celerity`
- `Jack Of All Trades`
- `Legend: Alacrity`
- `Legend: Haste`
- `Magical Footwear`
- `Nimbus Cloak`

Runes currently unmodeled (`39`):
- `Absolute Focus`
- `Absorb Life`
- `Approach Velocity`
- `Axiom Arcanist`
- `Biscuit Delivery`
- `Bone Plating`
- `Cash Back`
- `Cheap Shot`
- `Conditioning`
- `Cosmic Insight`
- `Coup de Grace`
- `Cut Down`
- `Deep Ward`
- `Demolish`
- `Font of Life`
- `Glacial Augment`
- `Grisly Mementos`
- `Guardian`
- `Hextech Flashtraption`
- `Last Stand`
- `Legend: Bloodline`
- `Manaflow Band`
- `Overgrowth`
- `Presence of Mind`
- `Relentless Hunter`
- `Revitalize`
- `Scorch`
- `Shield Bash`
- `Sixth Sense`
- `Sudden Impact`
- `Taste of Blood`
- `Time Warp Tonic`
- `Transcendence`
- `Treasure Hunter`
- `Triple Tonic`
- `Ultimate Hunter`
- `Unflinching`
- `Unsealed Spellbook`
- `Waterwalking`

Mastery system coverage:
- Legacy `Season2016` masteries are intentionally retired and unsupported by simulator runtime.

## Stat And Shard Gaps
- Tenacity shard stat now parses into deterministic stats, but runtime crowd-control duration reduction from tenacity is not yet modeled.
- `crit_chance_percent` is loaded into stats but has no combat-time critical-strike behavior model.

## Engine And Physics Fidelity Gaps
- 2D-only combat geometry (`x`, `y`); no validated `z` interaction model.
- Movement/pathing model is deterministic and simplified relative to live collision/turn/path behavior.
- Projectile blocking/collision remains simplified relative to full live projectile rules.
- Effect scheduling and timing are discrete-tick approximations, not full frame-accurate engine emulation.

## Combat System Gaps
- Resource systems (mana/energy/other champion resources) are not modeled as first-class runtime constraints.
- Many champion-specific conditional states/interactions are not yet represented.
- Item passives/actives and rune interactions outside modeled set are absent from runtime combat outcomes.

## AI And Scenario Fidelity Gaps
- Enemy and controlled champion tactical behavior is policy-driven but still narrow versus full gameplay decision space.
- Scenario scope is teamfight-centric; lane-state/objective-state/map-state dynamics are not modeled.

## Tracking Notes
- Search now has explicit quality gates for both unmodeled runes and unmodeled item effects.
- Reports and diagnostics expose gate policy and rejected/penalized candidate counters.
- This file should be updated whenever:
  - modeled effect sets change,
  - new champion scripts land,
  - item/rune/shard support is added,
  - or engine fidelity assumptions materially change.
