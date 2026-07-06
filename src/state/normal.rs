use super::{State, Transition, VimState};
use crate::state::{ActiveWidget, command, insert, visual};

pub struct NormalMode;

impl State for NormalMode {
    fn handle_input(&self, input: char, active_widget: &ActiveWidget) -> Transition {
        //TODO: Flesh this out...
        match input {
            'i' => return Transition::ChangeState(VimState::Insert(insert::InsertMode)),
            'v' => return Transition::ChangeState(VimState::Visual(visual::VisualMode)),
            'L' => return Transition::ChangeFocus(ActiveWidget::Lists, None),
            'T' => return Transition::ChangeFocus(ActiveWidget::Todos, None),
            'E' => return Transition::ChangeFocus(ActiveWidget::Editor, None),
            _ => (),
        }

        Transition::Stay
    }
}
