use crate::model::{ALL_GEAR_SLOT_TYPES, ALL_GEAR_SLOTS, Gear, GearSlotType};

use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug)]
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

    pub fn all_gears(&self) -> Vec<&Gear> {
        self.gears_by_slot.values().flat_map(|gears| gears.iter()).collect()
    }

    pub fn size(&self) -> usize {
        self.gears_by_slot.values().map(|gears| gears.len()).sum()
    }

    pub fn retain<F>(&mut self, predicate: F)
    where
        F: Fn(&Gear) -> bool + Copy,
    {
        for gears in self.gears_by_slot.values_mut() {
            gears.retain(predicate);
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
            write!(&mut result, "  #{}: {},\n", gear_slot_type, number_of_gears).unwrap();
        }
        for gear_slot in ALL_GEAR_SLOTS {
            let number_of_gears = self.get_gears(GearSlotType::from(gear_slot)).len();
            build_possible = build_possible * number_of_gears as u128;
        }
        write!(&mut result, "  #Build: {}\n", build_possible).unwrap();
        result.push_str("}");

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use crate::test_support::strategies::*;

    proptest! {
        #[test]
        fn all_gears(gears in proptest::collection::vec(gen_gear(), 0..50)) {
            let catalog = GearCatalog::new(gears.clone());
            let all_gears = catalog.all_gears();

            prop_assert!(all_gears.len() == gears.len());
            
            for gear in gears {
                let found_gear = all_gears.iter().find(|g| g.id == gear.id);

                prop_assert!(found_gear == Some(&&gear));
            }
        }

        #[test]
        fn size_stays_the_same(gears in proptest::collection::vec(gen_gear(), 0..50)) {
            let size = gears.len();
            let catalog = GearCatalog::new(gears);
            
            prop_assert!(catalog.size() == size);
        }

        #[test]
        fn all_gears_can_be_retrieved_by_slot_type(gears in proptest::collection::vec(gen_gear(), 0..50)) {
            let catalog = GearCatalog::new(gears.clone());

            for gear in gears {
                let slot_type = GearSlotType::from(&gear.gear_type);
                let found_gear = catalog.get_gears(slot_type).iter().find(|g| g.id == gear.id);

                prop_assert!(found_gear == Some(&gear));
            }
        }

        #[test]
        fn retain_per_level((mut catalog, min_level) in (gen_gear_catalog(0..50), gen_gear_level())) {
            let count = catalog.all_gears().iter().filter(|gear| gear.level >= min_level).count();
            catalog.retain(|gear| gear.level >= min_level);

            prop_assert!(catalog.size() == count);
            
            for gear in catalog.all_gears() {
                prop_assert!(gear.level >= min_level);
            }
        }
    }

}