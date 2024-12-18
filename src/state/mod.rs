use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::num::Wrapping;
use lazy_static::lazy_static;

#[derive(Debug, Default)]
struct AppState {
    pub counter: Wrapping<u8>,
    pub exit: bool,
}

lazy_static! {
    static ref APP_STATE: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState::default()));
}

pub struct StateManager {}

impl StateManager {

    pub fn get_counter() -> Wrapping<u8> {
        APP_STATE.lock().unwrap().counter
    }

    pub fn increment() {
        APP_STATE.lock().unwrap().counter += 1;
    }

    pub fn decrement() {
        APP_STATE.lock().unwrap().counter -= 1;
    }

    pub fn set_exit(exit: bool) {
        APP_STATE.lock().unwrap().exit = exit;
    }

    pub fn should_exit() -> bool {
        APP_STATE.lock().unwrap().exit
    }
    
}
