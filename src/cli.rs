use color_eyre::eyre::{ErrReport, Result};
use clap::{Args, Parser, Subcommand};
use tokio::fs::read_to_string;
use termtree::Tree;
use crate::app::Config;
use crate::types::{Todo, TodoList};

pub async fn run(config: Config) -> Result<(), ErrReport> {

    //parse the command to ensure its valid
    let cli = Cli::parse();

    match cli.command {
        Commands::Nl(args) => create_list(config, &args.name).await,
        Commands::Nt(args) => create_todo( config, &args.list, &args.name, args.desc).await,
        Commands::Ul(args) => update_list(config, args.list,args.name).await,
        Commands::Ut(args) => update_todo(config, args.list, args.name, args.new_name, args.desc, args.complete).await,
        Commands::C(args) => update_todo(config, args.list, args.name, None, None, Some(true)).await,
        Commands::Rl(args) => remove_list(config, &args.name).await,
        Commands::Rt(args) => remove_todo(config, &args.list, &args.name).await,
        Commands::Print => print(config).await,
    } 

    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ///Creates a new list
    Nl(NewListArgs),
    ///Creates a new todo with a given list
    Nt(NewTodoArgs),
    ///Removes all lists with given name
    Rl(RemoveListArgs),
    ///Removes all todos from a given list that matches the todo name
    Rt(RemoveTodoArgs),
    ///Complete a given todo
    C(CompleteArgs),
    ///Update list name
    Ul(UpdateListArgs),
    ///Update a given todo's name, desc, or complete status
    Ut(UpdateTodoArgs),
    Print,
}

#[derive(Args)]
struct NewListArgs {
    #[arg(long="name", required=true)]
    name: String
}

#[derive(Args)]
struct NewTodoArgs {
    #[arg(short='l', long="list", required=true)]
    list: String,

    #[arg(long="name", required=true)]
    name: String,

    desc: Option<String>,
}

#[derive(Args)]
struct CompleteArgs {
    #[arg(short='l', long="list", required=true)]
    list: String,

    #[arg(short='n', long="name", required=true)]
    name: String,
}

#[derive(Args)]
struct RemoveListArgs {
    #[arg(long="name", required=true)]
    name: String,
}

#[derive(Args)]
struct RemoveTodoArgs {
    #[arg(short='l', long="list", required= true)]
    list: String,

    #[arg(short='t', long="name", required=true)]
    name: String, 
}

#[derive(Args)]
struct UpdateListArgs {
    
    #[arg(long="list", required=true)]
    list: String,
    
    #[arg(long="name", required=true)]
    name: String,  
}

#[derive(Args)]
struct UpdateTodoArgs {
    #[arg(short='l', long="list_name", required= true)]
    list: String,
    
    #[arg(short='t', long="todo_name", required=true)]
    name: String,

    #[arg(short='n', long="new_name", required=false)]
    new_name: Option<String>,
    #[arg(short='d', long="desc", required=false)]
    desc: Option<String>,
    #[arg(short='c', long="complete", required=false)]
    complete: Option<bool>,

}

async fn get_lists(config: &Config) -> Option<Vec<TodoList>> {
    let path = format!("{}/{}", config.save_dir.to_string_lossy(), "lists.json");
    let result = read_to_string(&path).await;
    
    match result {
        Ok(data) => {
           let lists: Vec<TodoList> = serde_json::from_str(data.as_str()).unwrap();
           return Some(lists); 
        }
        Err(e) => None
    }

}

async fn write_lists(config: Config, lists: &Vec<TodoList>) -> Result<(), tokio::io::Error> {
    let path = format!("{}/{}", config.save_dir.to_string_lossy(), "lists.json");
    let data = serde_json::to_string_pretty(&lists).unwrap();
    tokio::fs::write(path, data.as_bytes()).await

}

async fn get_todo(lists: &mut Vec<TodoList>, list_name: String, todo_name: String) -> Option<&mut Todo> {
    for list in lists {
        if list.title == list_name {
            for todo in &mut list.todos {
                if todo.title == todo_name {
                    return Some(todo);
                }
            }
        }
    }
    None
}

async fn create_list(config: Config, name: &str) {
    //Open the save file
    let path = format!("{}/{}", config.save_dir.to_string_lossy(), "lists.json");
    let result = read_to_string(&path).await;
    match result {
        Ok(data) => {
            let list = TodoList {
                id: String::from(""),
                title: name.to_string(),
                todos: Vec::new(),
            };

            let mut lists: Vec<TodoList> = serde_json::from_str(data.as_str()).unwrap();
            lists.push(list);
            let data = serde_json::to_string_pretty(&lists).unwrap();
            let res = tokio::fs::write(path, data.as_bytes()).await;    
        },
        Err(e) => panic!("{}", e)
    }
}

async fn create_todo(config: Config, list_name: &str, name: &str, desc: Option<String>) {
    if let Some(lists) = &mut get_lists(&config).await {
        let desc = desc.unwrap_or_default();
        let todo = Todo {
            id: None,
            title: name.to_string(),
            description: desc,
            completed: false
        };

        for list in &mut *lists {
            if list.title == list_name {
                list.todos.push(todo);
                break;
            }
        } 

        let res = write_lists(config, lists).await;

    }
}

async fn update_list(config: Config, list_name: String, new_name: String) {
    if let Some(lists) = &mut get_lists(&config).await {
        for list in &mut *lists {
            if list.title == list_name {
                list.title = new_name;
                break;
            }
        }

        let res = write_lists(config, &lists).await;
    }
}

async fn remove_list(config: Config, name: &str) {
    if let Some(lists) = get_lists(&config).await {
       let lists: Vec<TodoList> = lists.into_iter().filter(|l| l.title != name).collect();
       let res = write_lists(config, &lists).await;
    }
}

async fn update_todo(config: Config, list_name: String, name: String, new_name: Option<String>, desc: Option<String>, complete: Option<bool>) {
    if let Some(lists) = &mut get_lists(&config).await {

        if let Some(todo) = get_todo(lists, list_name, name).await {
            if let Some(new_name) = new_name {
                todo.title = new_name;
            }
            if let Some(desc) = desc {
                todo.description = desc;
            }
            if let Some(complete) = complete {
                todo.completed = complete;
            }
        }

        let res = write_lists(config, lists).await;
    }
}

async fn remove_todo(config: Config, list_name: &str, name: &str) {
    if let Some(mut lists) = get_lists(&config).await {
        for list in lists.iter_mut() {
            if list.title == list_name {
                list.todos.retain(|t| t.title != name);
            }
        }
        let res = write_lists(config, &lists).await;
    }

}

async fn print(config: Config) {
    let mut root = Tree::new("To-Do List's".to_string());

    if let Some(lists) = get_lists(&config).await {
        for list in lists {
            let mut item = Tree::new(list.title);
            for todo in list.todos {
                let checkbox = if todo.completed {"[X]"} else {"[ ]"};
                let text = format!("{} {}", checkbox, todo.title);
                item.push(text);
            }

            root.push(item);
        }
    }

    println!{"{root}"};
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_commands_test() {

    }
}