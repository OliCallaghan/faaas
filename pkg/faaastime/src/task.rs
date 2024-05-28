use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TaskInvocation {
    pub id: String,
    pub task_id: String,
    pub args: Vec<String>,
}

impl TaskInvocation {
    pub fn new(id: &str, task_id: &str) -> Self {
        Self {
            id: id.to_owned(),
            task_id: task_id.to_owned(),
            args: Vec::new(),
        }
    }
}
