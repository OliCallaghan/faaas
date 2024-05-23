use std::collections::{HashMap, HashSet};
use std::iter::zip;
use std::sync::Arc;

use anyhow::Result;
use bytes::Bytes;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use wac_graph::{CompositionGraph, EncodeOptions};

use super::resolve::ComponentResolver;

mod dependencies;
mod generate;
mod primitives;
mod register;

use self::dependencies::{Dependency, DependencyCollector};
use self::generate::Generate;
use self::primitives::Primitive;
use self::register::Register;

#[derive(Deserialize, Serialize)]
pub struct Workflow(pub Primitive);

impl Workflow {
    pub fn from_str(workflow_str: &str) -> Result<Self> {
        let workflow = serde_json::from_str(workflow_str)?;

        Ok(workflow)
    }

    pub async fn resolve_dependencies(
        &self,
        r: &dyn ComponentResolver,
    ) -> Result<HashMap<String, Dependency>> {
        let mut deps = HashSet::new();

        self.0.collect_dependencies(&mut deps)?;

        let dep_futs = deps.iter().map(|dep| r.resolve_component(&dep));
        let dep_data = join_all(dep_futs)
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()?;

        let deps_lookup = zip(deps, dep_data).collect::<HashMap<_, _>>();

        Ok(deps_lookup)
    }

    pub fn compile(&self, deps: &HashMap<String, Arc<Bytes>>) -> Result<Bytes> {
        let mut graph = CompositionGraph::new();

        self.register(&mut graph, &deps)?;
        self.generate(&mut graph)?;

        let encoding = graph.encode(EncodeOptions::default())?;

        Ok(encoding.into())
    }
}
