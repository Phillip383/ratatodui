use std::{path::PathBuf, time::Duration};
use app_dirs2::{AppDataType, AppInfo};

use color_eyre::eyre::{ErrReport, Result};
use crossterm::event::{self, Event, KeyCode};
use serde::{Deserialize, Serialize};
use tokio::{sync::mpsc};

use crate::{state::{
    State,
    Transition::{self, Action, ChangeFocus, ChangeState},
    VimState::{self, Normal, Visual},
    command, normal,
}, types::ActiveWidget::{EditorListName, StatusBar}};

use crate::types::{
    ActiveWidget::{self, EditorTodoDesc, EditorTodoName},
    AppAction::{self, *},
    Todo,
    TodoList,
    AppStatus,
};

const APP_INFO: AppInfo = AppInfo {
    name: "data",
    author: "Ratatodui"
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub save_dir: PathBuf
}

impl Default for Config {
    fn default() -> Self {
        Self { 
            version: String::from("0.1.0"), 
            save_dir: app_dirs2::app_dir(AppDataType::SharedData, &APP_INFO, "todos").unwrap(),
        }
    }
}

pub struct App {
    config: Config,
    pub save_tx: mpsc::UnboundedSender<AppStatus>,
    pub save_rx: mpsc::UnboundedReceiver<AppStatus>,
    pub init_tx: mpsc::UnboundedSender<Vec<TodoList>>,
    pub init_rx: mpsc::UnboundedReceiver<Vec<TodoList>>,
    pub lists: Vec<TodoList>,
    pub current_mode: VimState,
    pub active_widget: ActiveWidget,
    pub active_list_item: usize,
    pub active_todo: usize,
    pub b_quit: bool,
    pub app_status: AppStatus,
    pub command_buffer: String,
}

impl App {
    pub fn new(config: Config) -> Self {
        let (save_tx, save_rx) = mpsc::unbounded_channel();
        let (init_tx, init_rx) = mpsc::unbounded_channel();
        App {
            config,
            save_rx,
            save_tx,
            init_tx,
            init_rx,
            lists: Vec::new(),
            current_mode: VimState::Normal(normal::NormalMode),
            active_widget: ActiveWidget::Todos,
            active_list_item: 0,
            active_todo: 0,
            b_quit: false,
            app_status: AppStatus::Idle,
            command_buffer: String::new(),
        }
    }

    pub fn init(&mut self) {
        
        self.app_status = AppStatus::Loading;

        let path = format!("{}/{}", &self.config.save_dir.to_string_lossy(), "lists.json").clone();
        let tx = self.init_tx.clone();
        
        tokio::spawn(async move {
            let data = tokio::fs::read_to_string(path).await.unwrap_or_default();
            let lists: Vec<TodoList> = serde_json::from_str(data.as_str()).unwrap_or_default();
            let _ = tx.send(lists);
        });

    }

