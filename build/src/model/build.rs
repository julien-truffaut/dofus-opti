use crate::model::{BuildError, Effects, Gear, GearSlot, GearSlotType};

#[derive(Debug, PartialEq)]
pub struct Build<'a> {
  amulet: Option<&'a Gear>,
  belt: Option<&'a Gear>,
  boots: Option<&'a Gear>,
  cloak: Option<&'a Gear>,
  hat: Option<&'a Gear>,
  ring_1: Option<&'a Gear>,
  ring_2: Option<&'a Gear>,
  shield: Option<&'a Gear>,
  weapon: Option<&'a Gear>,
  effects: Effects,
}

impl<'a> Build<'a> {
    pub fn empty() -> Self {
        Build { 
            amulet: None, 
            belt: None, 
            boots: None, 
            cloak: None, 
            hat: None, 
            ring_1: None, 
            ring_2: None, 
            shield: None, 
            weapon: None, 
            effects: Effects::empty(),
        }
    }

    pub fn get_gear(& self, gear_slot: &GearSlot) -> Option<&'a Gear> {
        match gear_slot {
            GearSlot::Amulet => self.amulet,
            GearSlot::Belt   => self.belt,
            GearSlot::Boots  => self.boots,
            GearSlot::Cloak  => self.cloak,
            GearSlot::Hat    => self.hat,
            GearSlot::Ring1  => self.ring_1,
            GearSlot::Ring2  => self.ring_2,
            GearSlot::Shield => self.shield,
            GearSlot::Weapon => self.weapon,
        }
    }

    pub fn set_gear(&mut self, gear: &'a Gear, gear_slot: &GearSlot) -> Result<(), BuildError> {
        check_gear_slot(gear, gear_slot)?;
        let gear_effects = gear.effects.clone();
        match gear_slot {
            GearSlot::Amulet => {
              self.amulet.iter().for_each(|current_gear| self.effects -= current_gear.effects.clone());
              self.effects += gear_effects;
              self.amulet = Some(gear);
            },
            GearSlot::Belt => {
              self.belt.iter().for_each(|current_gear| self.effects -= current_gear.effects.clone());
              self.effects += gear_effects;
              self.belt = Some(gear);
            },
            GearSlot::Boots => {
              self.boots.iter().for_each(|current_gear| self.effects -= current_gear.effects.clone());
              self.effects += gear_effects;
              self.boots = Some(gear);
            },
            GearSlot::Cloak => {
              self.cloak.iter().for_each(|current_gear| self.effects -= current_gear.effects.clone());
              self.effects += gear_effects;
              self.cloak = Some(gear);
            },
            GearSlot::Hat => {
              self.hat.iter().for_each(|current_gear| self.effects -= current_gear.effects.clone());
              self.effects += gear_effects;
              self.hat = Some(gear);
            },
            GearSlot::Ring1 => {
              self.ring_1.iter().for_each(|current_gear| self.effects -= current_gear.effects.clone());
              self.effects += gear_effects;
              self.ring_1 = Some(gear);
            },
            GearSlot::Ring2 => {
              self.ring_2.iter().for_each(|current_gear| self.effects -= current_gear.effects.clone());
              self.effects += gear_effects;
              self.ring_2 = Some(gear);
            },
            GearSlot::Shield => {
              self.shield.iter().for_each(|current_gear| self.effects -= current_gear.effects.clone());
              self.effects += gear_effects;
              self.shield = Some(gear);
            },
            GearSlot::Weapon => {
              self.weapon.iter().for_each(|current_gear| self.effects -= current_gear.effects.clone());
              self.effects += gear_effects;
              self.weapon = Some(gear);
            },
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
    use crate::model::ALL_GEAR_SLOTS;
    use dofus_opti_core::{Id, GearType};
    use std::collections::HashMap;

    #[test]
    fn set_get_gear_round_trip() -> Result<(), BuildError> {
        let mut build = Build::empty();
        let mut gears_map: HashMap<GearSlot, Gear> = HashMap::new();
        let default_gear = Gear {
          id: Id::from("gear_id"),
          name: String::from("gear name"),
          gear_type: GearType::Amulet,
          level: 200,
          effects: Effects::empty(),
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

        assert_eq!(gears, found_gears);
        Ok(())
    }

}