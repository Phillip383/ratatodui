use ratatui::widgets::ListItem;

use crate::{
    app::AppAction::{
        Backspace, InsertChar, Quit, UpdateActiveList, UpdateActiveTodo, UpdateListTitle,
        UpdateTodoDate, UpdateTodoDescription, UpdateTodoTitle,
    },
    state::{ActiveWidget, StateContext},
};

pub struct Todo {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<String>, //TODO: Check for a better way to store dates.
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

enum AppAction {
    UpdateTodoTitle(String),
    UpdateTodoDescription(String),
    UpdateTodoDate(String),
    UpdateActiveList(usize),
    UpdateListTitle(String),
    UpdateActiveTodo(usize),
    InsertChar(char),
    Backspace,
    Quit,
}

pub struct App {
    pub lists: Vec<TodoList>,
    pub active_list_item: usize,
    pub active_todo: usize,
    pub state_context: StateContext,
}

impl App {
    pub fn new(state_context: StateContext) -> Self {
        App {
            lists: vec![TodoList {
                title: "Default".to_string(),
                todos: vec![
                    Todo {
                        title: "Bread".to_string(),
                        description: None,
                        due_date: None,
                        subtasks: None,
                        is_complete: false,
                    },
                    Todo {
                        title: "Milk".to_string(),
                        description: None,
                        due_date: None,
                        subtasks: None,
                        is_complete: true,
                    },
                ],
            }],
            active_todo: 0,
            active_list_item: 0,
            state_context,
        }
    }

    pub fn dispatch(&mut self, action: AppAction) {
        let active_list = &mut self.lists[self.active_list_item];
        let active_todo = &mut active_list.todos[self.active_todo];

        match action {
            UpdateTodoTitle(title) => active_todo.title = title,
            UpdateTodoDescription(desc) => active_todo.description = Some(desc),
            UpdateTodoDate(date) => active_todo.due_date = Some(date),
            UpdateActiveList(index) => self.active_list_item = index,
            UpdateListTitle(title) => active_list.title = title,
            UpdateActiveTodo(index) => self.active_todo = index,
            InsertChar(c) => self.handle_char_input(c),
            Backspace => (),
            Quit => (),
        }
    }

    fn handle_char_input(&mut self, c: char) {
        //TODO: Inject into the currently focused widgets state.
        match self.state_context.active_widget {
            ActiveWidget::EditorTodoName => (),
            ActiveWidget::EditorTodoDesc => (),
            _ => (),
        }
    }
}
