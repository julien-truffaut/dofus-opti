use crate::model::{EffectsStructOpt, Gear, TranslatedName};
use dofus_opti_core::model::Gear as CoreGear;
use dofus_opti_core::model::{CharacteristicRange, CharacteristicType};

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

pub fn parse_effects(characteristics: Vec<CharacteristicRange>) -> EffectsStructOpt {
    let mut effects = EffectsStructOpt::empty();

    characteristics.iter().for_each(|characteristic_range| match characteristic_range.kind {
        CharacteristicType::AbilityPoint => effects.ability_point = Some(characteristic_range.max),
        CharacteristicType::AbilityPointParry => {
            effects.ability_point_parry = Some(characteristic_range.max)
        }
        CharacteristicType::AbilityPointReduction => {
            effects.ability_point_reduction = Some(characteristic_range.max)
        }
        CharacteristicType::Agility => effects.agility = Some(characteristic_range.max),
        CharacteristicType::AirDamage => effects.air_damage = Some(characteristic_range.max),
        CharacteristicType::AirResistance => {
            effects.air_resistance = Some(characteristic_range.max)
        }
        CharacteristicType::AirResistancePercent => {
            effects.air_resistance_percent = Some(characteristic_range.max)
        }
        CharacteristicType::Chance => effects.chance = Some(characteristic_range.max),
        CharacteristicType::Critical => effects.critical = Some(characteristic_range.max),
        CharacteristicType::CriticalDamage => {
            effects.critical_damage = Some(characteristic_range.max)
        }
        CharacteristicType::CriticalResistance => {
            effects.critical_resistance = Some(characteristic_range.max)
        }
        CharacteristicType::Damage => effects.damage = Some(characteristic_range.max),
        CharacteristicType::Dodge => effects.dodge = Some(characteristic_range.max),
        CharacteristicType::EarthDamage => effects.earth_damage = Some(characteristic_range.max),
        CharacteristicType::EarthResistance => {
            effects.earth_resistance = Some(characteristic_range.max)
        }
        CharacteristicType::EarthResistancePercent => {
            effects.earth_resistance_percent = Some(characteristic_range.max)
        }
        CharacteristicType::FireDamage => effects.fire_damage = Some(characteristic_range.max),
        CharacteristicType::FireResistance => {
            effects.fire_resistance = Some(characteristic_range.max)
        }
        CharacteristicType::FireResistancePercent => {
            effects.fire_resistance_percent = Some(characteristic_range.max)
        }
        CharacteristicType::Heals => effects.heals = Some(characteristic_range.max),
        CharacteristicType::Initiative => effects.initiative = Some(characteristic_range.max),
        CharacteristicType::Intelligence => effects.intelligence = Some(characteristic_range.max),
        CharacteristicType::Lock => effects.lock = Some(characteristic_range.max),
        CharacteristicType::MeleeDamage => effects.melee_damage = Some(characteristic_range.max),
        CharacteristicType::MeleeResistance => {
            effects.melee_resistance = Some(characteristic_range.max)
        }
        CharacteristicType::MovementPoint => {
            effects.movement_point = Some(characteristic_range.max)
        }
        CharacteristicType::MovementPointParry => {
            effects.movement_point_parry = Some(characteristic_range.max)
        }
        CharacteristicType::MovementPointReduction => {
            effects.movement_point_reduction = Some(characteristic_range.max)
        }
        CharacteristicType::NeutralDamage => {
            effects.neutral_damage = Some(characteristic_range.max)
        }
        CharacteristicType::NeutralResistance => {
            effects.neutral_resistance = Some(characteristic_range.max)
        }
        CharacteristicType::NeutralResistancePercent => {
            effects.neutral_resistance_percent = Some(characteristic_range.max)
        }
        CharacteristicType::Pods => effects.pods = Some(characteristic_range.max),
        CharacteristicType::Power => effects.power = Some(characteristic_range.max),
        CharacteristicType::Prospecting => effects.prospecting = Some(characteristic_range.max),
        CharacteristicType::PushBackDamage => {
            effects.push_back_damage = Some(characteristic_range.max)
        }
        CharacteristicType::PushBackResistance => {
            effects.push_back_resistance = Some(characteristic_range.max)
        }
        CharacteristicType::Range => effects.range = Some(characteristic_range.max),
        CharacteristicType::RangeDamage => effects.range_damage = Some(characteristic_range.max),
        CharacteristicType::RangeResistance => {
            effects.range_resistance = Some(characteristic_range.max)
        }
        CharacteristicType::ReflectedDamage => {
            effects.reflected_damage = Some(characteristic_range.max)
        }
        CharacteristicType::SpellDamage => effects.spell_damage = Some(characteristic_range.max),
        CharacteristicType::Strength => effects.strength = Some(characteristic_range.max),
        CharacteristicType::Summon => effects.summon = Some(characteristic_range.max),
        CharacteristicType::TrapDamage => effects.trap_damage = Some(characteristic_range.max),
        CharacteristicType::TrapPower => effects.trap_power = Some(characteristic_range.max),
        CharacteristicType::Vitality => effects.vitality = Some(characteristic_range.max),
        CharacteristicType::WaterDamage => effects.water_damage = Some(characteristic_range.max),
        CharacteristicType::WaterResistance => {
            effects.water_resistance = Some(characteristic_range.max)
        }
        CharacteristicType::WaterResistancePercent => {
            effects.water_resistance_percent = Some(characteristic_range.max)
        }
        CharacteristicType::WeaponDamage => effects.weapon_damage = Some(characteristic_range.max),
        CharacteristicType::Wisdom => effects.wisdom = Some(characteristic_range.max),
    });

    effects
}
