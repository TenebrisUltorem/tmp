use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;

use crate::app::AppState;
use crate::interaction::InteractionState;
use crate::interaction::InteractiveWidget;

const SHUFFLE_TOGGLE_LABEL: &str = "△▽";
const SHUFFLE_TOGGLE_LABEL_STRONG: &str = "▲▼";
const SHUFFLE_TOGGLE_LABEL_PRESSED: &str = "▴▾";

pub fn shuffle_toggle(app_state: &AppState) -> InteractiveWidget {
    InteractiveWidget::default()
        .on_mouse_down({
            let app_state = app_state.clone();
            move |_, _| on_click(&app_state)
        })
        .draw({
            let app_state = app_state.clone();
            move |widget_state, area, buf| draw_shuffle_toggle(widget_state, &app_state, area, buf)
        })
}

fn on_click(app_state: &AppState) {
    app_state.set_shuffle_state(!app_state.shuffle_state());
}

fn draw_shuffle_toggle(widget_state: InteractionState, app_state: &AppState, area: Rect, buf: &mut Buffer) {
    let view = match widget_state {
        InteractionState::Hovered => Paragraph::new(Line::from(SHUFFLE_TOGGLE_LABEL_STRONG).centered())
            .block(Block::bordered().border_type(BorderType::Thick)),
        InteractionState::Pressed => {
            Paragraph::new(Line::from(SHUFFLE_TOGGLE_LABEL_PRESSED).centered()).block(Block::bordered())
        }
        _ => match app_state.shuffle_state() {
            true => {
                Paragraph::new(Line::from(SHUFFLE_TOGGLE_LABEL_STRONG).centered()).block(Block::bordered())
            }
            false => Paragraph::new(Line::from(SHUFFLE_TOGGLE_LABEL).centered()).block(Block::bordered()),
        },
    };

    view.render(area, buf);
}
