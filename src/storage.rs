use std::{
    fs::File,
    io::{BufReader, BufWriter, Result},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

pub fn save_task(tasks: &Vec<Task>, file_path: &Path) -> Result<()> {
    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, tasks)?;
    Ok(())
}

pub fn load_tasks(file_path: &Path) -> Result<Vec<Task>> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return Ok(vec![]),
    };

    let reader = BufReader::new(file);
    let tasks = match serde_json::from_reader(reader) {
        Ok(tasks) => tasks,
        Err(_) => vec![],
    };

    Ok(tasks)
}
