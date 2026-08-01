use super::{State, Transition, VimState};
use crate::state::{insert, visual};
use crate::types::ActiveWidget;
use crate::types::AppAction;
use crossterm::event::KeyCode;

pub struct NormalMode;

impl State for NormalMode {
    fn handle_input(&self, input: KeyCode, _active_widget: &ActiveWidget) -> Transition {
        //TODO: Flesh this out...
        if let Some(c) = input.as_char() {
            match c {
                'i' => return Transition::ChangeState(VimState::Insert(insert::InsertMode)),
                'v' => return Transition::ChangeState(VimState::Visual(visual::VisualMode)),
                'L' => return Transition::ChangeFocus(ActiveWidget::Lists, None),
                'T' => return Transition::ChangeFocus(ActiveWidget::Todos, None),
                'N' => return Transition::ChangeFocus(ActiveWidget::EditorTodoName, None),
                'S' => return Transition::Action(AppAction::Save),
                'j' => match _active_widget {
                    ActiveWidget::Todos => {
                        return Transition::Action(AppAction::UpdateActiveTodo(1));
                    }
                    ActiveWidget::Lists => {
                        return Transition::Action(AppAction::UpdateActiveList(1));
                    }
                    _ => (),
                },
                'k' => match _active_widget {
                    ActiveWidget::Todos => {
                        return Transition::Action(AppAction::UpdateActiveTodo(-1));
                    }
                    ActiveWidget::Lists => {
                        return Transition::Action(AppAction::UpdateActiveList(-1));
                    }
                    _ => (),
                },
                'C' => match _active_widget {
                    ActiveWidget::Todos => {
                        return Transition::Action(AppAction::CreateTodo);
                    }
                    ActiveWidget::Lists => {
                        return Transition::Action(AppAction::CreateList);
                    }
                    _ => (),
                },
                'D' => match _active_widget {
                    ActiveWidget::Todos => {
                        return Transition::Action(AppAction::DeleteTodo);
                    }
                    ActiveWidget::Lists => {
                        return Transition::Action(AppAction::DeleteList);
                    }
                    _ => (),
                },
                _ => (),
            }
        }
        Transition::Stay
    }
}
