use serde::{Deserialize, Serialize};
use ratatui::widgets::ListItem;


#[derive(PartialEq, Eq)]
pub enum ActiveWidget {
    Todos,
    Lists,
    Editor,
    StatusBar,
    EditorTodoName,
    EditorTodoDesc,
    EditorListName,
}

pub enum AppAction {
    UpdateActiveList(i8),
    UpdateActiveTodo(i8),
    CompleteTodo,
    InsertChar(char),
    CreateList,
    CreateTodo,
    DeleteList,
    DeleteTodo,
    Save,
    Backspace,
    Execute,
}

pub enum AppStatus {
    Idle,
    Saving,
    Loading,
    Success,
    Error(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoList {
    #[serde(rename = "_id")]        
    pub id: String,
    pub title: String,
    pub todos: Vec<Todo>,
}

impl<'a> From<&'a Todo> for ListItem<'a> {
    fn from(todo: &'a Todo) -> Self {
        let checkbox = if todo.completed { "[x] " } else { "[ ] " };
        let text = format!("{}{}", checkbox, todo.title);

        ListItem::new(text)
    }
}

impl<'a> From<&'a TodoList> for ListItem<'a> {
    fn from(list: &'a TodoList) -> Self {
        ListItem::new(list.title.as_str())
    }
}