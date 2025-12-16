use crate::model::{CharacteristicType, Language, TranslatedName};

use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq)]
pub struct EffectsStructOpt {
    pub ability_point: Option<i32>,
    pub ability_point_parry: Option<i32>,
    pub ability_point_reduction: Option<i32>,
    pub agility: Option<i32>,
    pub air_damage: Option<i32>,
    pub air_resistance: Option<i32>,
    pub air_resistance_percent: Option<i32>,
    pub chance: Option<i32>,
    pub critical: Option<i32>,
    pub critical_damage: Option<i32>,
    pub critical_resistance: Option<i32>,
    pub damage: Option<i32>,
    pub dodge: Option<i32>,
    pub earth_damage: Option<i32>,
    pub earth_resistance: Option<i32>,
    pub earth_resistance_percent: Option<i32>,
    pub fire_damage: Option<i32>,
    pub fire_resistance: Option<i32>,
    pub fire_resistance_percent: Option<i32>,
    pub heals: Option<i32>,
    pub initiative: Option<i32>,
    pub intelligence: Option<i32>,
    pub lock: Option<i32>,
    pub melee_damage: Option<i32>,
    pub melee_resistance: Option<i32>,
    pub movement_point: Option<i32>,
    pub movement_point_parry: Option<i32>,
    pub movement_point_reduction: Option<i32>,
    pub neutral_damage: Option<i32>,
    pub neutral_resistance: Option<i32>,
    pub neutral_resistance_percent: Option<i32>,
    pub pods: Option<i32>,
    pub power: Option<i32>,
    pub prospecting: Option<i32>,
    pub push_back_damage: Option<i32>,
    pub push_back_resistance: Option<i32>,
    pub range: Option<i32>,
    pub range_damage: Option<i32>,
    pub range_resistance: Option<i32>,
    pub reflected_damage: Option<i32>,
    pub spell_damage: Option<i32>,
    pub strength: Option<i32>,
    pub summon: Option<i32>,
    pub trap_damage: Option<i32>,
    pub trap_power: Option<i32>,
    pub vitality: Option<i32>,
    pub water_damage: Option<i32>,
    pub water_resistance: Option<i32>,
    pub water_resistance_percent: Option<i32>,
    pub weapon_damage: Option<i32>,
    pub wisdom: Option<i32>,
}

impl EffectsStructOpt {
    pub fn empty() -> EffectsStructOpt {
        EffectsStructOpt {
            ability_point: None,
            ability_point_parry: None,
            ability_point_reduction: None,
            agility: None,
            air_damage: None,
            air_resistance: None,
            air_resistance_percent: None,
            chance: None,
            critical: None,
            critical_damage: None,
            critical_resistance: None,
            damage: None,
            dodge: None,
            earth_damage: None,
            earth_resistance: None,
            earth_resistance_percent: None,
            fire_damage: None,
            fire_resistance: None,
            fire_resistance_percent: None,
            heals: None,
            initiative: None,
            intelligence: None,
            lock: None,
            melee_damage: None,
            melee_resistance: None,
            movement_point: None,
            movement_point_parry: None,
            movement_point_reduction: None,
            neutral_damage: None,
            neutral_resistance: None,
            neutral_resistance_percent: None,
            pods: None,
            power: None,
            prospecting: None,
            push_back_damage: None,
            push_back_resistance: None,
            range: None,
            range_damage: None,
            range_resistance: None,
            reflected_damage: None,
            spell_damage: None,
            strength: None,
            summon: None,
            trap_damage: None,
            trap_power: None,
            vitality: None,
            water_damage: None,
            water_resistance: None,
            water_resistance_percent: None,
            weapon_damage: None,
            wisdom: None,
        }
    }

    pub fn add(&mut self, other: &EffectsStructOpt) {
        *self += other;
    }

