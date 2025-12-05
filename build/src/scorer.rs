use crate::model::{MinRequirement, Effects, RequirementId};

pub fn default_score(requirements: &Vec<MinRequirement>, effects: &Effects) -> i32 {
    requirements
        .iter()
        .map(|req| match req.id {
            RequirementId::Strength => effects.derived_strength(),
            RequirementId::Vitality => effects.vitality.unwrap_or(0) / 5,
        })
        .sum()
}
