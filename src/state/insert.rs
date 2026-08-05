use crate::types::{
    ActiveWidget::{EditorTodoDesc, EditorTodoName, EditorListName},
    AppAction,
};

use super::{ActiveWidget, State, Transition};
use crossterm::event::KeyCode;

pub struct InsertMode;

impl State for InsertMode {
    fn handle_input(&self, _input: KeyCode, _active_widget: &ActiveWidget) -> Transition {
        //TODO: Flesh this out...
        if _input == KeyCode::Backspace {
            return Transition::Action(AppAction::Backspace);
        } else if _input == KeyCode::Enter && *_active_widget == ActiveWidget::EditorTodoDesc {
            return Transition::Action(AppAction::InsertChar('\n'));
        } else {
            if let Some(c) = _input.as_char() {
                match _active_widget {
                    EditorTodoDesc => return Transition::Action(AppAction::InsertChar(c)),
                    EditorTodoName => return Transition::Action(AppAction::InsertChar(c)),
                    EditorListName => return Transition::Action(AppAction::InsertChar(c)),
                    _ => (),
                }
            }
        }

        Transition::Stay
    }
}
