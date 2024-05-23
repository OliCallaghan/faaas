use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Primitive {
    #[serde(rename = "linear")]
    Linear(Box<Primitive>, Box<Primitive>),
    #[serde(rename = "task")]
    Task(String),
}

pub const LINEAR_WASM: &[u8] =
    include_bytes!("../../node_modules/@faaas/linear/target/wasm32-wasi/release/linear.wasm");
