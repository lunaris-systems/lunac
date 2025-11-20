use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "lunac")]
#[command(about = "Lunaris compiler - build tool and plugin manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build Lunaris
    Build {
        /// Build with release optimizations
        #[arg(short, long)]
        release: bool,

        /// Build without the runtime (barebones)
        #[arg(long)]
        barebones: bool,

        /// Additional arguments to pass to cargo
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Run Lunaris
    Run {
        /// Run with release optimizations
        #[arg(short, long)]
        release: bool,

        /// Run without the runtime (barebones)
        #[arg(long)]
        barebones: bool,

        /// Additional arguments to pass to cargo
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Check code without building
    Check {
        /// Additional arguments to pass to cargo
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Run clippy linter
    Clippy {
        /// Additional arguments to pass to cargo
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Run tests
    Test {
        /// Additional arguments to pass to cargo
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Update plugin linker
    Update,

    /// Add a plugin dependency
    Add {
        /// Plugin name or path
        plugin: String,
    },

    /// Remove a plugin dependency
    Remove {
        /// Plugin name
        plugin: String,
    },

    /// Align plugin versions
    Align,

    /// Validate lunaris.toml
    Validate,

    /// Create new plugin
    New {
        /// Plugin type (effect, timeline, codec, etc.)
        plugin_type: String,
        /// Plugin name
        name: String,
    },
}

fn main() -> Result<()> {
    // Find workspace root by looking for Cargo.toml with [workspace]
    let workspace_root = find_workspace_root()?;
    std::env::set_current_dir(&workspace_root)?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Build {
            release,
            barebones,
            args,
        } => {
            let mut cmd_args = vec!["build", "--package", "lunaris"];
            if !barebones {
                cmd_args.push("--features");
                cmd_args.push("full");
            }
            if release {
                cmd_args.push("--release");
            }
            let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            cmd_args.extend(str_args);
            cargo(&cmd_args)
        }

        Commands::Run {
            release,
            barebones,
            args,
        } => {
            update()?;
            let mut cmd_args = vec!["run", "--package", "lunaris"];
            if !barebones {
                cmd_args.push("--features");
                cmd_args.push("full");
            }
            if release {
                cmd_args.push("--release");
            }
            let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            cmd_args.extend(str_args);
            cargo(&cmd_args)
        }

        Commands::Check { args } => {
            let mut cmd_args = vec!["check", "--package", "lunaris"];
            let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            cmd_args.extend(str_args);
            cargo(&cmd_args)
        }

        Commands::Clippy { args } => {
            let mut cmd_args = vec!["clippy", "--package", "lunaris"];
            let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            cmd_args.extend(str_args);
            cargo(&cmd_args)
        }

        Commands::Test { args } => {
            let mut cmd_args = vec!["test", "--package", "lunaris"];
            let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            cmd_args.extend(str_args);
            cargo(&cmd_args)
        }

        Commands::Update => update(),

        Commands::Add { plugin } => {
            println!("Adding plugin: {}", plugin);
            println!("(Not implemented yet - will install from registry)");
            Ok(())
        }

        Commands::Remove { plugin } => {
            println!("Removing plugin: {}", plugin);
            println!("(Not implemented yet)");
            Ok(())
        }

        Commands::Align => {
            println!("Aligning plugin versions...");
            println!("(Not implemented yet)");
            Ok(())
        }

        Commands::Validate => {
            println!("Validating lunaris.toml...");
            println!("(Not implemented yet)");
            Ok(())
        }

        Commands::New { plugin_type, name } => {
            println!("Creating new {} plugin: {}", plugin_type, name);
            println!("(Not implemented yet)");
            Ok(())
        }
    }
}

fn cargo(args: &[&str]) -> Result<()> {
    let status = Command::new("cargo")
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        anyhow::bail!("cargo command failed");
    }

    Ok(())
}

fn update() -> Result<()> {
    cargo(&[
        "run",
        "-q",
        "-p",
        "linker_updater",
        "--",
        "crates/linker/Cargo.toml",
        "plugins/",
    ])
}

fn find_workspace_root() -> Result<std::path::PathBuf> {
    let mut current = std::env::current_dir()?;

    loop {
        let cargo_toml = current.join("Cargo.toml");

        if cargo_toml.exists() {
            // Check if this is a workspace root
            let content = std::fs::read_to_string(&cargo_toml)?;
            if content.contains("[workspace]") {
                return Ok(current);
            }
        }

        // Move up to parent directory
        if !current.pop() {
            anyhow::bail!(
                "Could not find Lunaris workspace root.\n\
                 lunac must be run from within the Lunaris project directory."
            );
        }
    }
}
