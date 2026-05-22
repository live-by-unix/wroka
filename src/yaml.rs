use anyhow::Result;
use serde::Deserialize;
use serde_yaml::{Mapping, Value};
use tokio::fs;

#[derive(Debug, Deserialize)]
pub struct RawConfig {
    pub name: String,
    pub version: Value,
    pub steps: Vec<Mapping>,
}

#[derive(Debug, Clone)]
pub struct Step {
    pub plugin: String,
    pub config: Value,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub name: String,
    pub version: Value,
    pub steps: Vec<Step>,
}

pub async fn load_yaml(path: &str) -> Result<Config> {
    let content = fs::read_to_string(path).await?;
    let raw: RawConfig = serde_yaml::from_str(&content)?;

    let mut steps = Vec::new();

    for mapping in raw.steps {
        for (key, value) in mapping {
            if let Some(plugin) = key.as_str() {
                steps.push(Step {
                    plugin: plugin.to_string(),
                    config: value,
                });
            }
        }
    }

    Ok(Config {
        name: raw.name,
        version: raw.version,
        steps,
    })
}
