mod cli;
mod error;
mod service;
mod storage;
mod todo;

use clap::Parser;
use cli::{Cli, Commands};
use error::TodoError;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), TodoError> {
    let cli = Cli::parse();
    let mut todos = storage::load_todos()?;

    match cli.command {
        Commands::Add { title } => {
            service::add_todo(&mut todos, title);
            storage::save_todos(&todos)?;
            println!("Todo added");
        }
        Commands::List => {
            service::list_todos(&todos);
        }
        Commands::Done { id } => {
            service::mark_done(&mut todos, id)?;
            storage::save_todos(&todos)?;
            println!("Todo marked as done!");
        }
        Commands::Delete { id } => {
            service::delete_todo(&mut todos, id)?;
            storage::save_todos(&todos)?;
            println!("Todo deleted!");
        }
    }

    Ok(())
}
