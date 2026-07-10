pub mod command;
pub mod insert;
pub mod normal;
pub mod visual;

use crate::types::{
    ActiveWidget::{self},
    AppAction::{self},
};

pub trait State {
    fn handle_input(&self, input: char, active_widget: &ActiveWidget) -> Transition;
}

pub enum VimState {
    Normal(normal::NormalMode),
    Visual(visual::VisualMode),
    Command(command::CommandMode),
    Insert(insert::InsertMode),
}

pub enum Transition {
    Stay,
    ChangeState(VimState),
    ChangeFocus(ActiveWidget, Option<VimState>),
    Action(AppAction),
}
