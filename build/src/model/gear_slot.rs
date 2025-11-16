use std::fmt;

use crate::model::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GearSlot {
    Amulet,
    Belt,
    Boots,
    Cloak,
    Hat,
    Ring1,
    Ring2,
    Shield,
    Weapon,
}

impl GearSlot {
    pub fn localized(&self, language: Language) -> &str {
        match self {
            GearSlot::Amulet => match language {
                Language::English => "Amulet",
                Language::French => "Amulette",
            },
            GearSlot::Belt => match language {
                Language::English => "Belt",
                Language::French => "Ceinture",
            },
            GearSlot::Boots => match language {
                Language::English => "Boots",
                Language::French => "Bottes",
            },
            GearSlot::Cloak => match language {
                Language::English => "Cloack",
                Language::French => "Cape",
            },
            GearSlot::Hat => match language {
                Language::English => "Hat",
                Language::French => "Chapeau",
            },
            GearSlot::Ring1 => match language {
                Language::English => "Ring 1",
                Language::French => "Anneau 1",
            },
            GearSlot::Ring2 => match language {
                Language::English => "Ring 2",
                Language::French => "Anneau 2",
            },
            GearSlot::Shield => match language {
                Language::English => "Shield",
                Language::French => "Bouclier",
            },
            GearSlot::Weapon => match language {
                Language::English => "Weapon",
                Language::French => "Arme",
            },
        }
    }
}

pub static ALL_GEAR_SLOTS: &[GearSlot] = &[
    GearSlot::Amulet,
    GearSlot::Belt,
    GearSlot::Boots,
    GearSlot::Cloak,
    GearSlot::Hat,
    GearSlot::Ring1,
    GearSlot::Ring2,
    GearSlot::Shield,
    GearSlot::Weapon,
];

impl fmt::Display for GearSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GearSlot::Amulet => write!(f, "Amulet"),
            GearSlot::Belt => write!(f, "Belt"),
            GearSlot::Boots => write!(f, "Boots"),
            GearSlot::Cloak => write!(f, "Cloak"),
            GearSlot::Hat => write!(f, "Hat"),
            GearSlot::Ring1 => write!(f, "Ring 1"),
            GearSlot::Ring2 => write!(f, "Ring 2"),
            GearSlot::Shield => write!(f, "Shield"),
            GearSlot::Weapon => write!(f, "Weapon"),
        }
    }
}
