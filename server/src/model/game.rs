use std::{collections::HashMap, sync::Arc};

use async_scoped::TokioScope;
use baodatui_common::{ext::IntoArcTMutex, model::config::GameConfig};
use tokio::{spawn, sync::Mutex};

use super::user::User;

use std::sync::OnceLock;

#[allow(dead_code)]
static GAME_MANAGER: OnceLock<GameManager> = OnceLock::new();

pub fn game_manager() -> &'static GameManager {
    GAME_MANAGER.get_or_init(|| GameManager::default())
}

pub struct Game {
    pub id: u32,
    /// a game can be destroyed at any time
    pub destroyed: bool,
    pub game_config: GameConfig,
    pub users: Vec<Arc<Mutex<User>>>,
}

#[derive(Default)]
pub struct GameManager {
    pub game_map: HashMap<u32, Game>,
}

impl GameManager {
    async fn run(&mut self, game: Game) {
        let id: u32 = game.id;
        TokioScope::scope_and_block(|s| {})
    }
}