    pub fn get(&self, characteristic_type: &CharacteristicType) -> i32 {
        match characteristic_type {
            CharacteristicType::AbilityPoint => self.ability_point.unwrap_or(0),
            CharacteristicType::AbilityPointParry => self.ability_point_parry.unwrap_or(0),
            CharacteristicType::AbilityPointReduction => self.ability_point_reduction.unwrap_or(0),
            CharacteristicType::Agility => self.agility.unwrap_or(0),
            CharacteristicType::AirDamage => self.air_damage.unwrap_or(0),
            CharacteristicType::AirResistance => self.air_resistance.unwrap_or(0),
            CharacteristicType::AirResistancePercent => self.air_resistance_percent.unwrap_or(0),
            CharacteristicType::Chance => self.chance.unwrap_or(0),
            CharacteristicType::Critical => self.critical.unwrap_or(0),
            CharacteristicType::CriticalDamage => self.critical_damage.unwrap_or(0),
            CharacteristicType::CriticalResistance => self.critical_resistance.unwrap_or(0),
            CharacteristicType::Damage => self.damage.unwrap_or(0),
            CharacteristicType::Dodge => self.dodge.unwrap_or(0),
            CharacteristicType::EarthDamage => self.earth_damage.unwrap_or(0),
            CharacteristicType::EarthResistance => self.earth_resistance.unwrap_or(0),
            CharacteristicType::EarthResistancePercent => {
                self.earth_resistance_percent.unwrap_or(0)
            }
            CharacteristicType::FireDamage => self.fire_damage.unwrap_or(0),
            CharacteristicType::FireResistance => self.fire_resistance.unwrap_or(0),
            CharacteristicType::FireResistancePercent => self.fire_resistance_percent.unwrap_or(0),
            CharacteristicType::Heals => self.heals.unwrap_or(0),
            CharacteristicType::Initiative => self.initiative.unwrap_or(0),
            CharacteristicType::Intelligence => self.intelligence.unwrap_or(0),
            CharacteristicType::Lock => self.lock.unwrap_or(0),
            CharacteristicType::MeleeDamage => self.melee_damage.unwrap_or(0),
            CharacteristicType::MeleeResistance => self.melee_resistance.unwrap_or(0),
            CharacteristicType::MovementPoint => self.movement_point.unwrap_or(0),
            CharacteristicType::MovementPointParry => self.movement_point_parry.unwrap_or(0),
            CharacteristicType::MovementPointReduction => {
                self.movement_point_reduction.unwrap_or(0)
            }
            CharacteristicType::NeutralDamage => self.neutral_damage.unwrap_or(0),
            CharacteristicType::NeutralResistance => self.neutral_resistance.unwrap_or(0),
            CharacteristicType::NeutralResistancePercent => {
                self.neutral_resistance_percent.unwrap_or(0)
            }
            CharacteristicType::Pods => self.pods.unwrap_or(0),
            CharacteristicType::Power => self.power.unwrap_or(0),
            CharacteristicType::Prospecting => self.prospecting.unwrap_or(0),
            CharacteristicType::PushBackDamage => self.push_back_damage.unwrap_or(0),
            CharacteristicType::PushBackResistance => self.push_back_resistance.unwrap_or(0),
            CharacteristicType::Range => self.range.unwrap_or(0),
            CharacteristicType::RangeDamage => self.range_damage.unwrap_or(0),
            CharacteristicType::RangeResistance => self.range_resistance.unwrap_or(0),
            CharacteristicType::ReflectedDamage => self.reflected_damage.unwrap_or(0),
            CharacteristicType::SpellDamage => self.spell_damage.unwrap_or(0),
            CharacteristicType::Strength => self.strength.unwrap_or(0),
            CharacteristicType::Summon => self.summon.unwrap_or(0),
            CharacteristicType::TrapDamage => self.trap_damage.unwrap_or(0),
            CharacteristicType::TrapPower => self.trap_power.unwrap_or(0),
            CharacteristicType::Vitality => self.vitality.unwrap_or(0),
            CharacteristicType::WaterDamage => self.water_damage.unwrap_or(0),
            CharacteristicType::WaterResistance => self.water_resistance.unwrap_or(0),
            CharacteristicType::WaterResistancePercent => {
                self.water_resistance_percent.unwrap_or(0)
            }
            CharacteristicType::WeaponDamage => self.weapon_damage.unwrap_or(0),
            CharacteristicType::Wisdom => self.wisdom.unwrap_or(0),
        }
    }

