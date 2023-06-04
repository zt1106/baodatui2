use std::{collections::HashMap, sync::Arc};

use baodatui_common::{ext::IntoArcTMutex, model::config::GameConfig};
use tokio::{spawn, sync::Mutex};

use super::user::User;

pub struct Game {
    pub id: u32,
    /// a game can be destroyed at any time
    pub destroyed: bool,
    pub game_config: GameConfig,
    pub users: Vec<Arc<Mutex<User>>>,
}

#[derive(Default)]
pub struct GameManager {
    pub game_map: Arc<Mutex<HashMap<u32, Arc<Mutex<Game>>>>>,
}

impl GameManager {
    async fn run(&mut self, game: Game) {
        let id: u32 = game.id;
        let game_arc = game.to_arc_t_mutex();
        let game_arc_cloned = game_arc.clone();
        self.game_map.lock().await.insert(id, game_arc);
        let game_map = self.game_map.clone();
        spawn(async move { while !game_arc_cloned.lock().await.destroyed {} });
    }
}
