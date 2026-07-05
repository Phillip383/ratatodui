pub mod command;
pub mod insert;
pub mod normal;
pub mod visual;

use color_eyre::eyre::{ErrReport, Result};
use crossterm::event::{self, Event, KeyCode};

use crate::state::{
    Transition::{ChangeState, Command},
    VimState::{Normal, Visual},
};

pub trait State {
    fn handle_input(&self, input: char) -> Transition;
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
    Command(String),
}

pub struct StateContext {
    pub current_mode: VimState,
}

impl StateContext {
    pub fn new() -> Self {
        Self {
            current_mode: VimState::Normal(normal::NormalMode),
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
                let transition = match &self.current_mode {
                    Normal(mode) => mode.handle_input(k),
                    Visual(mode) => mode.handle_input(k),
                    VimState::Command(mode) => mode.handle_input(k),
                    VimState::Insert(mode) => mode.handle_input(k),
                };

                match transition {
                    ChangeState(new_state) => self.current_mode = new_state,
                    Command(cmd) => {
                        if cmd == "q" {
                            return Ok(Some(()));
                        }
                    }
                    _ => (),
                }
            }
        }
        Ok(None)
    }
}
