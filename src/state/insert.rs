use crate::types::{
    ActiveWidget::{EditorTodoDesc, EditorTodoName},
    AppAction,
};

use super::{ActiveWidget, State, Transition};

pub struct InsertMode;

impl State for InsertMode {
    fn handle_input(&self, _input: char, _active_widget: &ActiveWidget) -> Transition {
        //TODO: Flesh this out...

        match _active_widget {
            EditorTodoDesc => return Transition::Action(AppAction::InsertChar(_input)),
            EditorTodoName => return Transition::Action(AppAction::InsertChar(_input)),
            _ => (),
        }

        Transition::Stay
    }
}
