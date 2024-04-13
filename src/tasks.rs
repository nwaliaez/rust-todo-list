
use std::{io, path::Path};
use crate::storage::{load_tasks, save_tasks, Task, TaskList};

pub fn add_task(file_path: &Path) {
    let mut task_list = load_tasks(file_path).expect("Failed to load tasks");

    println!("Enter task description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read task description");

    let id = task_list.tasks.len() as i32 + 1;
    let task = Task::new(id, description.trim().to_string());
    task_list.tasks.push(task);

    save_tasks(&task_list, file_path).expect("Failed to save tasks");
}

pub fn list_tasks(file_path: &Path) {
    let task_list = load_tasks(file_path).expect("Failed to load tasks");

    if task_list.tasks.is_empty() {
        println!("No tasks found");
        return;
    }

    for task in &task_list.tasks {
        let status = if task.completed { "Completed" } else { "Pending" };
        println!("id: {}, description: {}, Status: {}", task.id, task.description, status);
    }
}

pub fn complete_task(file_path: &Path) {
    let mut task_list = load_tasks(file_path).expect("Failed to load tasks");

    println!("Enter the ID of the task you want to mark as done:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    if let Ok(id) = input.trim().parse::<i32>() {
        if let Some(task) = task_list.tasks.iter_mut().find(|task| task.id == id) {
            task.completed = true;
            save_tasks(&task_list, file_path).expect("Failed to save tasks");
            return;
        }
    }
    println!("Task not found");
}

pub fn remove_task(file_path: &Path) {
    let mut task_list = load_tasks(file_path).expect("Failed to load tasks");

    println!("Enter the ID of the task you want to remove:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    if let Ok(id) = input.trim().parse::<i32>() {
        task_list.tasks.retain(|task| task.id != id);
        save_tasks(&task_list, file_path).expect("Failed to save tasks");
        return;
    }
    println!("Task not found");
}
