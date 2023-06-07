use std::sync::Arc;

use tokio::sync::{oneshot::error::RecvError, Mutex};

use crate::MyResult;

pub trait IntoBoxExt<T> {
    fn to_box(self) -> Box<T>;
}

impl<T> IntoBoxExt<T> for T {
    fn to_box(self) -> Box<T> {
        Box::new(self)
    }
}

pub trait IntoArcTMutex<T> {
    fn to_arc_t_mutex(self) -> Arc<Mutex<T>>;
}

impl<T> IntoArcTMutex<T> for T {
    fn to_arc_t_mutex(self) -> Arc<Mutex<T>> {
        Arc::new(Mutex::new(self))
    }
}

pub trait ToMyResultExt<T> {
    fn to_my_result(self) -> MyResult<T>;
}

impl<T> ToMyResultExt<T> for Option<T> {
    fn to_my_result(self) -> MyResult<T> {
        match self {
            Some(t) => Ok(t),
            None => Err("opt is null".to_string()),
        }
    }
}

impl<T> ToMyResultExt<T> for Result<T, serde_json::error::Error> {
    fn to_my_result(self) -> MyResult<T> {
        match self {
            Ok(t) => Ok(t),
            Err(err) => Err(err.to_string()),
        }
    }
}
