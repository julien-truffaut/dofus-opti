use dofus_opti_core::model::*;

use crate::model::{DofusDbCharacteristicTypeId, DofusDbObject, DofusDbTypeId, Effect};


pub fn parse_gear(object: DofusDbObject) -> Result<Gear, String> {
    Ok(Gear { 
        name: object.name.en, 
        gear_type: parse_gear_type(object.typeId)?, 
        level: object.level, 
        characteristics: parse_characteristics(object.effects)?
    })
}

fn parse_gear_type(id: DofusDbTypeId) -> Result<GearType, String> {
    ALL_GEAR_TYPES
      .iter()
      .find(|gear_type| DofusDbTypeId::from(*gear_type) == id)
      .ok_or(format!("Unrecognized object type id {}", id.0))
      .map(|g| g.to_owned()) 
}

fn parse_characteristics(effects: Vec<Effect>) -> Result<Vec<CharacteristicRange>, String> {
    effects
        .into_iter()
        .map(parse_characteristic_range)
        .collect()
}

fn parse_characteristic_range(effect: Effect) -> Result<CharacteristicRange, String> {
    let max = if effect.to == 0 {
        effect.from
    } else {
        effect.to
    };
    Ok(CharacteristicRange {
        kind: parse_characteristic_type(effect.characteristic)?,
        min: effect.from,
        max: max
    })
}

fn parse_characteristic_type(characteristic: DofusDbCharacteristicTypeId) -> Result<CharacteristicType, String> {
    ALL_CHARACTERISTIC_TYPES
      .iter()
      .find(|charac_type| DofusDbCharacteristicTypeId::from(*charac_type) == characteristic)
      .ok_or(format!("Unrecognized characteristic type id {}", characteristic.0))
      .map(|g| g.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use CharacteristicType::*;

    #[test]
    fn parse_valid_gear_types() {
        for gear_type in ALL_GEAR_TYPES {
            let type_id = DofusDbTypeId::from(gear_type);
            assert_eq!(parse_gear_type(type_id), Ok(gear_type.clone()));
        }
    
    }

    #[test]
    fn parse_invalid_gear_types() {
        let invalid_type_id = DofusDbTypeId(-2);
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
        let invalid_type_id = DofusDbCharacteristicTypeId(-2);
        assert_eq!(parse_characteristic_type(invalid_type_id), Err(String::from("Unrecognized characteristic type id -2")));
    }

    #[test]
    fn parse_characteristics_discard_invalid() {
        let vitality = Effect {
            from: 10,
            to: 80,
            characteristic: DofusDbCharacteristicTypeId(11),
        };
        let power = Effect {
            from: -20,
            to: -5,
            characteristic: DofusDbCharacteristicTypeId(25),
        };
        let expected_vitality = CharacteristicRange {
            kind: Vitality,
            min: 10,
            max: 80,
        };
        let expected_power = CharacteristicRange {
            kind: Power,
            min: -20,
            max: -5,
        };

        assert_eq!(
            parse_characteristics(vec!(vitality, power)), 
            Ok(vec!(expected_vitality, expected_power))
        );
    }

    #[test]
    fn parse_characteristics_return_first_invalid() {
        let vitality = Effect {
            from: 10,
            to: 80,
            characteristic: DofusDbCharacteristicTypeId(11),
        };
        let power = Effect {
            from: -20,
            to: -5,
            characteristic: DofusDbCharacteristicTypeId(25),
        };
        let unknown_1 = Effect {
            from: 0,
            to: 100,
            characteristic: DofusDbCharacteristicTypeId(99),
        };
        let unknown_2 = Effect {
            from: 0,
            to: 100,
            characteristic: DofusDbCharacteristicTypeId(8234),
        };

        assert_eq!(
            parse_characteristics(vec!(vitality, unknown_1, power, unknown_2)), 
            Err(String::from("Unrecognized characteristic type id 99"))
        );
    }

    #[test]
    fn parse_golden_gear() -> anyhow::Result<()> {
        use crate::file::read_json;
        use std::path::Path;

        let file_path = Path::new("golden").join("gargandyas_necklace.json");
        let json = read_json(file_path)?;
        let dofus_db_object: DofusDbObject = serde_json::from_value(json)?;

        let gear = parse_gear(dofus_db_object);
        let expected_gear = Gear {
            name: String::from("Gargandyas's Necklace"),
            gear_type: GearType::Amulet,
            level: 200,
            characteristics: vec!(
                CharacteristicRange { kind: Vitality, min: 451, max: 500 }, 
                CharacteristicRange { kind: Wisdom, min: 41, max: 60 },
                CharacteristicRange { kind: Power, min: 41, max: 60 }, 
                CharacteristicRange { kind: Critical, min: 3, max: 4 }, 
                CharacteristicRange { kind: AbilityPoint, min: 2, max: 2 }, 
                CharacteristicRange { kind: MovementPoint, min: -1, max: -1 }, 
                CharacteristicRange { kind: Range, min: 1, max:  1}, 
                CharacteristicRange { kind: Summon, min: 2, max: 2 }, 
                CharacteristicRange { kind: Dodge, min: -20, max: -20 }, 
                CharacteristicRange { kind: MovementPointParry, min: -20, max: -20 }, 
                CharacteristicRange { kind: PushBackDamage, min: 16, max: 20 }, 
                CharacteristicRange { kind: PushBackResistance, min: 31, max: 40 }, 
                CharacteristicRange { kind: MeleeResistance, min: 3, max: 5 }, 
                CharacteristicRange { kind: RangeResistance, min: 3, max: 5 }
            )
        };

        assert_eq!(gear, Ok(expected_gear));

        Ok(())
    }      
}