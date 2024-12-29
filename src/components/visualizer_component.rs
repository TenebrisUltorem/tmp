use rand::Rng;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::app::AppState;

const INFO_PADDING: Padding = Padding::new(2, 2, 1, 1);

const BAR_SYMBOL: char = '⣿';

pub struct VisualizerComponent {
    app_state: AppState,
}

impl VisualizerComponent {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }
}

impl Widget for VisualizerComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let frame = Block::bordered();
        let frame_area = frame.inner(area);
        frame.render(area, buf);

        let [info_area, visualizer_area, debug_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Min(5), Constraint::Length(1)])
            .areas(frame_area);

        if let Some(info) = self.app_state.current_track_info() {
            Paragraph::new(vec![
                Line::from(info.title).bold(),
                Line::from(format!("{} - {}", info.artist, info.album).italic()),
            ])
            .block(Block::default().padding(INFO_PADDING))
            .render(info_area, buf);
        }

        render_visualizer(visualizer_area, buf);

        let text = format!("{}\n{}", self.app_state.debug_string(), self.app_state.input_string());
        Paragraph::new(text).render(debug_area, buf);
    }
}

fn render_visualizer(area: Rect, buf: &mut Buffer) {
    //TODO: использовать реальные данные и выпилить рандом (либу rand тоже надо выпилить)
    let mut random = rand::thread_rng();
    let amplitudes = (0..area.width).map(|_| random.gen()).collect::<Vec<f32>>();

    let bar_heights = amplitudes
        .iter().map(|amp| (amp * area.height as f32) as u16).collect::<Vec<u16>>();

    let lines = (0..area.height).rev().map(|y| {
        let mut line = String::new();
        bar_heights.iter().for_each(|height| {
            line.push(if y < *height { BAR_SYMBOL } else { ' ' });
        });
        line
    });

    Paragraph::new(lines.map(|line| Line::from(line)).collect::<Vec<Line>>()).render(area, buf);
}
