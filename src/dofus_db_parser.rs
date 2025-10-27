use crate::dofus_db_models::{DofusDbCharacteristicTypeId, DofusDbObject, DofusDbTypeId, Effect};
use crate::models::*;


pub fn parse_gear(object: DofusDbObject) -> Result<Gear, String> {
    Ok(Gear { 
        name: object.name.en, 
        gear_type: parse_gear_type(object.typeId)?, 
        level: object.level, 
        characteristics: parse_characteristics(object.effects)
    })
}

fn parse_gear_type(id: DofusDbTypeId) -> Result<GearType, String> {
    ALL_GEAR_TYPES
      .iter()
      .find(|gear_type| DofusDbTypeId::from(*gear_type) == id)
      .ok_or(format!("Unrecognized object type id {}", id.value))
      .map(|g| g.to_owned()) 
}

fn parse_characteristics(effects: Vec<Effect>) -> Vec<CharacteristicRange> {
    effects
        .into_iter()
        .filter_map(|e| parse_characteristic_range(e).ok())
        .collect()
}

fn parse_characteristic_range(effect: Effect) -> Result<CharacteristicRange, String> {
    Ok(CharacteristicRange {
        kind: parse_characteristic_type(effect.characteristic)?,
        min: effect.from,
        max: effect.to
    })
}

fn parse_characteristic_type(characteristic: DofusDbCharacteristicTypeId) -> Result<CharacteristicType, String> {
    ALL_CHARACTERISTIC_TYPES
      .iter()
      .find(|charac_type| DofusDbCharacteristicTypeId::from(*charac_type) == characteristic)
      .ok_or(format!("Unrecognized characteristic type id {}", characteristic.value))
      .map(|g| g.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_gear_types() {
        for gear_type in ALL_GEAR_TYPES {
            let type_id = DofusDbTypeId::from(gear_type);
            assert_eq!(parse_gear_type(type_id), Ok(gear_type.clone()));
        }
    
    }

    #[test]
    fn parse_invalid_gear_types() {
        let invalid_type_id = DofusDbTypeId{ value: -2 };
        assert_eq!(parse_gear_type(invalid_type_id), Err(String::from("Unrecognized object type id -2")));
    }

    #[test]
    fn parse_valid_characteristic_types() {
        for charac in ALL_CHARACTERISTIC_TYPES {
            let type_id = DofusDbCharacteristicTypeId::from(charac);
            assert_eq!(parse_characteristic_type(type_id), Ok(charac.clone()));
        }
    }

    #[test]
    fn parse_invalid_characteristic_type() {
        let invalid_type_id = DofusDbCharacteristicTypeId{ value: -2 };
        assert_eq!(parse_characteristic_type(invalid_type_id), Err(String::from("Unrecognized characteristic type id -2")));
    }

    #[test]
    fn parse_characteristics_discard_invalid() {
        let vitality = Effect {
            from: 10,
            to: 80,
            characteristic: DofusDbCharacteristicTypeId{ value: 11 },
        };
        let power = Effect {
            from: -20,
            to: -5,
            characteristic: DofusDbCharacteristicTypeId{ value: 25 },
        };
        let unknown = Effect {
            from: 0,
            to: 100,
            characteristic: DofusDbCharacteristicTypeId{ value: 99 },
        };
        let expected_vitality = CharacteristicRange {
            kind: CharacteristicType::Vitality,
            min: 10,
            max: 80,
        };
        let expected_power = CharacteristicRange {
            kind: CharacteristicType::Power,
            min: -20,
            max: -5,
        };

        assert_eq!(
            parse_characteristics(vec!(vitality, unknown, power)), 
            vec!(expected_vitality, expected_power)
        );
    }
}