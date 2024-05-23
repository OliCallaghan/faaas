use std::{collections::HashSet, sync::Arc};

use anyhow::Result;
use bytes::Bytes;

use super::Primitive;

pub type Dependency = Arc<Bytes>;

pub trait DependencyCollector {
    fn collect_dependencies(&self, d: &mut HashSet<String>) -> Result<()>;
}

impl DependencyCollector for Primitive {
    fn collect_dependencies(&self, d: &mut HashSet<String>) -> Result<()> {
        match self {
            Primitive::Task(task_id) => {
                d.insert(task_id.clone());
            }
            Primitive::Linear(p1, p2) => {
                p1.collect_dependencies(d)?;
                p2.collect_dependencies(d)?;
            }
        };

        Ok(())
    }
}
