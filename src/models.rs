use std::fmt;

#[derive(Debug)]
pub struct Gear {
  pub name: String,  
  pub gear_type: GearType,
  pub level: u32,
  pub characteristics: Vec<CharacteristicRange>
}

#[derive(Debug, Clone)]
pub enum GearType {
    Amulet,
    Axe,
    Belt,
    Boots,
    Bow,
    Cloak,
    Dagger,
    Hat,
    Lance,
    Hammer,
    Ring,
    Scythe,
    Shield,
    Shovel,
    Staff,
    Sword,
    Wand
}

pub static ALL_GEAR_TYPES: &[GearType] = &[
    GearType::Amulet,
    GearType::Axe,
    GearType::Belt,
    GearType::Boots,
    GearType::Bow,
    GearType::Cloak,
    GearType::Dagger,
    GearType::Hammer,
    GearType::Hat,
    GearType::Lance,
    GearType::Ring,
    GearType::Scythe,
    GearType::Shield,
    GearType::Shovel,
    GearType::Staff,
    GearType::Sword,
    GearType::Wand,
];


impl fmt::Display for GearType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GearType::Amulet => write!(f, "Amulet"),
            GearType::Axe    => write!(f, "Axe"),
            GearType::Belt   => write!(f, "Belt"),
            GearType::Boots  => write!(f, "Boots"),
            GearType::Bow    => write!(f, "Bow"),
            GearType::Cloak  => write!(f, "Cloak"),
            GearType::Dagger => write!(f, "Dagger"),
            GearType::Hat    => write!(f, "Hat"),
            GearType::Hammer => write!(f, "Hammer"),
            GearType::Lance  => write!(f, "Lance"),
            GearType::Ring   => write!(f, "Ring"),
            GearType::Scythe => write!(f, "Scythe"),
            GearType::Shield => write!(f, "Shield"),
            GearType::Shovel => write!(f, "Shovel"),
            GearType::Staff  => write!(f, "Staff"),
            GearType::Sword  => write!(f, "Sword"),
            GearType::Wand   => write!(f, "Wand"),
        }
    }
}

#[derive(Debug)]
pub struct CharacteristicRange {
    pub kind: CharacteristicType,
    pub min: i32,
    pub max: i32
}

#[derive(Debug)]
pub enum CharacteristicType {
    Vitality,
    Power,
}