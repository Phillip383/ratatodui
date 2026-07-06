use crate::state::ActiveWidget;

use super::{State, Transition};
pub struct VisualMode;

impl State for VisualMode {
    fn handle_input(&self, input: char, active_widget: &ActiveWidget) -> Transition {
        //TODO: Flesh this out...
        Transition::Stay
    }
}
