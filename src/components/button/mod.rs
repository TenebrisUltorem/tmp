pub mod buttons;

use std::sync::{Arc, Mutex};

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};

use crate::app::AppState;

use crate::event_handler::{Interactive, InteractiveState};

struct ButtonState {
    interactive_state: InteractiveState,
    label: String,
    border_type: BorderType,
}

#[derive(Clone)]
pub struct Button {
    state: Arc<Mutex<ButtonState>>,
    mouse_down_handler: Option<Arc<dyn Fn(&mut Button, &mut AppState)>>,
    mouse_over_handler: Option<Arc<dyn Fn(&mut Button, &mut AppState)>>,
    mouse_leave_handler: Option<Arc<dyn Fn(&mut Button, &mut AppState)>>
}

impl Button {

    pub fn new(label: impl Into<String>) -> Self {
        Self { 
            state: Arc::new(Mutex::new(ButtonState {
                interactive_state: InteractiveState::Default,
                label: label.into(),
                border_type: BorderType::Plain
            })),
            mouse_down_handler: None,
            // Устанавливаем дефолтный обработчик для mouse_over
            mouse_over_handler: Some(Arc::new(|button: &mut Button, _: &mut AppState| {
                button.set_border_type(BorderType::Thick);
            })),
            // Устанавливаем дефолтный обработчик для mouse_leave
            mouse_leave_handler: Some(Arc::new(|button: &mut Button, _: &mut AppState| {
                button.set_border_type(BorderType::Plain);
            }))
        }
    }

    pub fn on_mouse_down(mut self, handler: fn(&mut Button, &mut AppState)) -> Self {
        self.mouse_down_handler = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_over(mut self, handler: fn(&mut Button, &mut AppState)) -> Self {
        self.mouse_over_handler = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_leave(mut self, handler: fn(&mut Button, &mut AppState)) -> Self {
        self.mouse_leave_handler = Some(Arc::new(handler));
        self
    }
    
    pub fn border_type(&self) -> BorderType {
        self.state.lock().unwrap().border_type
    }

    pub fn set_border_type(&self, border_type: BorderType) {
        self.state.lock().unwrap().border_type = border_type;
    }

    pub fn label(&self) -> String {
        self.state.lock().unwrap().label.clone()
    }

    pub fn set_label(&self, label: impl Into<String>) {
        self.state.lock().unwrap().label = label.into();
    }
}

impl Widget for &Button {

    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Line::from(self.label()).centered())
            .block(Block::bordered().border_type(self.border_type()))
            .render(area, buf);
    }
    
}

impl Interactive for Button {
    fn handle_mouse_over(&mut self, app_state: &mut AppState) {
        if let Some(handler) = &self.mouse_over_handler.clone() {
            handler(self, app_state);
        }
    }

    fn handle_mouse_leave(&mut self, app_state: &mut AppState) {
        if let Some(handler) = &self.mouse_leave_handler.clone() {
            let handler = handler.as_ref();
            handler(self, app_state);
        }
    }

    fn handle_mouse_down(&mut self, app_state: &mut AppState) {
        if let Some(handler) = &self.mouse_down_handler.clone() {
            let handler = handler.as_ref();
            handler(self, app_state);
        }
    }

    fn get_state(&self) -> InteractiveState {
        self.state.lock().unwrap().interactive_state
    }

    fn set_state(&mut self, state: InteractiveState) {
        self.state.lock().unwrap().interactive_state = state;
    }
}
