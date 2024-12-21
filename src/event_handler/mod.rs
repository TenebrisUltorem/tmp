use std::{collections::HashMap, io::Error};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::{Position, Rect};

use crate::app::AppState;

use std::sync::Arc;

#[derive(Default, Clone)]
pub struct InteractionActions {
    on_mouse_over_handler: Option<Arc<dyn Fn(&mut dyn Interactive, &mut AppState)>>,
    on_mouse_down_handler: Option<Arc<dyn Fn(&mut dyn Interactive, &mut AppState)>>,
    on_mouse_up_handler: Option<Arc<dyn Fn(&mut dyn Interactive, &mut AppState)>>
}

impl InteractionActions {
    pub fn on_mouse_over(mut self, handler: fn(&mut dyn Interactive, &mut AppState)) -> Self{
        self.on_mouse_over_handler = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_down(mut self, handler: fn(&mut dyn Interactive, &mut AppState)) -> Self{
        self.on_mouse_down_handler = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_up(mut self, handler: fn(&mut dyn Interactive, &mut AppState)) -> Self{
        self.on_mouse_up_handler = Some(Arc::new(handler));
        self
    }

    fn handle_on_mouse_over(&self, component: &mut dyn Interactive, app_state: &mut AppState) {
        if let Some(handler) = &self.on_mouse_over_handler {
            handler(component, app_state);
        }
    }

    fn handle_on_mouse_down(&self, component: &mut dyn Interactive, app_state: &mut AppState) {
        if let Some(handler) = &self.on_mouse_down_handler {
            handler(component, app_state);
        }
    }

    fn handle_on_mouse_up(&self, component: &mut dyn Interactive, app_state: &mut AppState) {
        if let Some(handler) = &self.on_mouse_up_handler {
            handler(component, app_state);
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum InteractiveState {
    Default,
    Hovered,
    Pressed,
    Active
}

pub trait Interactive {
    fn actions(&self) -> InteractionActions;
    fn state(&self) -> InteractiveState;
    fn set_state(&mut self, state: InteractiveState);
}

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
            let component = component.as_mut();

            match mouse_event.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    if area.contains(mouse_position) { 
                        component.actions().handle_on_mouse_down(component, app_state);
                    }
                }
                MouseEventKind::Up(MouseButton::Left) | MouseEventKind::Moved => { 
                    if area.contains(mouse_position) { 
                        component.actions().handle_on_mouse_over(component, app_state);
                    }
                    else {
                        component.actions().handle_on_mouse_up(component, app_state);
                    }
                }
                _ => { app_state.string = format!("{:?} \n", mouse_event); }
            }
        }
    }
}
