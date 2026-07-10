use crate::state::ActiveWidget;

use super::{State, Transition};
use crossterm::event::KeyCode;
pub struct VisualMode;

impl State for VisualMode {
    fn handle_input(&self, _input: KeyCode, _active_widget: &ActiveWidget) -> Transition {
        //TODO: Flesh this out...
        Transition::Stay
    }
}
