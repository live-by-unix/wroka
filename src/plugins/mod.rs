pub mod apt;
pub mod azure;
pub mod docker;
pub mod github;
pub mod homebrew;

use anyhow::Result;
use async_trait::async_trait;
use serde_yaml::Value;

#[async_trait]
pub trait Plugin: Send + Sync {
    async fn execute(&self, config: Value) -> Result<()>;
}