    pub fn set(&mut self, characteristic_type: &CharacteristicType, new_value: i32) {
        match characteristic_type {
            CharacteristicType::AbilityPoint => self.ability_point = Some(new_value),
            CharacteristicType::AbilityPointParry => self.ability_point_parry = Some(new_value),
            CharacteristicType::AbilityPointReduction => {
                self.ability_point_reduction = Some(new_value)
            }
            CharacteristicType::Agility => self.agility = Some(new_value),
            CharacteristicType::AirDamage => self.air_damage = Some(new_value),
            CharacteristicType::AirResistance => self.air_resistance = Some(new_value),
            CharacteristicType::AirResistancePercent => {
                self.air_resistance_percent = Some(new_value)
            }
            CharacteristicType::Chance => self.chance = Some(new_value),
            CharacteristicType::Critical => self.critical = Some(new_value),
            CharacteristicType::CriticalDamage => self.critical_damage = Some(new_value),
            CharacteristicType::CriticalResistance => self.critical_resistance = Some(new_value),
            CharacteristicType::Damage => self.damage = Some(new_value),
            CharacteristicType::Dodge => self.dodge = Some(new_value),
            CharacteristicType::EarthDamage => self.earth_damage = Some(new_value),
            CharacteristicType::EarthResistance => self.earth_resistance = Some(new_value),
            CharacteristicType::EarthResistancePercent => {
                self.earth_resistance_percent = Some(new_value)
            }
            CharacteristicType::FireDamage => self.fire_damage = Some(new_value),
            CharacteristicType::FireResistance => self.fire_resistance = Some(new_value),
            CharacteristicType::FireResistancePercent => {
                self.fire_resistance_percent = Some(new_value)
            }
            CharacteristicType::Heals => self.heals = Some(new_value),
            CharacteristicType::Initiative => self.initiative = Some(new_value),
            CharacteristicType::Intelligence => self.intelligence = Some(new_value),
            CharacteristicType::Lock => self.lock = Some(new_value),
            CharacteristicType::MeleeDamage => self.melee_damage = Some(new_value),
            CharacteristicType::MeleeResistance => self.melee_resistance = Some(new_value),
            CharacteristicType::MovementPoint => self.movement_point = Some(new_value),
            CharacteristicType::MovementPointParry => self.movement_point_parry = Some(new_value),
            CharacteristicType::MovementPointReduction => {
                self.movement_point_reduction = Some(new_value)
            }
            CharacteristicType::NeutralDamage => self.neutral_damage = Some(new_value),
            CharacteristicType::NeutralResistance => self.neutral_resistance = Some(new_value),
            CharacteristicType::NeutralResistancePercent => {
                self.neutral_resistance_percent = Some(new_value)
            }
            CharacteristicType::Pods => self.pods = Some(new_value),
            CharacteristicType::Power => self.power = Some(new_value),
            CharacteristicType::Prospecting => self.prospecting = Some(new_value),
            CharacteristicType::PushBackDamage => self.push_back_damage = Some(new_value),
            CharacteristicType::PushBackResistance => self.push_back_resistance = Some(new_value),
            CharacteristicType::Range => self.range = Some(new_value),
            CharacteristicType::RangeDamage => self.range_damage = Some(new_value),
            CharacteristicType::RangeResistance => self.range_resistance = Some(new_value),
            CharacteristicType::ReflectedDamage => self.reflected_damage = Some(new_value),
            CharacteristicType::SpellDamage => self.spell_damage = Some(new_value),
            CharacteristicType::Strength => self.strength = Some(new_value),
            CharacteristicType::Summon => self.summon = Some(new_value),
            CharacteristicType::TrapDamage => self.trap_damage = Some(new_value),
            CharacteristicType::TrapPower => self.trap_power = Some(new_value),
            CharacteristicType::Vitality => self.vitality = Some(new_value),
            CharacteristicType::WaterDamage => self.water_damage = Some(new_value),
            CharacteristicType::WaterResistance => self.water_resistance = Some(new_value),
            CharacteristicType::WaterResistancePercent => {
                self.water_resistance_percent = Some(new_value)
            }
            CharacteristicType::WeaponDamage => self.weapon_damage = Some(new_value),
            CharacteristicType::Wisdom => self.wisdom = Some(new_value),
        }
    }

    pub fn derived_strength(&self) -> i32 {
        self.strength.unwrap_or(0) + self.power.unwrap_or(0)
    }

    pub fn summary(&self, language: Language) -> String {
        format!(
            "{} {{
    {}: {},
    {}: {},
    {}: {},
}}",
            TranslatedName {
                en: "Effects".to_string(),
                fr: "Effets".to_string(),
            }
            .localized(language),
            CharacteristicType::Power.localized(language),
            self.power.unwrap_or(0),
            CharacteristicType::Strength.localized(language),
            self.strength.unwrap_or(0),
            CharacteristicType::Vitality.localized(language),
            self.vitality.unwrap_or(0),
        )
    }
}

