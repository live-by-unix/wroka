use super::Plugin;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_yaml::Value;
use tokio::process::Command;

#[derive(Debug, Deserialize)]
struct AptConfig {
    repo: String,
    package: String,
}

pub struct AptPlugin;

#[async_trait]
impl Plugin for AptPlugin {
    async fn execute(&self, config: Value) -> Result<()> {
        let cfg: AptConfig = serde_yaml::from_value(config)?;

        run("sudo", &["add-apt-repository", "-y", &cfg.repo]).await?;
        run("sudo", &["apt-get", "update"]).await?;
        run("sudo", &["apt-get", "install", "-y", &cfg.package]).await?;

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
