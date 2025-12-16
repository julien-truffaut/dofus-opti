use crate::model::{CharacteristicType, Language, TranslatedName};

#[derive(Clone, Debug, PartialEq)]
pub struct EffectsStruct {
    pub ability_point: i32,
    pub ability_point_parry: i32,
    pub ability_point_reduction: i32,
    pub agility: i32,
    pub air_damage: i32,
    pub air_resistance: i32,
    pub air_resistance_percent: i32,
    pub chance: i32,
    pub critical: i32,
    pub critical_damage: i32,
    pub critical_resistance: i32,
    pub damage: i32,
    pub dodge: i32,
    pub earth_damage: i32,
    pub earth_resistance: i32,
    pub earth_resistance_percent: i32,
    pub fire_damage: i32,
    pub fire_resistance: i32,
    pub fire_resistance_percent: i32,
    pub heals: i32,
    pub initiative: i32,
    pub intelligence: i32,
    pub lock: i32,
    pub melee_damage: i32,
    pub melee_resistance: i32,
    pub movement_point: i32,
    pub movement_point_parry: i32,
    pub movement_point_reduction: i32,
    pub neutral_damage: i32,
    pub neutral_resistance: i32,
    pub neutral_resistance_percent: i32,
    pub pods: i32,
    pub power: i32,
    pub prospecting: i32,
    pub push_back_damage: i32,
    pub push_back_resistance: i32,
    pub range: i32,
    pub range_damage: i32,
    pub range_resistance: i32,
    pub reflected_damage: i32,
    pub spell_damage: i32,
    pub strength: i32,
    pub summon: i32,
    pub trap_damage: i32,
    pub trap_power: i32,
    pub vitality: i32,
    pub water_damage: i32,
    pub water_resistance: i32,
    pub water_resistance_percent: i32,
    pub weapon_damage: i32,
    pub wisdom: i32,
}

impl EffectsStruct {
    pub fn empty() -> EffectsStruct {
        EffectsStruct {
            ability_point: 0,
            ability_point_parry: 0,
            ability_point_reduction: 0,
            agility: 0,
            air_damage: 0,
            air_resistance: 0,
            air_resistance_percent: 0,
            chance: 0,
            critical: 0,
            critical_damage: 0,
            critical_resistance: 0,
            damage: 0,
            dodge: 0,
            earth_damage: 0,
            earth_resistance: 0,
            earth_resistance_percent: 0,
            fire_damage: 0,
            fire_resistance: 0,
            fire_resistance_percent: 0,
            heals: 0,
            initiative: 0,
            intelligence: 0,
            lock: 0,
            melee_damage: 0,
            melee_resistance: 0,
            movement_point: 0,
            movement_point_parry: 0,
            movement_point_reduction: 0,
            neutral_damage: 0,
            neutral_resistance: 0,
            neutral_resistance_percent: 0,
            pods: 0,
            power: 0,
            prospecting: 0,
            push_back_damage: 0,
            push_back_resistance: 0,
            range: 0,
            range_damage: 0,
            range_resistance: 0,
            reflected_damage: 0,
            spell_damage: 0,
            strength: 0,
            summon: 0,
            trap_damage: 0,
            trap_power: 0,
            vitality: 0,
            water_damage: 0,
            water_resistance: 0,
            water_resistance_percent: 0,
            weapon_damage: 0,
            wisdom: 0,
        }
    }

