use core::fmt;
use std::{
    ops::{Add, AddAssign},
    sync::LazyLock,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LevelData {
    pub level: Level,
    pub xp: Xp,
}

impl Default for LevelData {
    fn default() -> Self {
        Self {
            level: Level(1),
            xp: Xp(0),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Xp(pub u32);
impl AddAssign for Xp {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Level(pub u8);
impl AddAssign for Level {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}
impl Add<u8> for Level {
    type Output = Level;

    fn add(self, rhs: u8) -> Self::Output {
        Level(self.0 + rhs)
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Level:{}", self.0)
    }
}

pub fn xp_for_level(level: Level) -> Xp {
    let result = (1..level.0)
        .map(|i| i as u64 + (300.0 * 2.0_f64.powf(i as f64 / 7.0)) as u64)
        .sum::<u64>()
        / 4;

    Xp(result as u32)
}

static THRESHOLDS: LazyLock<Vec<Xp>> =
    LazyLock::new(|| (1..=99).map(|l| xp_for_level(Level(l))).collect());

pub fn calculate_level(xp: Xp) -> Level {
    let (level_idx, _) = THRESHOLDS
        .iter()
        .enumerate()
        .rev()
        .find(|(_, threshold)| xp.0 >= threshold.0)
        .expect("Level out of bounds");
    Level(level_idx as u8 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xp_for_level_2() {
        assert_eq!(xp_for_level(Level(2)).0, 83);
    }

    #[test]
    fn xp_for_level_10() {
        assert_eq!(xp_for_level(Level(10)).0, 1154);
    }

    #[test]
    fn xp_for_level_99() {
        assert_eq!(xp_for_level(Level(99)).0, 13_034_431);
    }

    #[test]
    fn leve_2_for_83_xp() {
        assert_eq!(calculate_level(Xp(83)), Level(2));
    }
    #[test]
    fn leve_10_for_1154_xp() {
        assert_eq!(calculate_level(Xp(1154)), Level(10));
    }
    #[test]
    fn leve_99_for_13_034_431_xp() {
        assert_eq!(calculate_level(Xp(13_034_431)), Level(99));
    }
}
