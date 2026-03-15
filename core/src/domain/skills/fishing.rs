use std::time::Duration;

use crate::domain::{
    level_data::{Level, Xp},
    skill_method::{DurationType, SkillMethod, SkillMethodData},
};

#[derive(Debug, Clone, strum::EnumIter, strum::Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum FishingMethod {
    Shrimp,
    Sardine,
    Herring,
    Anchovies,
    Trout,
    Pike,
    Salmon,
    Tuna,
    Lobster,
    Swordfish,
    Monkfish,
    Shark,
    Anglerfish,
    DarkCrab,
}

impl SkillMethodData for FishingMethod {
    fn level_needed(&self) -> Level {
        match self {
            FishingMethod::Shrimp => Level(1),
            FishingMethod::Sardine => Level(5),
            FishingMethod::Herring => Level(10),
            FishingMethod::Anchovies => Level(15),
            FishingMethod::Trout => Level(20),
            FishingMethod::Pike => Level(25),
            FishingMethod::Salmon => Level(30),
            FishingMethod::Tuna => Level(35),
            FishingMethod::Lobster => Level(40),
            FishingMethod::Swordfish => Level(50),
            FishingMethod::Monkfish => Level(62),
            FishingMethod::Shark => Level(76),
            FishingMethod::Anglerfish => Level(82),
            FishingMethod::DarkCrab => Level(85),
        }
    }

    fn xp_award_duration(&self) -> DurationType {
        match self {
            FishingMethod::Shrimp => DurationType::Dynamic {
                min: Duration::from_secs(4),
                max: Duration::from_secs(8),
            },
            FishingMethod::Sardine => DurationType::Dynamic {
                min: Duration::from_secs(4),
                max: Duration::from_secs(8),
            },
            FishingMethod::Herring => DurationType::Dynamic {
                min: Duration::from_secs(4),
                max: Duration::from_secs(8),
            },
            FishingMethod::Anchovies => DurationType::Dynamic {
                min: Duration::from_secs(3),
                max: Duration::from_secs(8),
            },
            FishingMethod::Trout => DurationType::Dynamic {
                min: Duration::from_secs(4),
                max: Duration::from_secs(10),
            },
            FishingMethod::Pike => DurationType::Dynamic {
                min: Duration::from_secs(3),
                max: Duration::from_secs(10),
            },
            FishingMethod::Salmon => DurationType::Dynamic {
                min: Duration::from_secs(4),
                max: Duration::from_secs(10),
            },
            FishingMethod::Tuna => DurationType::Dynamic {
                min: Duration::from_secs(4),
                max: Duration::from_secs(11),
            },
            FishingMethod::Lobster => DurationType::Dynamic {
                min: Duration::from_secs(4),
                max: Duration::from_secs(11),
            },
            FishingMethod::Swordfish => DurationType::Dynamic {
                min: Duration::from_secs(5),
                max: Duration::from_secs(12),
            },
            FishingMethod::Monkfish => DurationType::Dynamic {
                min: Duration::from_secs(6),
                max: Duration::from_secs(14),
            },
            FishingMethod::Shark => DurationType::Dynamic {
                min: Duration::from_secs(7),
                max: Duration::from_secs(15),
            },
            FishingMethod::Anglerfish => DurationType::Dynamic {
                min: Duration::from_secs(8),
                max: Duration::from_secs(16),
            },
            FishingMethod::DarkCrab => DurationType::Dynamic {
                min: Duration::from_secs(8),
                max: Duration::from_secs(15),
            },
        }
    }

    fn xp_award_amount(&self) -> Xp {
        match self {
            FishingMethod::Shrimp => Xp(5),      // Melvor: 5
            FishingMethod::Sardine => Xp(10),     // Melvor: 10
            FishingMethod::Herring => Xp(15),     // Melvor: 15
            FishingMethod::Anchovies => Xp(18),   // interpolated
            FishingMethod::Trout => Xp(20),       // Melvor: 20
            FishingMethod::Pike => Xp(30),        // interpolated
            FishingMethod::Salmon => Xp(40),      // Melvor: 40
            FishingMethod::Tuna => Xp(45),        // interpolated
            FishingMethod::Lobster => Xp(50),     // Melvor: 50
            FishingMethod::Swordfish => Xp(80),   // Melvor: 80
            FishingMethod::Monkfish => Xp(100),   // interpolated
            FishingMethod::Shark => Xp(120),      // Melvor: 150, adjusted down (lower level than Anglerfish)
            FishingMethod::Anglerfish => Xp(150), // highest reward, reflects OSRS high value
            FishingMethod::DarkCrab => Xp(130),   // interpolated
        }
    }
}
