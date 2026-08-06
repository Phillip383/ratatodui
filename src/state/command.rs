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

        if input == KeyCode::Enter {
            //TODO: Process the command in the command buffer.
            return Transition::Action(AppAction::Execute);
        }
        if input == KeyCode::Backspace {
            return Transition::Action(AppAction::Backspace);
        }

        if let Some(c) = input.as_char() {
            return Transition::Action(AppAction::InsertChar(c));
        }
        Transition::Stay
    }
}
