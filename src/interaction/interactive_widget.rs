use ratatui::{
    buffer::Buffer, crossterm::event::KeyCode, layout::{Position, Rect}, widgets::Widget
};
use std::{collections::HashMap, sync::{Arc, Mutex}};

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
type MouseHandler = dyn Fn(&mut InteractiveWidget, Position) + Send + Sync;

/// Тип обработки события клавиатуры
type KeyboardHandler = dyn Fn(&mut InteractiveWidget, KeyCode) + Send + Sync;

/// Тип функции отрисовки
type DrawHandler = dyn Fn(InteractionState, Rect, &mut Buffer) + Send + Sync;

/// Тип функции обработки события вставки
type PasteHandler = dyn Fn(&mut InteractiveWidget, String) + Send + Sync;

/// Интерактивный виджет с поддержкой событий мыши
#[derive(Default, Clone)]
pub struct InteractiveWidget {
    area: Arc<Mutex<Rect>>,
    state: Arc<Mutex<InteractionState>>,
    on_mouse_down_fn: Option<Arc<MouseHandler>>,
    on_mouse_drag_fn: Option<Arc<MouseHandler>>,
    on_mouse_scroll_up_fn: Option<Arc<MouseHandler>>,
    on_mouse_scroll_down_fn: Option<Arc<MouseHandler>>,
    on_paste_fn: Option<Arc<PasteHandler>>,
    on_key_down_fns: Arc<Mutex<HashMap<KeyCode, Box<KeyboardHandler>>>>,
    draw_fn: Option<Arc<DrawHandler>>
}

impl InteractiveWidget {
    // Builder методы
    pub fn on_mouse_down<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut InteractiveWidget, Position) + Send + Sync + 'static,
    {
        self.on_mouse_down_fn = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_drag<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut InteractiveWidget, Position) + Send + Sync + 'static,
    {
        self.on_mouse_drag_fn = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_scroll_up<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut InteractiveWidget, Position) + Send + Sync + 'static,
    {
        self.on_mouse_scroll_up_fn = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_scroll_down<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut InteractiveWidget, Position) + Send + Sync + 'static,
    {
        self.on_mouse_scroll_down_fn = Some(Arc::new(handler));
        self
    }

    pub fn on_paste<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut InteractiveWidget, String) + Send + Sync + 'static,
    {
        self.on_paste_fn = Some(Arc::new(handler));
        self
    }

    pub fn on_key_down<F>(self, key: KeyCode, handler: F) -> Self
    where
        F: Fn(&mut InteractiveWidget, KeyCode) + Send + Sync + 'static,
    {
        if let Ok(mut on_key_down_fns) = self.on_key_down_fns.lock() {
            on_key_down_fns.insert(key, Box::new(handler));
        }
        self
    }

    pub fn draw<F>(mut self, draw_fn: F) -> Self
    where
        F: Fn(InteractionState, Rect, &mut Buffer) + Send + Sync + 'static,
    {
        self.draw_fn = Some(Arc::new(draw_fn));
        self
    }

    // Геттеры и сеттеры
    pub fn area(&self) -> Rect {
        *self.area.lock().unwrap()
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

impl Widget for &InteractiveWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.clone().set_area(area);

        if let Some(draw_fn) = &self.draw_fn.clone() {
            draw_fn(self.state(), area, buf);
        }
    }
}

impl Handelable for InteractiveWidget {
    // Обработчики событий
    fn handle_mouse_event(&mut self, event_type: MouseEventType, position: Position) {
        match event_type {
            MouseEventType::Over => self.set_state(InteractionState::Hovered),
            MouseEventType::Out => self.set_state(InteractionState::Default),
            MouseEventType::Down => {
                if let Some(handler) = &self.on_mouse_down_fn.clone() {
                    self.set_state(InteractionState::Pressed);
                    handler(self, position);
                }
            }
            MouseEventType::Drag => {
                if let Some(handler) = &self.on_mouse_drag_fn.clone() {
                    self.set_state(InteractionState::Pressed);
                    handler(self, position);
                }
            }
            MouseEventType::ScrollUp => {
                if let Some(handler) = &self.on_mouse_scroll_up_fn.clone() {
                    handler(self, position);
                }
            }
            MouseEventType::ScrollDown => {
                if let Some(handler) = &self.on_mouse_scroll_down_fn.clone() {
                    handler(self, position);
                }
            }
        }
    }

    fn handle_key_event(&mut self, key_code: KeyCode) {
        if let Ok(on_key_down_fns) = self.on_key_down_fns.clone().lock() {
            if let Some(handler) = on_key_down_fns.get(&key_code).clone() {
                handler(self, key_code);
            }
        }
    }

    fn handle_paste_event(&mut self, paste_event: String) {
        if let Some(handler) = &self.on_paste_fn.clone() {
            handler(self, paste_event);
        }
    }
}
