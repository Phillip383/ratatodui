use color_eyre::eyre::{ErrReport, Result};
use clap::{Args, Parser, Subcommand};
use tokio::fs::read_to_string;

use crate::app::Config;
use crate::types::{Todo, TodoList};

pub async fn run(config: Config) -> Result<(), ErrReport> {

    //parse the command to ensure its valid
    let cli = Cli::parse();

    match cli.command {
        Commands::Nl(args) => create_list(config, &args.name).await,
        Commands::Nt(args) => create_todo( config, &args.list, &args.name, args.desc).await,
        Commands::Ul(args) => update_list(config, args.list,args.name).await,
        _ => (),
        // Commands::Rl(args) => remove_list(config, &args.name),
        // Commands::Rt(args) => remove_todo(config, &args.list, &args.name),
        // Commands::Ut(args) => update_todo(config, args.list, args.name, args.new_name, args.desc, args.complete),
        // Commands::C(args) => update_todo(config, args.list, args.name, None, None, Some(true)),
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
    Nl(NewListArgs),
    Nt(NewTodoArgs),
    Rl(RemoveListArgs),
    Rt(RemoveTodoArgs),
    C(CompleteArgs),
    Ul(UpdateListArgs),
    Ut(UpdateTodoArgs),
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

    #[arg(long="name", required=true)]
    name: String,
}

#[derive(Args)]
struct RemoveListArgs {
    #[arg(long="name", required=true)]
    name: String,
}

#[derive(Args)]
struct RemoveTodoArgs {
    #[arg(long="list", required= true)]
    list: String,

    #[arg(long="name", required=true)]
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
    #[arg(long="list", required= true)]
    list: String,
    
    #[arg(long="name", required=true)]
    name: String,

    new_name: Option<String>,

    desc: Option<String>,

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

fn remove_list(config: Config, name: &str) {
    println!("Executing: {}", name);
}

fn update_todo(config: Config, list_name: String, name: String, new_name: Option<String>, desc: Option<String>, complete: Option<bool>) {
        println!("Executing: {}", name);
}

fn remove_todo(config: Config, list_name: &str, name: &str) {
    println!("Executing: {}", name);
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_commands_test() {

    }
}