use std::time::Duration;

use crate::domain::{level_data::Xp, skill_method::SkillMethodData};

#[derive(Debug, Clone, strum::EnumIter, strum::Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum CookingMethod {
    Shrimp,
    Anchovies,
}

impl SkillMethodData for CookingMethod {
    fn xp_award_duration(&self) -> Duration {
        match self {
            CookingMethod::Shrimp => Duration::from_secs(5),
            CookingMethod::Anchovies => todo!(),
        }
    }

    fn xp_award_amount(&self) -> Xp {
        match self {
            CookingMethod::Shrimp => Xp(11),
            CookingMethod::Anchovies => todo!(),
        }
    }
}
