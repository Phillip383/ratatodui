pub struct TodoList {
    items: Vec<String>,
    selected_index: usize, // UI state lives HERE, not in a global context
    pub is_active: bool,
}

pub struct App {
    //FIXME: These will change to the correct types.
    todos: String,
    lists: String,
    events: String,
    active_todo: String,
    active_list_item: String,
    active_event: String,
    editor_buffer: String,
}

impl App {
    pub fn new() -> Self {
        App {
            todos: String::from(""),
            lists: String::from(""),
            events: String::from(""),
            active_todo: String::from(""),
            active_list_item: String::from(""),
            active_event: String::from(""),
            editor_buffer: String::from(""),
        }
    }
}
