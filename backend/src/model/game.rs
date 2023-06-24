use std::sync::Arc;

use baodatui_common::model::config::GameConfig;
use baodatui_macros::ID;
use tokio::sync::Mutex;

use super::user::User;

/// core game logic
#[derive(ID, Default)]
pub struct Game {
    pub id: u32,
    /// a game can be destroyed at any time
    pub destroyed: bool,
    pub game_config: GameConfig,
    pub users: Vec<Arc<Mutex<User>>>,
}
