use crate::dofus_db_models::{DofusDbObject, Effect};
use crate::models::*;


pub fn parse_gear(object: DofusDbObject) -> Result<Gear, String> {
    Ok(Gear { 
        name: object.name.en, 
        gear_type: parse_gear_type(object.typeId)?, 
        level: object.level, 
        characteristics: parse_characteristics(object.effects)
    })
}

pub fn gear_type_to_code(gear_type: &GearType) -> i32 {
    match gear_type {
        GearType::Amulet => 1,
        GearType::Axe    => 19,
        GearType::Belt   => 30,
        GearType::Boots  => 11,
        GearType::Bow    => 2,
        GearType::Cloak  => 17,
        GearType::Dagger => 5,
        GearType::Hammer => 7,
        GearType::Hat    => 16,
        GearType::Lance  => 271,
        GearType::Ring   => 9,
        GearType::Scythe => 22,
        GearType::Shield => 82,
        GearType::Shovel => 8,
        GearType::Staff  => 4,
        GearType::Sword  => 6,
        GearType::Wand   => 3,
    }
}

fn parse_gear_type(id: i32) -> Result<GearType, String> {
    ALL_GEAR_TYPES
      .iter()
      .find(|gear_type| gear_type_to_code(gear_type) == id)
      .ok_or(format!("Unrecognized object type {}", id))
      .map(|g| g.to_owned()) 
}

fn parse_characteristics(effects: Vec<Effect>) -> Vec<CharacteristicRange> {
    effects
        .into_iter()
        .filter_map(|e| parse_characteristic(e).ok())
        .collect()
}

fn parse_characteristic(effect: Effect) -> Result<CharacteristicRange, String> {
    Ok(CharacteristicRange {
        kind: parse_characteristic_type(effect.characteristic)?,
        min: effect.from,
        max: effect.to
    })
}

fn parse_characteristic_type(characteristic: i32) -> Result<CharacteristicType, String> {
    match characteristic {
        11 => Ok(CharacteristicType::Vitality),
        25 => Ok(CharacteristicType::Power),
        _ => Err(format!("Unrecognized characteristic type {}", characteristic)),
    }
}