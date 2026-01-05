use crate::model::{CharacteristicType, Effects, Gear, TranslatedName};
use dofus_opti_core::model::Gear as CoreGear;
use dofus_opti_core::model::CharacteristicType as CoreCharacteristicType;
use dofus_opti_core::model::CharacteristicRange;

pub fn parse_gear(gear: CoreGear) -> Gear {
    Gear {
        id: gear.id,
        name: TranslatedName {
            en: gear.name.en,
            fr: gear.name.fr,
        },
        gear_type: gear.gear_type,
        has_set: gear.set.is_some(),
        level: gear.level,
        effects: parse_effects(gear.characteristics),
    }
}

pub fn parse_effects(characteristics: Vec<CharacteristicRange>) -> Effects {
    let mut effects = Effects::empty();

    characteristics.iter().for_each(|characteristic_range| 
        effects.set(&parse_characteristic_type(&characteristic_range.kind), characteristic_range.max));

    effects
}

pub fn parse_characteristic_type(characteristic_type: &CoreCharacteristicType) -> CharacteristicType {
    match characteristic_type {
        CoreCharacteristicType::AbilityPoint => CharacteristicType::AbilityPoint,
        CoreCharacteristicType::AbilityPointParry => CharacteristicType::AbilityPointParry,
        CoreCharacteristicType::AbilityPointReduction => CharacteristicType::AbilityPointReduction,
        CoreCharacteristicType::Agility => CharacteristicType::Agility,
        CoreCharacteristicType::AirDamage => CharacteristicType::AirDamage,
        CoreCharacteristicType::AirResistance => CharacteristicType::AirResistance,
        CoreCharacteristicType::AirResistancePercent => CharacteristicType::AirResistancePercent,
        CoreCharacteristicType::Chance => CharacteristicType::Chance,
        CoreCharacteristicType::Critical => CharacteristicType::Critical,
        CoreCharacteristicType::CriticalDamage => CharacteristicType::CriticalDamage,
        CoreCharacteristicType::CriticalResistance => CharacteristicType::CriticalResistance,
        CoreCharacteristicType::Damage => CharacteristicType::Damage,
        CoreCharacteristicType::Dodge => CharacteristicType::Dodge,
        CoreCharacteristicType::EarthDamage => CharacteristicType::EarthDamage,
        CoreCharacteristicType::EarthResistance => CharacteristicType::EarthResistance,
        CoreCharacteristicType::EarthResistancePercent => CharacteristicType::EarthResistancePercent,
        CoreCharacteristicType::FireDamage => CharacteristicType::FireDamage,
        CoreCharacteristicType::FireResistance => CharacteristicType::FireResistance,
        CoreCharacteristicType::FireResistancePercent => CharacteristicType::FireResistancePercent,
        CoreCharacteristicType::Heals => CharacteristicType::Heals,
        CoreCharacteristicType::Initiative => CharacteristicType::Initiative,
        CoreCharacteristicType::Intelligence => CharacteristicType::Intelligence,
        CoreCharacteristicType::Lock => CharacteristicType::Lock,
        CoreCharacteristicType::MeleeDamage => CharacteristicType::MeleeDamage,
        CoreCharacteristicType::MeleeResistance => CharacteristicType::MeleeResistance,
        CoreCharacteristicType::MovementPoint => CharacteristicType::MovementPoint,
        CoreCharacteristicType::MovementPointParry => CharacteristicType::MovementPointParry,
        CoreCharacteristicType::MovementPointReduction => CharacteristicType::MovementPointReduction,
        CoreCharacteristicType::NeutralDamage => CharacteristicType::NeutralDamage,
        CoreCharacteristicType::NeutralResistance => CharacteristicType::NeutralResistance,
        CoreCharacteristicType::NeutralResistancePercent => CharacteristicType::NeutralResistancePercent,
        CoreCharacteristicType::Pods => CharacteristicType::Pods,
        CoreCharacteristicType::Power => CharacteristicType::Power,
        CoreCharacteristicType::Prospecting => CharacteristicType::Prospecting,
        CoreCharacteristicType::PushBackDamage => CharacteristicType::PushBackDamage,
        CoreCharacteristicType::PushBackResistance => CharacteristicType::PushBackResistance,
        CoreCharacteristicType::Range => CharacteristicType::Range,
        CoreCharacteristicType::RangeDamage => CharacteristicType::RangeDamage,
        CoreCharacteristicType::RangeResistance => CharacteristicType::RangeResistance,
        CoreCharacteristicType::ReflectedDamage => CharacteristicType::ReflectedDamage,
        CoreCharacteristicType::SpellDamage => CharacteristicType::SpellDamage,
        CoreCharacteristicType::Strength => CharacteristicType::Strength,
        CoreCharacteristicType::Summon => CharacteristicType::Summon,
        CoreCharacteristicType::TrapDamage => CharacteristicType::TrapDamage,
        CoreCharacteristicType::TrapPower => CharacteristicType::TrapPower,
        CoreCharacteristicType::Vitality => CharacteristicType::Vitality,
        CoreCharacteristicType::WaterDamage => CharacteristicType::WaterDamage,
        CoreCharacteristicType::WaterResistance => CharacteristicType::WaterResistance,
        CoreCharacteristicType::WaterResistancePercent => CharacteristicType::WaterResistancePercent,
        CoreCharacteristicType::WeaponDamage => CharacteristicType::WeaponDamage,
        CoreCharacteristicType::Wisdom => CharacteristicType::Wisdom,
    }
}