use thiserror::Error;

use crate::model::GearSlot;
use dofus_opti_core::model::Id;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BuildError {
    #[error("Gear cannot be put in the expected slot, gear: {0}, slot: {1}")]
    InvalidGearSlot(Id, GearSlot),
}
