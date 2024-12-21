use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;

use crate::event_handler::InteractionState;
use crate::event_handler::InteractiveWidget;

const SHUFFLE_TOGGLE_LABEL: &str = "△▽";
const SHUFFLE_TOGGLE_LABEL_STRONG: &str = "▲▼";
const SHUFFLE_TOGGLE_LABEL_PRESSED: &str = "▴▾";

pub fn shuffle_toggle() -> InteractiveWidget {
    InteractiveWidget::default()
        .on_mouse_down(|button, app_state| { 
            app_state.shuffle_state = !app_state.shuffle_state;
            button.set_state(InteractionState::Pressed);
            app_state.string += format!("Shuffle toggle switched to: {}\n", app_state.shuffle_state).as_str();
        })
        .on_mouse_up(|button, _| {
            button.set_state(InteractionState::Default);
        })
        .on_mouse_over(|button, _| {
            button.set_state(InteractionState::Hovered);
        })
        .draw(|this, app_state, area, buf| {
            this.set_area(area);

            let view = match this.state() {
                InteractionState::Hovered => 
                    Paragraph::new(Line::from(SHUFFLE_TOGGLE_LABEL_STRONG).centered())
                        .block(Block::bordered().border_type(BorderType::Thick)),
                InteractionState::Pressed => 
                    Paragraph::new(Line::from(SHUFFLE_TOGGLE_LABEL_PRESSED).centered())
                        .block(Block::bordered()),
                _ => {
                    match app_state.shuffle_state {
                        true =>  
                            Paragraph::new(Line::from(SHUFFLE_TOGGLE_LABEL_STRONG).centered())
                                .block(Block::bordered()),
                        false => 
                            Paragraph::new(Line::from(SHUFFLE_TOGGLE_LABEL).centered())
                                .block(Block::bordered())
                    }
                }
                    
            };

            view.render(area, buf);
        })
}
