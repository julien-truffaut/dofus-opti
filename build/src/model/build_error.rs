use thiserror::Error;

use dofus_opti_core::model::Id;
use crate::model::GearSlot;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("Gear cannot be put in the expected slot, gear: {0}, slot: {1}")]
    InvalidGearSlot(Id, GearSlot),
}