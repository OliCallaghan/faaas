use async_trait::async_trait;
use futures::future::join_all;
use std::{
    collections::{HashMap, HashSet},
    iter::zip,
    sync::Arc,
};

use anyhow::Result;
use bytes::Bytes;

use crate::{Primitive, Workflow};

#[async_trait]
pub trait ComponentResolver: Send + Sync {
    async fn resolve_component(&self, component_id: &str) -> Result<Arc<Bytes>>;
}

pub trait DependencyResolver {
    fn extract(&self, d: &mut HashSet<String>) -> Result<()>;
}

impl Workflow {
    pub async fn extract_and_resolve(
        &self,
        r: &dyn ComponentResolver,
    ) -> Result<HashMap<String, Arc<Bytes>>> {
        let mut deps = HashSet::new();

        self.0.extract(&mut deps)?;

        let dep_futs = deps.iter().map(|dep| r.resolve_component(&dep));
        let dep_data = join_all(dep_futs)
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()?;

        let deps_lookup = zip(deps, dep_data).collect::<HashMap<_, _>>();

        Ok(deps_lookup)
    }
}

impl DependencyResolver for Primitive {
    fn extract(&self, d: &mut HashSet<String>) -> Result<()> {
        match self {
            Primitive::Task(task_id) => {
                d.insert(task_id.clone());
            }
            Primitive::Linear(p1, p2) => {
                p1.extract(d)?;
                p2.extract(d)?;
            }
        };

        Ok(())
    }
}
