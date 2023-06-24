use serde::{Deserialize, Serialize};

use super::room::RoomState;
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LobbyState {
    pub room_states: Vec<RoomState>,
}
