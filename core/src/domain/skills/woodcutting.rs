use std::time::Duration;

use crate::domain::{level_data::Xp, skill_method::SkillMethodData};

#[derive(Debug, Clone, strum::EnumIter, strum::Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum WoodCuttingMethod {
    Tree,
    OakTree,
}

impl SkillMethodData for WoodCuttingMethod {
    fn xp_award_duration(&self) -> Duration {
        match self {
            WoodCuttingMethod::Tree => todo!(),
            WoodCuttingMethod::OakTree => todo!(),
        }
    }

    fn xp_award_amount(&self) -> Xp {
        match self {
            WoodCuttingMethod::Tree => todo!(),
            WoodCuttingMethod::OakTree => todo!(),
        }
    }
    
    fn level_needed(&self) -> crate::domain::level_data::Level {
        todo!()
    }
}
