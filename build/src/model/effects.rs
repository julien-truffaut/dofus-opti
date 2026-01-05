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

    fn create_test_effects() -> Effects {
        let mut effects = Effects::empty();

        for (i, characteristic_type) in ALL_CHARACTERISTIC_TYPES.iter().enumerate() {
            effects.set(characteristic_type, i as i32 + 1);
        }

        effects
    }

    #[test]
    fn empty_get() {
        let effects = Effects::empty();

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
    fn sub() {
        let mut effects = create_test_effects();

        effects.sub(&create_test_effects());

        for characteristic_type in ALL_CHARACTERISTIC_TYPES {
            assert_eq!(effects.get(characteristic_type), 0);
        }
    }
}
