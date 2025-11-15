use crate::model::{Effects, Gear, GearSlotType};

use std::cmp::Reverse;
use std::collections::HashMap;

pub struct GearCatalog {
    gears_by_slot: HashMap<GearSlotType, Vec<Gear>>,
}

impl GearCatalog {
    pub fn new<'a, F>(gears: Vec<Gear>, scorer: F) -> Self
    where
        F: Fn(&Effects) -> i32,
    {
        let mut map: HashMap<GearSlotType, Vec<Gear>> = HashMap::new();

        for gear in gears {
            let slot_type = GearSlotType::from(&gear.gear_type);
            map.entry(slot_type).or_default().push(gear.clone());
        }

        for gears in map.values_mut() {
            gears.sort_by_key(|g| Reverse(scorer(&g.effects)));
        }

        Self {
            gears_by_slot: map,
        }
    }

    pub fn get_gears(&self, slot_type: GearSlotType) -> &[Gear] {
        self.gears_by_slot.get(&slot_type).map(|v| v.as_slice()).unwrap_or(&[])
    }
}
