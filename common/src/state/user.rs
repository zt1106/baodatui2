pub enum UserLocation {
    LOBBY,
    ROOM(u32),
}

pub struct UserState {
    pub id: String,
    pub nick_name: String,
    pub location: UserLocation,
}
