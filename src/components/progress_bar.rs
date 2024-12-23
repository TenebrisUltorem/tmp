use ratatui::{
    buffer::Buffer, layout::{Position, Rect}, style::Stylize, text::Line, widgets::{Block, Padding, Paragraph, Widget}
};

use crate::{
    app::AppState, 
    event_handler::{InteractionState, InteractiveWidget}
};

const PADDING: Padding = Padding::new(1, 1, 0, 0);
const BORDER_WIDTH: u16 = 2;

const PROGRESS_BAR_CHARACTER: char = '━';
const PROGRESS_BAR_SLIDER_CHARACTER: char = '█';

pub fn progress_bar() -> InteractiveWidget {
    InteractiveWidget::default()
        .draw(draw_progress_bar)
        .on_mouse_down(on_click)
        .on_mouse_drag(on_click)
}

fn draw_progress_bar(_: InteractionState, app_state: AppState, area: Rect, buf: &mut Buffer) {
    let progress_bar_width = area.width - 4;
    let progress_bar_slider_position = progress_bar_width as f64 * app_state.play_progress;
    let bar_length = (progress_bar_slider_position - 1.0).max(0.0) as usize;

    let mut string: String = String::new();
    for _ in 0..bar_length { string.push(PROGRESS_BAR_CHARACTER); }

    string.push(PROGRESS_BAR_SLIDER_CHARACTER);

    Paragraph::new(Line::from(string).bold())
        .block(Block::bordered().padding(PADDING))
        .render(area, buf);
}

fn on_click(widget: &mut InteractiveWidget, mouse_position: Position, app_state: &mut AppState) {
    // Вычисляем ширину активной области слайдера
    let clickable_width = widget.area().width - PADDING.left - PADDING.right - BORDER_WIDTH;

    // Вычисляем позицию клика относительно начала слайдера
    let click_position = mouse_position.x as i16 - PADDING.left as i16;

    // Преобразуем позицию в значение от 0 до 1
    let normalized_position = click_position.clamp(0, clickable_width as i16) as f64;
    app_state.play_progress = normalized_position / clickable_width as f64;
}


