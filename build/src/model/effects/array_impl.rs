use crate::model::{ALL_CHARACTERISTIC_TYPES, CharacteristicType};

#[derive(Debug)]
pub struct EffectsArray {
    values: [i32; CharacteristicType::COUNT],
}

impl EffectsArray {
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

    pub fn add(&mut self, other: &EffectsArray) {
        for i in 0..ALL_CHARACTERISTIC_TYPES.len() {
            self.values[i] += other.values[i];
        }
    }

    pub fn minus(&mut self, other: &EffectsArray) {
        for i in 0..ALL_CHARACTERISTIC_TYPES.len() {
            self.values[i] -= other.values[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{ALL_CHARACTERISTIC_TYPES, EffectsArray};

    fn create_test_effects() -> EffectsArray {
        let mut effects = EffectsArray::empty();

        for (i, characteristic_type) in ALL_CHARACTERISTIC_TYPES.iter().enumerate() {
            effects.set(characteristic_type, i as i32 + 1);
        }

        effects
    }

    #[test]
    fn empty_get() {
        let effects = EffectsArray::empty();

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

        effects.minus(&create_test_effects());

        for characteristic_type in ALL_CHARACTERISTIC_TYPES {
            assert_eq!(effects.get(characteristic_type), 0);
        }
    }
}
