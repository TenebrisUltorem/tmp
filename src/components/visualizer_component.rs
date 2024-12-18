use ratatui::{
    buffer::Buffer, 
    layout::Rect, 
    widgets::{Block, Widget}
};

#[derive(Default)]
pub struct VisualizerComponent {}

impl Widget for &VisualizerComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered().render(area, buf);
    }
}