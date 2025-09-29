use std::fmt;

#[derive(Debug)]
pub struct Gear {
  pub name: String,  
  pub gear_type: GearType,
  pub level: u32,
  pub characteristics: Vec<CharacteristicRange>
}

#[derive(Debug)]
pub enum GearType {
    Amulet,
    Hat,
    Ring,
    Shield
}


impl fmt::Display for GearType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GearType::Amulet => write!(f, "Amulet"),
            GearType::Hat => write!(f, "Hat"),
            GearType::Ring => write!(f, "Ring"),
            GearType::Shield => write!(f, "Shield"),
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