use crate::model::*;
use dofus_opti_core::model::{GearType, Id};

use proptest::prelude::*;

pub fn gen_gear_catalog(size: std::ops::Range<usize>) -> impl Strategy<Value = GearCatalog> {
    proptest::collection::vec(gen_gear(), size).prop_map(GearCatalog::new)
}

pub fn gen_gear() -> impl Strategy<Value = Gear> {
    (gen_id(), "[a-z]{6,20}", gen_gear_type(), gen_gear_level(), gen_effects()).prop_map(
        |(id, name, gear_type, level, effects)| Gear {
            id: id,
            name: TranslatedName {
                en: name.clone(),
                fr: name,
            },
            gear_type: gear_type,
            level: level,
            has_set: false,
            effects: effects,
        },
    )
}

pub fn gen_id() -> impl Strategy<Value = Id> {
    "[a-z0-9]{8,12}".prop_map(|id| Id(id))
}

pub fn gen_gear_type() -> impl Strategy<Value = GearType> {
    prop_oneof![
        Just(GearType::Amulet),
        Just(GearType::Axe),
        Just(GearType::Belt),
        Just(GearType::Boots),
        Just(GearType::Bow),
        Just(GearType::Cloak),
        Just(GearType::Amulet),
        Just(GearType::Dagger),
        Just(GearType::Hammer),
        Just(GearType::Hat),
        Just(GearType::Lance),
        Just(GearType::Ring),
        Just(GearType::Scythe),
        Just(GearType::Shield),
        Just(GearType::Shovel),
        Just(GearType::Staff),
        Just(GearType::Sword),
        Just(GearType::Wand),
    ]
}

pub fn gen_gear_level() -> impl Strategy<Value = u32> {
    1u32..200
}

pub fn gen_effects() -> impl Strategy<Value = Effects> {
    let updates = proptest::collection::vec(
        (proptest::sample::select(ALL_CHARACTERISTIC_TYPES), -100..400),
        0..20,
    );

    updates.prop_map(|ops| {
        let mut effects = Effects::empty();
        for (c, v) in ops {
            effects.set(&c, v);
        }
        effects
    })
}
