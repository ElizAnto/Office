// src/task.rs
#[derive(Debug)]
pub struct Task {
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Task {
            description,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn display(&self) {
        let status = if self.completed { "Completed" } else { "Pending" };
        println!("Task: {} - Status: {}", self.description, status);
    }
}
