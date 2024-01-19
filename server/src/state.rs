use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState<T> {
    pub data: Arc<Mutex<T>>
}

impl<T> AppState<T> {
    pub fn new(state: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(state)),
        }
    }
}