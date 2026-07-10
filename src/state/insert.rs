use super::{ActiveWidget, State, Transition};

pub struct InsertMode;

impl State for InsertMode {
    fn handle_input(&self, _input: char, _active_widget: &ActiveWidget) -> Transition {
        //TODO: Flesh this out...

        Transition::Stay
    }
}
