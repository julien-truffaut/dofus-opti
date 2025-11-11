use dofus_opti_core::model::{GearType, Id}; 
use crate::model::Effects;

#[derive(Debug, PartialEq)]
pub struct Gear {
  pub id: Id,  
  pub name: String,  
  pub gear_type: GearType,
  pub level: u32,
  pub effects: Effects
}