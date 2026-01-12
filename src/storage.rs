use std::fs;
use std::path::Path;

use crate::error::TodoError;
use crate::todo::Todo;

const FILE_PATH: &str = "todo.json";

pub fn load_todos_from(path: &str) -> Result<Vec<Todo>, TodoError> {
    if !Path::new(path).exists() {
        return Ok(vec![]);
    };

    let data = fs::read_to_string(path)?;
    let todos = serde_json::from_str(&data)?;

    Ok(todos)
}

pub fn save_todos_to(path: &str, todos: &Vec<Todo>) -> Result<(), TodoError> {
    let data = serde_json::to_string_pretty(todos).expect("Failed to serialize");
    fs::write(path, data).expect("Failed to write file");

    Ok(())
}

pub fn load_todos() -> Result<Vec<Todo>, TodoError> {
    load_todos_from(FILE_PATH)
}

pub fn save_todos(todos: &Vec<Todo>) -> Result<(), TodoError> {
    save_todos_to(FILE_PATH, todos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::Todo;
    use std::fs;

    const TEST_FILE: &str = "test.json";

    fn cleanup() {
        fs::remove_file(TEST_FILE).ok();
    }

    #[test]
    fn save_and_load_todos() {
        cleanup();

        let todos = vec![Todo {
            id: 1,
            title: "Test".into(),
            completed: false,
        }];
        save_todos_to(TEST_FILE, &todos).unwrap();
        let loaded = load_todos_from(TEST_FILE).unwrap();

        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id, 1);
        assert_eq!(loaded[0].title, "Test");
        assert!(!loaded[0].completed);

        cleanup();
    }

    #[test]
    fn load_returns_empty_if_file_missing() {
        cleanup();

        let todos = load_todos_from(TEST_FILE).unwrap();

        assert!(todos.is_empty());
    }
}
