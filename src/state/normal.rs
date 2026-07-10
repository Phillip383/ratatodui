use super::{State, Transition, VimState};
use crate::state::{
    ActiveWidget::{self, Editor, Lists, Todos},
    Direction, insert, visual,
};

pub struct NormalMode;

impl State for NormalMode {
    fn handle_input(&self, input: char, active_widget: &ActiveWidget) -> Transition {
        //TODO: Flesh this out...
        match input {
            'i' => return Transition::ChangeState(VimState::Insert(insert::InsertMode)),
            'v' => return Transition::ChangeState(VimState::Visual(visual::VisualMode)),
            'L' => return Transition::ChangeFocus(ActiveWidget::Lists(None), None),
            'T' => return Transition::ChangeFocus(ActiveWidget::Todos(None), None),
            'E' => return Transition::ChangeFocus(ActiveWidget::Editor(None), None),
            'J' => return handle_vertical_focus(Direction::DOWN, active_widget),
            'K' => return handle_vertical_focus(Direction::UP, active_widget),
            _ => (),
        }

        Transition::Stay
    }
}

fn handle_vertical_focus(_direction: Direction, active_widget: &ActiveWidget) -> Transition {
    match active_widget {
        Todos(None) => return Transition::ChangeFocus(Todos(Some(_direction)), None),
        Editor(None) => return Transition::ChangeFocus(Editor(Some(_direction)), None),
        Lists(None) => return Transition::ChangeFocus(Lists(Some(_direction)), None),
        _ => Transition::Stay,
    }
}
