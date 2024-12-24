use std::io::Error;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind}, 
    layout::Position
};

use crate::app::AppState;

use super::{event_type::MouseEventType, InteractiveWidget};

/// Обработчик событий для всего приложения
#[derive(Default)]
pub struct EventHandler {
    components: Vec<InteractiveWidget>,
}

pub trait Handelable {
    fn handle_mouse_event(
        &mut self, event_type: MouseEventType, position: Position, app_state: &AppState
    );
}

impl EventHandler {
    pub fn register_component(&mut self, component: InteractiveWidget) -> InteractiveWidget {
        self.components.push(component.clone());
        component
    }

    pub fn handle_events(&mut self, app_state: &AppState) -> Result<(), Error> {
        match event::read()? {
            Event::Key(key_event) => self.handle_key_event(app_state, key_event),
            Event::Mouse(mouse_event) => self.handle_mouse_event(app_state, mouse_event),
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, app_state: &AppState, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                app_state.set_exit(true);
            },
            KeyCode::Enter => {
                if !app_state.get_input_string().is_empty() {
                    app_state.add_track(app_state.get_input_string().clone());
                    app_state.set_input_string(String::new());
                }
            },
            KeyCode::Char(char) => {
                let mut input = app_state.get_input_string();
                input.push(char);
                app_state.set_input_string(input);
            }
            _ => {}
        }
    }

        fn handle_mouse_event(&mut self, app_state: &AppState, mouse_event: MouseEvent) {
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
                    component.handle_mouse_event(
                        MouseEventType::Over,
                         relative_mouse_position, 
                         app_state
                    );
                }
                _ => {}
            }
        }
    }
}
