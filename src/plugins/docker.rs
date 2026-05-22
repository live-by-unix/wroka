use super::Plugin;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_yaml::Value;
use tokio::process::Command;

#[derive(Debug, Deserialize)]
struct DockerConfig {
    image: String,
    push: Option<bool>,
}

pub struct DockerPlugin;

#[async_trait]
impl Plugin for DockerPlugin {
    async fn execute(&self, config: Value) -> Result<()> {
        let cfg: DockerConfig = serde_yaml::from_value(config)?;

        run("docker", &["build", "-t", &cfg.image, "."]).await?;

        if cfg.push.unwrap_or(false) {
            run("docker", &["push", &cfg.image]).await?;
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