macro_rules! impl_effects_ops {
    ($($field:ident),+) => {
        impl Add for EffectsStructOpt {
            type Output = EffectsStructOpt;

            fn add(self, other: EffectsStructOpt) -> EffectsStructOpt {
                EffectsStructOpt {
                    $(
                        $field: match (self.$field, other.$field) {
                            (Some(a), Some(b)) => Some(a + b),
                            (Some(a), None) => Some(a),
                            (None, Some(b)) => Some(b),
                            (None, None) => None,
                        },
                    )+
                }
            }
        }

        impl Sub for EffectsStructOpt {
            type Output = EffectsStructOpt;

            fn sub(self, other: EffectsStructOpt) -> EffectsStructOpt {
                EffectsStructOpt {
                    $(
                        $field: match (self.$field, other.$field) {
                            (Some(a), Some(b)) => Some(a - b),
                            (Some(a), None) => Some(a),
                            (None, Some(b)) => Some(-b),
                            (None, None) => None,
                        },
                    )+
                }
            }
        }

        impl<'a, 'b> Add<&'b EffectsStructOpt> for &'a EffectsStructOpt {
            type Output = EffectsStructOpt;

            fn add(self, other: &'b EffectsStructOpt) -> EffectsStructOpt {
                EffectsStructOpt {
                    $(
                        $field: match (self.$field, other.$field) {
                            (Some(a), Some(b)) => Some(a + b),
                            (Some(a), None) => Some(a),
                            (None, Some(b)) => Some(b),
                            (None, None) => None,
                        },
                    )+
                }
            }
        }

        impl<'a, 'b> Sub<&'b EffectsStructOpt> for &'a EffectsStructOpt {
            type Output = EffectsStructOpt;

            fn sub(self, other: &'b EffectsStructOpt) -> EffectsStructOpt {
                EffectsStructOpt {
                    $(
                        $field: match (self.$field, other.$field) {
                            (Some(a), Some(b)) => Some(a - b),
                            (Some(a), None) => Some(a),
                            (None, Some(b)) => Some(-b),
                            (None, None) => None,
                        },
                    )+
                }
            }
        }
    };
}

impl_effects_ops!(
    ability_point,
    ability_point_parry,
    ability_point_reduction,
    agility,
    air_damage,
    air_resistance,
    air_resistance_percent,
    chance,
    critical,
    critical_damage,
    critical_resistance,
    damage,
    dodge,
    earth_damage,
    earth_resistance,
    earth_resistance_percent,
    fire_damage,
    fire_resistance,
    fire_resistance_percent,
    heals,
    initiative,
    intelligence,
    lock,
    melee_damage,
    melee_resistance,
    movement_point,
    movement_point_parry,
    movement_point_reduction,
    neutral_damage,
    neutral_resistance,
    neutral_resistance_percent,
    pods,
    power,
    prospecting,
    push_back_damage,
    push_back_resistance,
    range,
    range_damage,
    range_resistance,
    reflected_damage,
    spell_damage,
    strength,
    summon,
    trap_damage,
    trap_power,
    vitality,
    water_damage,
    water_resistance,
    water_resistance_percent,
    weapon_damage,
    wisdom
);

impl AddAssign<&EffectsStructOpt> for EffectsStructOpt {
    fn add_assign(&mut self, other: &Self) {
        *self = &*self + &other;
    }
}

impl SubAssign<&EffectsStructOpt> for EffectsStructOpt {
    fn sub_assign(&mut self, other: &Self) {
        *self = &*self - &other;
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{ALL_CHARACTERISTIC_TYPES, EffectsStructOpt};

    fn create_test_effects() -> EffectsStructOpt {
        let mut effects = EffectsStructOpt::empty();

        for (i, characteristic_type) in ALL_CHARACTERISTIC_TYPES.iter().enumerate() {
            effects.set(characteristic_type, i as i32 + 1);
        }

        effects
    }

    #[test]
    fn empty_get() {
        let effects = EffectsStructOpt::empty();

        for characteristic_type in ALL_CHARACTERISTIC_TYPES {
            assert_eq!(effects.get(characteristic_type), 0)
        }
    }

    #[test]
    fn set_get() {
        let effects = create_test_effects();

        for (i, characteristic_type) in ALL_CHARACTERISTIC_TYPES.iter().enumerate() {
            let expected = i as i32 + 1;
            assert_eq!(effects.get(characteristic_type), expected);
        }
    }

    #[test]
    fn add() {
        let mut effects = create_test_effects();

        effects += &create_test_effects();

        for (i, characteristic_type) in ALL_CHARACTERISTIC_TYPES.iter().enumerate() {
            let expected = (i as i32 + 1) * 2;
            assert_eq!(effects.get(characteristic_type), expected);
        }
    }

    #[test]
    fn minus() {
        let mut effects = create_test_effects();

        effects -= &create_test_effects();

        for characteristic_type in ALL_CHARACTERISTIC_TYPES {
            assert_eq!(effects.get(characteristic_type), 0);
        }
    }
}
