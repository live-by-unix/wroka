WROKA — The One-Command DevOps Engine

WROKA is a tiny, fast, YAML-driven automation engine written in Rust.
It turns a single YAML file into a full DevOps workflow: updates, installs, builds, deploys, and anything else you can script.

WROKA is built on three principles:

- Simplicity — one binary, one YAML, one command
- Power — plugins are real Rust code with full system access
- Speed — zero bloat, zero ceremony, instant execution

Features

- YAML-driven automation
- Plugin-based architecture (Rust, Python, Bash, anything)
- Cross-platform (Linux, macOS, Windows WSL)
- Zero dependencies
- Instant startup
- Extremely small binary
- Deterministic execution
- Human-readable workflows

Installation

Install via Cargo:

    cargo install wroka

Or build from source:

    git clone https://github.com/live-by-unix/wroka.git
    cd wroka
    cargo build --release

The compiled binary will be located at:

    target/release/wroka

How It Works

WROKA reads a YAML file and executes each step using plugins.

Example:

    update:
      - apt.update
      - docker.pull: "nginx:latest"
      - github.clone: "https://github.com/example/repo"

Execution flow:

1. Parse YAML
2. Resolve plugin names
3. Pass arguments to plugin
4. Execute plugin logic
5. Continue until workflow completes

Every plugin is just a Rust module inside src/plugins/ — or a script in any language.

Plugins

Plugins live in:

    src/plugins/

Each plugin exposes a single function:

    pub fn run(args: &YamlValue) -> Result<(), String>

Plugins can:

- Run commands
- Modify files
- Install packages
- Call APIs
- Build containers
- Deploy services
- Do anything Rust can do

See WRITEPLUGINS.md for full details.

Project Structure

    wroka/
    │
    ├── src/
    │   ├── main.rs
    │   ├── yaml.rs
    │   ├── utils.rs
    │   └── plugins/
    │       ├── apt.rs
    │       ├── docker.rs
    │       ├── github.rs
    │       └── ...
    │
    ├── examples/
    │   ├── basic.yaml
    │   ├── deploy.yaml
    │   └── update.yaml
    │
    ├── README.md
    ├── SECURITY.md
    └── WRITEPLUGINS.md

Example Workflow

    setup:
      - apt.update
      - apt.install: ["git", "curl"]
      - docker.pull: "redis:latest"
      - github.clone: "https://github.com/example/app"
      - script.run: "./scripts/bootstrap.sh"

Run it:

    wroka setup.yaml

Command Line Usage

    wroka <file.yaml>

Optional flags:

    --dry-run     Show what would run without executing
    --verbose     Print detailed plugin logs
    --trace       Show internal execution flow

Security

WROKA plugins are full Rust code compiled into your binary.
They run with full system access.

Read SECURITY.md before installing third-party plugins.

Testing

Run all tests:

    cargo test

Test a specific plugin:

    cargo test docker

Manual testing:

    cargo run -- example.yaml

Documentation

- WRITEPLUGINS.md — how to build plugins
- SECURITY.md — plugin safety model
- examples/ — sample YAML workflows


Contributing

Pull requests are welcome.
Plugins should be:

- Small
- Focused
- Easy to review
- Easy to trust

License

MIT License.
See LICENSE for details.
