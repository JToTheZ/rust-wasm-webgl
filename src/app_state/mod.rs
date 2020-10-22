use lazy_static::*;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn app_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

pub fn set_points(points: Vec<f32>) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(AppState {
        points: points,
        ..*data.clone()
    });
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub points: Vec<f32>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            points: Vec::new(),
        }
    }
}