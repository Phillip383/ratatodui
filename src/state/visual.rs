use super::{State, Transition};

pub struct VisualMode;

impl State for VisualMode {
    fn handle_input(&self, input: char) -> Transition {
        //TODO: Flesh this out...
        Transition::Stay
    }
}
