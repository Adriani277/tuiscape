use std::{collections::HashMap, time::Duration};

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    domain::{
        level_data::{calculate_level, LevelData},
        skill_method::{SkillMethod, SkillMethodData},
        skills::skill_type::Skill,
    },
    storage,
};

#[derive(Serialize, Deserialize)]
pub struct Player {
    levels: HashMap<Skill, LevelData>,
}

impl Player {
    pub fn level_data(&self, skill: &Skill) -> &LevelData {
        self.levels
            .get(skill)
            .expect("all Skill variants are inserted in Player::default()")
    }

    pub fn level_data_mut(&mut self, skill: &Skill) -> &mut LevelData {
        self.levels
            .get_mut(skill)
            .expect("all Skill variants are inserted in Player::default()")
    }

    pub fn can_use_method(&self, method: &SkillMethod) -> bool {
        self.level_data(&method.skill_type()).level >= method.level_needed()
    }

    pub fn skill_tick(
        &mut self,
        method: &SkillMethod,
        tick_delta: Duration,
        skill_progress: Duration,
    ) -> Duration {
        let require_level = method.level_needed();
        if self.level_data(&method.skill_type()).level < require_level {
            skill_progress
        } else {
            let method_duration = method.xp_award_duration();
            let skill = method.skill_type();
            let data = self.level_data_mut(&skill);
            let mut accum = skill_progress + tick_delta;

            while accum >= method_duration {
                accum -= method_duration;
                data.xp += method.xp_award_amount();
                data.level = calculate_level(data.xp);
            }

            accum
        }
    }

    pub fn init() -> Player {
        storage::fetch_player_data()
    }
}

impl Default for Player {
    fn default() -> Self {
        let mut levels_map = HashMap::new();
        for k in Skill::iter() {
            levels_map.insert(k, LevelData::default());
        }
        Self { levels: levels_map }
    }
}
