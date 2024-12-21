mod button;
pub use button::Button;

use button::ButtonAppearance;
use ratatui::widgets::BorderType;

use crate::event_handler::{InteractionActions, InteractiveState};

pub fn play_pause_button() -> Button {
    let default_appearance = ButtonAppearance::new("▷");
    let pressed_appearance = ButtonAppearance::new("▸");
    let hovered_appearance = ButtonAppearance::new("▶").with_border(BorderType::Thick);

    let actions = InteractionActions::default()
        .on_mouse_down(|button, app_state| { 
            button.set_state(InteractiveState::Pressed);
            app_state.string += "Play button clicked \n";
        })
        .on_mouse_up(|button, _| {
            button.set_state(InteractiveState::Default);
        })
        .on_mouse_over(|button, _| {
            button.set_state(InteractiveState::Hovered);
        });

    Button::new(default_appearance)
        .set_appearance_on(InteractiveState::Pressed, pressed_appearance)
        .set_appearance_on(InteractiveState::Hovered, hovered_appearance)
        .set_actions(actions)
}

pub fn last_track_button() -> Button {
    let default_appearance = ButtonAppearance::new("|◁◁");
    let pressed_appearance = ButtonAppearance::new("|◂◂");
    let hovered_appearance = ButtonAppearance::new("|◀◀").with_border(BorderType::Thick);

    let actions = InteractionActions::default()
        .on_mouse_down(|button, app_state| { 
            button.set_state(InteractiveState::Pressed);
            app_state.string += "Last track button clicked \n";
        })
        .on_mouse_up(|button, _| {
            button.set_state(InteractiveState::Default);
        })
        .on_mouse_over(|button, _| {
            button.set_state(InteractiveState::Hovered);
        });
    
    Button::new(default_appearance)
        .set_appearance_on(InteractiveState::Pressed, pressed_appearance)
        .set_appearance_on(InteractiveState::Hovered, hovered_appearance)
        .set_actions(actions)
}   

pub fn next_track_button() -> Button {
    let default_appearance = ButtonAppearance::new("▷▷|");
    let pressed_appearance = ButtonAppearance::new("▸▸|");
    let hovered_appearance = ButtonAppearance::new("▶▶|").with_border(BorderType::Thick);

    let actions = InteractionActions::default()
        .on_mouse_down(|button, app_state| { 
            button.set_state(InteractiveState::Pressed);
            app_state.string += "Next track button clicked \n";
        })
        .on_mouse_up(|button, _| {
            button.set_state(InteractiveState::Default);
        })
        .on_mouse_over(|button, _| {
            button.set_state(InteractiveState::Hovered);
        });

    Button::new(default_appearance)
        .set_appearance_on(InteractiveState::Pressed, pressed_appearance)
        .set_appearance_on(InteractiveState::Hovered, hovered_appearance)
        .set_actions(actions)
}   

pub fn stop_button() -> Button {
    let default_appearance = ButtonAppearance::new("□");
    let pressed_appearance = ButtonAppearance::new("▪");
    let hovered_appearance = ButtonAppearance::new("■").with_border(BorderType::Thick);

    let actions = InteractionActions::default()
        .on_mouse_down(|button, app_state| { 
            button.set_state(InteractiveState::Pressed);
            app_state.string += "Stop button clicked \n";
        })
        .on_mouse_up(|button, _| {
            button.set_state(InteractiveState::Default);
        })
        .on_mouse_over(|button, _| {
            button.set_state(InteractiveState::Hovered);
        });

    Button::new(default_appearance)
        .set_appearance_on(InteractiveState::Pressed, pressed_appearance)
        .set_appearance_on(InteractiveState::Hovered, hovered_appearance)
        .set_actions(actions)
}

pub fn mixin_toggle() -> Button {
    let default_appearance = ButtonAppearance::new("△▽");
    let active_appearance = ButtonAppearance::new("▲▼");
    let hovered_appearance = ButtonAppearance::new("▲▼").with_border(BorderType::Thick);
    let pressed_appearance = ButtonAppearance::new("▴▾").with_border(BorderType::Thick);

    let actions = InteractionActions::default()
        .on_mouse_down(|button, app_state| { 
            app_state.mixin_state = !app_state.mixin_state;
            button.set_state(InteractiveState::Pressed);
            app_state.string += format!("Mixin toggle switched to: {}\n", app_state.mixin_state).as_str();
        })
        .on_mouse_up(|button, app_state| {
            if app_state.mixin_state {
                button.set_state(InteractiveState::Active);
            }
            else {
                button.set_state(InteractiveState::Default);
            }
        })
        .on_mouse_over(|button, _| {
            button.set_state(InteractiveState::Hovered);
        });

    Button::new(default_appearance)
        .set_appearance_on(InteractiveState::Pressed, pressed_appearance)
        .set_appearance_on(InteractiveState::Active, active_appearance)
        .set_appearance_on(InteractiveState::Hovered, hovered_appearance)
        .set_actions(actions)
}

pub fn repeat_toggle() -> Button {
    let default_appearance = ButtonAppearance::new("⮎⮌");
    let active_appearance = ButtonAppearance::new("⮬⮯");
    let hovered_appearance = ButtonAppearance::new("⮬⮯").with_border(BorderType::Thick);
    let pressed_appearance = ButtonAppearance::new("R").with_border(BorderType::Thick);

    let actions = InteractionActions::default()
        .on_mouse_down(|button, app_state| { 
            app_state.repeat_state = !app_state.repeat_state;
            button.set_state(InteractiveState::Pressed);
            app_state.string += format!("Repeat toggle switched to: {}\n", app_state.repeat_state).as_str();
        })
        .on_mouse_up(|button, app_state| {
            if app_state.repeat_state {
                button.set_state(InteractiveState::Active);
            }
            else {
                button.set_state(InteractiveState::Default);
            }
        })
        .on_mouse_over(|button, _| {
            button.set_state(InteractiveState::Hovered);
        });

    Button::new(default_appearance)
        .set_appearance_on(InteractiveState::Pressed, pressed_appearance)
        .set_appearance_on(InteractiveState::Active, active_appearance)
        .set_appearance_on(InteractiveState::Hovered, hovered_appearance)
        .set_actions(actions)
}