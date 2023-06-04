use baodatui_common::{model::config::GameConfig, phase::GamePhase};

use super::user::User;

pub struct Game {
    pub id: u32,
    pub game_config: GameConfig,
    pub cur_phase: GamePhase,
    pub users: Vec<User>,
}
