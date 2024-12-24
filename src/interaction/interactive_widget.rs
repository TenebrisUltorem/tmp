use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
};
use std::sync::{Arc, Mutex};

use crate::app::AppState;

use super::{event_handler::Handelable, event_type::MouseEventType};

/// Состояние интерактивного виджета
#[derive(Default, Eq, PartialEq, Hash, Clone)]
pub enum InteractionState {
    #[default]
    Default,
    Hovered,
    Pressed,
}

/// Тип обработчика событий мыши
type MouseHandler = dyn Fn(&mut InteractiveWidget, Position, &AppState);
/// Тип функции отрисовки
type DrawHandler = dyn Fn(InteractionState, &AppState, Rect, &mut Buffer);

/// Интерактивный виджет с поддержкой событий мыши
#[derive(Default, Clone)]
pub struct InteractiveWidget {
    area: Arc<Mutex<Rect>>,
    state: Arc<Mutex<InteractionState>>,
    on_mouse_down_fn: Option<Arc<MouseHandler>>,
    on_mouse_drag_fn: Option<Arc<MouseHandler>>,
    on_mouse_scroll_up_fn: Option<Arc<MouseHandler>>,
    on_mouse_scroll_down_fn: Option<Arc<MouseHandler>>,
    draw_fn: Option<Arc<DrawHandler>>,
}

impl InteractiveWidget {
    // Builder методы
    pub fn on_mouse_down(mut self, handler: fn(&mut InteractiveWidget, Position, &AppState)) -> Self {
        self.on_mouse_down_fn = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_drag(mut self, handler: fn(&mut InteractiveWidget, Position, &AppState)) -> Self {
        self.on_mouse_drag_fn = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_scroll_up(
        mut self, handler: fn(&mut InteractiveWidget, Position, &AppState),
    ) -> Self {
        self.on_mouse_scroll_up_fn = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_scroll_down(
        mut self, handler: fn(&mut InteractiveWidget, Position, &AppState),
    ) -> Self {
        self.on_mouse_scroll_down_fn = Some(Arc::new(handler));
        self
    }

    pub fn draw(mut self, draw_fn: fn(InteractionState, &AppState, Rect, &mut Buffer)) -> Self {
        self.draw_fn = Some(Arc::new(draw_fn));
        self
    }

    // Методы отрисовки
    pub fn render(&mut self, app_state: &AppState, area: Rect, buf: &mut Buffer) {
        self.set_area(area);
        if let Some(draw_fn) = &self.draw_fn.clone() {
            draw_fn(self.state(), app_state, area, buf);
        }
    }

    // Геттеры и сеттеры
    pub fn area(&self) -> Rect {
        self.area.lock().unwrap().clone()
    }

    pub fn set_area(&mut self, new_area: Rect) {
        if let Ok(mut area) = self.area.lock() {
            *area = new_area;
        }
    }

    pub fn state(&self) -> InteractionState {
        self.state.lock().unwrap().clone()
    }

    pub fn set_state(&mut self, new_state: InteractionState) {
        if let Ok(mut state) = self.state.lock() {
            if *state != new_state {
                *state = new_state;
            }
        }
    }
}

impl Handelable for InteractiveWidget {
    // Обработчики событий
    fn handle_mouse_event(
        &mut self, event_type: MouseEventType, position: Position, app_state: &AppState,
    ) {
        match event_type {
            MouseEventType::Over => self.set_state(InteractionState::Hovered),
            MouseEventType::Out => self.set_state(InteractionState::Default),
            MouseEventType::Down => {
                if let Some(handler) = &self.on_mouse_down_fn.clone() {
                    self.set_state(InteractionState::Pressed);
                    handler(self, position, app_state);
                }
            }
            MouseEventType::Drag => {
                if let Some(handler) = &self.on_mouse_drag_fn.clone() {
                    self.set_state(InteractionState::Pressed);
                    handler(self, position, app_state);
                }
            }
            MouseEventType::ScrollUp => {
                if let Some(handler) = &self.on_mouse_scroll_up_fn.clone() {
                    handler(self, position, app_state);
                }
            }
            MouseEventType::ScrollDown => {
                if let Some(handler) = &self.on_mouse_scroll_down_fn.clone() {
                    handler(self, position, app_state);
                }
            }
        }
    }
    
}