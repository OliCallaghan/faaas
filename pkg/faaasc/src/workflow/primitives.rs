use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use wac_graph::{CompositionGraph, EncodeOptions};

use super::generate::Generate;
use super::register::Register;

#[derive(Deserialize, Serialize)]
pub struct Workflow(pub Primitive);

impl Workflow {
    pub fn from_str(workflow_str: &str) -> Result<Self> {
        let workflow = serde_json::from_str(workflow_str)?;

        Ok(workflow)
    }

    pub fn compile(&self, deps: &HashMap<String, Arc<Bytes>>) -> Result<Bytes> {
        let mut graph = CompositionGraph::new();

        self.register(&mut graph, &deps)?;
        self.generate(&mut graph)?;

        let encoding = graph.encode(EncodeOptions::default())?;

        Ok(encoding.into())
    }
}

#[derive(Deserialize, Serialize)]
pub enum Primitive {
    #[serde(rename = "linear")]
    Linear(Box<Primitive>, Box<Primitive>),
    #[serde(rename = "task")]
    Task(String),
}
