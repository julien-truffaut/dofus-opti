use crate::model::Effects;
use dofus_opti_core::model::{GearType, Id};

#[derive(Debug, PartialEq, Clone)]
pub struct Gear {
    pub id: Id,
    pub name: String,
    pub gear_type: GearType,
    pub level: u32,
    pub effects: Effects,
}
