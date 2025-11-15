use crate::model::{Gear, GearSlotType};

use std::collections::HashMap;

pub struct GearCatalog {
    gears_by_slot: HashMap<GearSlotType, Vec<Gear>>,
}

impl GearCatalog {
    pub fn new<'a, F>(gears: Vec<Gear>, mut gear_selector: F) -> Self
    where
        F: FnMut(&mut Vec<Gear>),
    {
        let mut map: HashMap<GearSlotType, Vec<Gear>> = HashMap::new();

        for gear in gears {
            let slot_type = GearSlotType::from(&gear.gear_type);
            map.entry(slot_type).or_default().push(gear);
        }

        for gears in map.values_mut() {
            gear_selector(gears);
        }

        Self {
            gears_by_slot: map,
        }
    }

    pub fn get_gears(&self, slot_type: GearSlotType) -> &[Gear] {
        self.gears_by_slot.get(&slot_type).map(|v| v.as_slice()).unwrap_or(&[])
    }
}
