use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
#[repr(transparent)]
pub struct AppState<T> {
    pub data: Arc<RwLock<T>>
}

impl<T> AppState<T> {
    pub fn new(state: T) -> Self {
        Self {
            data: Arc::new(RwLock::new(state)),
        }
    }
}