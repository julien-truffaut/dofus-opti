#![allow(non_snake_case)]

use crate::models::{CharacteristicType, GearType}; 

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
  pub characteristic: DofusDbCharacteristicTypeId,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DofusDbTypeId(pub i32);

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
        DofusDbTypeId(id)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DofusDbCharacteristicTypeId(pub i32);

impl From<&CharacteristicType> for DofusDbCharacteristicTypeId {
    fn from(characteristique_type: &CharacteristicType) -> Self {
        let id = match characteristique_type {
            CharacteristicType::AbilityPoint       => 1,
            CharacteristicType::Critical           => 18,
            CharacteristicType::Dodge              => 78,
            CharacteristicType::MeleeResistance    => 124,
            CharacteristicType::MovementPoint      => 23,
            CharacteristicType::MovementPointParry => 28,
            CharacteristicType::Power              => 25,
            CharacteristicType::PushBackDamage     => 84,
            CharacteristicType::PushBackResistance => 85,
            CharacteristicType::Range              => 19,
            CharacteristicType::RangeResistance    => 121,
            CharacteristicType::Summon             => 26,
            CharacteristicType::Vitality           => 11,
            CharacteristicType::Wisdom             => 12,
        };
        DofusDbCharacteristicTypeId(id)
    }
}