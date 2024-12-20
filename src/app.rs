use std::io::Error;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{DisableMouseCapture,EnableMouseCapture};
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::Line;
use ratatui::widgets::{Block, Padding, Widget};
use ratatui::{crossterm, DefaultTerminal};

use crate::components::buttons::{last_track_button, next_track_button, play_button, stop_button};
use crate::components::{Button, PlaylistComponent, VisualizerComponent};
use crate::event_handler::EventHandler;

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub exit: bool,
    pub string: String
}


pub struct App {
    state: AppState,
    event_handler: EventHandler,

    play_button: Button,
    last_track_button: Button,
    next_track_button: Button,
    stop_button: Button
}

impl Default for App {

    fn default() -> Self {
        Self {
            state: AppState::default(),
            event_handler: EventHandler::default(),
            play_button: play_button(),
            last_track_button: last_track_button(),
            next_track_button: next_track_button(),
            stop_button: stop_button()
        }
    }

}
impl App {

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Error> {
        crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;

        while !self.state.exit {
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;
            self.event_handler.handle_events(&mut self.state)?;
        }   

        crossterm::execute!(std::io::stdout(), DisableMouseCapture)?;
        Ok(())
    }

}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::from(" TMP ".bold()).centered())
            .title_bottom(Line::from(vec![" Quit".into(), "<Q>".blue()]).left_aligned())
            .padding(Padding::new(1, 1, 0, 0))
            .border_set(border::THICK);
        let inner = block.inner(area);
        block.render(area, buf);

        let [upper_area, controls_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(15), 
                Constraint::Length(3)
            ])
            .areas(inner);

        let [visualizer_area, playlist_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70), 
                Constraint::Min(20)
            ])
            .areas(upper_area);

        VisualizerComponent::new(self.state.clone()).render(visualizer_area, buf);
        PlaylistComponent::default().render(playlist_area, buf);

        let [
            play_button_area,
            last_track_button_area,
            next_track_button_area,
            stop_button_area
        ] = Layout::horizontal([
            Constraint::Length(9), 
            Constraint::Length(9), 
            Constraint::Length(9),
            Constraint::Length(9)
        ]).flex(Flex::Start).areas(controls_area);

        self.play_button.render(play_button_area, buf);
        self.last_track_button.render(last_track_button_area, buf);
        self.next_track_button.render(next_track_button_area, buf);
        self.stop_button.render(stop_button_area, buf);
        
        self.event_handler.register_component(Box::new(self.play_button.clone()), play_button_area);
        self.event_handler.register_component(Box::new(self.last_track_button.clone()), last_track_button_area);
        self.event_handler.register_component(Box::new(self.next_track_button.clone()), next_track_button_area);
        self.event_handler.register_component(Box::new(self.stop_button.clone()), stop_button_area);
    }

}
