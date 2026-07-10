use super::{State, Transition};
use crate::types::{
    ActiveWidget,
    AppAction::{self, Quit},
};

pub struct CommandMode;

impl State for CommandMode {
    fn handle_input(&self, input: char, _active_widget: &ActiveWidget) -> Transition {
        //TODO: Add save commands, and exit command state

        if input == 'q' {
            return Transition::Action(AppAction::Quit);
        }
        Transition::Stay
    }
}
