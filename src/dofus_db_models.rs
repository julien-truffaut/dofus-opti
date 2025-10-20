#![allow(non_snake_case)]

use crate::models::GearType; 

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetObjectsResponse {
    pub total: u32,
    pub limit: u32,
    pub skip: u32,
    pub data: Vec<serde_json::Value>
}

#[derive(Debug, Deserialize)]
pub struct DofusDbObject {
    pub name: TranslatedString,
    pub typeId: DofusDbTypeId,
    pub level: u32,
    pub img: String,
    pub effects: Vec<Effect>
}


#[derive(Debug, Deserialize)]
pub struct TranslatedString {
    pub en: String,
    pub fr: String
}

#[derive(Debug, Deserialize)]
pub struct Effect {
  pub from: i32,
  pub to: i32,
  pub characteristic: i32,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DofusDbTypeId { pub value: i32 }

impl From<&GearType> for DofusDbTypeId {
    fn from(gear_type: &GearType) -> Self {
        let id = match gear_type {
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
        };
        DofusDbTypeId{value: id}
    }
}