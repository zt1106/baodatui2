use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug)]
pub struct RoomState {
    pub id: u64,
}
