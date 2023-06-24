use std::collections::{HashMap, HashSet, VecDeque};

use baodatui_common::{card::Suit, model::config::GameConfig};
#[derive(Default)]
pub struct GameState {
    game_config: GameConfig,
    cur_phase: GamePhase,
    prime_suit: Option<Suit>,
    host_player_idx: Option<usize>,
    recruit_count_map: HashMap<u32, u32>,
    /// players
    players: Vec<PlayerState>,
    /// cards
    draw_deque: VecDeque<u32>,
    return_set: HashSet<u32>,
    remainder_set: HashSet<u32>,
    temp_set: HashSet<u32>,
    // prev game info (optional)
}
#[derive(Default)]
pub struct PlayerState {
    /// player's cards
    hand_set: HashSet<u32>,
    /// player's current play, toss or challenge
    cur_action_set: HashSet<u32>,
    discard_set: HashSet<u32>,
    score_set: HashSet<u32>,
    // prev game info (optional)
}

impl GameState {
    async fn start_with_request_handler(self) {
        todo!("handle request and emit state")
    }

    async fn start_with_mdp_agent(self) {
        todo!("interact with an agent, with other auto agents")
    }
}

// TODO implement a random agent, all actions are random

#[derive(Default)]
pub enum GamePhase {
    #[default]
    Initialize,
}
