WROKA Security Model

WROKA is designed for maximum power. This means plugins are not sandboxed and run with full system access of the user running the WROKA program. This document explains what that means and how to use WROKA safely.

Security Warning

WROKA plugins are:

- Real Rust code
- Compiled directly into your binary
- Executed with your user permissions
- Capable of performing any system action

A plugin can:

- Read, write, or delete files
- Run shell commands
- Access environment variables
- Make network requests
- Install or remove software
- Upload or download data
- Modify system configuration

If you would not run a script from someone, do not install their plugin.

Why This Is a Feature

DevOps automation requires real power:

- Deploying infrastructure
- Building containers
- Managing packages
- Running CI/CD steps
- Editing configuration files

Sandboxing would make WROKA useless. WROKA is honest: it gives you power, not guardrails.

Best Practices

1. Only install plugins from trusted developers.
2. Review plugin source code before using it.
3. Use version control for your plugin folder.
4. Do not run WROKA as root unless absolutely necessary.
5. Keep your YAML files in Git.
6. Pin plugin versions to avoid supply chain attacks.

Threat Model

Threat: Malicious plugin  
Mitigation: Only install trusted plugins

Threat: Plugin deletes files  
Mitigation: Version control and backups

Threat: Plugin exfiltrates data  
Mitigation: Review code before installing

Threat: Plugin runs harmful commands  
Mitigation: Run WROKA as a non-root user

Threat: Supply chain attack  
Mitigation: Pin plugin versions

IF YOU DISCOVER A VULNERABLILITY, PLEASE OPEN A GITHUB ISSUE RIGHT AWAY. IF YOU DO NOT HAVE A GITHUB ACCOUNT, TELL A FAMILY OR FRIEND WITH ONE TO REPORT THE ISSUE RIGHT AWAY. 
