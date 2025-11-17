use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Gear {
    pub id: Id,
    pub name: TranslatedName,
    pub gear_type: GearType,
    pub set: Option<Id>,
    pub level: u32,
    pub characteristics: Vec<CharacteristicRange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum GearType {
    Amulet,
    Axe,
    Belt,
    Boots,
    Bow,
    Cloak,
    Dagger,
    Hat,
    Lance,
    Hammer,
    Ring,
    Scythe,
    Shield,
    Shovel,
    Staff,
    Sword,
    Wand,
}

pub static ALL_GEAR_TYPES: &[GearType] = &[
    GearType::Amulet,
    GearType::Axe,
    GearType::Belt,
    GearType::Boots,
    GearType::Bow,
    GearType::Cloak,
    GearType::Dagger,
    GearType::Hammer,
    GearType::Hat,
    GearType::Lance,
    GearType::Ring,
    GearType::Scythe,
    GearType::Shield,
    GearType::Shovel,
    GearType::Staff,
    GearType::Sword,
    GearType::Wand,
];

impl fmt::Display for GearType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GearType::Amulet => write!(f, "Amulet"),
            GearType::Axe => write!(f, "Axe"),
            GearType::Belt => write!(f, "Belt"),
            GearType::Boots => write!(f, "Boots"),
            GearType::Bow => write!(f, "Bow"),
            GearType::Cloak => write!(f, "Cloak"),
            GearType::Dagger => write!(f, "Dagger"),
            GearType::Hat => write!(f, "Hat"),
            GearType::Hammer => write!(f, "Hammer"),
            GearType::Lance => write!(f, "Lance"),
            GearType::Ring => write!(f, "Ring"),
            GearType::Scythe => write!(f, "Scythe"),
            GearType::Shield => write!(f, "Shield"),
            GearType::Shovel => write!(f, "Shovel"),
            GearType::Staff => write!(f, "Staff"),
            GearType::Sword => write!(f, "Sword"),
            GearType::Wand => write!(f, "Wand"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CharacteristicRange {
    pub kind: CharacteristicType,
    pub min: i32,
    pub max: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CharacteristicType {
    AbilityPoint,
    AbilityPointParry,
    AbilityPointReduction,
    Agility,
    AirDamage,
    AirResistance,
    AirResistancePercent,
    Chance,
    Critical,
    CriticalDamage,
    CriticalResistance,
    Damage,
    Dodge,
    EarthDamage,
    EarthResistance,
    EarthResistancePercent,
    FireDamage,
    FireResistance,
    FireResistancePercent,
    Heals,
    Initiative,
    Intelligence,
    Lock,
    MeleeDamage,
    MeleeResistance,
    MovementPoint,
    MovementPointParry,
    MovementPointReduction,
    NeutralDamage,
    NeutralResistance,
    NeutralResistancePercent,
    Pods,
    Power,
    Prospecting,
    PushBackDamage,
    PushBackResistance,
    Range,
    RangeDamage,
    RangeResistance,
    ReflectedDamage,
    SpellDamage,
    Strength,
    Summon,
    TrapDamage,
    TrapPower,
    Vitality,
    WaterDamage,
    WaterResistance,
    WaterResistancePercent,
    WeaponDamage,
    Wisdom,
}

pub static ALL_CHARACTERISTIC_TYPES: &[CharacteristicType] = &[
    CharacteristicType::AbilityPoint,
    CharacteristicType::AbilityPointParry,
    CharacteristicType::AbilityPointReduction,
    CharacteristicType::Agility,
    CharacteristicType::AirDamage,
    CharacteristicType::AirResistance,
    CharacteristicType::AirResistancePercent,
    CharacteristicType::Chance,
    CharacteristicType::Critical,
    CharacteristicType::CriticalDamage,
    CharacteristicType::CriticalResistance,
    CharacteristicType::Damage,
    CharacteristicType::Dodge,
    CharacteristicType::EarthDamage,
    CharacteristicType::EarthResistance,
    CharacteristicType::EarthResistancePercent,
    CharacteristicType::FireDamage,
    CharacteristicType::FireResistance,
    CharacteristicType::FireResistancePercent,
    CharacteristicType::Heals,
    CharacteristicType::Initiative,
    CharacteristicType::Intelligence,
    CharacteristicType::Lock,
    CharacteristicType::MeleeDamage,
    CharacteristicType::MeleeResistance,
    CharacteristicType::MovementPoint,
    CharacteristicType::MovementPointParry,
    CharacteristicType::MovementPointReduction,
    CharacteristicType::NeutralDamage,
    CharacteristicType::NeutralResistance,
    CharacteristicType::NeutralResistancePercent,
    CharacteristicType::Pods,
    CharacteristicType::Power,
    CharacteristicType::Prospecting,
    CharacteristicType::PushBackDamage,
    CharacteristicType::PushBackResistance,
    CharacteristicType::Range,
    CharacteristicType::RangeDamage,
    CharacteristicType::RangeResistance,
    CharacteristicType::ReflectedDamage,
    CharacteristicType::SpellDamage,
    CharacteristicType::Strength,
    CharacteristicType::Summon,
    CharacteristicType::TrapDamage,
    CharacteristicType::TrapPower,
    CharacteristicType::Vitality,
    CharacteristicType::WaterDamage,
    CharacteristicType::WaterResistance,
    CharacteristicType::WaterResistancePercent,
    CharacteristicType::WeaponDamage,
    CharacteristicType::Wisdom,
];

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Id(pub String);

impl From<&str> for Id {
    fn from(str: &str) -> Self {
        Id(String::from(str))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TranslatedName {
    pub en: String,
    pub fr: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ItemSet {
    pub id: Id,
    pub name: TranslatedName,
    pub effects: Vec<Vec<Effect>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Effect {
    pub kind: CharacteristicType,
    pub value: i32,
}
