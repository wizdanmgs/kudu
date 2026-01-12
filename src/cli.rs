use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "todo",
    version,
    about = "A simple CLI todo list application",
    long_about = None,
    after_help = "Data is stored locally in a JSON file."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new todo item
    ///
    /// Example:
    ///   todo add "Buy a bottle of milk"
    Add {
        /// The title or description of the todo
        title: String,
    },

    /// List all todo items
    List,

    /// Mark a todo item as completed
    Done {
        /// ID of the todo to mark as done
        id: u32,
    },

    /// Delete a todo item
    Delete {
        /// ID of the todo to delete
        id: u32,
    },
}
