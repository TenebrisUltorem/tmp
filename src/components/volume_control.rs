use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

use crate::{
    app::AppState,
    event_handler::{InteractionState, InteractiveWidget},
};

const VOLUME_BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
const VOLUME_SCROLL_STEP: f64 = 0.03;

const PADDING: Padding = Padding::new(1, 1, 0, 0);
const BORDER_WIDTH: u16 = 1;

pub fn volume_control() -> InteractiveWidget {
    InteractiveWidget::default()
        .on_mouse_down(on_click)
        .on_mouse_drag(on_click)
        .on_mouse_scroll_up(increase_volume)
        .on_mouse_scroll_down(decrease_volume)
        .draw(draw_volume_control)
}

fn draw_volume_control(
    interaction_state: InteractionState, app_state: &AppState, area: Rect, buf: &mut Buffer,
) {
    let mut frame = Block::bordered().padding(PADDING);

    let mut frame_label = Line::from(format!(" Vol {}% ", (app_state.get_volume() * 100.0) as u8));

    if interaction_state == InteractionState::Hovered {
        frame = frame.border_type(BorderType::Thick);
        frame_label = frame_label.bold();
    }

    frame = frame.title(frame_label);

    let volume_gauge = get_volume_string(app_state.get_volume());
    let view = Paragraph::new(volume_gauge).block(frame);
    view.render(area, buf);
}

fn on_click(widget: &mut InteractiveWidget, mouse_position: Position, app_state: &AppState) {
    // Вычисляем ширину активной области слайдера
    let clickable_width = widget.area().width - PADDING.left - PADDING.right - BORDER_WIDTH;

    // Вычисляем позицию клика относительно начала слайдера
    let click_position = mouse_position.x as i16 - PADDING.left as i16;

    // Преобразуем позицию в значение от 0 до 1
    let normalized_position = click_position.clamp(0, clickable_width as i16) as f64;
    app_state.set_volume(normalized_position / clickable_width as f64);
}

fn increase_volume(_: &mut InteractiveWidget, _: Position, app_state: &AppState) {
    app_state.set_volume((app_state.get_volume() + VOLUME_SCROLL_STEP).min(1.0));
}

fn decrease_volume(_: &mut InteractiveWidget, _: Position, app_state: &AppState) {
    app_state.set_volume((app_state.get_volume() - VOLUME_SCROLL_STEP).max(0.0));
}

fn get_volume_string(volume_ratio: f64) -> String {
    let volume_ratio = volume_ratio.clamp(0.0, 1.0);
    let blocks_count = (volume_ratio * VOLUME_BLOCKS.len() as f64).ceil() as usize;

    VOLUME_BLOCKS.iter().take(blocks_count).collect::<String>()
}
