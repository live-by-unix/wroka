use super::Plugin;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_yaml::Value;
use tokio::process::Command;

#[derive(Debug, Deserialize)]
struct HomebrewConfig {
    formula: String,
    tap: String,
}

pub struct HomebrewPlugin;

#[async_trait]
impl Plugin for HomebrewPlugin {
    async fn execute(&self, config: Value) -> Result<()> {
        let cfg: HomebrewConfig = serde_yaml::from_value(config)?;

        run("brew", &["tap", &cfg.tap]).await?;
        run("brew", &["update"]).await?;
        run("brew", &["upgrade", &cfg.formula]).await?;

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
