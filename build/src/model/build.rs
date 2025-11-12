use crate::model::{BuildError, Effects, Gear, GearSlot, GearSlotType};
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};

#[derive(Debug, PartialEq)]
pub struct Build<'a> {
  gears: HashMap<GearSlot, &'a Gear>,
  effects: Effects,
}

impl<'a> Build<'a> {
    pub fn empty() -> Self {
        Build { 
            gears  : HashMap::new(),
            effects: Effects::empty(),
        }
    }

    pub fn get_gear(& self, gear_slot: &GearSlot) -> Option<&'a Gear> {
        self.gears.get(gear_slot).map(|g| *g)
    }

    pub fn delete_gear(&mut self, gear_slot: &GearSlot) {
      if let Some(old_gear) = self.gears.remove(gear_slot) {
        self.effects -= &old_gear.effects;
    }
  }

    pub fn set_gear(&mut self, gear: &'a Gear, gear_slot: &GearSlot) -> Result<(), BuildError> {
        check_gear_slot(gear, gear_slot)?;
        let gear_effects = gear.effects.clone();
        match self.gears.entry(*gear_slot) {
          Vacant(entry) => {
            self.effects += &gear_effects;
            entry.insert(gear);
          },  
          Occupied(mut entry) => {
            let old_gear = entry.get();
            self.effects -= &old_gear.effects;
            self.effects += &gear_effects;
            entry.insert(gear);
          }
        }
        
        Ok(())
    }

}

fn check_gear_slot(gear: &Gear, gear_slot: &GearSlot) -> Result<(), BuildError> {
  if GearSlotType::from(&gear.gear_type) == GearSlotType::from(gear_slot) {
    Ok(())
  } else {
    Err(BuildError::InvalidGearSlot(gear.id.clone(), gear_slot.to_owned()))
  }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ALL_GEAR_SLOTS};
    use dofus_opti_core::{Id, GearType};

    #[test]
    fn set_delete_round_trip() -> Result<(), BuildError> {
      let mut build = Build::empty();
      let amulet = Gear {
        id: Id::from("gear_id"),
        name: String::from("gear name"),
        gear_type: GearType::Amulet,
        level: 200,
        effects: Effects::empty(),
      };
      build.set_gear(&amulet, &GearSlot::Amulet)?;
      build.delete_gear(&GearSlot::Amulet);
      assert_eq!(build, Build::empty());
      Ok(())
    }

    #[test]
    fn set_erase_existing_gear() -> Result<(), BuildError> {
      let mut build_1 = Build::empty();
      let mut build_2 = Build::empty();
      let mut effects_1 = Effects::empty();
      let mut effects_2 = Effects::empty();
      effects_1.agility = Some(24);
      effects_2.agility = Some(32);
      let amulet_1 = Gear {
        id: Id::from("gear_id"),
        name: String::from("gear name"),
        gear_type: GearType::Amulet,
        level: 200,
        effects: effects_1,
      };
      let amulet_2 = Gear {
        effects: effects_2,
        ..amulet_1.clone()
      };
      build_1.set_gear(&amulet_1, &GearSlot::Amulet)?;
      build_1.set_gear(&amulet_2, &GearSlot::Amulet)?;
      build_2.set_gear(&amulet_2, &GearSlot::Amulet)?;
    
      assert_eq!(build_1, build_2);
      Ok(())
    }

    #[test]
    fn set_get_gear_round_trip() -> Result<(), BuildError> {
        let mut build = Build::empty();
        let mut gears_map: HashMap<GearSlot, Gear> = HashMap::new();
        let mut effects = Effects::empty();
        effects.strength = Some(1);
        let default_gear = Gear {
          id: Id::from("gear_id"),
          name: String::from("gear name"),
          gear_type: GearType::Amulet,
          level: 200,
          effects: effects,
        };
        for gear_slot in ALL_GEAR_SLOTS {
            let mut gear= default_gear.clone();
            match gear_slot {
                GearSlot::Amulet => gear.gear_type = GearType::Amulet,
                GearSlot::Belt   => gear.gear_type = GearType::Belt,
                GearSlot::Boots  => gear.gear_type = GearType::Boots,
                GearSlot::Cloak  => gear.gear_type = GearType::Cloak,
                GearSlot::Hat    => gear.gear_type = GearType::Hat,
                GearSlot::Ring1  => gear.gear_type = GearType::Ring,
                GearSlot::Ring2  => gear.gear_type = GearType::Ring,
                GearSlot::Shield => gear.gear_type = GearType::Shield,
                GearSlot::Weapon => gear.gear_type = GearType::Sword,
            }
            gears_map.entry(gear_slot.clone()).insert_entry(gear);
        }

        gears_map.iter().for_each(|(gear_slot, gear)| build.set_gear(&gear, gear_slot).unwrap());

        let mut gears: Vec<Option<Gear>> = vec!(); 
        let mut found_gears: Vec<Option<Gear>> = vec!();
        for gear_slot in ALL_GEAR_SLOTS {
          gears.push(gears_map.get(gear_slot).cloned());
          found_gears.push(build.get_gear(gear_slot).cloned());
        }

        let mut expected_effects = Effects::empty();
        expected_effects.strength = Some(9);

        assert_eq!(gears, found_gears);
        assert_eq!(build.effects, expected_effects);
        Ok(())
    }

    #[test]
    fn set_gear_using_wrong_slot() {
      let mut build = Build::empty();
      let amulet = Gear {
        id: Id::from("gear_id"),
        name: String::from("gear name"),
        gear_type: GearType::Amulet,
        level: 200,
        effects: Effects::empty(),
      };
      let result = build.set_gear(&amulet, &GearSlot::Belt);
      assert_eq!(result, Err(BuildError::InvalidGearSlot(Id::from("gear_id"), GearSlot::Belt)));
    }

}