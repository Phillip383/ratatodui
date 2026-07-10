use ratatui::widgets::ListItem;

use color_eyre::eyre::{ErrReport, Result};
use crossterm::event::{self, Event, KeyCode};
use tui_textarea::CursorMove::Back;

use crate::state::{
    State,
    Transition::{self, Action, ChangeFocus, ChangeState},
    VimState::{self, Normal, Visual},
    command, normal,
};

use crate::types::{
    ActiveWidget::{self, EditorTodoDesc, EditorTodoName},
    AppAction::{self, *},
};

pub struct Todo {
    pub title: String,
    pub description: String,
    pub due_date: String, //TODO: Check for a better way to store dates.
    pub subtasks: Option<Box<Vec<Todo>>>,
    pub is_complete: bool,
}

impl<'a> From<&'a Todo> for ListItem<'a> {
    fn from(todo: &'a Todo) -> Self {
        let checkbox = if todo.is_complete { "[x] " } else { "[ ] " };
        let text = format!("{}{}", checkbox, todo.title);

        ListItem::new(text)
    }
}

pub struct TodoList {
    pub title: String,
    pub todos: Vec<Todo>,
}

impl<'a> From<&'a TodoList> for ListItem<'a> {
    fn from(list: &'a TodoList) -> Self {
        ListItem::new(list.title.as_str())
    }
}

pub struct App {
    pub lists: Vec<TodoList>,
    pub current_mode: VimState,
    pub active_widget: ActiveWidget,
    pub active_list_item: usize,
    pub active_todo: usize,
    pub b_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            lists: vec![TodoList {
                title: "Default".to_string(),
                todos: vec![
                    Todo {
                        title: "Bread".to_string(),
                        description: String::new(),
                        due_date: String::new(),
                        subtasks: None,
                        is_complete: false,
                    },
                    Todo {
                        title: "Milk".to_string(),
                        description: String::new(),
                        due_date: String::new(),
                        subtasks: None,
                        is_complete: true,
                    },
                ],
            }],
            current_mode: VimState::Normal(normal::NormalMode),
            active_widget: ActiveWidget::Todos(None),
            active_list_item: 0,
            active_todo: 0,
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
                EditorTodoName => self.lists[self.active_list_item].todos[self.active_todo]
                    .title
                    .push(c),
                EditorTodoDesc => self.lists[self.active_list_item].todos[self.active_todo]
                    .description
                    .push(c),
                _ => (),
            },
            Backspace => match self.active_widget {
                EditorTodoName => {
                    self.lists[self.active_list_item].todos[self.active_todo]
                        .title
                        .pop();
                }
                EditorTodoDesc => {
                    self.lists[self.active_list_item].todos[self.active_todo]
                        .description
                        .pop();
                }
                _ => (),
            },
            Quit => self.b_quit = true,
        }
    }
}
