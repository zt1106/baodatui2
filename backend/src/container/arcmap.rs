use std::{collections::HashMap, sync::Arc};

use baodatui_common::ext::IntoArcTMutex;
use tokio::sync::Mutex;

// TODO should implement this by macro
pub trait WithId {
    fn set_id(&mut self, id: u32);
    fn id(&self) -> u32;
}

/// both map itself and values are wrapped in Arc + Mutex
#[derive(Default)]
pub struct ArcMap<T: WithId> {
    cur_id: Arc<Mutex<u32>>,
    inner_map: Arc<Mutex<HashMap<u32, Arc<Mutex<T>>>>>,
}

impl<T: WithId> ArcMap<T> {
    pub async fn add(&self, mut t: T) -> Arc<Mutex<T>> {
        let mut map = self.inner_map.lock().await;
        let mut cur_id = self.cur_id.lock().await;
        t.set_id(*cur_id);
        let arc = t.to_arc_t_mutex();
        let arc_cloned = arc.clone();
        map.insert(*cur_id, arc);
        *cur_id += 1;
        arc_cloned
    }

    pub async fn remove_id(&self, id: u32) {
        let mut map = self.inner_map.lock().await;
        map.remove(&id);
    }

    pub async fn remove(&self, t: Arc<Mutex<T>>) {
        let id = t.lock().await.id();
        self.remove_id(id).await
    }

    pub async fn get(&self, id: u32) -> Option<Arc<Mutex<T>>> {
        let map = self.inner_map.lock().await;
        map.get(&id).and_then(|t| Some(t.clone()))
    }

    pub async fn is_valid_id(&self, id: u32) -> bool {
        self.inner_map.lock().await.contains_key(&id)
    }
}
