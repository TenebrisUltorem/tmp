use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};

use crate::event_handler::{InteractionActions, Interactive, InteractiveState};

#[derive(Clone)]
pub enum LabelAlignment {
    Left,
    Center,
    Right,
}


#[derive(Clone)]
pub struct ButtonAppearance {
    pub label: String,
    pub label_alignment: LabelAlignment,
    pub border_type: BorderType,
}

impl ButtonAppearance {
    pub fn new(label: impl Into<String>) -> Self {
        Self { 
            label: label.into(), 
            label_alignment: LabelAlignment::Center,
            border_type: BorderType::Plain
        }
    }

    pub fn with_alignment(mut self, alignment: LabelAlignment) -> Self {
        self.label_alignment = alignment;
        self
    }

    pub fn with_border(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }
}

#[derive(Clone)]
pub struct Button {
    button_state: Arc<Mutex<InteractiveState>>,
    appearance_map: Arc<Mutex<HashMap<InteractiveState, ButtonAppearance>>>,
    interaction_actions: Arc<Mutex<InteractionActions>>
}

impl Button {

    pub fn new(default_appearance: ButtonAppearance) -> Self {
        let mut appearance_map = HashMap::new();
        appearance_map.insert(InteractiveState::Default, default_appearance);

        Self { 
            button_state: Arc::new(Mutex::new(InteractiveState::Default)),
            interaction_actions: Arc::new(Mutex::new(InteractionActions::default())),
            appearance_map: Arc::new(Mutex::new(appearance_map)),
        }
    }

    pub fn set_actions(self, actions: InteractionActions) -> Self {
        *self.interaction_actions.lock().unwrap() = actions;
        self
    }

    pub fn set_appearance_on(self, state: InteractiveState, appearance: ButtonAppearance) -> Self {
        self.appearance_map.lock().unwrap().insert(state, appearance);
        self
    }

    pub fn set_state(&mut self, new_state: InteractiveState) {
        let mut state = self.button_state.lock().unwrap();

        if *state != new_state { *state = new_state; }
    }

    fn get_appearance(&self) -> ButtonAppearance {
        let map = self.appearance_map.lock().unwrap();
        map.get(&self.button_state.lock().unwrap().clone())
            .unwrap_or_else(|| map.get(&InteractiveState::Default).unwrap())
            .clone()
    }


}

impl Widget for &Button {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let appearance = self.get_appearance();
        let mut label = Line::from(appearance.label);

        match appearance.label_alignment {
            LabelAlignment::Left => { label = label.left_aligned(); }
            LabelAlignment::Center => { label = label.centered(); }
            LabelAlignment::Right => { label = label.right_aligned(); }
        }

        Paragraph::new(label)
            .block(Block::bordered().border_type(appearance.border_type))
            .render(area, buf);
    }
    
}

impl Interactive for Button {
    fn actions(&self) -> InteractionActions { self.interaction_actions.lock().unwrap().clone() }

    fn state(&self) -> InteractiveState { self.button_state.lock().unwrap().clone() }

    fn set_state(&mut self, new_state: InteractiveState) {
        let mut state = self.button_state.lock().unwrap();
        if *state != new_state { *state = new_state; }
    }
}
