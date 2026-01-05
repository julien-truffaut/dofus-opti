use crate::model::{Effects, MinRequirement, RequirementId};

pub fn default_score(requirements: &Vec<MinRequirement>, effects: &Effects) -> i32 {
    requirements
        .iter()
        .map(|req| match req.id {
            RequirementId::Strength => effects.derived_strength(),
            RequirementId::Vitality => effects.vitality() / 5,
        })
        .sum()
}
