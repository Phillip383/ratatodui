#[derive(PartialEq, Eq)]
pub enum ActiveWidget {
    Todos,
    Lists,
    Editor,
    StatusBar,
    EditorTodoName,
    EditorTodoDesc,
}

pub enum AppAction {
    UpdateActiveList(i8),
    UpdateActiveTodo(i8),
    InsertChar(char),
    CreateList,
    CreateTodo,
    DeleteList,
    DeleteTodo,
    Save,
    Backspace,
    Quit,
}
