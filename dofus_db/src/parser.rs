use dofus_opti_core::model::*;
use dofus_opti_core::model::{Effect as CoreEffect, ItemSet as CoreItemSet};

use std::collections::HashSet;

use crate::model::{
    DofusDbCharacteristicTypeId, DofusDbObject, DofusDbTypeId, Effect as DofusDbEffect,
    ItemSet as DofusDbItemSet, ItemSetField, TranslatedString,
};

pub fn parse_gear(object: DofusDbObject) -> Result<Gear, String> {
    Ok(Gear {
        id: make_id(&object.name.en),
        name: parse_translated_name(object.name),
        gear_type: parse_gear_type(object.typeId)?,
        set: match object.itemSet {
            ItemSetField::Set(item_set) => Some(make_id(&item_set.name.en)),
            ItemSetField::Bool(_) => None,
        },
        level: object.level,
        characteristics: parse_characteristics(object.effects)?,
    })
}

fn parse_gear_type(id: DofusDbTypeId) -> Result<GearType, String> {
    ALL_GEAR_TYPES
        .iter()
        .find(|gear_type| DofusDbTypeId::from(*gear_type) == id)
        .ok_or(format!("Unrecognized object type id {}", id.0))
        .map(|g| g.to_owned())
}

fn parse_characteristics(effects: Vec<DofusDbEffect>) -> Result<Vec<CharacteristicRange>, String> {
    effects
        .into_iter()
        .filter(
            |e| {
                e.characteristic.0 != -1 &&
                e.characteristic.0 != 15 &&
                e.characteristic.0 != 38 &&
                e.characteristic.0 != 0  && // hunting?
                e.characteristic.0 != 140
            }, // reduce size -40% (rikiki)
        )
        .map(parse_characteristic_range)
        .collect()
}

fn parse_characteristic_range(effect: DofusDbEffect) -> Result<CharacteristicRange, String> {
    let max = if effect.to == 0 {
        effect.from
    } else {
        effect.to
    };
    Ok(CharacteristicRange {
        kind: parse_characteristic_type(effect.characteristic)?,
        min: effect.from,
        max: max,
    })
}

fn parse_characteristic_type(
    characteristic: DofusDbCharacteristicTypeId,
) -> Result<CharacteristicType, String> {
    ALL_CHARACTERISTIC_TYPES
        .iter()
        .find(|charac_type| DofusDbCharacteristicTypeId::from(*charac_type) == characteristic)
        .ok_or(format!("Unrecognized characteristic type id {}", characteristic.0))
        .map(|g| g.to_owned())
}

fn make_id(name: &String) -> Id {
    Id(name.to_lowercase().trim().replace(' ', "_").replace('-', "_").replace("'s", ""))
}

pub fn parse_all_sets(objects: Vec<DofusDbObject>) -> Result<Vec<CoreItemSet>, String> {
    let opt_sets: Vec<Option<CoreItemSet>> =
        objects.into_iter().map(|o| parse_set(o.itemSet)).collect::<Result<Vec<_>, _>>()?;
    let mut seen = HashSet::new();
    let sets: Vec<CoreItemSet> =
        opt_sets.into_iter().flatten().filter(|set| seen.insert(set.id.clone())).collect();

    Ok(sets)
}

fn parse_set(item_set: ItemSetField) -> Result<Option<CoreItemSet>, String> {
    match item_set {
        ItemSetField::Bool(_) => Ok(None),
        ItemSetField::Set(item_set) => Ok(Some(parse_item_set(item_set)?)),
    }
}

fn parse_item_set(item_set: DofusDbItemSet) -> Result<CoreItemSet, String> {
    Ok(CoreItemSet {
        id: make_id(&item_set.name.en),
        name: parse_translated_name(item_set.name),
        effects: parse_set_bonuses(item_set.effects)?,
    })
}

fn parse_set_bonuses(bonuses: Vec<Vec<DofusDbEffect>>) -> Result<Vec<Vec<CoreEffect>>, String> {
    let set_bonuses: Vec<Vec<CoreEffect>> =
        bonuses.into_iter().map(parse_effects).collect::<Result<Vec<_>, _>>()?;
    let filtered = set_bonuses.into_iter().filter(|vec| !vec.is_empty()).collect();
    Ok(filtered)
}

fn parse_effects(effects: Vec<DofusDbEffect>) -> Result<Vec<CoreEffect>, String> {
    let core_effects: Vec<CharacteristicRange> = parse_characteristics(effects)?;
    let result = core_effects
        .into_iter()
        .map(|c| CoreEffect {
            kind: c.kind,
            value: c.max,
        })
        .collect();
    Ok(result)
}

