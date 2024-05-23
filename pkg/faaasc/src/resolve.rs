use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait ComponentResolver: Send + Sync {
    async fn resolve_component(&self, component_id: &str) -> Result<Arc<Bytes>>;
}
