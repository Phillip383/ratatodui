use crate::state::ActiveWidget;

use super::{State, Transition};

pub struct CommandMode;

impl State for CommandMode {
    fn handle_input(&self, input: char, active_widget: &ActiveWidget) -> Transition {
        //TODO: Add save commands, and exit command state
        if input == 'q' {
            return Transition::Command(String::from(input));
        }
        Transition::Stay
    }
}
