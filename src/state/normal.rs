use super::{State, Transition, VimState};
use crate::state::{
    ActiveWidget::{self, Editor, Lists, Todos},
    insert, visual,
};

enum Direction {
    UP,
    DOWN,
}

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
            'J' => handle_vertical_focus(Direction::DOWN, active_widget),
            'K' => handle_vertical_focus(Direction::UP, active_widget),
            _ => (),
        }

        Transition::Stay
    }
}

fn handle_vertical_focus(direction: Direction, active_widget: &ActiveWidget) {
    match active_widget {
        Todos => (),
        Editor => (),
        Lists => (),
        _ => (),
    }
}
