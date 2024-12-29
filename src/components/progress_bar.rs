use std::time::Duration;

use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::{app::AppState, interaction::InteractiveWidget, player::Player};

const PADDING: Padding = Padding::new(1, 1, 0, 0);
const BORDER_WIDTH: u16 = 2;

const PROGRESS_BAR_CHARACTER: char = '━';
const PROGRESS_BAR_SLIDER_CHARACTER: char = '█';

pub fn progress_bar(app_state: &AppState, player: &Player) -> InteractiveWidget {
    InteractiveWidget::default()
        .draw({
            let app_state = app_state.clone();
            move |_, area, buf| draw_progress_bar(&app_state, area, buf)
        })
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
}

fn draw_progress_bar(app_state: &AppState, area: Rect, buf: &mut Buffer) {
    let progress_bar_width = area.width - 4;
    let progress_bar_slider_position = progress_bar_width as f64 * app_state.get_play_progress();
    let bar_length = (progress_bar_slider_position - 1.0).max(0.0) as usize;

    let mut string: String = String::new();
    for _ in 0..bar_length {
        string.push(PROGRESS_BAR_CHARACTER);
    }

    string.push(PROGRESS_BAR_SLIDER_CHARACTER);

    let mut border_block = Block::bordered().padding(PADDING);

    if let Some(info) = app_state.current_track_info() {
        let title = progress_bar_title(info.played_duration, info.duration);
        border_block = border_block.title(Line::from(title).right_aligned());
    }

    Paragraph::new(Line::from(string).bold()).block(border_block).render(area, buf);
}

fn on_click(widget: &mut InteractiveWidget, mouse_position: Position, app_state: &AppState, player: &Player) {
    // Вычисляем ширину активной области слайдера
    let clickable_width = widget.area().width - PADDING.left - PADDING.right - BORDER_WIDTH;

    // Вычисляем позицию клика относительно начала слайдера
    let click_position = mouse_position.x as i16 - PADDING.left as i16;

    // Выравниваем позицию клика относительно слайдера
    let normalized_position = click_position.clamp(0, clickable_width as i16) as f64;
    let progress_ratio = normalized_position / clickable_width as f64;
    app_state.set_play_progress(progress_ratio);
    player.seek(progress_ratio);
}

fn progress_bar_title(played_duration: Duration, full_duration: Duration) -> String {
    let played_duration = format_duration(played_duration);
    let full_duration = format_duration(full_duration);
    format!(" {} / {} ", played_duration, full_duration)
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs() % 60;
    let minutes = duration.as_secs() / 60;
    let minutes = minutes % 60;
    let hours = minutes / 60;

    let mut result = String::new();
    if hours > 0 {
        result.push_str(&format!("{}:", hours));
    }
    result.push_str(&format!("{:02}:{:02}", minutes, seconds));
    result
}
