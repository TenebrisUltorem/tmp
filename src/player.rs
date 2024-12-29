use std::{
    fs::File,
    io::BufReader,
    sync::{mpsc::{self, Sender}, Arc},
    thread,
    time::Duration,
};

use audiotags::Tag;
use rodio::{Decoder, OutputStream, Sink, Source};

use crate::app::{AppState, CurrentTrackInfo, PlayerState};

#[derive(Debug)]
enum PlayerCommand {
    Play(String),
    Stop,
    Pause,
    Resume,
    Seek(f64),
    SetVolume(f32),
}

#[derive(Clone)]
pub struct Player {
    command_sender: Sender<PlayerCommand>,
}

impl Player {
    pub fn new(app_state: &AppState) -> Self {
        let (sender, receiver) = mpsc::channel();

        thread::spawn({
            let app_state = app_state.clone();
            move || {
                let mut current_file_path = Option::<String>::None;
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let sink = Arc::new(Sink::try_new(&stream_handle).unwrap());

                for command in receiver {
                    match command {
                        PlayerCommand::Play(path) => {
                            if let Ok(file) = File::open(path.clone()) {
                                current_file_path = Some(path.clone());

                                let decoder = Decoder::new(BufReader::new(file)).unwrap();

                                if let Some(duration) = decoder.total_duration() {
                                    app_state.set_current_track_info(current_track_info(&path, duration));
                                }

                                sink.append(decoder);
                            }
                            sink.play();
                            launch_sink_state_checker(&app_state, sink.clone());
                        }
                        PlayerCommand::Stop => {
                            sink.stop();
                            current_file_path = None;
                        }
                        PlayerCommand::Pause => {
                            sink.pause();
                        }
                        PlayerCommand::Resume => {
                            sink.play();
                        }
                        PlayerCommand::SetVolume(volume) => {
                            sink.set_volume(volume);
                        }
                        PlayerCommand::Seek(ratio) => {
                            sink.stop();

                            if let Some(path) = current_file_path.clone() {
                                if let Ok(file) = File::open(path.clone()) {
                                    current_file_path = Some(path);
                                    let mut decoder = Decoder::new(BufReader::new(file)).unwrap();
                                    let total_duration = decoder
                                        .total_duration()
                                        .unwrap_or(Duration::from_secs(0))
                                        .as_secs_f64();
                                    let duration = total_duration * ratio;
                                    let _ = decoder.try_seek(Duration::from_secs_f64(duration));
                                    sink.append(decoder);
                                }
                            }
                        }
                    }
                }
            }
        });

        Self { command_sender: sender }
    }

    // Методы для управления из UI
    pub fn play(&self, path: String) {
        self.command_sender.send(PlayerCommand::Play(path)).unwrap();
    }

    pub fn stop(&self) {
        self.command_sender.send(PlayerCommand::Stop).unwrap();
    }

    pub fn pause(&self) {
        self.command_sender.send(PlayerCommand::Pause).unwrap();
    }

    pub fn resume(&self) {
        self.command_sender.send(PlayerCommand::Resume).unwrap();
    }

    pub fn set_volume(&self, volume: f32) {
        self.command_sender.send(PlayerCommand::SetVolume(volume)).unwrap();
    }

    pub fn seek(&self, position: f64) {
        self.command_sender.send(PlayerCommand::Seek(position)).unwrap();
    }
}

fn current_track_info(path: &String, track_duration: Duration) -> Option<CurrentTrackInfo> {
    if let Some(tag) = Tag::new().read_from_path(&path).ok() {
        let title = tag.title().unwrap_or("Unknown");
        let artist = tag.artist().unwrap_or("Unknown");

        let album =
            if let Some(album) = tag.album() { album.title.to_string() } else { "Unknown".to_string() };

        return Some(CurrentTrackInfo::new(
            title.to_string(),
            artist.to_string(),
            album.to_string(),
            track_duration,
            Duration::from_secs(0),
        ));
    }

    None
}


fn launch_sink_state_checker(app_state: &AppState, sink: Arc<Sink>) {
    thread::spawn({
        let sink = sink.clone();
        let app_state = app_state.clone();
        move || {
            while !sink.empty() {
                thread::sleep(Duration::from_secs(1));
                app_state.set_debug_string("Track is playing");
            }
            app_state.set_debug_string("Track is not playing");

            app_state.set_current_track_info(None);
            app_state.set_player_state(PlayerState::Stopped);
        }
    });
}