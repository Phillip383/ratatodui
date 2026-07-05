use super::{State, Transition};


pub struct CommandMode;

impl State for CommandMode {
    fn handle_input(&self, input: char) -> Transition {
        //TODO: Add save commands, and exit command state
        if input == 'q' {
            return Transition::Command(String::from(input));
        }
        Transition::Stay
    }
}
