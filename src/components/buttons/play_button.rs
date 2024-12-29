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

const PLAY_BUTTON_LABEL: &str = "▷";
const PLAY_BUTTON_LABEL_HOVERED: &str = "▶";
const PLAY_BUTTON_LABEL_PRESSED: &str = "▸";

const PLAY_BUTTON_LABEL_PAUSE: &str = "‖";
const PLAY_BUTTON_LABEL_PAUSE_HOVERED: &str = "‖";
const PLAY_BUTTON_LABEL_PAUSE_PRESSED: &str = "∥";

pub fn play_button(app_state: &AppState, player: &Player) -> InteractiveWidget {
    InteractiveWidget::default()
        .on_mouse_down({
            let app_state = app_state.clone();
            let player = player.clone();

            move |_, _| on_click(&app_state, &player)
        })
        .draw({
            let app_state = app_state.clone();
            move |widget_state, area, buf| draw_play_button(widget_state, &app_state, area, buf)
        })
}

fn on_click(app_state: &AppState, player: &Player) {
    let mut debug_string = app_state.debug_string();
    debug_string.push_str("Play button clicked \n");

    app_state.set_debug_string(debug_string);

    match app_state.player_state() {
        PlayerState::Playing => {
            player.pause();
            app_state.set_player_state(PlayerState::Paused);
        }
        PlayerState::Paused => {
            player.resume();
            app_state.set_player_state(PlayerState::Playing);
        }
        PlayerState::Stopped => {
            player.play("test.mp3".to_string());
            app_state.set_player_state(PlayerState::Playing);
        }
    }
}

fn draw_play_button(widget_state: InteractionState, app_state: &AppState, area: Rect, buf: &mut Buffer) {
    let view = match widget_state {
        InteractionState::Hovered => {
            if app_state.player_state() == PlayerState::Playing {
                Paragraph::new(Line::from(PLAY_BUTTON_LABEL_PAUSE_HOVERED).centered())
                    .block(Block::bordered().border_type(BorderType::Thick))
            } else {
                Paragraph::new(Line::from(PLAY_BUTTON_LABEL_HOVERED).centered())
                    .block(Block::bordered().border_type(BorderType::Thick))
            }
        }
        InteractionState::Pressed => {
            //Меняем местами,чтобы лого кнопки менялось после отжатия, а не сразу
            if app_state.player_state() == PlayerState::Playing {
                Paragraph::new(Line::from(PLAY_BUTTON_LABEL_PRESSED).centered()).block(Block::bordered())
            } else {
                Paragraph::new(Line::from(PLAY_BUTTON_LABEL_PAUSE_PRESSED).centered())
                    .block(Block::bordered())
            }
        }
        _ => {
            if app_state.player_state() == PlayerState::Playing {
                Paragraph::new(Line::from(PLAY_BUTTON_LABEL_PAUSE).centered()).block(Block::bordered())
            } else {
                Paragraph::new(Line::from(PLAY_BUTTON_LABEL).centered()).block(Block::bordered())
            }
        }
    };

    view.render(area, buf);
}
