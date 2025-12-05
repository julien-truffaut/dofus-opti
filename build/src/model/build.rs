use crate::model::{
    BuildError, Effects, Gear, GearSlot, GearSlotType, Language, MinRequirement,
    RequirementId, TranslatedName,
};
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

#[derive(Debug, PartialEq)]
pub struct Build<'a> {
    gear_slots: HashMap<GearSlot, &'a Gear>,
    pub effects: Effects,
}

impl<'a> Build<'a> {
    pub fn new() -> Self {
        Self {
            gear_slots: HashMap::new(),
            effects: Effects::empty(),
        }
    }

    pub fn get_gear(&self, gear_slot: &GearSlot) -> Option<&'a Gear> {
        self.gear_slots.get(gear_slot).map(|g| *g)
    }

    pub fn delete_gear(&mut self, gear_slot: &GearSlot) {
        if let Some(old_gear) = self.gear_slots.remove(gear_slot) {
            self.effects -= &old_gear.effects;
        }
    }

    pub fn set_gear(&mut self, gear_slot: GearSlot, gear: &'a Gear) -> Result<(), BuildError> {
        check_gear_slot(gear, &gear_slot)?;
        let gear_effects = gear.effects.clone();
        match self.gear_slots.entry(gear_slot) {
            Vacant(entry) => {
                self.effects += &gear_effects;
                entry.insert(gear);
            }
            Occupied(mut entry) => {
                let old_gear = entry.get();
                self.effects -= &old_gear.effects;
                self.effects += &gear_effects;
                entry.insert(gear);
            }
        }

        if (gear_slot == GearSlot::Ring1) || (gear_slot == GearSlot::Ring2) {
            check_not_duplicate_set_ring(
                self.get_gear(&GearSlot::Ring1),
                self.get_gear(&GearSlot::Ring2),
            )?;
        }

        Ok(())
    }

    pub fn satisfy_requirements(&self, requirements: &Vec<MinRequirement>) -> bool {
        requirements
            .iter()
            .all(|requirement| self.satisfy_requirement(requirement))
    }

    pub fn satisfy_requirement(&self, requirement: &MinRequirement) -> bool {
        match requirement.id {
            RequirementId::Strength => self.effects.derived_strength() >= requirement.desired_value,
            RequirementId::Vitality => {
                self.effects.vitality.unwrap_or(0) >= requirement.desired_value
            }
        }
    }

    pub fn summary(&self, language: Language) -> String {
        let text = format!(
            "Build {{
    {}: {},
    {}: {},
    {}: {},
    {}: {},
    {}: {},
    {}: {},
    {}: {},
    {}: {},
    {}: {}
}}",
            GearSlot::Amulet.localized(language),
            self.get_gear_name(&GearSlot::Amulet, language),
            GearSlot::Belt.localized(language),
            self.get_gear_name(&GearSlot::Belt, language),
            GearSlot::Boots.localized(language),
            self.get_gear_name(&GearSlot::Boots, language),
            GearSlot::Cloak.localized(language),
            self.get_gear_name(&GearSlot::Cloak, language),
            GearSlot::Ring1.localized(language),
            self.get_gear_name(&GearSlot::Ring1, language),
            GearSlot::Ring2.localized(language),
            self.get_gear_name(&GearSlot::Ring2, language),
            GearSlot::Shield.localized(language),
            self.get_gear_name(&GearSlot::Shield, language),
            GearSlot::Weapon.localized(language),
            self.get_gear_name(&GearSlot::Weapon, language),
            TranslatedName {
                en: "effects".to_string(),
                fr: "effets".to_string(),
            }
            .localized(language),
            pad_from_line2(self.effects.summary(language), "    "),
        );
        format!("{text}")
    }

    pub fn get_gear_name(&self, gear_slot: &GearSlot, language: Language) -> &str {
        self.get_gear(gear_slot).map(|g| g.name.localized(language)).unwrap_or("-")
    }
}

fn check_gear_slot(gear: &Gear, gear_slot: &GearSlot) -> Result<(), BuildError> {
    if GearSlotType::from(&gear.gear_type) == GearSlotType::from(gear_slot) {
        Ok(())
    } else {
        Err(BuildError::InvalidGearSlot(gear.id.clone(), gear_slot.to_owned()))
    }
}

