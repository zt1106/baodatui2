use std::sync::OnceLock;

use crate::model::game::Game;

use super::arcmap::ArcMap;

#[allow(dead_code)]
static GAME_MANAGER: OnceLock<ArcMap<Game>> = OnceLock::new();

pub fn game_maanger() -> &'static ArcMap<Game> {
    GAME_MANAGER.get_or_init(|| ArcMap::default())
}
