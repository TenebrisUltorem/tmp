use std::{io::Error, sync::{Arc, Mutex}, thread};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind}, 
    layout::Position
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
    fn handle_mouse_event(
        &mut self, event_type: MouseEventType, position: Position, app_state: &AppState
    );
}

impl EventHandler {
    pub fn new(app_state: AppState) -> Self {
        Self {
            app_state,
            components: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn register_component(&mut self, component: InteractiveWidget) -> InteractiveWidget {
        self.components.lock().unwrap().push(component.clone());
        component
    }

    pub fn start(&mut self) -> Result<(), Error> {
        let mut event_handler = self.clone();

        thread::spawn(move || -> Result<(), Error> {
            while !event_handler.app_state.should_exit() {
                event_handler.handle_events()?;
            }

            Ok(())
        });
        Ok(())
    }

    pub fn handle_events(&mut self) -> Result<(), Error> {
        match event::read()? {
            Event::Key(key_event) => self.handle_key_event(key_event),
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.app_state.set_exit(true);
            },
            KeyCode::Enter => {
                if !self.app_state.get_input_string().is_empty() {
                    self.app_state.add_track(self.app_state.get_input_string().clone());
                    self.app_state.set_input_string(String::new());
                }
            },
            KeyCode::Char(char) => {
                let mut input = self.app_state.get_input_string();
                input.push(char);
                self.app_state.set_input_string(input);
            }
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        let mouse_position = Position::new(mouse_event.column, mouse_event.row);

        for component in self.components.lock().unwrap().iter_mut() {
            let area = component.area();
            if !area.contains(mouse_position) {
                component.handle_mouse_event(MouseEventType::Out, mouse_position, &self.app_state);
                continue;
            }

            let relative_mouse_position = Position::new(mouse_position.x - area.x, mouse_position.y - area.y);

            match mouse_event.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    component.handle_mouse_event(MouseEventType::Down, relative_mouse_position, &self.app_state);
                }
                MouseEventKind::Drag(MouseButton::Left) => {
                    component.handle_mouse_event(MouseEventType::Drag, relative_mouse_position, &self.app_state);
                }
                MouseEventKind::ScrollDown => {
                    component.handle_mouse_event(
                        MouseEventType::ScrollDown,
                        relative_mouse_position,
                        &self.app_state,
                    );
                }
                MouseEventKind::ScrollUp => {
                    component.handle_mouse_event(
                        MouseEventType::ScrollUp,
                        relative_mouse_position,
                        &self.app_state,
                    );
                }
                MouseEventKind::Up(_) | MouseEventKind::Moved => {
                    component.handle_mouse_event(
                        MouseEventType::Over,
                         relative_mouse_position, 
                         &self.app_state
                    );
                }
                _ => {}
            }
        }
    }
}
