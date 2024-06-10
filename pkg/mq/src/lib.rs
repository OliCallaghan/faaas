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

impl Into<MqValue> for &str {
    fn into(self) -> MqValue {
        if self == "true" {
            return MqValue::Bool(true);
        }

        if self == "false" {
            return MqValue::Bool(false);
        }

        if let Ok(i) = self.parse::<i32>() {
            return MqValue::Int(i);
        }

        if let Ok(u) = self.parse::<u32>() {
            return MqValue::Uint(u);
        }

        if let Ok(f) = self.parse::<f64>() {
            return MqValue::Float(f);
        }

        MqValue::String(self.to_owned())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MqTaskContext {
    pub id: String,
    pub task_id: String,
    pub continuation_id: String,
    pub args: Vec<MqValue>,
    pub state: HashMap<String, MqValue>,
}

impl MqTaskContext {
    pub fn new(id: &str, task_id: &str, continuation_id: &str) -> Self {
        Self {
            id: id.to_owned(),
            task_id: task_id.to_owned(),
            continuation_id: continuation_id.to_owned(),
            args: Vec::new(),
            state: Default::default(),
        }
    }

    pub fn new_with_data(id: &str, task_id: &str, continuation_id: &str, data: String) -> Self {
        Self {
            id: id.to_owned(),
            task_id: task_id.to_owned(),
            continuation_id: continuation_id.to_owned(),
            args: vec![MqValue::String(data)],
            state: Default::default(),
        }
    }

    pub fn continuation(&self, task_id: &str, continuation_id: &str, args: Vec<MqValue>) -> Self {
        Self {
            id: self.id.clone(),
            task_id: task_id.to_owned(),
            continuation_id: continuation_id.to_owned(),
            state: self.state.clone(),
            args,
        }
    }

    pub fn set_continuation(&mut self, task_id: &str, args: Vec<MqValue>) {
        // self.continuation = Some(task_id.to_owned());
        // self.continuation_args = args;
    }
}
