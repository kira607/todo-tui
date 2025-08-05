use crate::utils::generate_id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub done: bool,
    pub created: DateTime<Utc>,
}

#[derive(Default)]
pub struct Tasks {
    file_path: PathBuf,
    tasks: Vec<Task>,
}

impl Tasks {
    /// Create an instance of a TaskManager from a vector of Tasks.
    pub fn from_tasks(tasks: Vec<Task>) -> Self {
        Self {
            file_path: PathBuf::default(),
            tasks: tasks,
        }
    }

    /// Create an instance of a TaskManager from a tasks file path.
    pub fn from_path(path: &Path) -> Self {
        let mut s = Self {
            file_path: path.to_path_buf(),
            tasks: Vec::default(),
        };
        let _ = s.load();
        s
    }

    /// Add a new task.
    pub fn add_task(&mut self, title: &str) -> &Task {
        let task = Task {
            id: generate_id(None),
            title: title.to_string(),
            done: false,
            created: Utc::now(),
        };
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }

    /// Get a vector of all tasks.
    pub fn all(&self) -> &[Task] {
        &self.tasks
    }

    /// Get a task by id.
    pub fn get(&self, id: &str) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    /// Get a task at given index.
    pub fn at(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }

    /// Get the number of tasks.
    pub fn count(&self) -> usize {
        self.tasks.len()
    }

    /// Toggle a task completion.
    pub fn toggle(&mut self, id: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = !task.done;
        }
    }

    /// Update a task titile.
    pub fn update_title(&mut self, id: &str, new_title: &str) {
        let task = self.tasks.iter_mut().find(|t| t.id == id);
        if let Some(task) = task {
            task.title = String::from(new_title);
        }
    }

    /// Delete a task.
    pub fn delete(&mut self, id: &str) {
        self.tasks.retain(|t| t.id != id);
    }

    /// Override tasks with the given list. All current tasks are lost.
    fn set_tasks(&mut self, tasks: Vec<Task>) {
        self.tasks = tasks;
    }

    /// Save current state to a file.
    pub fn save(&self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self.tasks)?;
        fs::write(self.file_path.to_path_buf(), json)?;
        Ok(())
    }

    /// Load tasks from a file.
    fn load(&mut self) -> std::io::Result<()> {
        let data = fs::read_to_string(self.file_path.clone())?;
        let tasks: Vec<Task> = serde_json::from_str(&data)?;
        self.set_tasks(tasks);
        Ok(())
    }
}
