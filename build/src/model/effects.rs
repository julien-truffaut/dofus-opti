use crate::model::{Language, TranslatedName};

use crate::model::{ALL_CHARACTERISTIC_TYPES, CharacteristicType};

#[derive(Debug, Clone, PartialEq)]
pub struct Effects {
    values: [i32; CharacteristicType::COUNT],
}

impl Effects {
    pub fn empty() -> Self {
        Self {
            values: [0; CharacteristicType::COUNT],
        }
    }

    pub fn get(&self, characteristic_type: &CharacteristicType) -> i32 {
        self.values[characteristic_type.index()]
    }

    pub fn set(&mut self, characteristic_type: &CharacteristicType, new_value: i32) {
        self.values[characteristic_type.index()] = new_value;
    }

    pub fn add(&mut self, other: &Effects) {
        for i in 0..ALL_CHARACTERISTIC_TYPES.len() {
            self.values[i] += other.values[i];
        }
    }

    pub fn sub(&mut self, other: &Effects) {
        for i in 0..ALL_CHARACTERISTIC_TYPES.len() {
            self.values[i] -= other.values[i];
        }
    }

    pub fn power(&self) -> i32 {
        self.get(&CharacteristicType::Power)
    }

    pub fn strength(&self) -> i32 {
        self.get(&CharacteristicType::Strength)
    }

    pub fn vitality(&self) -> i32 {
        self.get(&CharacteristicType::Vitality)
    }

    pub fn derived_strength(&self) -> i32 {
        self.strength() + self.power()
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
            self.power(),
            CharacteristicType::Strength.localized(language),
            self.strength(),
            CharacteristicType::Vitality.localized(language),
            self.vitality(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{ALL_CHARACTERISTIC_TYPES, Effects};
    use crate::test_support::strategies::*;
    use proptest::prelude::*;

    #[test]
    fn empty_get() {
        let effects = Effects::empty();

        for characteristic_type in ALL_CHARACTERISTIC_TYPES {
            assert_eq!(effects.get(characteristic_type), 0)
        }
    }

    proptest! {
        #[test]
        fn set_get((mut effects, new_value) in (gen_effects(), -100..500)) {
            for characteristic_type in ALL_CHARACTERISTIC_TYPES {
                effects.set(characteristic_type, new_value);
                prop_assert!(effects.get(characteristic_type) == new_value);
            }
        }

        #[test]
        fn get_set(mut effects in gen_effects()) {
            let original = effects.clone();
            for characteristic_type in ALL_CHARACTERISTIC_TYPES {
                let current_value = effects.get(characteristic_type);
                effects.set(characteristic_type, current_value);
                prop_assert!(effects == original);
            }
        }

        #[test]
        fn add((effects1, effects2) in (gen_effects(), gen_effects())) {
            let mut effects = effects1.clone();
            effects.add(&effects2);

            for characteristic_type in ALL_CHARACTERISTIC_TYPES {
                let expected = effects1.get(characteristic_type) + effects2.get(characteristic_type);
                let found = effects.get(characteristic_type);
                prop_assert!(found == expected);
            }
        }

        #[test]
        fn sub((effects1, effects2) in (gen_effects(), gen_effects())) {
            let mut effects = effects1.clone();
            effects.sub(&effects2);

            for characteristic_type in ALL_CHARACTERISTIC_TYPES {
                let expected = effects1.get(characteristic_type) - effects2.get(characteristic_type);
                let found = effects.get(characteristic_type);
                prop_assert!(found == expected);
            }
        }
    }
}