    pub fn handle_events(&mut self) -> Result<Option<()>, ErrReport> {
        if event::poll(Duration::from_millis(50))? {
            
            if let Event::Key(key) = event::read()? {
                //Handle escape, every state goes back to normal mode via escape.
                if key.is_release() {
                    return Ok(None); //Ignore releases for now, we only care about key presses.
                }
                
                if key.code == KeyCode::Esc {
                    self.current_mode = VimState::Normal(normal::NormalMode);
                    return Ok(None);
                }
                
                let transition: Transition = match key.code {
                    KeyCode::Char(':') => Transition::ChangeFocus(
                        ActiveWidget::StatusBar,
                        Some(VimState::Command(command::CommandMode)),
                    ),
                    
                    _ => match &self.current_mode {
                        Normal(mode) => mode.handle_input(key.code, &self.active_widget),
                        Visual(mode) => mode.handle_input(key.code, &self.active_widget),
                        VimState::Command(mode) => mode.handle_input(key.code, &self.active_widget),
                        VimState::Insert(mode) => mode.handle_input(key.code, &self.active_widget),
                    },
                };
                
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
            //TODO: Handle index out of bounds, wrap to top/bottom list item.
            UpdateActiveList(index) => {
                    self.active_todo = 0; //Reset active todo when switching lists.
                    if self.active_list_item as i8 + index < 0 {
                        self.active_list_item = self.lists.len() - 1;
                    } else if (self.active_list_item as i8 + index) >= self.lists.len() as i8 {
                        self.active_list_item = 0;
                    } else {
                        self.active_list_item = (self.active_list_item as i8 + index) as usize;
                    }
                }
            UpdateActiveTodo(index) => {
                    if let Some(list) = self.lists.get(self.active_list_item) {
                        let todos = &list.todos;
                        if self.active_todo as i8 + index < 0 {
                            self.active_todo = todos.len() - 1;
                        } else if (self.active_todo as i8 + index) >= todos.len() as i8
                        {
                            self.active_todo = 0;
                        } else {
                            self.active_todo = (self.active_todo as i8 + index) as usize;
                        }
                    } 
                    
                }
            CompleteTodo => {
                if let Some(active_list) = self.lists.get_mut(self.active_list_item) {
                    if let Some(active_todo) = active_list.todos.get_mut(self.active_todo) {
                        active_todo.completed = !active_todo.completed;
                    }
                }
            }
            InsertChar(c) => match self.active_widget {
                //TODO: Handle index out of bounds...
                EditorTodoName => {
                    if let Some(active_list) = self.lists.get_mut(self.active_list_item) {
                        if let Some(active_todo) = active_list.todos.get_mut(self.active_todo) {
                            active_todo.title.push(c);
                        }
                    }
                }
                EditorTodoDesc => {
                    if let Some(active_list) = self.lists.get_mut(self.active_list_item) {
                        if let Some(active_todo) = active_list.todos.get_mut(self.active_todo) {
                            active_todo.description.push(c);
                        }
                    }
                }
                EditorListName => {
                    if let Some(active_list) = self.lists.get_mut(self.active_list_item) {
                        active_list.title.push(c);
                    } 
                }
                StatusBar => {
                    self.command_buffer.push(c);
                }
                _ => (),
            },
            CreateList => self.create_list(),
            CreateTodo => self.create_todo(),
            DeleteList => self.delete_list(),
            DeleteTodo => self.delete_todo(),
            Backspace => match self.active_widget {
                //TODO: Handle index out of bounds...
                EditorTodoName => {
                    if let Some(active_list) = self.lists.get_mut(self.active_list_item) {
                        if let Some(active_todo) = active_list.todos.get_mut(self.active_todo) {
                            active_todo.title.pop();
                        }
                    }
                }
                EditorTodoDesc => {
                    if let Some(active_list) = self.lists.get_mut(self.active_list_item) {
                        if let Some(active_todo) = active_list.todos.get_mut(self.active_todo) {
                            active_todo.description.pop();
                        }
                    }
                }
                EditorListName => {
                    if let Some(active_list) = self.lists.get_mut(self.active_list_item) {
                        active_list.title.pop();
                    } 
                }
                StatusBar => {
                    self.command_buffer.pop();
                }
                _ => (),
            },
            Save => self.save(),
            Execute => self.exec_command(),
        }
    }

    fn save(&mut self) {

        //Don't allow double saves, or saves while loading.
        if self.app_status != AppStatus::Idle {
            return;
        }
        //Set status to saving...
        self.app_status = AppStatus::Saving;
        
        let d = serde_json::to_string_pretty(&self.lists).unwrap_or_default();
        let path = format!("{}/{}", &self.config.save_dir.to_string_lossy(), "lists.json").clone();
        let tx = self.save_tx.clone();
        
        tokio::spawn(async move {
            let res = tokio::fs::write(path, d).await;
            match res {
                Ok(()) => tx.send(AppStatus::Idle),
                Err(e) => tx.send(AppStatus::Error(e.to_string())),
            }
        });
    }

    fn exec_command(&mut self) {
        self.app_status = AppStatus::Idle; //Reset app status from previous fail.
        match self.command_buffer.as_str() {
            "q" => self.b_quit = true,
            "wa" => {let _ = self.save();},
            _ => self.app_status = AppStatus::Error("Command Not Found".to_string())
        }

        self.command_buffer.clear();
    }

    //TODO: Handle id's
    fn create_list(&mut self) {
        let list = TodoList {
            id: String::from(""),
            title: "New List".to_string(),
            todos: Vec::new(),
        };
        self.lists.push(list);
    }

    //TODO: Handle id's
    fn create_todo(&mut self) {
        if let Some(list) = self.lists.get_mut(self.active_list_item) {
            list.todos.push(
                Todo {
                    id: None,
                    title: "New Todo".to_string(),
                    description: String::new(),
                    completed: false,
            });
        } 
    }

    fn delete_list(&mut self) {
        if !self.lists.is_empty() {
            self.lists.remove(self.active_list_item);
            self.active_list_item = 0;
        }
    }

    fn delete_todo(&mut self) {
        if !self.lists[self.active_list_item].todos.is_empty() {
            self.lists[self.active_list_item].todos.remove(self.active_todo);
            self.active_todo = 0;
        }
    }
    
}
