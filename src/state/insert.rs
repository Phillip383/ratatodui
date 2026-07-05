use super::{State, Transition};

pub struct InsertMode;

impl State for InsertMode {


    fn handle_input(&self, input: char) -> Transition {
        //TODO: Flesh this out...

        Transition::Stay
    }
}
