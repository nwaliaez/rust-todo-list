use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Result},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: i32, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { tasks: Vec::new() }
    }
}

pub fn save_tasks(tasks: &TaskList, file_path: &Path) -> Result<()> {
    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, tasks)?;
    Ok(())
}

pub fn load_tasks(file_path: &Path) -> Result<TaskList> {
    if !file_path.exists() {
        return Ok(TaskList::new());
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}
