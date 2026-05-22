use super::Plugin;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_yaml::Value;
use tokio::process::Command;

#[derive(Debug, Deserialize)]
struct AzureConfig {
    webapp: String,
    deploy: Option<bool>,
}

pub struct AzurePlugin;

#[async_trait]
impl Plugin for AzurePlugin {
    async fn execute(&self, config: Value) -> Result<()> {
        let cfg: AzureConfig = serde_yaml::from_value(config)?;

        if cfg.deploy.unwrap_or(false) {
            run("az", &["webapp", "up", "--name", &cfg.webapp]).await?;
        }

        Ok(())
    }
}

async fn run(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd).args(args).status().await?;

    if !status.success() {
        return Err(anyhow!("command failed"));
    }

    Ok(())
}
