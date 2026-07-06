use ratatui::widgets::ListItem;

use crate::state::StateContext;

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

pub struct App {
    pub lists: Vec<TodoList>,
    pub active_list_item: usize,
    pub active_todo: usize,
    pub state_context: StateContext,
}

impl App {
    pub fn new() -> Self {
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
            state_context: StateContext::new(),
        }
    }
}
