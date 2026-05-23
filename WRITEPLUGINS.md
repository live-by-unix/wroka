Writing WROKA Plugins

WROKA plugins are tiny, simple, and extremely powerful. This guide explains how to write them safely and correctly.

Plugin Location

All plugins live in:

    src/plugins/

Each plugin is either:

- a single Rust file, or
- a folder containing multiple Rust files

Plugin Structure

A plugin exposes a single function:

    pub fn run(args: &YamlValue) -> Result<(), String>

This function receives the YAML arguments and performs the plugin's logic.

Example Plugin

    use crate::utils::run;

    pub fn run(args: &YamlValue) -> Result<(), String> {
        let image = args.as_str().unwrap();
        run(&["docker", "pull", image])?;
        Ok(())
    }

This plugin pulls a Docker image.

How Plugins Are Called

YAML:

    docker.pull: "nginx:latest"

WROKA:

1. Finds docker.rs
2. Calls run()
3. Passes "nginx:latest" as args

Creating a New Plugin

1. Create a file:

       src/plugins/myplugin.rs

2. Add the function:

       pub fn run(args: &YamlValue) -> Result<(), String> {
           println!("My plugin received: {:?}", args);
           Ok(())
       }

3. Register it in mod.rs:

       pub mod myplugin;

4. Use it in YAML:

       myplugin: "hello world"

Multi-File Plugins

If your plugin needs more files:

    src/plugins/myplugin/
        mod.rs
        logic.rs
        helpers.rs

mod.rs:

    mod logic;
    mod helpers;

    pub fn run(args: &YamlValue) -> Result<(), String> {
        logic::execute(args)
    }

Plugins in Other Languages

Plugins can call external scripts:

    run(&["python3", "myscript.py", arg])?;

Or:

    run(&["bash", "myscript.sh"])?;

The Rust plugin acts as the glue.

Testing Plugins

Run all tests:

    cargo test

Test manually:

    cargo run -- myplugin: "test"

Plugin Philosophy

Plugins should be:

- Small
- Focused
- Easy to read
- Easy to trust

If your plugin grows beyond 50 lines, split it into multiple files.

You are now ready to build:

- Single-file plugins
- Multi-file plugins
- Script-backed plugins
- System-level automation plugins

