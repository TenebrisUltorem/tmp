use ratatui::widgets::BorderType;

use super::Button;

const PLAY_BUTTON_LABEL: &str = "▷";
const PLAY_BUTTON_LABEL_OVER: &str = "▶";
const PLAY_BUTTON_LABEL_PRESSED: &str = "▸";

const STOP_BUTTON_LABEL: &str = "□";
const STOP_BUTTON_LABEL_OVER: &str = "■";
const STOP_BUTTON_LABEL_PRESSED: &str = "▪";

const LAST_TRACK_BUTTON_LABEL: &str = "|◁◁";
const LAST_TRACK_BUTTON_LABEL_OVER: &str = "|◀◀";
const LAST_TRACK_BUTTON_LABEL_PRESSED: &str = "|◂◂";

const NEXT_TRACK_BUTTON_LABEL: &str = "▷▷|";
const NEXT_TRACK_BUTTON_LABEL_OVER: &str = "▶▶|";
const NEXT_TRACK_BUTTON_LABEL_PRESSED: &str = "▸▸|";

pub fn play_button() -> Button {
    Button::new(PLAY_BUTTON_LABEL)
        .on_mouse_down(|me, app_state| { 
            app_state.string += "Play button clicked \n";
            me.set_label(PLAY_BUTTON_LABEL_PRESSED);
        })
        .on_mouse_over(|me, _|{
            me.set_label(PLAY_BUTTON_LABEL_OVER);
            me.set_border_type(BorderType::Thick);
        })
        .on_mouse_leave(|me, _|{
            me.set_label(PLAY_BUTTON_LABEL);
            me.set_border_type(BorderType::Plain);
        })
}

pub fn last_track_button() -> Button {
    Button::new(LAST_TRACK_BUTTON_LABEL)
        .on_mouse_down(|me, app_state| { 
            app_state.string += "Last track button clicked \n";
            me.set_label(LAST_TRACK_BUTTON_LABEL_PRESSED);
        })
        .on_mouse_over(|me, _|{
            me.set_label(LAST_TRACK_BUTTON_LABEL_OVER);
            me.set_border_type(BorderType::Thick);
        })
        .on_mouse_leave(|me, _|{
            me.set_label(LAST_TRACK_BUTTON_LABEL);
            me.set_border_type(BorderType::Plain);
        })
}   

pub fn next_track_button() -> Button {
    Button::new(NEXT_TRACK_BUTTON_LABEL)
        .on_mouse_down(|me, app_state| { 
            app_state.string += "Next track button clicked \n";
            me.set_label(NEXT_TRACK_BUTTON_LABEL_PRESSED);
        })
        .on_mouse_over(|me, _|{
            me.set_label(NEXT_TRACK_BUTTON_LABEL_OVER);
            me.set_border_type(BorderType::Thick);
        })
        .on_mouse_leave(|me, _|{
            me.set_label(NEXT_TRACK_BUTTON_LABEL);
            me.set_border_type(BorderType::Plain);
        })
}   

pub fn stop_button() -> Button {
    Button::new(STOP_BUTTON_LABEL)
        .on_mouse_down(|me, app_state| { 
            app_state.string += "Stop button clicked \n";
            me.set_label(STOP_BUTTON_LABEL_PRESSED);
        })
        .on_mouse_over(|me, _|{
            me.set_label(STOP_BUTTON_LABEL_OVER);
            me.set_border_type(BorderType::Thick);
        })
        .on_mouse_leave(|me, _|{
            me.set_label(STOP_BUTTON_LABEL);
            me.set_border_type(BorderType::Plain);
        })
}   