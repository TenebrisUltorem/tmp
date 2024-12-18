use std::io::Error;
use ratatui::{
    buffer::Buffer, 
    layout::{Constraint, Direction, Layout, Rect}, 
    style::Stylize, 
    symbols::border, 
    text::Line, 
    widgets::{Block, Padding, Widget}, 
    DefaultTerminal
};

use crate::{
    components::{PlaylistComponent, VisualizerComponent, ControlsComponent}, 
    state::StateManager
};

#[derive(Default)]
pub struct App {
    visualizer_component: VisualizerComponent,
    playlist_component: PlaylistComponent,
    controls_component: ControlsComponent
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::from(" TMP ".bold()).centered())
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
                
        self.visualizer_component.render(visualizer_area, buf);
        self.playlist_component.render(playlist_area, buf);
        self.controls_component.render(controls_area, buf);
    }

}

impl App {

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Error> {

        while !StateManager::should_exit() {
            terminal.draw(|frame| frame.render_widget(&*self, frame.area()))?;
            self.handle_events()?;
        }

        Ok(())
    }

    pub fn handle_events(&mut self) -> Result<(), Error> {
        Ok(())
    }

}
