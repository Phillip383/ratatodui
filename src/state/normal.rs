use super::{State, Transition, VimState};
use crate::state::{insert, visual};
use crate::types::ActiveWidget;

pub struct NormalMode;

impl State for NormalMode {
    fn handle_input(&self, input: char, _active_widget: &ActiveWidget) -> Transition {
        //TODO: Flesh this out...
        match input {
            'i' => return Transition::ChangeState(VimState::Insert(insert::InsertMode)),
            'v' => return Transition::ChangeState(VimState::Visual(visual::VisualMode)),
            'L' => return Transition::ChangeFocus(ActiveWidget::Lists(None), None),
            'T' => return Transition::ChangeFocus(ActiveWidget::Todos(None), None),
            'N' => return Transition::ChangeFocus(ActiveWidget::EditorTodoName, None),
            'D' => return Transition::ChangeFocus(ActiveWidget::EditorTodoDesc, None),
            _ => (),
        }

        Transition::Stay
    }
}
