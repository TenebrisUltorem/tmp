use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::{
    app::{AppState, PlayerState},
    interaction::{InteractionState, InteractiveWidget},
    player::Player,
};

const STOP_BUTTON_LABEL: &str = "▢";
const STOP_BUTTON_LABEL_HOVERED: &str = "■";
const STOP_BUTTON_LABEL_PRESSED: &str = "▪";

pub fn stop_button(app_state: &AppState, player: &Player) -> InteractiveWidget {
    let app_state = app_state.clone();
    let player = player.clone();

    InteractiveWidget::default()
        .on_mouse_down(move |_, _| on_click(&app_state, &player))
        .draw(draw_stop_button)
}

fn on_click(app_state: &AppState, player: &Player) {
    let mut debug_string = app_state.debug_string();
    debug_string.push_str("Stop button clicked\n");

    app_state.set_debug_string(debug_string);
    app_state.set_player_state(PlayerState::Stopped);
    player.stop();
}

fn draw_stop_button(widget_state: InteractionState, area: Rect, buf: &mut Buffer) {
    let view = match widget_state {
        InteractionState::Hovered => Paragraph::new(Line::from(STOP_BUTTON_LABEL_HOVERED).centered())
            .block(Block::bordered().border_type(BorderType::Thick)),
        InteractionState::Pressed => {
            Paragraph::new(Line::from(STOP_BUTTON_LABEL_PRESSED).centered()).block(Block::bordered())
        }
        _ => Paragraph::new(Line::from(STOP_BUTTON_LABEL).centered()).block(Block::bordered()),
    };

    view.render(area, buf);
}
