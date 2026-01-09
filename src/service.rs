use crate::error::TodoError;
use crate::todo::Todo;

pub fn add_todo(todos: &mut Vec<Todo>, title: String) {
    let id = todos.len() as u32 + 1;
    todos.push(Todo {
        id,
        title,
        completed: false,
    });
}

pub fn list_todos(todos: &Vec<Todo>) {
    for todo in todos {
        let status = if todo.completed { "✔" } else { " " };
        println!("[{}] {} - {}", status, todo.id, todo.title);
    }
}

pub fn mark_done(todos: &mut [Todo], id: u32) -> Result<(), TodoError> {
    let todo = todos.iter_mut().find(|t| t.id == id);
    match todo {
        Some(t) => {
            t.completed = true;
            Ok(())
        }
        None => Err(TodoError::NotFound),
    }
}

pub fn delete_todo(todos: &mut Vec<Todo>, id: u32) -> Result<(), TodoError> {
    let original_len = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() == original_len {
        return Err(TodoError::NotFound);
    }

    reassign_ids(todos);
    Ok(())
}

fn reassign_ids(todos: &mut [Todo]) {
    for (index, todo) in todos.iter_mut().enumerate() {
        todo.id = (index + 1) as u32;
    }
}
