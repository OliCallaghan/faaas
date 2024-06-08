use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MqValue {
    Bool(bool),
    Int(i32),
    Uint(u32),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MqTaskContext {
    pub id: String,
    pub task_id: String,
    pub args: Vec<MqValue>,
    pub data: HashMap<String, MqValue>,
    pub continuation: Option<String>,
    pub continuation_args: Vec<MqValue>,
}

impl MqTaskContext {
    pub fn new(id: &str, task_id: &str) -> Self {
        Self {
            id: id.to_owned(),
            task_id: task_id.to_owned(),
            args: Vec::new(),
            data: Default::default(),
            continuation: None,
            continuation_args: Vec::new(),
        }
    }

    pub fn continuation(&mut self) {
        if let Some(continuation) = self.continuation.take() {
            self.task_id = continuation;
            self.args = self.continuation_args.clone();
            self.continuation_args = vec![]
        }
    }

    pub fn set_continuation(&mut self, task_id: &str, args: Vec<MqValue>) {
        self.continuation = Some(task_id.to_owned());
        self.continuation_args = args;
    }
}

pub fn get_task_init_id(task_id: &str) -> String {
    format!("{}_0", task_id)
}
