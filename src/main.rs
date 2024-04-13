mod storage;
mod tasks;

use std::{io, path::Path};

use crate::tasks::{add_task, complete_task, list_tasks, remove_task};

fn main() {
    let file_path = Path::new("tasks.json");

    loop {
        println!("Enter command (add, list, done, remove, exit):");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("failed to read command");

        match command.trim() {
            "add" => add_task(file_path),
            "list" => list_tasks(file_path),
            "done" => complete_task(file_path),
            "remove" => remove_task(file_path),
            "exit" => break,
            _ => println!("unknown command"),
        }
    }
}
