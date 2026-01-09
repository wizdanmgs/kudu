use std::fs;
use std::path::Path;

use crate::error::TodoError;
use crate::todo::Todo;

const FILE_PATH: &str = "todo.json";

pub fn load_todos() -> Result<Vec<Todo>, TodoError> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(vec![]);
    };

    let data = fs::read_to_string(FILE_PATH)?;
    let todos = serde_json::from_str(&data)?;

    Ok(todos)
}

pub fn save_todos(todos: &Vec<Todo>) -> Result<(), TodoError> {
    let data = serde_json::to_string_pretty(todos).expect("Failed to serialize");
    fs::write(FILE_PATH, data).expect("Failed to write file");

    Ok(())
}