    pub fn add(&mut self, other: &EffectsStruct) {
        self.ability_point += other.ability_point;
        self.ability_point_parry += other.ability_point_parry;
        self.ability_point_reduction += other.ability_point_reduction;
        self.agility += other.agility;
        self.air_damage += other.air_damage;
        self.air_resistance += other.air_resistance;
        self.air_resistance_percent += other.air_resistance_percent;
        self.chance += other.chance;
        self.critical += other.critical;
        self.critical_damage += other.critical_damage;
        self.critical_resistance += other.critical_resistance;
        self.damage += other.damage;
        self.dodge += other.dodge;
        self.earth_damage += other.earth_damage;
        self.earth_resistance += other.earth_resistance;
        self.earth_resistance_percent += other.earth_resistance_percent;
        self.fire_damage += other.fire_damage;
        self.fire_resistance += other.fire_resistance;
        self.fire_resistance_percent += other.fire_resistance_percent;
        self.heals += other.heals;
        self.initiative += other.initiative;
        self.intelligence += other.intelligence;
        self.lock += other.lock;
        self.melee_damage += other.melee_damage;
        self.melee_resistance += other.melee_resistance;
        self.movement_point += other.movement_point;
        self.movement_point_parry += other.movement_point_parry;
        self.movement_point_reduction += other.movement_point_reduction;
        self.neutral_damage += other.neutral_damage;
        self.neutral_resistance += other.neutral_resistance;
        self.neutral_resistance_percent += other.neutral_resistance_percent;
        self.pods += other.pods;
        self.power += other.power;
        self.prospecting += other.prospecting;
        self.push_back_damage += other.push_back_damage;
        self.push_back_resistance += other.push_back_resistance;
        self.range += other.range;
        self.range_damage += other.range_damage;
        self.range_resistance += other.range_resistance;
        self.reflected_damage += other.reflected_damage;
        self.spell_damage += other.spell_damage;
        self.strength += other.strength;
        self.summon += other.summon;
        self.trap_damage += other.trap_damage;
        self.trap_power += other.trap_power;
        self.vitality += other.vitality;
        self.water_damage += other.water_damage;
        self.water_resistance += other.water_resistance;
        self.water_resistance_percent += other.water_resistance_percent;
        self.weapon_damage += other.weapon_damage;
        self.wisdom += other.wisdom;
    }

    pub fn sub(&mut self, other: &EffectsStruct) {
        self.ability_point -= other.ability_point;
        self.ability_point_parry -= other.ability_point_parry;
        self.ability_point_reduction -= other.ability_point_reduction;
        self.agility -= other.agility;
        self.air_damage -= other.air_damage;
        self.air_resistance -= other.air_resistance;
        self.air_resistance_percent -= other.air_resistance_percent;
        self.chance -= other.chance;
        self.critical -= other.critical;
        self.critical_damage -= other.critical_damage;
        self.critical_resistance -= other.critical_resistance;
        self.damage -= other.damage;
        self.dodge -= other.dodge;
        self.earth_damage -= other.earth_damage;
        self.earth_resistance -= other.earth_resistance;
        self.earth_resistance_percent -= other.earth_resistance_percent;
        self.fire_damage -= other.fire_damage;
        self.fire_resistance -= other.fire_resistance;
        self.fire_resistance_percent -= other.fire_resistance_percent;
        self.heals -= other.heals;
        self.initiative -= other.initiative;
        self.intelligence -= other.intelligence;
        self.lock -= other.lock;
        self.melee_damage -= other.melee_damage;
        self.melee_resistance -= other.melee_resistance;
        self.movement_point -= other.movement_point;
        self.movement_point_parry -= other.movement_point_parry;
        self.movement_point_reduction -= other.movement_point_reduction;
        self.neutral_damage -= other.neutral_damage;
        self.neutral_resistance -= other.neutral_resistance;
        self.neutral_resistance_percent -= other.neutral_resistance_percent;
        self.pods -= other.pods;
        self.power -= other.power;
        self.prospecting -= other.prospecting;
        self.push_back_damage -= other.push_back_damage;
        self.push_back_resistance -= other.push_back_resistance;
        self.range -= other.range;
        self.range_damage -= other.range_damage;
        self.range_resistance -= other.range_resistance;
        self.reflected_damage -= other.reflected_damage;
        self.spell_damage -= other.spell_damage;
        self.strength -= other.strength;
        self.summon -= other.summon;
        self.trap_damage -= other.trap_damage;
        self.trap_power -= other.trap_power;
        self.vitality -= other.vitality;
        self.water_damage -= other.water_damage;
        self.water_resistance -= other.water_resistance;
        self.water_resistance_percent -= other.water_resistance_percent;
        self.weapon_damage -= other.weapon_damage;
        self.wisdom -= other.wisdom;
    }

