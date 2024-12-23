use ratatui::buffer::Buffer;
use ratatui::layout::Position;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;

use crate::app::AppState;
use crate::event_handler::InteractionState;
use crate::event_handler::InteractiveWidget;

const SHUFFLE_TOGGLE_LABEL: &str = "△▽";
const SHUFFLE_TOGGLE_LABEL_STRONG: &str = "▲▼";
const SHUFFLE_TOGGLE_LABEL_PRESSED: &str = "▴▾";

pub fn shuffle_toggle() -> InteractiveWidget {
    InteractiveWidget::default().on_mouse_down(on_click).draw(draw_shuffle_toggle)
}

fn on_click(_: &mut InteractiveWidget, _: Position, app_state: &AppState) {
    app_state.set_shuffle_state(!app_state.get_shuffle_state());

    let mut debug_string = app_state.get_debug_string();
    debug_string.push_str(&format!("Shuffle toggle switched to: {}\n", app_state.get_shuffle_state()));
    app_state.set_debug_string(debug_string);
}

fn draw_shuffle_toggle(widget_state: InteractionState, app_state: &AppState, area: Rect, buf: &mut Buffer) {
    let view = match widget_state {
        InteractionState::Hovered => Paragraph::new(Line::from(SHUFFLE_TOGGLE_LABEL_STRONG).centered())
            .block(Block::bordered().border_type(BorderType::Thick)),
        InteractionState::Pressed => {
            Paragraph::new(Line::from(SHUFFLE_TOGGLE_LABEL_PRESSED).centered()).block(Block::bordered())
        }
        _ => match app_state.get_shuffle_state() {
            true => {
                Paragraph::new(Line::from(SHUFFLE_TOGGLE_LABEL_STRONG).centered()).block(Block::bordered())
            }
            false => Paragraph::new(Line::from(SHUFFLE_TOGGLE_LABEL).centered()).block(Block::bordered()),
        },
    };

    view.render(area, buf);
}
