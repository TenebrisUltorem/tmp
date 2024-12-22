use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind},
    layout::{Position, Rect},
};
use std::{
    io::Error,
    sync::{Arc, Mutex},
};

use crate::app::AppState;

/// Состояние интерактивного виджета
#[derive(Default, Eq, PartialEq, Hash, Clone)]
pub enum InteractionState {
    #[default]
    Default,
    Hovered,
    Pressed,
}

/// Тип обработчика событий мыши
type MouseHandler = dyn Fn(&mut InteractiveWidget, Position, &mut AppState);
/// Тип функции отрисовки
type DrawHandler = dyn Fn(InteractionState, AppState, Rect, &mut Buffer);

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
    pub fn on_mouse_down(mut self, handler: fn(&mut InteractiveWidget, Position, &mut AppState)) -> Self {
        self.on_mouse_down_fn = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_drag(mut self, handler: fn(&mut InteractiveWidget, Position, &mut AppState)) -> Self {
        self.on_mouse_drag_fn = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_scroll_up(
        mut self, handler: fn(&mut InteractiveWidget, Position, &mut AppState),
    ) -> Self {
        self.on_mouse_scroll_up_fn = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_scroll_down(
        mut self, handler: fn(&mut InteractiveWidget, Position, &mut AppState),
    ) -> Self {
        self.on_mouse_scroll_down_fn = Some(Arc::new(handler));
        self
    }

    pub fn draw(mut self, draw_fn: fn(InteractionState, AppState, Rect, &mut Buffer)) -> Self {
        self.draw_fn = Some(Arc::new(draw_fn));
        self
    }

    // Методы отрисовки
    pub fn render(&mut self, app_state: AppState, area: Rect, buf: &mut Buffer) {
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

    // Обработчики событий
    fn handle_mouse_event(
        &mut self, event_type: MouseEventType, position: Position, app_state: &mut AppState,
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

/// Типы событий мыши
#[derive(Debug)]
enum MouseEventType {
    Over,
    Out,
    Down,
    Drag,
    ScrollUp,
    ScrollDown,
}

/// Обработчик событий для всего приложения
#[derive(Default)]
pub struct EventHandler {
    components: Vec<InteractiveWidget>,
}

impl EventHandler {
    pub fn register_component(&mut self, component: InteractiveWidget) -> InteractiveWidget {
        self.components.push(component.clone());
        component
    }

    pub fn handle_events(&mut self, app_state: &mut AppState) -> Result<(), Error> {
        match event::read()? {
            Event::Key(key_event) => self.handle_key_event(app_state, key_event),
            Event::Mouse(mouse_event) => self.handle_mouse_event(app_state, mouse_event),
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, app_state: &mut AppState, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                app_state.exit = true;
            }
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, app_state: &mut AppState, mouse_event: MouseEvent) {
        let mouse_position = Position::new(mouse_event.column, mouse_event.row);

        for component in &mut self.components {
            let area = component.area();
            if !area.contains(mouse_position) {
                component.handle_mouse_event(MouseEventType::Out, mouse_position, app_state);
                continue;
            }

            let relative_mouse_position = Position::new(mouse_position.x - area.x, mouse_position.y - area.y);

            match mouse_event.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    component.handle_mouse_event(MouseEventType::Down, relative_mouse_position, app_state);
                }
                MouseEventKind::Drag(MouseButton::Left) => {
                    component.handle_mouse_event(MouseEventType::Drag, relative_mouse_position, app_state);
                }
                MouseEventKind::ScrollDown => {
                    component.handle_mouse_event(
                        MouseEventType::ScrollDown,
                        relative_mouse_position,
                        app_state,
                    );
                }
                MouseEventKind::ScrollUp => {
                    component.handle_mouse_event(
                        MouseEventType::ScrollUp,
                        relative_mouse_position,
                        app_state,
                    );
                }
                MouseEventKind::Up(_) | MouseEventKind::Moved => {
                    component.handle_mouse_event(MouseEventType::Over, relative_mouse_position, app_state);
                }
                _ => {}
            }
        }
    }
}
