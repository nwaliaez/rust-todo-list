mod storage;
mod tasks;

use std::{io, path::Path};

use crate::tasks::{add_task, done_task, list_task, remove_task};

fn main() {
    let mut command = String::new();
    println!("Enter command (add, list, done, remove)");

    io::stdin()
        .read_line(&mut command)
        .expect("Not able to read command");

    // File path
    let file_path = Path::new("tasks.json");

    match command.trim() {
        "add" => add_task(file_path),
        "list" => list_task(file_path),
        "done" => done_task(file_path),
        "remove" => remove_task(file_path),
        _ => println!("unknown command"),
    }
}
