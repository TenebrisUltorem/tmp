use std::sync::{Arc, Mutex};


/// Состояние приложения
/// 
/// # Fields
/// 
/// * `exit` - Флаг выхода из приложения
/// * `shuffle_state` - Флаг состояния перемешивания
/// * `repeat_state` - Флаг состояния повтора
/// * `volume` - Громкость (от 0 до 1)
/// * `play_progress` - Прогресс воспроизведения (от 0 до 1)
/// * `playlist` - Плейлист
/// * `input_string` - Строка ввода (для перехвата ввода файлов)
#[derive(Default, Clone)]
pub struct AppState {
    exit: Arc<Mutex<bool>>,
    debug_string: Arc<Mutex<String>>,
    shuffle_state: Arc<Mutex<bool>>,
    repeat_state: Arc<Mutex<bool>>,
    volume: Arc<Mutex<f64>>,
    play_progress: Arc<Mutex<f64>>,
    playlist: Arc<Mutex<Vec<String>>>,
    input_string: Arc<Mutex<String>>,
}

impl AppState {

    pub fn set_exit(&self, value: bool) {
        if let Ok(mut exit) = self.exit.lock() {
            *exit = value;
        }
    }

    pub fn should_exit(&self) -> bool {
        self.exit.lock().unwrap().clone()
    }

    pub fn set_debug_string(&self, value: String) {
        if let Ok(mut debug_string) = self.debug_string.lock() {
            *debug_string = value;
        }
    }

    pub fn get_debug_string(&self) -> String {
        self.debug_string.lock().unwrap().clone()
    }

    pub fn set_shuffle_state(&self, value: bool) {
        if let Ok(mut shuffle_state) = self.shuffle_state.lock() {
            *shuffle_state = value;
        }
    }

    pub fn get_shuffle_state(&self) -> bool {
        self.shuffle_state.lock().unwrap().clone()
    }

    pub fn set_repeat_state(&self, value: bool) {
        if let Ok(mut repeat_state) = self.repeat_state.lock() {
            *repeat_state = value;
        }
    }   

    pub fn get_repeat_state(&self) -> bool {
        self.repeat_state.lock().unwrap().clone()
    }

    pub fn set_volume(&self, value: f64) {
        if let Ok(mut volume) = self.volume.lock() {
            *volume = value;
        }
    }

    pub fn get_volume(&self) -> f64 {
        self.volume.lock().unwrap().clone()
    }

    pub fn set_play_progress(&self, value: f64) {       
        if let Ok(mut play_progress) = self.play_progress.lock() {
            *play_progress = value;
        }
    }

    pub fn get_play_progress(&self) -> f64 {
        self.play_progress.lock().unwrap().clone()
    }   

    pub fn add_track(&self, track: String) {
        if let Ok(mut playlist) = self.playlist.lock() {
            playlist.push(track);
        }
    }

    pub fn get_playlist(&self) -> Vec<String> {
        self.playlist.lock().unwrap().clone()
    }

    pub fn get_input_string(&self) -> String {
        self.input_string.lock().unwrap().clone()
    }

    pub fn set_input_string(&self, value: String) {
        if let Ok(mut input_string) = self.input_string.lock() {
            *input_string = value;
        }
    }


}