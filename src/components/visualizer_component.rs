use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Paragraph, Widget},
};

use crate::app::AppState;

pub struct VisualizerComponent {
    app_state: AppState,
}

impl VisualizerComponent {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }
}

impl Widget for VisualizerComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = format!("{}\n{}", self.app_state.debug_string(), self.app_state.input_string());
        Paragraph::new(text).block(Block::bordered()).render(area, buf);
    }
}
