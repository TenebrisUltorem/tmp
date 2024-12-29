use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, List, Widget},
};

use crate::{
    app::AppState,
    interaction::{InteractionState, InteractiveWidget},
};

pub fn playlist(app_state: &AppState) -> InteractiveWidget {
    let app_state = app_state.clone();

    InteractiveWidget::default()
        .draw(move |widget_state, area, buf| draw_playlist(widget_state, &app_state, area, buf))
}

fn draw_playlist(_: InteractionState, app_state: &AppState, area: Rect, buf: &mut Buffer) {
    let playlist = app_state.playlist();

    List::new(playlist).block(Block::bordered().title(" Playlist ")).render(area, buf);
}
