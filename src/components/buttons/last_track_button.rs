use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::{
    app::AppState,
    event_handler::{InteractionState, InteractiveWidget},
};

const LAST_TRACK_BUTTON_LABEL: &str = "|◁◁";
const LAST_TRACK_BUTTON_LABEL_HOVERED: &str = "|◀◀";
const LAST_TRACK_BUTTON_LABEL_PRESSED: &str = "|◂◂";

/// Создает кнопку перехода к предыдущему треку
pub fn last_track_button() -> InteractiveWidget {
    InteractiveWidget::default().on_mouse_down(on_click).draw(draw_last_track_button)
}

fn on_click(_: &mut InteractiveWidget, _: Position, app_state: &mut AppState) {
    app_state.string += "Last track button clicked\n";
}

fn draw_last_track_button(widget_state: InteractionState, _: AppState, area: Rect, buf: &mut Buffer) {
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
