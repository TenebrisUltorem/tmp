use std::{fs::File, io::BufReader, sync::mpsc::{self, Sender}, thread, time::Duration};

use rodio::{Decoder, OutputStream, Sink, Source};

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
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let mut current_file_path = Option::<String>::None;
            let (_stream, stream_handle) = OutputStream::try_default()
                .unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            for command in receiver {
                match command {
                    PlayerCommand::Play(path) => {
                        if let Ok(file) = File::open(path.clone()) {
                            current_file_path = Some(path);
                            let decoder = Decoder::new(
                                BufReader::new(file)
                            ).unwrap();
                            sink.append(decoder);
                        }
                        sink.play();
                    },
                    PlayerCommand::Stop => {
                        sink.stop();
                    },
                    PlayerCommand::Pause => {
                        sink.pause();
                    },
                    PlayerCommand::Resume => {
                        sink.play();
                    },
                    PlayerCommand::SetVolume(volume) => {
                        sink.set_volume(volume);
                    },
                    PlayerCommand::Seek(ratio) => {
                        sink.stop();

                        if let Some(path) = current_file_path.clone() {
                            if let Ok(file) = File::open(path.clone()) {
                                current_file_path = Some(path);
                                let mut decoder = Decoder::new(
                                    BufReader::new(file)
                                ).unwrap();
                                let total_duration = decoder.total_duration().unwrap().as_secs_f64();
                                let duration = total_duration * ratio;
                                let _ = decoder.try_seek(Duration::from_secs_f64(duration));
                                sink.append(decoder);
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
