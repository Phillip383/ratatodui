pub mod command;
pub mod insert;
pub mod normal;
pub mod visual;

use color_eyre::eyre::{ErrReport, Result};
use crossterm::event::{self, Event, KeyCode};
use std::any::Any;

use crate::state::Transition::{ChangeState, Command};

pub trait State: Any {
    fn handle_input(&self, input: char) -> Transition;
}

pub enum Transition {
    Stay,
    ChangeState(Box<dyn State>),
    Command(String),
}

pub struct StateContext {
    pub current_mode: Box<dyn State>,
}

impl StateContext {
    pub fn new() -> Self {
        Self {
            current_mode: Box::new(normal::NormalMode),
        }
    }

    /// Returns () if quit was initiated
    /// TODO: This will become more robust.
    pub fn handle_events(&mut self) -> Result<Option<()>, ErrReport> {
        if let Event::Key(key) = event::read()? {
            //Handle escape, every state goes back to normal mode via escape.
            if key.code == KeyCode::Esc {
                self.current_mode = Box::new(normal::NormalMode);
                return Ok(None);
            }

            if let Some(k) = key.code.as_char() {
                let transition = self.current_mode.handle_input(k);
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
