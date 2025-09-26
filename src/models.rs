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