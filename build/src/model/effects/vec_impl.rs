use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct EffectsVec {
    values: Vec<Effect>
}

impl EffectsVec {

    fn one(effect: Effect) -> Self {
        Self { values: vec!(effect) }
    }

    pub fn empty() -> Self {
        Self { values: vec!() }
    }

    pub fn get(&self, characteristic_type: &CharacteristicType) -> i32 {
        self.values.iter().find(|e| e.kind == *characteristic_type).map(|e| e.value).unwrap_or(0)
    }

    pub fn set(&mut self, characteristic_type: &CharacteristicType, new_value: i32) {
        if let Some(existing) = self.values.iter_mut().find(|e| e.kind == *characteristic_type) {
            existing.value = new_value;
        } else {
            self.values.push(Effect { kind: *characteristic_type, value: new_value });
        }
    }

    pub fn add(&mut self, other: &Self) {
        for effect in &other.values {
            if let Some(existing) = self.values.iter_mut().find(|e| e.kind == effect.kind) {
                existing.value += effect.value;
            } else {
                self.values.push(effect.clone());
            }
        }
    }

    pub fn add_ordered(&mut self, other: &Self) {
        let mut i = 0;
        let mut j = 0;

        let mut result = Vec::with_capacity(ALL_CHARACTERISTIC_TYPES.len());

        while i < self.values.len() && j < other.values.len() {
            let left = &self.values[i];
            let right = &other.values[j];

            if left.kind == right.kind {
                result.push(Effect {
                    kind: left.kind,
                    value: left.value + right.value,
                });
                i += 1;
                j += 1;
            } else if left.kind < right.kind {
                result.push(left.clone());
                i += 1;
            } else {
                result.push(right.clone());
                j += 1;
            }
        }

        // append leftovers
        result.extend_from_slice(&self.values[i..]);
        result.extend(other.values[j..].iter().cloned());

        self.values = result;
    }

    pub fn random_sample() -> Self {
        let mut rng = thread_rng();

        // choose how many effects will be generated
        let n = rng.gen_range(5..15);

        let mut effects = Self::empty();

        for _ in 0..n {
            let effect = Effect {
                kind: CharacteristicType::random(),
                value: rng.gen_range(-50..200),
            };
            effects.add_ordered(&Self::one(effect));
        }

        effects
    }
}

#[derive(Debug, Clone)]
struct Effect {
    pub kind: CharacteristicType,
    pub value: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

static ALL_CHARACTERISTIC_TYPES: &[CharacteristicType] = &[
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

impl CharacteristicType {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        ALL_CHARACTERISTIC_TYPES[rng.gen_range(0..ALL_CHARACTERISTIC_TYPES.len())]
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{EffectsVec};
    use super::*;
    

    fn create_test_effects() -> EffectsVec {
        let mut effects = EffectsVec::empty();

        for (i, characteristic_type) in ALL_CHARACTERISTIC_TYPES.iter().enumerate() {
            effects.set(characteristic_type, i as i32 + 1);
        }

        effects
    }

    #[test]
    fn empty_get() {
        let effects = EffectsVec::empty();

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

}