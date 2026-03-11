use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Hash,
    Clone,
    strum::EnumIter,
    strum::EnumString,
    strum::Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]

#[derive(Serialize, Deserialize)]
pub enum Skill {
    Fishing,
    WoodCutting,
    Cooking,
}
