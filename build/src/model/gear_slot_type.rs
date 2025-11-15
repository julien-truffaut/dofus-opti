use std::fmt;

use crate::model::GearSlot;
use dofus_opti_core::model::GearType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GearSlotType {
    Amulet,
    Belt,
    Boots,
    Cloak,
    Hat,
    Ring,
    Shield,
    Weapon,
}

pub static ALL_GEAR_SLOT_TYPES: &[GearSlotType] = &[
    GearSlotType::Amulet,
    GearSlotType::Belt,
    GearSlotType::Boots,
    GearSlotType::Cloak,
    GearSlotType::Hat,
    GearSlotType::Ring,
    GearSlotType::Shield,
    GearSlotType::Weapon,
];

impl From<&GearType> for GearSlotType {
    fn from(value: &GearType) -> Self {
        match value {
            GearType::Amulet => GearSlotType::Amulet,
            GearType::Axe => GearSlotType::Weapon,
            GearType::Belt => GearSlotType::Belt,
            GearType::Boots => GearSlotType::Boots,
            GearType::Bow => GearSlotType::Weapon,
            GearType::Cloak => GearSlotType::Cloak,
            GearType::Dagger => GearSlotType::Weapon,
            GearType::Hat => GearSlotType::Hat,
            GearType::Lance => GearSlotType::Weapon,
            GearType::Hammer => GearSlotType::Weapon,
            GearType::Ring => GearSlotType::Ring,
            GearType::Scythe => GearSlotType::Weapon,
            GearType::Shield => GearSlotType::Shield,
            GearType::Shovel => GearSlotType::Weapon,
            GearType::Staff => GearSlotType::Weapon,
            GearType::Sword => GearSlotType::Weapon,
            GearType::Wand => GearSlotType::Weapon,
        }
    }
}

impl From<&GearSlot> for GearSlotType {
    fn from(value: &GearSlot) -> Self {
        match value {
            GearSlot::Amulet => GearSlotType::Amulet,
            GearSlot::Belt => GearSlotType::Belt,
            GearSlot::Boots => GearSlotType::Boots,
            GearSlot::Cloak => GearSlotType::Cloak,
            GearSlot::Hat => GearSlotType::Hat,
            GearSlot::Ring1 => GearSlotType::Ring,
            GearSlot::Ring2 => GearSlotType::Ring,
            GearSlot::Shield => GearSlotType::Shield,
            GearSlot::Weapon => GearSlotType::Weapon,
        }
    }
}

impl fmt::Display for GearSlotType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GearSlotType::Amulet => write!(f, "Amulet"),
            GearSlotType::Belt => write!(f, "Belt"),
            GearSlotType::Boots => write!(f, "Boots"),
            GearSlotType::Cloak => write!(f, "Cloak"),
            GearSlotType::Hat => write!(f, "Hat"),
            GearSlotType::Ring => write!(f, "Ring"),
            GearSlotType::Shield => write!(f, "Shield"),
            GearSlotType::Weapon => write!(f, "Weapon"),
        }
    }
}
