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
}
