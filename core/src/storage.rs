use std::fs;

use crate::player::Player;

static SAVE_PATH: &str = "./save.json";

pub fn store_player_data(player: &Player) {
    let data = serde_json::to_string(player).expect("Error serializing player data");
    fs::write(SAVE_PATH, data).expect("Error writing to file");
}

pub fn fetch_player_data() -> Player {
    let save_contents = fs::read_to_string(SAVE_PATH);
    match save_contents {
        Ok(save) => serde_json::from_str(&save).unwrap_or(Player::default()),
        Err(_) => Player::default(),
    }
}
