use ratatui::widgets::Widget;
use ratatui::layout::Rect;
use ratatui::buffer::Buffer;
use ratatui::text::Line;
use ratatui::widgets::Block;

#[derive(Default)]
pub struct PlaylistComponent {}

impl Widget for &PlaylistComponent {

    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .title(Line::from(" Playlist ").left_aligned())
            .render(area, buf);
    }

}