use ratatui::buffer::Buffer;
use ratatui::layout::Position;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;

use crate::app::AppState;
use crate::interaction::InteractionState;
use crate::interaction::InteractiveWidget;

const REPEAT_TOGGLE_LABEL: &str = "⮎⮌";

pub fn repeat_toggle() -> InteractiveWidget {
    InteractiveWidget::default().on_mouse_down(on_click).draw(draw_repeat_toggle)
}

fn on_click(_: &mut InteractiveWidget, _: Position, app_state: &AppState) {
    app_state.set_repeat_state(!app_state.get_repeat_state());

    let mut debug_string = app_state.get_debug_string();
    debug_string.push_str(&format!("Repeat toggle switched to: {}\n", app_state.get_repeat_state()));
    app_state.set_debug_string(debug_string);
}

fn draw_repeat_toggle(widget_state: InteractionState, app_state: &AppState, area: Rect, buf: &mut Buffer) {
    let view = match widget_state {
        InteractionState::Hovered => Paragraph::new(Line::from(REPEAT_TOGGLE_LABEL).centered().bold())
            .block(Block::bordered().border_type(BorderType::Thick)),
        InteractionState::Pressed => {
            Paragraph::new(Line::from(REPEAT_TOGGLE_LABEL).centered().bold()).block(Block::bordered())
        }
        _ => match app_state.get_repeat_state() {
            true => {
                Paragraph::new(Line::from(REPEAT_TOGGLE_LABEL).centered().bold()).block(Block::bordered())
            }
            false => Paragraph::new(Line::from(REPEAT_TOGGLE_LABEL).centered()).block(Block::bordered()),
        },
    };

    view.render(area, buf);
}