fn parse_translated_name(translated_string: TranslatedString) -> TranslatedName {
    TranslatedName {
        en: translated_string.en,
        fr: translated_string.fr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use CharacteristicType::*;

    #[test]
    fn make_id_foo() {
        assert_eq!(make_id(&String::from(" Foo Bar's Amulet")), Id::from("foo_bar_amulet"));
    }

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
        assert_eq!(
            parse_gear_type(invalid_type_id),
            Err(String::from("Unrecognized object type id -2"))
        );
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
        assert_eq!(
            parse_characteristic_type(invalid_type_id),
            Err(String::from("Unrecognized characteristic type id -2"))
        );
    }

    #[test]
    fn parse_characteristics_discard_invalid() {
        let vitality = DofusDbEffect {
            from: 10,
            to: 80,
            characteristic: DofusDbCharacteristicTypeId(11),
        };
        let power = DofusDbEffect {
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
        let vitality = DofusDbEffect {
            from: 10,
            to: 80,
            characteristic: DofusDbCharacteristicTypeId(11),
        };
        let power = DofusDbEffect {
            from: -20,
            to: -5,
            characteristic: DofusDbCharacteristicTypeId(25),
        };
        let unknown_1 = DofusDbEffect {
            from: 0,
            to: 100,
            characteristic: DofusDbCharacteristicTypeId(99),
        };
        let unknown_2 = DofusDbEffect {
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
        use std::fs::File;
        use std::io::BufReader;
        use std::path::Path;

        let file_path = Path::new("golden").join("gargandyas_necklace.json");
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let json: serde_json::Value = serde_json::from_reader(reader)?;
        let dofus_db_object: DofusDbObject = serde_json::from_value(json)?;

        let gear = parse_gear(dofus_db_object);
        let expected_gear = Gear {
            id: Id::from("gargandyas_necklace"),
            name: TranslatedName {
                en: String::from("Gargandyas's Necklace"),
                fr: String::from("Collier de Gargandyas"),
            },
            gear_type: GearType::Amulet,
            set: Some(Id::from("gargandyas_set")),
            level: 200,
            characteristics: vec![
                CharacteristicRange {
                    kind: Vitality,
                    min: 451,
                    max: 500,
                },
                CharacteristicRange {
                    kind: Wisdom,
                    min: 41,
                    max: 60,
                },
                CharacteristicRange {
                    kind: Power,
                    min: 41,
                    max: 60,
                },
                CharacteristicRange {
                    kind: Critical,
                    min: 3,
                    max: 4,
                },
                CharacteristicRange {
                    kind: AbilityPoint,
                    min: 2,
                    max: 2,
                },
                CharacteristicRange {
                    kind: MovementPoint,
                    min: -1,
                    max: -1,
                },
                CharacteristicRange {
                    kind: Range,
                    min: 1,
                    max: 1,
                },
                CharacteristicRange {
                    kind: Summon,
                    min: 2,
                    max: 2,
                },
                CharacteristicRange {
                    kind: Dodge,
                    min: -20,
                    max: -20,
                },
                CharacteristicRange {
                    kind: MovementPointParry,
                    min: -20,
                    max: -20,
                },
                CharacteristicRange {
                    kind: PushBackDamage,
                    min: 16,
                    max: 20,
                },
                CharacteristicRange {
                    kind: PushBackResistance,
                    min: 31,
                    max: 40,
                },
                CharacteristicRange {
                    kind: MeleeResistance,
                    min: 3,
                    max: 5,
                },
                CharacteristicRange {
                    kind: RangeResistance,
                    min: 3,
                    max: 5,
                },
            ],
        };

        assert_eq!(gear, Ok(expected_gear));

        Ok(())
    }

    #[test]
    fn parse_valid_item_set() {
        let item_set = crate::model::ItemSet {
            name: TranslatedString {
                en: "Super Set".to_string(),
                fr: "Set Super".to_string(),
            },
            effects: vec![
                vec![
                    DofusDbEffect {
                        from: 40,
                        to: 0,
                        characteristic: DofusDbCharacteristicTypeId(11),
                    },
                    DofusDbEffect {
                        from: 10,
                        to: 0,
                        characteristic: DofusDbCharacteristicTypeId(25),
                    },
                ],
                vec![
                    DofusDbEffect {
                        from: 60,
                        to: 0,
                        characteristic: DofusDbCharacteristicTypeId(11),
                    },
                    DofusDbEffect {
                        from: 10,
                        to: 0,
                        characteristic: DofusDbCharacteristicTypeId(25),
                    },
                ],
            ],
        };

        let expected_item_set = CoreItemSet {
            id: Id("super_set".to_string()),
            name: TranslatedName {
                en: "Super Set".to_string(),
                fr: "Set Super".to_string(),
            },
            effects: vec![
                vec![
                    CoreEffect {
                        kind: Vitality,
                        value: 40,
                    },
                    CoreEffect {
                        kind: Power,
                        value: 10,
                    },
                ],
                vec![
                    CoreEffect {
                        kind: Vitality,
                        value: 60,
                    },
                    CoreEffect {
                        kind: Power,
                        value: 10,
                    },
                ],
            ],
        };

        assert_eq!(parse_set(ItemSetField::Set(item_set)), Ok(Some(expected_item_set)));
    }
}
