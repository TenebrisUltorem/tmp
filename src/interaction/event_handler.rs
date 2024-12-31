use std::{
    io::Error,
    sync::{Arc, Mutex},
    thread,
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind},
    layout::Position,
};

use crate::app::AppState;

use super::{event_type::MouseEventType, InteractiveWidget};

/// Обработчик событий для всего приложения
#[derive(Clone)]
pub struct EventHandler {
    app_state: AppState,
    components: Arc<Mutex<Vec<InteractiveWidget>>>,
}

pub trait Handelable {
    /// Обработка событий мыши
    /// 
    /// # Args:
    /// * `event_type` - Тип события мыши
    /// * `relative_position` - Позиция события мыши (относительно виджета)
    fn handle_mouse_event(&mut self, event_type: MouseEventType, relative_position: Position);

    /// Обработка событий клавиатуры
    /// 
    /// # Args:
    /// * `key_code` - Код клавиши
    fn handle_key_event(&mut self, key_code: KeyCode);

    /// Обработка событий вставки
    /// 
    /// # Args:
    /// * `paste_event` - Событие вставки
    fn handle_paste_event(&mut self, paste_event: String);
}

impl EventHandler {
    pub fn new(app_state: &AppState) -> Self {
        Self { app_state: app_state.clone(), components: Arc::new(Mutex::new(Vec::new())) }
    }

    pub fn register_component(&mut self, component: InteractiveWidget) -> InteractiveWidget {
        self.components.lock().unwrap().push(component.clone());
        component
    }

    pub fn start(&mut self) -> Result<(), Error> {
        let event_handler = self.clone();

        thread::spawn(move || -> Result<(), Error> {
            while !event_handler.app_state.should_exit() {
                match event::read()? {
                    Event::Key(key_event) => event_handler.handle_key_event(key_event),
                    Event::Mouse(mouse_event) => event_handler.handle_mouse_event(mouse_event),
                    Event::Paste(paste_event) => event_handler.handle_paste_event(paste_event),
                    _ => {}
                };
            }

            Ok(())
        });
        Ok(())
    }

    fn handle_key_event(&self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.app_state.set_exit(true);
            }
            _ => {
                for component in self.components.lock().unwrap().iter_mut() {
                    component.handle_key_event(key_event.code);
                }
            }
        }
    }
 
    fn handle_mouse_event(&self, mouse_event: MouseEvent) {
        let mouse_position = Position::new(mouse_event.column, mouse_event.row);

        for component in self.components.lock().unwrap().iter_mut() {
            let area = component.area();
            if !area.contains(mouse_position) {
                component.handle_mouse_event(MouseEventType::Out, mouse_position);
                continue;
            }

            let relative_mouse_position = Position::new(mouse_position.x - area.x, mouse_position.y - area.y);

            match mouse_event.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    component.handle_mouse_event(MouseEventType::Down, relative_mouse_position)
                }
                MouseEventKind::Drag(MouseButton::Left) => {
                    component.handle_mouse_event(MouseEventType::Drag, relative_mouse_position)
                }
                MouseEventKind::ScrollDown => {
                    component.handle_mouse_event(MouseEventType::ScrollDown, relative_mouse_position)
                }
                MouseEventKind::ScrollUp => {
                    component.handle_mouse_event(MouseEventType::ScrollUp, relative_mouse_position)
                }
                MouseEventKind::Up(_) | MouseEventKind::Moved => {
                    component.handle_mouse_event(MouseEventType::Over, relative_mouse_position)
                }
                _ => {}
            }
        }
    }

    fn handle_paste_event(&self, paste_event: String) {
        for component in self.components.lock().unwrap().iter_mut() {
            component.handle_paste_event(paste_event.clone());
        }
    }
}