    pub fn get(&self, characteristic_type: &CharacteristicType) -> i32 {
        match characteristic_type {
            CharacteristicType::AbilityPoint => self.ability_point,
            CharacteristicType::AbilityPointParry => self.ability_point_parry,
            CharacteristicType::AbilityPointReduction => self.ability_point_reduction,
            CharacteristicType::Agility => self.agility,
            CharacteristicType::AirDamage => self.air_damage,
            CharacteristicType::AirResistance => self.air_resistance,
            CharacteristicType::AirResistancePercent => self.air_resistance_percent,
            CharacteristicType::Chance => self.chance,
            CharacteristicType::Critical => self.critical,
            CharacteristicType::CriticalDamage => self.critical_damage,
            CharacteristicType::CriticalResistance => self.critical_resistance,
            CharacteristicType::Damage => self.damage,
            CharacteristicType::Dodge => self.dodge,
            CharacteristicType::EarthDamage => self.earth_damage,
            CharacteristicType::EarthResistance => self.earth_resistance,
            CharacteristicType::EarthResistancePercent => self.earth_resistance_percent,
            CharacteristicType::FireDamage => self.fire_damage,
            CharacteristicType::FireResistance => self.fire_resistance,
            CharacteristicType::FireResistancePercent => self.fire_resistance_percent,
            CharacteristicType::Heals => self.heals,
            CharacteristicType::Initiative => self.initiative,
            CharacteristicType::Intelligence => self.intelligence,
            CharacteristicType::Lock => self.lock,
            CharacteristicType::MeleeDamage => self.melee_damage,
            CharacteristicType::MeleeResistance => self.melee_resistance,
            CharacteristicType::MovementPoint => self.movement_point,
            CharacteristicType::MovementPointParry => self.movement_point_parry,
            CharacteristicType::MovementPointReduction => self.movement_point_reduction,
            CharacteristicType::NeutralDamage => self.neutral_damage,
            CharacteristicType::NeutralResistance => self.neutral_resistance,
            CharacteristicType::NeutralResistancePercent => self.neutral_resistance_percent,
            CharacteristicType::Pods => self.pods,
            CharacteristicType::Power => self.power,
            CharacteristicType::Prospecting => self.prospecting,
            CharacteristicType::PushBackDamage => self.push_back_damage,
            CharacteristicType::PushBackResistance => self.push_back_resistance,
            CharacteristicType::Range => self.range,
            CharacteristicType::RangeDamage => self.range_damage,
            CharacteristicType::RangeResistance => self.range_resistance,
            CharacteristicType::ReflectedDamage => self.reflected_damage,
            CharacteristicType::SpellDamage => self.spell_damage,
            CharacteristicType::Strength => self.strength,
            CharacteristicType::Summon => self.summon,
            CharacteristicType::TrapDamage => self.trap_damage,
            CharacteristicType::TrapPower => self.trap_power,
            CharacteristicType::Vitality => self.vitality,
            CharacteristicType::WaterDamage => self.water_damage,
            CharacteristicType::WaterResistance => self.water_resistance,
            CharacteristicType::WaterResistancePercent => self.water_resistance_percent,
            CharacteristicType::WeaponDamage => self.weapon_damage,
            CharacteristicType::Wisdom => self.wisdom,
        }
    }

