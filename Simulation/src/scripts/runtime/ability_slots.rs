use std::collections::HashMap;

use crate::defaults::champion_slot_bindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AbilitySlotKey {
    Q,
    W,
    E,
    R,
    D,
    F,
}

impl AbilitySlotKey {
    pub(crate) fn from_key(key: &str) -> Option<Self> {
        match key.trim().to_ascii_uppercase().as_str() {
            "Q" => Some(Self::Q),
            "W" => Some(Self::W),
            "E" => Some(Self::E),
            "R" => Some(Self::R),
            "D" => Some(Self::D),
            "F" => Some(Self::F),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ActorAbilityLoadout {
    slot_to_ability: HashMap<AbilitySlotKey, String>,
    ability_to_slot: HashMap<String, AbilitySlotKey>,
}

impl ActorAbilityLoadout {
    pub(crate) fn bind(&mut self, slot: AbilitySlotKey, ability_id: impl Into<String>) {
        let ability_id = ability_id.into();
        if let Some(previous) = self.slot_to_ability.insert(slot, ability_id.clone()) {
            self.ability_to_slot.remove(&previous);
        }
        if let Some(previous_slot) = self.ability_to_slot.insert(ability_id.clone(), slot)
            && previous_slot != slot
        {
            self.slot_to_ability.remove(&previous_slot);
        }
    }

    pub(crate) fn slot_for_ability(&self, ability_id: &str) -> Option<AbilitySlotKey> {
        self.ability_to_slot.get(ability_id).copied()
    }

    pub(crate) fn assign_ability_to_slot(
        &mut self,
        ability_id: impl Into<String>,
        slot: AbilitySlotKey,
    ) {
        let ability_id = ability_id.into();
        if let Some(previous_slot) = self.ability_to_slot.get(&ability_id).copied() {
            self.slot_to_ability.remove(&previous_slot);
        }
        if let Some(previous_ability) = self.slot_to_ability.get(&slot).cloned() {
            self.ability_to_slot.remove(&previous_ability);
        }
        self.slot_to_ability.insert(slot, ability_id.clone());
        self.ability_to_slot.insert(ability_id, slot);
    }
}

pub(crate) fn default_champion_ability_loadout(champion_name: &str) -> ActorAbilityLoadout {
    let bindings = champion_slot_bindings(champion_name);
    let mut loadout = ActorAbilityLoadout::default();
    for (slot_key, ability_id) in bindings {
        if let Some(slot) = AbilitySlotKey::from_key(&slot_key) {
            loadout.bind(slot, ability_id);
        }
    }
    loadout
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loadout_swap_and_assign_supports_remapping() {
        let mut loadout = ActorAbilityLoadout::default();
        loadout.bind(AbilitySlotKey::Q, "a");
        loadout.bind(AbilitySlotKey::W, "b");

        loadout.assign_ability_to_slot("a", AbilitySlotKey::W);
        loadout.assign_ability_to_slot("b", AbilitySlotKey::Q);
        assert_eq!(loadout.slot_for_ability("a"), Some(AbilitySlotKey::W));
        assert_eq!(loadout.slot_for_ability("b"), Some(AbilitySlotKey::Q));

        loadout.assign_ability_to_slot("stolen_ultimate", AbilitySlotKey::R);
        assert_eq!(
            loadout.slot_for_ability("stolen_ultimate"),
            Some(AbilitySlotKey::R)
        );
    }

    #[test]
    fn default_vladimir_bindings_load_from_data() {
        let loadout = default_champion_ability_loadout("Vladimir");
        assert_eq!(
            loadout.slot_for_ability("vladimir_transfusion"),
            Some(AbilitySlotKey::Q)
        );
        assert_eq!(
            loadout.slot_for_ability("vladimir_sanguine_pool"),
            Some(AbilitySlotKey::W)
        );
    }
}
