
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
