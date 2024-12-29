use std::{
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

const TRACK_FORMATS: [&str; 3] = ["mp3", "ogg", "wav"];

#[derive(Default, Clone, PartialEq)]
pub enum PlayerState {
    #[default]
    Stopped,
    Playing,
    Paused,
}

/// Информация о текущем треке
///
/// # Fields
///
/// * `title` - Название трека
/// * `artist` - Исполнитель
/// * `album` - Альбом
/// * `duration` - Полная длительность трека
/// * `played_duration` - прошеднее время воспроизведения трека
#[derive(Clone)]
pub struct CurrentTrackInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: Duration,
    pub played_duration: Duration,
}

impl CurrentTrackInfo {
    pub fn new(
        title: String,
        artist: String,
        album: String,
        duration: Duration,
        played_duration: Duration,
    ) -> Self {
        Self { title, artist, album, duration, played_duration }
    }
}

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
    input_string: Arc<Mutex<String>>,

    player_state: Arc<Mutex<PlayerState>>,

    playlist: Arc<Mutex<Vec<String>>>,

    shuffle_state: Arc<Mutex<bool>>,
    repeat_state: Arc<Mutex<bool>>,

    volume: Arc<Mutex<f32>>,

    current_track_info: Arc<Mutex<Option<CurrentTrackInfo>>>,
}

impl AppState {
    pub fn set_exit(&self, value: bool) {
        if let Ok(mut exit) = self.exit.lock() {
            *exit = value;
        }
    }

    pub fn should_exit(&self) -> bool {
        *self.exit.lock().unwrap()
    }

    pub fn set_debug_string(&self, value: impl Into<String>) {
        if let Ok(mut debug_string) = self.debug_string.lock() {
            *debug_string = value.into();
        }
    }

    pub fn debug_string(&self) -> String {
        self.debug_string.lock().unwrap().clone()
    }

    pub fn set_player_state(&self, value: PlayerState) {
        if let Ok(mut player_state) = self.player_state.lock() {
            *player_state = value;
        }
    }

    pub fn player_state(&self) -> PlayerState {
        self.player_state.lock().unwrap().clone()
    }

    pub fn set_shuffle_state(&self, value: bool) {
        if let Ok(mut shuffle_state) = self.shuffle_state.lock() {
            *shuffle_state = value;
        }
    }

    pub fn shuffle_state(&self) -> bool {
        *self.shuffle_state.lock().unwrap()
    }

    pub fn set_repeat_state(&self, value: bool) {
        if let Ok(mut repeat_state) = self.repeat_state.lock() {
            *repeat_state = value;
        }
    }

    pub fn repeat_state(&self) -> bool {
        *self.repeat_state.lock().unwrap()
    }

    pub fn set_volume(&self, value: f32) {
        if let Ok(mut volume) = self.volume.lock() {
            *volume = value;
        }
    }

    pub fn volume(&self) -> f32 {
        *self.volume.lock().unwrap()
    }

    pub fn add_track(&self, track_file_path: String) {
        if let Ok(mut playlist) = self.playlist.lock() {
            let path = Path::new(&track_file_path);

            if let Some(extension) = path.extension() {
                if TRACK_FORMATS.contains(&extension.to_str().unwrap()) {
                    playlist.push(track_file_path);
                }
            }
        }
    }

    pub fn playlist(&self) -> Vec<String> {
        self.playlist.lock().unwrap().clone()
    }

    pub fn input_string(&self) -> String {
        self.input_string.lock().unwrap().clone()
    }

    pub fn set_input_string(&self, value: String) {
        if let Ok(mut input_string) = self.input_string.lock() {
            *input_string = value;
        }
    }

    pub fn set_current_track_info(&self, value: Option<CurrentTrackInfo>) {
        if let Ok(mut current_track_info) = self.current_track_info.lock() {
            *current_track_info = value;
        }
    }

    pub fn current_track_info(&self) -> Option<CurrentTrackInfo> {
        self.current_track_info.lock().unwrap().clone()
    }
}
