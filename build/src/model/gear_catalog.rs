use crate::model::{Gear, GearSlotType};

use std::collections::HashMap;

pub struct GearCatalog {
    gears_by_slot: HashMap<GearSlotType, Vec<Gear>>,
}

impl GearCatalog {
    pub fn new(gears: Vec<Gear>) -> Self {
        let mut map: HashMap<GearSlotType, Vec<Gear>> = HashMap::new();

        for gear in gears {
            let slot_type = GearSlotType::from(&gear.gear_type);
            map.entry(slot_type).or_default().push(gear);
        }

        Self {
            gears_by_slot: map,
        }
    }

    pub fn filter<F>(&mut self, mut gear_selector: F)
    where
        F: FnMut(&mut Vec<Gear>),
    {
        for gears in self.gears_by_slot.values_mut() {
            gear_selector(gears);
        }
    }

    pub fn get_gears(&self, slot_type: GearSlotType) -> &[Gear] {
        self.gears_by_slot.get(&slot_type).map(|v| v.as_slice()).unwrap_or(&[])
    }
}
