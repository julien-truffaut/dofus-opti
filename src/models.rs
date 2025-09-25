#[derive(Debug)]
pub struct Gear {
  pub name: String,  
  pub object_type: ObjectType,
  pub level: u32,
  pub characteristics: Vec<CharacteristicRange>
}

#[derive(Debug)]
pub enum ObjectType {
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