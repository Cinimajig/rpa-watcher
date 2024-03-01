use std::collections::HashMap;

use rpa::RpaData;
use tokio::{sync::*, time::Instant};

pub type RpaValue = (Instant, RpaData);
pub type RpaItems = HashMap<String, RpaValue>;


static mut DATA_SUCESS: Option<RwLock<RpaItems>> = None;
static mut DATA_FAILED: Option<RwLock<RpaItems>> = None;

pub async fn rpa_s_mut() -> RwLockWriteGuard<'static, RpaItems> {
    unsafe {
        if DATA_SUCESS.is_none() {
            DATA_SUCESS = Some(RwLock::new(HashMap::with_capacity(10)));
        }
        DATA_SUCESS.as_mut().unwrap().write().await
    }
}

pub async fn rpa_s() -> RwLockReadGuard<'static, RpaItems> {
    unsafe {
        if DATA_SUCESS.is_none() {
            DATA_SUCESS = Some(RwLock::new(HashMap::with_capacity(10)));
        }
        DATA_SUCESS.as_mut().unwrap().read().await
    }
}

pub async fn rpa_f_mut() -> RwLockWriteGuard<'static, RpaItems> {
    unsafe {
        if DATA_FAILED.is_none() {
            DATA_SUCESS = Some(RwLock::new(HashMap::with_capacity(10)));
        }
        DATA_FAILED.as_mut().unwrap().write().await
    }
}

pub async fn rpa_f() -> RwLockReadGuard<'static, RpaItems> {
    unsafe {
        if DATA_FAILED.is_none() {
            DATA_FAILED = Some(RwLock::new(HashMap::with_capacity(10)));
        }
        DATA_FAILED.as_mut().unwrap().read().await
    }
}