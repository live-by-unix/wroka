use crate::plugins::{
    apt::AptPlugin,
    azure::AzurePlugin,
    docker::DockerPlugin,
    github::GithubPlugin,
    homebrew::HomebrewPlugin,
    Plugin,
};
use crate::yaml::{Config, Step};
use anyhow::{anyhow, Result};
use colored::*;
use std::collections::HashMap;

pub struct Engine {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl Engine {
    pub fn new() -> Self {
        let mut plugins: HashMap<String, Box<dyn Plugin>> = HashMap::new();

        plugins.insert("github".to_string(), Box::new(GithubPlugin));
        plugins.insert("docker".to_string(), Box::new(DockerPlugin));
        plugins.insert("azure".to_string(), Box::new(AzurePlugin));
        plugins.insert("homebrew".to_string(), Box::new(HomebrewPlugin));
        plugins.insert("apt".to_string(), Box::new(AptPlugin));

        Self { plugins }
    }

    pub async fn run(&mut self, config: Config) -> Result<()> {
        println!("{}", format!("Deploying {}", config.name).green());

        for step in config.steps {
            self.execute(step).await?;
        }

        println!("{}", "Deployment completed".green());

        Ok(())
    }

    async fn execute(&self, step: Step) -> Result<()> {
        let plugin = self
            .plugins
            .get(&step.plugin)
            .ok_or_else(|| anyhow!("Unknown plugin {}", step.plugin))?;

        println!("{}", format!("Running {}", step.plugin).cyan());

        plugin.execute(step.config).await?;

        Ok(())
    }
}
