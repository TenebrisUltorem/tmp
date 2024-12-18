use ratatui::{
    buffer::Buffer, 
    layout::{Constraint, Flex, Layout, Rect}, 
    text::Line, 
    widgets::{Block, BorderType, Paragraph, Widget}
};

#[derive(Default)]
pub struct ControlsComponent {}

impl Widget for &ControlsComponent {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let [play_button_area, last_track_button_area, next_track_button_area, stop_button_area] = Layout::horizontal([
            Constraint::Length(9), 
            Constraint::Length(9), 
            Constraint::Length(9),
            Constraint::Length(9)
        ])
        .flex(Flex::Start)
        .areas(area);

        Button::new("▶").render(play_button_area, buf);
        Button::new("|◀◀").render(last_track_button_area, buf);
        Button::new("▶▶|").render(next_track_button_area, buf);
        Button::new("■").render(stop_button_area, buf);
    }

}

struct Button {
    label: String
}

impl Button {

    fn new(label: impl Into<String>) -> Self {
        Self { label: label.into() }
    }

}

impl Widget for &Button {

    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(vec![
            Line::from(self.label.clone()).centered(),
        ])
            .block(Block::bordered().border_type(BorderType::Rounded))
            .render(area, buf);
    }
    
}