use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

use crate::{
    app::AppState,
    interaction::{InteractionState, InteractiveWidget},
    player::Player,
};

const VOLUME_BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
const VOLUME_SCROLL_STEP: f32 = 0.03;

const PADDING: Padding = Padding::new(1, 1, 0, 0);
const BORDER_WIDTH: u16 = 1;

pub fn volume_control(app_state: &AppState, player: &Player) -> InteractiveWidget {
    InteractiveWidget::default()
        .on_mouse_down({
            let app_state = app_state.clone();
            let player = player.clone();

            move |widget, mouse_position| on_click(widget, mouse_position, &app_state, &player)
        })
        .on_mouse_drag({
            let app_state = app_state.clone();
            let player = player.clone();

            move |widget, mouse_position| on_click(widget, mouse_position, &app_state, &player)
        })
        .on_mouse_scroll_up({
            let app_state = app_state.clone();
            let player = player.clone();

            move |_, _| increase_volume(&app_state, &player)
        })
        .on_mouse_scroll_down({
            let app_state = app_state.clone();
            let player = player.clone();

            move |_, _| decrease_volume(&app_state, &player)
        })
        .draw({
            let app_state = app_state.clone();
            move |interaction_state, area, buf| draw_volume_control(interaction_state, &app_state, area, buf)
        })
}

fn draw_volume_control(
    interaction_state: InteractionState,
    app_state: &AppState,
    area: Rect,
    buf: &mut Buffer,
) {
    let mut frame = Block::bordered().padding(PADDING);

    let mut frame_label = Line::from(format!(" Vol {}% ", (app_state.volume() * 100.0) as u8));

    if interaction_state == InteractionState::Hovered {
        frame = frame.border_type(BorderType::Thick);
        frame_label = frame_label.bold();
    }

    frame = frame.title(frame_label);

    let volume_gauge = get_volume_string(app_state.volume());
    let view = Paragraph::new(volume_gauge).block(frame);
    view.render(area, buf);
}

fn on_click(widget: &mut InteractiveWidget, mouse_position: Position, app_state: &AppState, player: &Player) {
    // Вычисляем ширину активной области слайдера
    let clickable_width = widget.area().width - PADDING.left - PADDING.right - BORDER_WIDTH;

    // Вычисляем позицию клика относительно начала слайдера
    let click_position = mouse_position.x as i16 - PADDING.left as i16;

    // Преобразуем позицию в значение от 0 до 1
    let normalized_position = click_position.clamp(0, clickable_width as i16) as f32;

    let volume = normalized_position / clickable_width as f32;

    app_state.set_volume(volume);
    player.set_volume(volume);
}

fn increase_volume(app_state: &AppState, player: &Player) {
    let volume = (app_state.volume() + VOLUME_SCROLL_STEP).min(1.0);

    app_state.set_volume(volume);
    player.set_volume(volume);
}

fn decrease_volume(app_state: &AppState, player: &Player) {
    let volume = (app_state.volume() - VOLUME_SCROLL_STEP).max(0.0);

    app_state.set_volume(volume);
    player.set_volume(volume);
}

fn get_volume_string(volume_ratio: f32) -> String {
    let volume_ratio = volume_ratio.clamp(0.0, 1.0);
    let blocks_count = (volume_ratio * VOLUME_BLOCKS.len() as f32).ceil() as usize;

    VOLUME_BLOCKS.iter().take(blocks_count).collect::<String>()
}
