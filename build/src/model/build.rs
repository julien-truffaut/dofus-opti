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

    pub fn set_gear(&mut self, gear: &'a Gear, gear_slot: &GearSlot) -> Result<(), BuildError> {
        check_gear_slot(gear, gear_slot)?;
        let gear_effects = gear.effects.clone();
        match gear_slot {
            GearSlot::Amulet => match self.amulet {
                None               => {
                    self.effects += gear_effects;
                    self.amulet  = Some(gear);   
                },
                Some(current_gear) => {
                    self.effects -= current_gear.effects.clone();
                    self.effects += gear_effects;
                }
            },
            GearSlot::Belt => (),
            GearSlot::Boots => (),
            GearSlot::Cloak => (),
            GearSlot::Hat => (),
            GearSlot::Ring1 => (),
            GearSlot::Ring2 => (),
            GearSlot::Shield => (),
            GearSlot::Weapon => (),
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