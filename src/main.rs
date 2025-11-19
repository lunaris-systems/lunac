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
        
        /// Additional arguments to pass to cargo
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    
    /// Run Lunaris
    Run {
        /// Run with release optimizations
        #[arg(short, long)]
        release: bool,
        
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
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { release, args } => {
            let mut cmd_args = vec!["build", "--package", "lunaris_core"];
            if release {
                cmd_args.push("--release");
            }
            let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            cmd_args.extend(str_args);
            cargo(&cmd_args)
        }
        
        Commands::Run { release, args } => {
            update()?;
            let mut cmd_args = vec!["run", "--package", "lunaris_core"];
            if release {
                cmd_args.push("--release");
            }
            let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            cmd_args.extend(str_args);
            cargo(&cmd_args)
        }
        
        Commands::Check { args } => {
            let mut cmd_args = vec!["check", "--package", "lunaris_core"];
            let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            cmd_args.extend(str_args);
            cargo(&cmd_args)
        }
        
        Commands::Clippy { args } => {
            let mut cmd_args = vec!["clippy", "--package", "lunaris_core"];
            let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            cmd_args.extend(str_args);
            cargo(&cmd_args)
        }
        
        Commands::Test { args } => {
            let mut cmd_args = vec!["test", "--package", "lunaris_core"];
            let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            cmd_args.extend(str_args);
            cargo(&cmd_args)
        }
        
        Commands::Update => {
            update()
        }
        
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
        "linker/Cargo.toml",
        "plugins/",
    ])
}

