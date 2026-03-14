use std::time::Duration;

use crate::domain::{
    level_data::{Level, Xp},
    skill_method::SkillMethodData,
};

#[derive(Debug, Clone, strum::EnumIter, strum::Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum FishingMethod {
    Shrimp,
    Anchovies,
    TestMethod,
}

impl SkillMethodData for FishingMethod {
    fn level_needed(&self) -> Level {
        match self {
            FishingMethod::Shrimp => Level(1),
            FishingMethod::Anchovies => Level(5),
            FishingMethod::TestMethod => Level(1),
        }
    }

    fn xp_award_duration(&self) -> Duration {
        match self {
            FishingMethod::Shrimp => Duration::from_secs(5),
            FishingMethod::Anchovies => Duration::from_secs(2),
            FishingMethod::TestMethod => Duration::from_secs(10),
        }
    }

    fn xp_award_amount(&self) -> Xp {
        match self {
            FishingMethod::Shrimp => Xp(18),
            FishingMethod::Anchovies => Xp(56),
            FishingMethod::TestMethod => Xp(1000),
        }
    }
}
