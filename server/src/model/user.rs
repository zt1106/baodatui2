use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

pub struct User {
    pub id: u32,
    pub nick_name: String,
}

#[derive(Default)]
pub struct UserManager {
    pub cur_id: u32,
    pub user_map: HashMap<u32, Arc<Mutex<User>>>,
}

impl UserManager {
    pub fn by_id(&self, id: u32) -> Option<Arc<Mutex<User>>> {
        let user_opt = &self.user_map.get(&id);
        return match user_opt {
            Some(user) => Some((*user).clone()),
            None => None,
        };
    }

    pub async fn remove_by_id(&mut self, id: u32) {
        let _ = self.user_map.remove(&id);
    }

    pub fn add(&mut self, nick_name: String) -> User {
        self.cur_id += 1;
        User {
            id: self.cur_id,
            nick_name,
        }
    }
}
