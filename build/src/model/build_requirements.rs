pub struct BuildRequirements {
    pub requirements: Vec<Requirement>,
}

pub struct Requirement {
    pub id: RequirementId,
    pub desired_value: i32,
}

pub enum RequirementId {
    Strength,
    Vitality,
}
