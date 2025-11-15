use crate::model::{BuildRequirements, Effects, RequirementId};

pub fn default_score(build_requirements: &BuildRequirements, effects: &Effects) -> i32 {
    build_requirements
        .requirements
        .iter()
        .map(|req| match req.id {
            RequirementId::Strength => effects.derived_strength(),
            RequirementId::Vitality => effects.vitality.unwrap_or(0) / 5,
        })
        .sum()
}
