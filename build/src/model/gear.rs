use crate::model::{Effects, TranslatedName};
use dofus_opti_core::model::{GearType, Id};

#[derive(Debug, PartialEq, Clone)]
pub struct Gear {
    pub id: Id,
    pub name: TranslatedName,
    pub gear_type: GearType,
    pub has_set: bool,
    pub level: u32,
    pub effects: Effects,
}
