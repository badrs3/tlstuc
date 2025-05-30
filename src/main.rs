use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use log::{debug, info};
use std::path::{Path, PathBuf};
use std::process;

mod compiler;
mod runtime;
mod update;
mod utils;

#[derive(Parser)]
#[command(name = "tc")]
#[command(about = "tlstuc - A modern C language runtime", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// C file to compile and run
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new C project
    Init,
    
    /// Check for updates to tlstuc
    Update,
    
    /// Show version information
    Version,
}

fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    if cli.verbose {
        std::env::set_var("RUST_LOG", "debug");
        debug!("Verbose mode enabled");
    }
    
    match &cli.command {
        Some(Commands::Init) => {
            init_project()?;
        }
        Some(Commands::Update) => {
            update::check_and_update()?;
        }
        Some(Commands::Version) => {
            println!("tlstuc version {}", env!("CARGO_PKG_VERSION"));
        }
        None => {
            // If no subcommand is provided but a file is, compile and run it
            if let Some(file) = &cli.file {
                compile_and_run(file)?;
            } else {
                println!("No command or file specified. Use 'tc --help' for usage information.");
                process::exit(1);
            }
        }
    }
    
    Ok(())
}

/// Initialize a new C project with a template file
fn init_project() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let tc_file_path = current_dir.join("tc.c");
    
    if tc_file_path.exists() {
        println!("A tc.c file already exists in this directory.");
        return Ok(());
    }
    
    let template = r#"#include <stdio.h>

int main() {
    printf("Hello from tlstuc!\n");
    return 0;
}
"#;
    
    std::fs::write(&tc_file_path, template)
        .context("Failed to create tc.c file")?;
    
    println!("Created tc.c in {}", current_dir.display());
    println!("Run 'tc tc.c' to compile and run it.");
    
    Ok(())
}

/// Compile and run a C file
fn compile_and_run(file_path: &Path) -> Result<()> {
    info!("Compiling and running {}", file_path.display());
    
    // Check if the file exists
    if !file_path.exists() {
        anyhow::bail!("File '{}' does not exist", file_path.display());
    }
    
    // Check if it's a C file
    if file_path.extension().and_then(|ext| ext.to_str()) != Some("c") {
        anyhow::bail!("File '{}' is not a C file", file_path.display());
    }
    
    // Compile the file
    let executable = compiler::compile(file_path)?;
    
    // Run the executable
    runtime::run(&executable)?;
    
    Ok(())
}