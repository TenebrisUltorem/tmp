use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;

use crate::event_handler::InteractionState;
use crate::event_handler::InteractiveWidget;

const STOP_BUTTON_LABEL: &str = "▢";
const STOP_BUTTON_LABEL_HOVERED: &str = "■";
const STOP_BUTTON_LABEL_PRESSED: &str = "▪";

pub fn stop_button() -> InteractiveWidget {
    InteractiveWidget::default()
        .on_mouse_down(|button, app_state| { 
            button.set_state(InteractionState::Pressed);
            app_state.string += "Stop button clicked \n";
        })
        .on_mouse_up(|button, _| {
            button.set_state(InteractionState::Default);
        })
        .on_mouse_over(|button, _| {
            button.set_state(InteractionState::Hovered);
        })
        .draw(|this, _, area, buf| {
            this.set_area(area);

            let view = match this.state() {
                InteractionState::Hovered => 
                    Paragraph::new(Line::from(STOP_BUTTON_LABEL_HOVERED).centered())
                        .block(Block::bordered().border_type(BorderType::Thick)),
                InteractionState::Pressed => 
                    Paragraph::new(Line::from(STOP_BUTTON_LABEL_PRESSED).centered())
                        .block(Block::bordered()),
                _ => 
                    Paragraph::new(Line::from(STOP_BUTTON_LABEL).centered())
                        .block(Block::bordered())
            };

            view.render(area, buf);
        })
}