pub fn check_not_duplicate_set_ring(
    ring1: Option<&Gear>,
    ring2: Option<&Gear>,
) -> Result<(), BuildError> {
    match (ring1, ring2) {
        (Some(ring1), Some(ring2)) => {
            if ring1.id == ring2.id && ring1.has_set {
                Err(BuildError::DuplicateRingsInASet(ring1.id.clone()))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

fn pad_from_line2(text: String, prefix: &str) -> String {
    text.lines()
        .enumerate()
        .map(|(i, line)| {
            if i == 0 {
                format!("{line}")
            } else {
                format!("{prefix}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ALL_GEAR_SLOTS, TranslatedName};
    use dofus_opti_core::{GearType, Id};

    #[test]
    fn set_delete_round_trip() -> Result<(), BuildError> {
        let mut build = Build::new();
        let amulet = Gear {
            id: Id::from("gear_id"),
            name: TranslatedName {
                en: String::from("gear name"),
                fr: String::from("nom d'objet"),
            },
            gear_type: GearType::Amulet,
            has_set: true,
            level: 200,
            effects: Effects::empty(),
        };
        build.set_gear(GearSlot::Amulet, &amulet)?;
        build.delete_gear(&GearSlot::Amulet);
        assert_eq!(build, Build::new());
        Ok(())
    }

    #[test]
    fn set_erase_existing_gear() -> Result<(), BuildError> {
        let mut build_1 = Build::new();
        let mut build_2 = Build::new();
        let mut effects_1 = Effects::empty();
        let mut effects_2 = Effects::empty();
        effects_1.agility = Some(24);
        effects_2.agility = Some(32);
        let amulet_1 = Gear {
            id: Id::from("gear_id"),
            name: TranslatedName {
                en: String::from("gear name"),
                fr: String::from("nom d'objet"),
            },
            gear_type: GearType::Amulet,
            has_set: true,
            level: 200,
            effects: effects_1,
        };
        let amulet_2 = Gear {
            effects: effects_2,
            ..amulet_1.clone()
        };
        build_1.set_gear(GearSlot::Amulet, &amulet_1)?;
        build_1.set_gear(GearSlot::Amulet, &amulet_2)?;
        build_2.set_gear(GearSlot::Amulet, &amulet_2)?;

        assert_eq!(build_1, build_2);
        Ok(())
    }

    #[test]
    fn set_get_gear_round_trip() -> Result<(), BuildError> {
        let mut build = Build::new();
        let mut gears_map: HashMap<GearSlot, Gear> = HashMap::new();
        let mut effects = Effects::empty();
        effects.strength = Some(1);
        let default_gear = Gear {
            id: Id::from("gear_id"),
            name: TranslatedName {
                en: String::from("gear name"),
                fr: String::from("nom d'objet"),
            },
            gear_type: GearType::Amulet,
            has_set: true,
            level: 200,
            effects: effects,
        };
        for gear_slot in ALL_GEAR_SLOTS {
            let mut gear = default_gear.clone();
            match gear_slot {
                GearSlot::Amulet => gear.gear_type = GearType::Amulet,
                GearSlot::Belt => gear.gear_type = GearType::Belt,
                GearSlot::Boots => gear.gear_type = GearType::Boots,
                GearSlot::Cloak => gear.gear_type = GearType::Cloak,
                GearSlot::Hat => gear.gear_type = GearType::Hat,
                GearSlot::Ring1 => gear.gear_type = GearType::Ring,
                GearSlot::Ring2 => {
                    gear.id = Id::from("another_gear_id");
                    gear.gear_type = GearType::Ring;
                }
                GearSlot::Shield => gear.gear_type = GearType::Shield,
                GearSlot::Weapon => gear.gear_type = GearType::Sword,
            }
            gears_map.entry(gear_slot.clone()).insert_entry(gear);
        }

        gears_map.iter().for_each(|(gear_slot, gear)| build.set_gear(*gear_slot, &gear).unwrap());

        let mut gears: Vec<Option<Gear>> = vec![];
        let mut found_gears: Vec<Option<Gear>> = vec![];
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
        let mut build = Build::new();
        let amulet = Gear {
            id: Id::from("gear_id"),
            name: TranslatedName {
                en: String::from("gear name"),
                fr: String::from("nom d'objet"),
            },
            gear_type: GearType::Amulet,
            has_set: true,
            level: 200,
            effects: Effects::empty(),
        };
        let result = build.set_gear(GearSlot::Belt, &amulet);
        assert_eq!(result, Err(BuildError::InvalidGearSlot(Id::from("gear_id"), GearSlot::Belt)));
    }
}
