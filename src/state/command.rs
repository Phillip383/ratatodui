use super::{State, Transition};
use crate::types::{
    ActiveWidget,
    AppAction::{self},
};

use crossterm::event::KeyCode;
pub struct CommandMode;

impl State for CommandMode {
    fn handle_input(&self, input: KeyCode, _active_widget: &ActiveWidget) -> Transition {
        //TODO: Add save commands, and exit command state

        if let Some(c) = input.as_char() {
            if c == 'q' {
                return Transition::Action(AppAction::Quit);
            }
        }
        Transition::Stay
    }
}
