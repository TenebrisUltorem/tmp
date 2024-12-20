mod interactive;
pub use interactive::Interactive;
pub use interactive::InteractiveState;

use std::{collections::HashMap, io::Error};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::{Position, Rect};

use crate::app::AppState;

#[derive(Default)]
pub struct EventHandler {
    components: HashMap<Rect, Box<dyn Interactive>>
}

impl EventHandler {

    pub fn register_component(&mut self, component: Box<dyn Interactive>, area: Rect) {
        self.components.insert(area, component);
    }

    pub fn handle_events(&mut self, app_state: &mut AppState) -> Result<(), Error> {
        let event = event::read()?;
        match event {
            Event::Key(key_event) => { self.handle_key_event(app_state, key_event); }
            Event::Mouse(mouse_event) => { self.handle_mouse_event(app_state, mouse_event); }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, app_state: &mut AppState, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => { app_state.exit = true; }
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, app_state: &mut AppState, mouse_event: MouseEvent) {
        let mouse_position = Position::new(mouse_event.column, mouse_event.row);
        for (area, component) in &mut self.components {
            match mouse_event.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    if area.contains(mouse_position) { 
                        component.change_state(InteractiveState::Pressed, app_state); }
                }
                MouseEventKind::Up(MouseButton::Left) | MouseEventKind::Moved => { 
                    if area.contains(mouse_position) { 
                        component.change_state(InteractiveState::Hovered, app_state); 
                    }
                    else { 
                        component.change_state(InteractiveState::Default, app_state); 
                    }
                }
                _ => { app_state.string = format!("{:?} \n", mouse_event); }
            }
        }
    }
}
