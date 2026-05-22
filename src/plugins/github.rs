use super::Plugin;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::Utc;
use serde::Deserialize;
use serde_yaml::Value;
use tokio::process::Command;

#[derive(Debug, Deserialize)]
struct GithubConfig {
    repo: Option<String>,
    commit: Option<bool>,
    push: Option<bool>,
    tag: Option<String>,
    release: Option<bool>,
}

pub struct GithubPlugin;

#[async_trait]
impl Plugin for GithubPlugin {
    async fn execute(&self, config: Value) -> Result<()> {
        let cfg: GithubConfig = serde_yaml::from_value(config)?;

        run("git", &["init"]).await?;
        run("git", &["add", "."]).await?;

        if cfg.commit.unwrap_or(false) {
            let msg = format!("WROKA {}", Utc::now().timestamp());
            let _ = run("git", &["commit", "-m", &msg]).await;
        }

        if let Some(tag) = cfg.tag {
            let value = if tag == "auto" {
                format!("v{}", Utc::now().timestamp())
            } else {
                tag
            };

            let _ = run("git", &["tag", &value]).await;
        }

        if cfg.push.unwrap_or(false) {
            let _ = run("git", &["push"]).await;
            let _ = run("git", &["push", "--tags"]).await;
        }

        if cfg.release.unwrap_or(false) {
            if let Some(repo) = cfg.repo {
                let _ = run(
                    "gh",
                    &[
                        "release",
                        "create",
                        "latest",
                        "--repo",
                        &repo,
                        "--generate-notes",
                    ],
                )
                .await;
            }
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
