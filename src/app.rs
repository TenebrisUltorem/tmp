use ratatui::{
    buffer::Buffer,
    crossterm::{
        self,
        event::{DisableMouseCapture, EnableMouseCapture},
    },
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Padding, Widget},
    DefaultTerminal,
};
use std::io::Error;

use crate::components::{
    last_track_button, 
    next_track_button, 
    play_button, 
    progress_bar, 
    repeat_toggle, 
    shuffle_toggle, 
    stop_button,
    volume_control, 
    PlaylistComponent, 
    VisualizerComponent,
};
use crate::event_handler::{EventHandler, InteractiveWidget};

/// Состояние приложения
/// 
/// # Fields
/// 
/// * `exit` - Флаг выхода из приложения
/// * `shuffle_state` - Флаг состояния перемешивания
/// * `repeat_state` - Флаг состояния повтора
/// * `volume` - Громкость (от 0 до 1)
/// * `play_progress` - Прогресс воспроизведения (от 0 до 1)
#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub exit: bool,
    pub string: String,
    pub shuffle_state: bool,
    pub repeat_state: bool,
    pub volume: f64,
    pub play_progress: f64,
}

/// Главное приложение
pub struct App {
    state: AppState,
    event_handler: EventHandler,

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
        let mut event_handler = EventHandler::default();

        let progress_bar = event_handler.register_component(progress_bar());
        let play_button = event_handler.register_component(play_button());
        let last_track_button = event_handler.register_component(last_track_button());
        let next_track_button = event_handler.register_component(next_track_button());
        let stop_button = event_handler.register_component(stop_button());
        let volume_control = event_handler.register_component(volume_control());
        let shuffle_toggle = event_handler.register_component(shuffle_toggle());
        let repeat_toggle = event_handler.register_component(repeat_toggle());

        Self {
            state: AppState::default(),
            event_handler,
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
        self.main_loop(terminal)?;
        self.cleanup()?;
        Ok(())
    }

    fn setup(&mut self) -> Result<(), Error> {
        crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;
        self.state.volume = 1.0;
        Ok(())
    }

    fn cleanup(&self) -> Result<(), Error> {
        crossterm::execute!(std::io::stdout(), DisableMouseCapture)
    }

    fn main_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Error> {
        while !self.state.exit {
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;
            self.event_handler.handle_events(&mut self.state)?;
        }
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
            .title_bottom(Line::from(vec![" Quit".into(), "<Esc>".blue()]).left_aligned())
            .padding(Padding::new(1, 1, 0, 0))
            .border_set(border::THICK)
    }

    fn create_layout(&self, area: Rect) -> [Rect; 3] {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3), Constraint::Length(3)])
            .areas(area)
    }

    fn render_upper_section(&self, area: Rect, buf: &mut Buffer) {
        let [visualizer_area, playlist_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Min(20)])
            .areas(area);

        VisualizerComponent::new(self.state.clone()).render(visualizer_area, buf);
        PlaylistComponent::default().render(playlist_area, buf);
    }

    fn render_progress_bar(&mut self, area: Rect, buf: &mut Buffer) {
        self.progress_bar.render(self.state.clone(), area, buf);
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
        self.play_button.render(self.state.clone(), play, buf);
        self.last_track_button.render(self.state.clone(), last, buf);
        self.next_track_button.render(self.state.clone(), next, buf);
        self.stop_button.render(self.state.clone(), stop, buf);
    }

    fn render_right_controls(&mut self, area: Rect, buf: &mut Buffer) {
        let areas =
            Layout::horizontal([Constraint::Length(12), Constraint::Length(6), Constraint::Length(6)])
                .flex(Flex::End)
                .areas(area);

        let [volume, shuffle, repeat] = areas;
        self.volume_control.render(self.state.clone(), volume, buf);
        self.shuffle_toggle.render(self.state.clone(), shuffle, buf);
        self.repeat_toggle.render(self.state.clone(), repeat, buf);
    }
}
