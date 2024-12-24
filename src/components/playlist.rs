use ratatui::{
    buffer::Buffer, 
    layout::Rect, 
    widgets::{Block, List, Widget}
};

use crate::{
    app::AppState, 
    interaction::{InteractionState, InteractiveWidget}
};


pub fn playlist() -> InteractiveWidget {
    InteractiveWidget::default().draw(draw_playlist)
}

fn draw_playlist(_: InteractionState, app_state: &AppState, area: Rect, buf: &mut Buffer) {
    List::new(app_state.get_playlist())
        .block(Block::bordered().title(" Playlist "))
        .render(area, buf);
}
