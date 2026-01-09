use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Simple CLI todo app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { title: String },
    List,
    Done { id: u32 },
    Delete { id: u32 },
}

const FILE_PATH: &str = "todo.json";

fn load_todos() -> Vec<Todo> {
    if !Path::new(FILE_PATH).exists() {
        return vec![];
    };
    let data = fs::read_to_string(FILE_PATH).expect("Failed to read file");
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

fn save_todos(todos: &Vec<Todo>) {
    let data = serde_json::to_string_pretty(todos).expect("Failed to serialize");
    fs::write(FILE_PATH, data).expect("Failed to write file");
}

fn main() {
    let cli = Cli::parse();
    let mut todos = load_todos();

    match cli.command {
        Commands::Add { title } => {
            todos.push(Todo {
                id: todos.len() as u32 + 1,
                title,
                completed: false,
            });
            save_todos(&todos);
            println!("Todo added");
        }
        Commands::List => {
            for todo in &todos {
                let state = if todo.completed { "✔" } else { " " };
                println!("[{}] {} {}", state, todo.id, todo.title);
            }
        }
        Commands::Done { id } => {
            if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
                todo.completed = true;
                save_todos(&todos);
                println!("Todo marked as done!");
            } else {
                println!("Todo not found");
            }
        }
        Commands::Delete { id } => {
            todos.retain(|todo| todo.id != id);
            save_todos(&todos);
            println!("Todo deleted");
        }
    }
}
