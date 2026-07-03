use crate::state::command;

use super::{State, Transition};

pub struct NormalMode;

impl State for NormalMode {
    fn handle_input(&self, input: char) -> Transition {
        //TODO: Flesh this out...
        if input == ':' {
            return Transition::ChangeState(Box::new(command::CommandMode));
        }
        Transition::Stay
    }
}
