use crate::model::{ALL_GEAR_SLOT_TYPES, Gear, GearSlotType};

use std::collections::HashMap;
use std::fmt::Write;

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

    pub fn summarize(&self) -> String {
        let mut result = String::new();
        let mut build_possible: u128 = 1;

        result.push_str("GearCatalog {\n");
        for gear_slot_type in ALL_GEAR_SLOT_TYPES {
            let number_of_gears = self.get_gears(*gear_slot_type).len();
            build_possible = build_possible * number_of_gears as u128;
            write!(&mut result, "  #{}: {},\n", gear_slot_type, number_of_gears).unwrap();
        }
        write!(&mut result, "  #Build: {}\n", build_possible).unwrap();
        result.push_str("}");

        result
    }
}
