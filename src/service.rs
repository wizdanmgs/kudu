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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::Todo;

    fn sample_todos() -> Vec<Todo> {
        vec![
            Todo {
                id: 1,
                title: "Task 1".into(),
                completed: false,
            },
            Todo {
                id: 2,
                title: "Task 2".into(),
                completed: false,
            },
            Todo {
                id: 3,
                title: "Task 3".into(),
                completed: false,
            },
        ]
    }

    #[test]
    fn add_todo_appends_item() {
        let mut todos = vec![];

        add_todo(&mut todos, "Learn Rust".into());

        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].id, 1);
        assert_eq!(todos[0].title, "Learn Rust");
        assert!(!todos[0].completed);
    }

    #[test]
    fn mark_done_sets_complete() {
        let mut todos = sample_todos();

        let result = mark_done(&mut todos, 2);

        assert!(result.is_ok());
        assert!(todos[1].completed);
    }

    #[test]
    fn mark_done_reutrns_error_if_not_found() {
        let mut todos = sample_todos();

        let result = mark_done(&mut todos, 100);

        assert!(matches!(result, Err(TodoError::NotFound)));
    }

    #[test]
    fn delete_todo_removes_item_and_reassigns_ids() {
        let mut todos = sample_todos();

        let result = delete_todo(&mut todos, 2);

        assert!(result.is_ok());
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].id, 1);
        assert_eq!(todos[1].id, 2);
        assert_eq!(todos[1].title, "Task 3");
    }

    #[test]
    fn delete_todo_reutrns_error_if_not_found() {
        let mut todos = sample_todos();

        let result = delete_todo(&mut todos, 100);

        assert!(matches!(result, Err(TodoError::NotFound)));
    }
}
