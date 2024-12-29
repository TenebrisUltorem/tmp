mod app_state;
pub use app_state::AppState;
pub use app_state::CurrentTrackInfo;
pub use app_state::PlayerState;

use ratatui::{
    buffer::Buffer,
    crossterm::{
        self,
        event::{DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture},
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Padding, Widget},
    DefaultTerminal,
};
use std::io::Error;
use std::thread;
use std::time::Duration;

use crate::interaction::{EventHandler, InteractiveWidget};
use crate::{
    components::{
        last_track_button, next_track_button, play_button, playlist, progress_bar, repeat_toggle,
        shuffle_toggle, stop_button, volume_control, VisualizerComponent,
    },
    player::Player,
};

const FRAME_TIME: u64 = 62; // ~ 16 fps

/// Главное приложение
pub struct App {
    app_state: AppState,
    event_handler: EventHandler,
    player: Player,

    playlist: InteractiveWidget,
    progress_bar: InteractiveWidget,
    play_button: InteractiveWidget,
    last_track_button: InteractiveWidget,
    next_track_button: InteractiveWidget,
    stop_button: InteractiveWidget,
    volume_control: InteractiveWidget,
    shuffle_toggle: InteractiveWidget,
    repeat_toggle: InteractiveWidget,
}

impl Default for App {
    fn default() -> Self {
        let app_state = AppState::default();
        let mut event_handler = EventHandler::new(&app_state);
        let player = Player::new(&app_state);

        let playlist = event_handler.register_component(playlist(&app_state));
        let progress_bar = event_handler.register_component(progress_bar(&app_state, &player));
        let play_button = event_handler.register_component(play_button(&app_state, &player));
        let last_track_button = event_handler.register_component(last_track_button(&app_state));
        let next_track_button = event_handler.register_component(next_track_button(&app_state));
        let stop_button = event_handler.register_component(stop_button(&app_state, &player));
        let volume_control = event_handler.register_component(volume_control(&app_state, &player));
        let shuffle_toggle = event_handler.register_component(shuffle_toggle(&app_state));
        let repeat_toggle = event_handler.register_component(repeat_toggle(&app_state));

        Self {
            app_state,
            event_handler,
            player,
            playlist,
            progress_bar,
            play_button,
            last_track_button,
            next_track_button,
            stop_button,
            volume_control,
            shuffle_toggle,
            repeat_toggle,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Error> {
        self.setup()?;
        self.event_handler.start()?;
        launch_track_progression_handler(&self.app_state);
        self.main_loop(terminal)?;
        self.cleanup()?;
        Ok(())
    }

    fn setup(&mut self) -> Result<(), Error> {
        crossterm::execute!(
            std::io::stdout(),
            EnterAlternateScreen,
            EnableMouseCapture,
            EnableBracketedPaste
        )?;
        self.app_state.set_volume(1.0);
        Ok(())
    }

    fn cleanup(&self) -> Result<(), Error> {
        crossterm::execute!(
            std::io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            DisableBracketedPaste
        )
    }

    fn main_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Error> {
        while !self.app_state.should_exit() {
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;
            std::thread::sleep(std::time::Duration::from_millis(FRAME_TIME));
        }
        self.player.stop();

        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = self.create_main_block();
        let inner = block.inner(area);
        block.render(area, buf);

        let [upper_area, progress_bar_area, controls_area] = self.create_layout(inner);
        self.render_upper_section(upper_area, buf);
        self.render_progress_bar(progress_bar_area, buf);
        self.render_controls(controls_area, buf);
    }
}

impl App {
    fn create_main_block(&self) -> Block {
        Block::bordered()
            .title(Line::from(" 𝄞 TMP 𝄞 ".bold()).centered())
            .title_bottom(Line::from(vec![" Quit ".into(), "<Esc> ".blue()]).left_aligned())
            .padding(Padding::new(1, 1, 0, 0))
            .border_set(border::THICK)
    }

    fn create_layout(&self, area: Rect) -> [Rect; 3] {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3), Constraint::Length(3)])
            .areas(area)
    }

    fn render_upper_section(&mut self, area: Rect, buf: &mut Buffer) {
        let [visualizer_area, playlist_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Min(35)])
            .areas(area);

        VisualizerComponent::new(self.app_state.clone()).render(visualizer_area, buf);
        self.playlist.render(playlist_area, buf);
    }

    fn render_progress_bar(&mut self, area: Rect, buf: &mut Buffer) {
        self.progress_bar.render(area, buf);
    }

    fn render_controls(&mut self, area: Rect, buf: &mut Buffer) {
        let [left_controls, right_controls] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);

        self.render_left_controls(left_controls, buf);
        self.render_right_controls(right_controls, buf);
    }

    fn render_left_controls(&mut self, area: Rect, buf: &mut Buffer) {
        let areas = Layout::horizontal([
            Constraint::Length(9),
            Constraint::Length(9),
            Constraint::Length(9),
            Constraint::Length(9),
        ])
        .flex(Flex::Start)
        .areas(area);

        let [play, last, next, stop] = areas;
        self.play_button.render(play, buf);
        self.last_track_button.render(last, buf);
        self.next_track_button.render(next, buf);
        self.stop_button.render(stop, buf);
    }

    fn render_right_controls(&mut self, area: Rect, buf: &mut Buffer) {
        let areas =
            Layout::horizontal([Constraint::Length(12), Constraint::Length(6), Constraint::Length(6)])
                .flex(Flex::End)
                .areas(area);

        let [volume, shuffle, repeat] = areas;
        self.volume_control.render(volume, buf);
        self.shuffle_toggle.render(shuffle, buf);
        self.repeat_toggle.render(repeat, buf);
    }
}


fn launch_track_progression_handler(app_state: &AppState) {
    let app_state = app_state.clone();
    thread::spawn(move || {
        loop {
            if let Some(info) = app_state.current_track_info() {
                if app_state.player_state() == PlayerState::Playing {
                    app_state.set_current_track_info(Some(CurrentTrackInfo::new(
                        info.title, 
                        info.artist, 
                        info.album, 
                        info.duration, 
                        Duration::from_secs(info.played_duration.as_secs() + 1)
                    )));
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}