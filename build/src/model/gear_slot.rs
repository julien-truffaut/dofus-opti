use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
            GearSlot::Belt   => write!(f, "Belt"),
            GearSlot::Boots  => write!(f, "Boots"),
            GearSlot::Cloak  => write!(f, "Cloak"),
            GearSlot::Hat    => write!(f, "Hat"),
            GearSlot::Ring1  => write!(f, "Ring 1"),
            GearSlot::Ring2  => write!(f, "Ring 2"),
            GearSlot::Shield => write!(f, "Shield"),
            GearSlot::Weapon => write!(f, "Weapon"),
        }
    }
}