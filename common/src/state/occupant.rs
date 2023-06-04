use super::user::UserState;

pub struct OccupantState {
    pub user: UserState,
    pub prepared: bool,
    pub last_game_cache: Option<LastGameCache>,
    pub history_statistics: HistoryStatistics,
}

pub struct LastGameCache {
    pub owe_tribute_count: u32,
    pub deserve_tribute_count: u32,
    pub is_first_draw_in_last_game: bool,
}

pub struct HistoryStatistics {
    pub total_personal_pure_score: u32,
    pub total_personal_final_round_score: u32,
    pub total_pay_tribute_count: u32,
    pub total_receive_tribute_count: u32,
    pub total_win_count: u32,
    pub total_lose_count: u32,
}