    pub fn set(&mut self, characteristic_type: &CharacteristicType, new_value: i32) {
        match characteristic_type {
            CharacteristicType::AbilityPoint => self.ability_point = new_value,
            CharacteristicType::AbilityPointParry => self.ability_point_parry = new_value,
            CharacteristicType::AbilityPointReduction => self.ability_point_reduction = new_value,
            CharacteristicType::Agility => self.agility = new_value,
            CharacteristicType::AirDamage => self.air_damage = new_value,
            CharacteristicType::AirResistance => self.air_resistance = new_value,
            CharacteristicType::AirResistancePercent => self.air_resistance_percent = new_value,
            CharacteristicType::Chance => self.chance = new_value,
            CharacteristicType::Critical => self.critical = new_value,
            CharacteristicType::CriticalDamage => self.critical_damage = new_value,
            CharacteristicType::CriticalResistance => self.critical_resistance = new_value,
            CharacteristicType::Damage => self.damage = new_value,
            CharacteristicType::Dodge => self.dodge = new_value,
            CharacteristicType::EarthDamage => self.earth_damage = new_value,
            CharacteristicType::EarthResistance => self.earth_resistance = new_value,
            CharacteristicType::EarthResistancePercent => self.earth_resistance_percent = new_value,
            CharacteristicType::FireDamage => self.fire_damage = new_value,
            CharacteristicType::FireResistance => self.fire_resistance = new_value,
            CharacteristicType::FireResistancePercent => self.fire_resistance_percent = new_value,
            CharacteristicType::Heals => self.heals = new_value,
            CharacteristicType::Initiative => self.initiative = new_value,
            CharacteristicType::Intelligence => self.intelligence = new_value,
            CharacteristicType::Lock => self.lock = new_value,
            CharacteristicType::MeleeDamage => self.melee_damage = new_value,
            CharacteristicType::MeleeResistance => self.melee_resistance = new_value,
            CharacteristicType::MovementPoint => self.movement_point = new_value,
            CharacteristicType::MovementPointParry => self.movement_point_parry = new_value,
            CharacteristicType::MovementPointReduction => self.movement_point_reduction = new_value,
            CharacteristicType::NeutralDamage => self.neutral_damage = new_value,
            CharacteristicType::NeutralResistance => self.neutral_resistance = new_value,
            CharacteristicType::NeutralResistancePercent => {
                self.neutral_resistance_percent = new_value
            }
            CharacteristicType::Pods => self.pods = new_value,
            CharacteristicType::Power => self.power = new_value,
            CharacteristicType::Prospecting => self.prospecting = new_value,
            CharacteristicType::PushBackDamage => self.push_back_damage = new_value,
            CharacteristicType::PushBackResistance => self.push_back_resistance = new_value,
            CharacteristicType::Range => self.range = new_value,
            CharacteristicType::RangeDamage => self.range_damage = new_value,
            CharacteristicType::RangeResistance => self.range_resistance = new_value,
            CharacteristicType::ReflectedDamage => self.reflected_damage = new_value,
            CharacteristicType::SpellDamage => self.spell_damage = new_value,
            CharacteristicType::Strength => self.strength = new_value,
            CharacteristicType::Summon => self.summon = new_value,
            CharacteristicType::TrapDamage => self.trap_damage = new_value,
            CharacteristicType::TrapPower => self.trap_power = new_value,
            CharacteristicType::Vitality => self.vitality = new_value,
            CharacteristicType::WaterDamage => self.water_damage = new_value,
            CharacteristicType::WaterResistance => self.water_resistance = new_value,
            CharacteristicType::WaterResistancePercent => self.water_resistance_percent = new_value,
            CharacteristicType::WeaponDamage => self.weapon_damage = new_value,
            CharacteristicType::Wisdom => self.wisdom = new_value,
        }
    }

    pub fn derived_strength(&self) -> i32 {
        self.strength + self.power
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
            self.power,
            CharacteristicType::Strength.localized(language),
            self.strength,
            CharacteristicType::Vitality.localized(language),
            self.vitality,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{ALL_CHARACTERISTIC_TYPES, EffectsStruct};

    fn create_test_effects() -> EffectsStruct {
        let mut effects = EffectsStruct::empty();

        for (i, characteristic_type) in ALL_CHARACTERISTIC_TYPES.iter().enumerate() {
            effects.set(characteristic_type, i as i32 + 1);
        }

        effects
    }

    #[test]
    fn empty_get() {
        let effects = EffectsStruct::empty();

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

        effects.add(&create_test_effects());

        for (i, characteristic_type) in ALL_CHARACTERISTIC_TYPES.iter().enumerate() {
            let expected = (i as i32 + 1) * 2;
            assert_eq!(effects.get(characteristic_type), expected);
        }
    }

    #[test]
    fn minus() {
        let mut effects = create_test_effects();

        effects.sub(&create_test_effects());

        for characteristic_type in ALL_CHARACTERISTIC_TYPES {
            assert_eq!(effects.get(characteristic_type), 0);
        }
    }
}
