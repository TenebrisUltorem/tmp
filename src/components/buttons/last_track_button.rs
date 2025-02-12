use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::{
    app::AppState,
    interaction::{InteractionState, InteractiveWidget},
};

const LAST_TRACK_BUTTON_LABEL: &str = "|◁◁";
const LAST_TRACK_BUTTON_LABEL_HOVERED: &str = "|◀◀";
const LAST_TRACK_BUTTON_LABEL_PRESSED: &str = "|◂◂";

/// Создает кнопку перехода к предыдущему треку
pub fn last_track_button(app_state: &AppState) -> InteractiveWidget {
    let app_state = app_state.clone();

    InteractiveWidget::default().on_mouse_down(move |_, _| on_click(&app_state)).draw(draw_last_track_button)
}

fn on_click(_: &AppState) {
}

fn draw_last_track_button(widget_state: InteractionState, area: Rect, buf: &mut Buffer) {
    let view = match widget_state {
        InteractionState::Hovered => Paragraph::new(Line::from(LAST_TRACK_BUTTON_LABEL_HOVERED).centered())
            .block(Block::bordered().border_type(BorderType::Thick)),
        InteractionState::Pressed => {
            Paragraph::new(Line::from(LAST_TRACK_BUTTON_LABEL_PRESSED).centered()).block(Block::bordered())
        }
        _ => Paragraph::new(Line::from(LAST_TRACK_BUTTON_LABEL).centered()).block(Block::bordered()),
    };

    view.render(area, buf);
}
