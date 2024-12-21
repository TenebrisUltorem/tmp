use std::io::Error;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::{Position, Rect};

use crate::app::AppState;

use std::sync::{Arc, Mutex};


#[derive(Default, Eq, PartialEq, Hash, Clone)]
pub enum InteractionState {
    #[default]
    Default,

    Hovered,

    Pressed
}

#[derive(Default, Clone)]
pub struct InteractiveWidget {
    area: Arc<Mutex<Rect>>,
    state: Arc<Mutex<InteractionState>>,
    on_mouse_over_fn: Option<Arc<dyn Fn(&mut InteractiveWidget, &mut AppState)>>,
    on_mouse_down_fn: Option<Arc<dyn Fn(&mut InteractiveWidget, &mut AppState)>>,
    on_mouse_up_fn: Option<Arc<dyn Fn(&mut InteractiveWidget, &mut AppState)>>,
    draw_fn: Option<Arc<dyn Fn(&mut InteractiveWidget, AppState, Rect, &mut Buffer)>>,
}

impl InteractiveWidget {

    pub fn on_mouse_over(mut self, handler: fn(&mut InteractiveWidget, &mut AppState)) -> Self{
        self.on_mouse_over_fn = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_down(mut self, handler: fn(&mut InteractiveWidget, &mut AppState)) -> Self{
        self.on_mouse_down_fn = Some(Arc::new(handler));
        self
    }

    pub fn on_mouse_up(mut self, handler: fn(&mut InteractiveWidget, &mut AppState)) -> Self{
        self.on_mouse_up_fn = Some(Arc::new(handler));
        self
    }

    pub fn draw(mut self, draw_fn: fn(&mut InteractiveWidget, AppState, Rect, &mut Buffer)) -> Self {
        self.draw_fn = Some(Arc::new(draw_fn));
        self
    }

    pub fn render(&mut self, app_state: AppState, area: Rect, buf: &mut Buffer) {
        if let Some(draw_fn) = &self.draw_fn.clone() {
            draw_fn(self, app_state, area, buf);
        }
    }

    pub fn area(&self) -> Rect { 
        self.area.lock().unwrap().clone() 
    }

    pub fn set_area(&mut self, new_area: Rect) {
        let mut area = self.area.lock().unwrap();
        *area = new_area;
    }

    pub fn state(&self) -> InteractionState { self.state.lock().unwrap().clone() }

    pub fn set_state(&mut self, new_state: InteractionState) {
        let mut state = self.state.lock().unwrap();
        if *state != new_state { *state = new_state; }
    }

    fn handle_on_mouse_over(&mut self, app_state: &mut AppState) {
        if let Some(handler) = &self.on_mouse_over_fn.clone() {
            handler(self, app_state);
        }
    }

    fn handle_on_mouse_down(&mut self, app_state: &mut AppState) {
        if let Some(handler) = &self.on_mouse_down_fn.clone() {
            handler(self, app_state);
        }
    }

    fn handle_on_mouse_up(&mut self, app_state: &mut AppState) {
        if let Some(handler) = &self.on_mouse_up_fn.clone() {
            handler(self, app_state);
        }
    }

}

#[derive(Default)]
pub struct EventHandler {
    components: Vec<InteractiveWidget>
}

impl EventHandler {

    pub fn register_component(&mut self, component: InteractiveWidget) {
        self.components.push(component);
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

        for component in &mut self.components {
            let area = component.area();

            match mouse_event.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    if area.contains(mouse_position) { 
                        component.handle_on_mouse_down(app_state);
                    }
                }
                MouseEventKind::Up(MouseButton::Left) | MouseEventKind::Moved => { 
                    if area.contains(mouse_position) { 
                        component.handle_on_mouse_over(app_state);
                    }
                    else {
                        component.handle_on_mouse_up(app_state);
                    }
                }
                _ => { app_state.string = format!("{:?} \n", mouse_event); }
            }
        }
    }
}
