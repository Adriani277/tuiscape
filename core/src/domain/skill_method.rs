use std::fmt;
use std::time::Duration;

use strum::IntoEnumIterator;

use crate::domain::{
    level_data::{Level, Xp},
    skills::{
        cooking::CookingMethod, fishing::FishingMethod, skill_type::Skill,
        woodcutting::WoodCuttingMethod,
    },
};
use rand::prelude::*;

pub enum DurationType {
    Static(Duration),
    Dynamic { min: Duration, max: Duration },
}

impl DurationType {
    pub fn resolve(&self) -> Duration {
        match self {
            DurationType::Static(duration) => *duration,
            DurationType::Dynamic { min, max } => {
                let mut rng = rand::rng();
                let secs = rng.random_range(min.as_secs()..=max.as_secs());
                Duration::from_secs(secs)
            }
        }
    }
}

pub trait SkillMethodData {
    fn level_needed(&self) -> Level;
    fn xp_award_duration(&self) -> DurationType;
    fn xp_award_amount(&self) -> Xp;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SkillMethod {
    Fishing(FishingMethod),
    WoodCutting(WoodCuttingMethod),
    Cooking(CookingMethod),
}

impl SkillMethod {
    pub fn skill_type(&self) -> Skill {
        match self {
            SkillMethod::Fishing(_) => Skill::Fishing,
            SkillMethod::WoodCutting(_) => Skill::WoodCutting,
            SkillMethod::Cooking(_) => Skill::Cooking,
        }
    }
}

impl fmt::Display for SkillMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkillMethod::Fishing(m) => write!(f, "{}", m),
            SkillMethod::WoodCutting(m) => write!(f, "{}", m),
            SkillMethod::Cooking(m) => write!(f, "{}", m),
        }
    }
}

impl SkillMethodData for SkillMethod {
    fn xp_award_duration(&self) -> DurationType {
        match self {
            SkillMethod::Fishing(m) => m.xp_award_duration(),
            SkillMethod::WoodCutting(m) => m.xp_award_duration(),
            SkillMethod::Cooking(m) => m.xp_award_duration(),
        }
    }

    fn xp_award_amount(&self) -> Xp {
        match self {
            SkillMethod::Fishing(m) => m.xp_award_amount(),
            SkillMethod::WoodCutting(m) => m.xp_award_amount(),
            SkillMethod::Cooking(m) => m.xp_award_amount(),
        }
    }

    fn level_needed(&self) -> Level {
        match self {
            SkillMethod::Fishing(fishing_method) => fishing_method.level_needed(),
            SkillMethod::WoodCutting(wood_cutting_method) => todo!(),
            SkillMethod::Cooking(cooking_method) => todo!(),
        }
    }
}

impl Skill {
    pub fn methods(&self) -> Vec<SkillMethod> {
        match self {
            Skill::Fishing => FishingMethod::iter().map(SkillMethod::Fishing).collect(),
            Skill::WoodCutting => WoodCuttingMethod::iter()
                .map(SkillMethod::WoodCutting)
                .collect(),
            Skill::Cooking => CookingMethod::iter().map(SkillMethod::Cooking).collect(),
        }
    }
}
