use std::{fs::metadata, path::Path};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, List, Widget},
};

use crate::{
    app::AppState,
    interaction::{InteractionState, InteractiveWidget},
};

const TRACK_FORMATS: [&str; 1] = ["mp3"];

pub fn playlist_widget(app_state: &AppState) -> InteractiveWidget {
    let app_state = app_state.clone();

    let initial_playlist = get_initial_playlist();
    app_state.set_playlist(initial_playlist);

    InteractiveWidget::default()
        .on_paste({
            let app_state = app_state.clone();

            move |_, paste_event| {
                walk_path(Path::new(&paste_event))
                    .iter()
                    .for_each(|path| app_state.add_track(path.to_string()));
            }
        })
        .draw(move |widget_state, area, buf| {
            draw_playlist(widget_state, &app_state, area, buf)
        })
}

fn draw_playlist(_: InteractionState, app_state: &AppState, area: Rect, buf: &mut Buffer) {
    let playlist = app_state.playlist();

    List::new(playlist).block(Block::bordered().title(" Playlist ")).render(area, buf);
}

fn get_initial_playlist() -> Vec<String> {
    let mut playlist = vec![];

    if let Some(path) = std::env::args().collect::<Vec<String>>().get(1) {
        let path = Path::new(path);
        playlist.extend(walk_path(path));
    }

    playlist
}

fn walk_path(path: &Path) -> Vec<String> {
    let mut playlist = vec![];

    let path = Path::new(path);
    if let Ok(metadata) = metadata(path) {
        if metadata.is_dir() {
            playlist.extend(scan_directory(path));
        } else if metadata.is_file() {
            if is_file_type_correct(path) {
                playlist.push(path.to_str().unwrap().to_string());
            }
        }
    }
    playlist
}

fn scan_directory(path: &Path) -> Vec<String> {
    let mut playlist = vec![];

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let path = path.as_path();
                if is_file_type_correct(path) { playlist.push(path.to_str().unwrap().to_string()); }
            }
        }
    }
    playlist
}

fn is_file_type_correct(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        if let Some(extension) = extension.to_str() {
            if TRACK_FORMATS.contains(&extension) {
                return true;
            }
        }
    }
    false
}
