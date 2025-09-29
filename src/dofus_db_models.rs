#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetObjectsResponse {
    pub total: u32,
    pub limit: u32,
    pub skip: u32,
    pub data: Vec<DofusDbObject>
}

#[derive(Debug, Deserialize)]
pub struct DofusDbObject {
    pub name: TranslatedString,
    pub typeId: i32,
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