use std::str::FromStr;


#[derive(Debug, PartialEq, Clone)]
pub struct MinRequirement {
    pub id: RequirementId,
    pub desired_value: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RequirementId {
    Strength,
    Vitality,
}

impl FromStr for MinRequirement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(format!("Invalid requirement format: {}", s));
        }
        let id: RequirementId = parts[0].parse()?;
        if parts[1] != ">=" {
            return Err(format!("Operator not supported: {}", parts[1]));
        }
        let desired_value: i32 = parts[2].parse().map_err(|_| "Invalid number".to_string())?;
        Ok(MinRequirement {
            id: id,
            desired_value: desired_value,
        })
    }
}

impl FromStr for RequirementId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "strength" => Ok(RequirementId::Strength),
            "vitality" => Ok(RequirementId::Vitality),
            _ => Err(format!("Unknown requirement id: {}", s)),
        }
    }
}
