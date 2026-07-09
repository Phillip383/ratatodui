pub mod command;
pub mod insert;
pub mod normal;
pub mod visual;

use color_eyre::eyre::{ErrReport, Result};
use crossterm::event::{self, Event, KeyCode};

use crate::state::{
    Transition::{ChangeFocus, ChangeState, Command},
    VimState::{Normal, Visual},
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
    Command(String),
}

#[derive(PartialEq, Eq)]
pub enum ActiveWidget {
    Todos,
    Lists,
    Editor,
    StatusBar,
    EditorTodoName,
    EditorTodoDesc,
}

pub struct StateContext {
    pub current_mode: VimState,
    pub active_widget: ActiveWidget,
}

impl StateContext {
    pub fn new() -> Self {
        Self {
            current_mode: VimState::Normal(normal::NormalMode),
            active_widget: ActiveWidget::Todos,
        }
    }

    /// Returns () if quit was initiated
    /// TODO: This will become more robust.
    pub fn handle_events(&mut self) -> Result<Option<()>, ErrReport> {
        if let Event::Key(key) = event::read()? {
            //Handle escape, every state goes back to normal mode via escape.
            if key.code == KeyCode::Esc {
                self.current_mode = VimState::Normal(normal::NormalMode);
                return Ok(None);
            }

            if let Some(k) = key.code.as_char() {
                let transition: Transition;
                if k == ':' {
                    transition = Transition::ChangeFocus(
                        ActiveWidget::StatusBar,
                        Some(VimState::Command(command::CommandMode)),
                    )
                } else {
                    transition = match &self.current_mode {
                        Normal(mode) => mode.handle_input(k, &self.active_widget),
                        Visual(mode) => mode.handle_input(k, &self.active_widget),
                        VimState::Command(mode) => mode.handle_input(k, &self.active_widget),
                        VimState::Insert(mode) => mode.handle_input(k, &self.active_widget),
                    };
                }

                match transition {
                    ChangeState(new_state) => self.current_mode = new_state,
                    Command(cmd) => {
                        if cmd == "q" {
                            return Ok(Some(()));
                        }
                    }
                    ChangeFocus(widget, mode) => {
                        self.active_widget = widget;
                        if let Some(m) = mode {
                            self.current_mode = m;
                        };
                    }
                    _ => (),
                }
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn change_focus() {}

    #[test]
    fn change_state() {}
}
