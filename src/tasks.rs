use std::{io, path::Path};

use crate::storage::{load_tasks, save_task, Task};

pub fn add_task(file_path: &Path) {
    let mut tasks = load_tasks(file_path).expect("Cannot load tasks");
    let mut next_id = tasks.iter().map(|task| task.id).max().unwrap_or(0);
    println!("Enter tasks (type 'done' to finish):");

   loop {
        // Read task
        let mut task = String::new();
        io::stdin().read_line(&mut task).expect("Cannot read task");

        if task.trim().eq_ignore_ascii_case("done") {
            break;
        }

        if task.trim().is_empty() {
            println!("Task description cannot be empty!");
            continue;
        }

        next_id += 1;
        let new_task = Task {
            id: next_id,
            description: task.trim().to_string(),
            completed: false,
        };

        println!("Adding task: {:?}", new_task); 

        tasks.push(new_task);
    }

    save_task(&tasks, file_path).unwrap();
}

pub fn list_task(file_path: &Path) {
    let tasks = load_tasks(file_path).expect("Failed to load tasks");
    if tasks.is_empty() {
        println!("No tasks found");
        return;
    }

    for task in tasks {
        let status = if task.completed {
            "Completed"
        } else {
            "Pending"
        };
        println!(
            "id: {}, description: {}, Status: {}",
            task.id, task.description, status
        );
    }
}

pub fn done_task(file_path: &Path) {
    let mut tasks = load_tasks(file_path).expect("Failed to load tasks");

    println!("Enter the ID of the task you want to mark as done:");
    let mut task_id = String::new();
    io::stdin()
        .read_line(&mut task_id)
        .expect("Failed to read input");
    let task_id: i32 = task_id.trim().parse().expect("Enter valid id");

    for task in tasks.iter_mut() {
        if task.id == task_id {
            task.completed = true;
            break;
        }
    }

    save_task(&tasks, file_path).expect("Failed to save");
}
pub fn remove_task(file_path: &Path) {
    let mut tasks = load_tasks(file_path).expect("Failed to load tasks");

    println!("Enter the ID of the task you want to remove:");
    let mut task_id = String::new();
    io::stdin()
        .read_line(&mut task_id)
        .expect("Failed to read input");
    let task_id: i32 = task_id.trim().parse().expect("Enter valid id");

    let pos = match tasks.iter().position(|task| task.id == task_id) {
        Some(pos) => pos,
        None => {
            println!("Task with ID {} not found.", task_id); 
            return;
        }
    };
    tasks.remove(pos);
    save_task(&tasks, file_path).expect("Failed to save");
}