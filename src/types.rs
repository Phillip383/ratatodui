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
    UpdateActiveList(usize),
    UpdateActiveTodo(usize),
    InsertChar(char),
    Save(String),
    Backspace,
    Quit,
}
