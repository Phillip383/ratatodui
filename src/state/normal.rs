use crate::state::{command, insert, visual};

use super::{State, Transition};

pub struct NormalMode;

impl State for NormalMode {
    fn handle_input(&self, input: char) -> Transition {
        //TODO: Flesh this out...
        match input {
            ':' => return Transition::ChangeState(Box::new(command::CommandMode)),
            'i' => return Transition::ChangeState(Box::new(insert::InsertMode)),
            'v' => return Transition::ChangeState(Box::new(visual::VisualMode)),
            _ => (),
        }

        Transition::Stay
    }
}
