pub mod command;
pub mod insert;
pub mod normal;
pub mod visual;

use color_eyre::eyre::{ErrReport, Result};
use crossterm::event::{self, Event, KeyCode};

use crate::state::{
    Transition::{Action, ChangeFocus, ChangeState},
    VimState::{Normal, Visual},
};

use crate::types::{
    ActiveWidget::{self, EditorTodoDesc, EditorTodoName},
    AppAction::{self, *},
    Direction,
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

pub struct StateContext {
    pub current_mode: VimState,
    pub active_widget: ActiveWidget,
    pub active_list_item: usize,
    pub active_todo: usize,
    pub todo_name: String,
    pub todo_desc: String,
    pub b_quit: bool,
}

impl StateContext {
    pub fn new() -> Self {
        Self {
            current_mode: VimState::Normal(normal::NormalMode),
            active_widget: ActiveWidget::Todos(None),
            active_list_item: 0,
            active_todo: 0,
            todo_name: String::new(),
            todo_desc: String::new(),
            b_quit: false,
        }
    }

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

                //TODO: Handle app actions
                match transition {
                    ChangeState(new_state) => self.current_mode = new_state,
                    ChangeFocus(widget, mode) => {
                        self.active_widget = widget;
                        if let Some(m) = mode {
                            self.current_mode = m;
                        };
                    }
                    Action(action) => self.handle_action(action),
                    _ => (),
                }
            }
        }
        Ok(None)
    }

    pub fn handle_action(&mut self, action: AppAction) {
        match action {
            UpdateActiveList(index) => self.active_list_item = index,
            UpdateActiveTodo(index) => self.active_todo = index,
            InsertChar(c) => match self.active_widget {
                EditorTodoName => self.todo_name.push(c),
                EditorTodoDesc => self.todo_desc.push(c),
                _ => (),
            },
            Backspace => (),
            Quit => self.b_quit = true,
            _ => (),
        }
    }

    fn handle_char_input(c: char) {}
}
