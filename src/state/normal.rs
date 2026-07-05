use super::{State, Transition, VimState};
use crate::state::{command, insert, visual};

pub struct NormalMode;

impl State for NormalMode {
    fn handle_input(&self, input: char) -> Transition {
        //TODO: Flesh this out...
        match input {
            ':' => return Transition::ChangeState(VimState::Command(command::CommandMode)),
            'i' => return Transition::ChangeState(VimState::Insert(insert::InsertMode)),
            'v' => return Transition::ChangeState(VimState::Visual(visual::VisualMode)),
            _ => (),
        }

        Transition::Stay
    }
}
