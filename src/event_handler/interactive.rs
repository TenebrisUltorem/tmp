use crate::app::AppState;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InteractiveState {
    Default,
    Pressed,
    Hovered
}

pub trait Interactive {

    fn get_state(&self) -> InteractiveState;
    fn set_state(&mut self, state: InteractiveState);

    fn handle_mouse_over(&mut self, app_state: &mut AppState);

    fn handle_mouse_leave(&mut self, app_state: &mut AppState);

    fn handle_mouse_down(&mut self, app_state: &mut AppState);

    fn change_state(&mut self, new_state: InteractiveState, app_state: &mut AppState) {
        if self.get_state() != new_state {
            app_state.string += format!("Interactive state changed from {:?} to {:?} \n", 
                self.get_state(), new_state).as_str();
            self.set_state(new_state);
            match new_state {
                InteractiveState::Default => self.handle_mouse_leave(app_state),
                InteractiveState::Pressed => self.handle_mouse_down(app_state),
                InteractiveState::Hovered => self.handle_mouse_over(app_state),
            }
        }
    }

}