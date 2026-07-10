#[derive(PartialEq, Eq)]
pub enum Direction {
    UP,
    DOWN,
}

#[derive(PartialEq, Eq)]
pub enum ActiveWidget {
    Todos(Option<Direction>),
    Lists(Option<Direction>),
    Editor(Option<Direction>),
    StatusBar,
    EditorTodoName,
    EditorTodoDesc,
}

pub enum AppAction {
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
