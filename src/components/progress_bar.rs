use ratatui::{
    buffer::Buffer, layout::Rect, style::Stylize, text::Line, widgets::{Block, Padding, Paragraph, Widget}
};

use crate::{
    app::AppState, 
    event_handler::{InteractionState, InteractiveWidget}
};

const PROGRESS_BAR_CHARACTER: char = '━';
const PROGRESS_BAR_SLIDER_CHARACTER: char = '█';

pub fn progress_bar() -> InteractiveWidget {
    InteractiveWidget::default().draw(draw_progress_bar)
        
}

fn draw_progress_bar(_: InteractionState, _: AppState, area: Rect, buf: &mut Buffer) {
    let mut string: String = String::new();
    for _ in 0..(area.width / 2) - 4 {
        string.push(PROGRESS_BAR_CHARACTER);
    }
    string.push(PROGRESS_BAR_SLIDER_CHARACTER);

    Paragraph::new(Line::from(string).bold())
        .block(Block::bordered()
            .padding(Padding::new(1, 1, 0, 0))
        )
        .render(area, buf);
}
