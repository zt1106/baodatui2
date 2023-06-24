use async_trait::async_trait;

use super::game::Game;

use std::sync::OnceLock;

pub fn phases() -> &'static GamePhases {
    static PHASES: OnceLock<GamePhases> = OnceLock::new();
    PHASES.get_or_init(|| GamePhases { phases: vec![] })
}

#[async_trait]
pub trait GamePhase {
    async fn run(&self, game: &mut Game);
}

pub struct GamePhases {
    pub phases: Vec<Box<dyn GamePhase + Send + Sync>>,
}
