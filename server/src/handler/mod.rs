pub mod user_handler;

use std::{
    collections::HashMap,
    future::Future,
    marker::Send,
    sync::{Arc, OnceLock},
};

use baodatui_common::{ext::ToMyResultExt, MyResult};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use tokio::{
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedSender},
        oneshot::{channel, Sender},
        Mutex,
    },
};

#[allow(dead_code)]
static GLOBAL_HANDLER_MAP: OnceLock<GlobalHandlerMap> = OnceLock::new();

pub fn global_handler_map() -> &'static GlobalHandlerMap {
    GLOBAL_HANDLER_MAP.get_or_init(|| GlobalHandlerMap::default())
}

type ReqValueAndId = (Value, u64);

#[derive(Default)]
pub struct GlobalHandlerMap {
    req_id: Arc<Mutex<u64>>,
    req_sender_map: Arc<Mutex<HashMap<&'static str, UnboundedSender<ReqValueAndId>>>>,
    unfinished_resp_sender_map: Arc<Mutex<HashMap<u64, Sender<MyResult<Value>>>>>,
}

impl GlobalHandlerMap {
    pub async fn handle_request(&self, value: Value, command: &'static str) -> MyResult<Value> {
        let req_sender_map = self.req_sender_map.lock().await;
        let req_sender = req_sender_map.get(command).to_my_result()?;
        let (result_send, result_recv) = channel::<MyResult<Value>>();
        let mut id_guard = self.req_id.lock().await;
        let id = *id_guard;
        *id_guard += 1;
        self.unfinished_resp_sender_map
            .lock()
            .await
            .insert(id, result_send);
        let _ = req_sender.send((value, id));
        match result_recv.await {
            Ok(r) => r,
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn register_handler<Fut, REQ, RESP>(
        &self,
        command: &'static str,
        mut handle_func: impl FnMut(REQ) -> Fut + Send + 'static,
    ) where
        REQ: DeserializeOwned + Send,
        RESP: Serialize + Send,
        Fut: Future<Output = MyResult<RESP>> + Send,
    {
        let mut req_sender_map = self.req_sender_map.lock().await;
        let (req_send, mut req_recv) = unbounded_channel::<ReqValueAndId>();
        req_sender_map.insert(command, req_send);
        let unfinished_resp_sender_map = self.unfinished_resp_sender_map.clone();
        spawn(async move {
            while let Some((req_value, id)) = req_recv.recv().await {
                let result = match serde_json::from_value::<REQ>(req_value).to_my_result() {
                    Ok(req) => {
                        let resp_result = handle_func(req).await;
                        resp_result.and_then(|resp| serde_json::to_value(resp).to_my_result())
                    }
                    Err(err) => Err(err),
                };
                if let Some(result_value_send) = unfinished_resp_sender_map.lock().await.remove(&id)
                {
                    let _ = result_value_send.send(result);
                }
            }
        });
    }
}
