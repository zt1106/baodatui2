use std::{collections::HashMap, sync::Arc};

use baodatui_common::ext::IntoArcTMutex;
use tokio::sync::Mutex;

/// both map itself and values are wrapped in Arc + Mutex
pub struct ArcMap<T> {
    inner_map: Arc<Mutex<HashMap<u32, Arc<Mutex<T>>>>>,
}

impl<T> ArcMap<T> {
    pub async fn add(&self, id: u32, t: T) -> Arc<Mutex<T>> {
        let mut map = self.inner_map.lock().await;
        let arc = t.to_arc_t_mutex();
        let arc_cloned = arc.clone();
        map.insert(id, arc);
        arc_cloned
    }

    pub async fn remove(&self, id: u32) {
        let mut map = self.inner_map.lock().await;
        map.remove(&id);
    }

    pub async fn get(&self, id: u32) -> Option<Arc<Mutex<T>>> {
        let map = self.inner_map.lock().await;
        map.get(&id).and_then(|t| Some(t.clone()))
    }

    pub async fn is_valid(&self, id: u32) -> bool {
        self.inner_map.lock().await.contains_key(&id)
    }
}
