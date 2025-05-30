use anyhow::{Context, Result};
use log::{debug, info};
use self_update::cargo_crate_version;
use std::path::PathBuf;

/// Repository information for updates
const REPO_OWNER: &str = "badrs3";
const REPO_NAME: &str = "tlstuc";

/// Check for updates and apply them if available
pub fn check_and_update() -> Result<()> {
    info!("Checking for updates");
    
    let status = self_update::backends::github::Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name("tc")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    
    match status {
        self_update::Status::UpToDate(version) => {
            println!("tlstuc is already up to date (version {})", version);
        }
        self_update::Status::Updated(version) => {
            println!("tlstuc has been updated to version {}", version);
        }
    }
    
    Ok(())
}

/// Get the installation directory
pub fn get_install_dir() -> Result<PathBuf> {
    // Try to find the executable path
    let exe_path = std::env::current_exe()
        .context("Failed to get current executable path")?;
    
    // The installation directory is the parent of the executable
    let install_dir = exe_path.parent()
        .context("Failed to get parent directory of executable")?;
    
    Ok(install_dir.to_path_buf())
}

/// Check if tlstuc is in the system PATH
pub fn is_in_path() -> Result<bool> {
    debug!("Checking if tlstuc is in PATH");
    
    // Use the which crate to check if tc is in the PATH
    match which::which("tc") {
        Ok(path) => {
            debug!("Found tc in PATH at {}", path.display());
            Ok(true)
        }
        Err(_) => {
            debug!("tc not found in PATH");
            Ok(false)
        }
    }
}

/// Add tlstuc to the system PATH
pub fn add_to_path() -> Result<()> {
    debug!("Adding tlstuc to PATH");
    
    let install_dir = get_install_dir()?;
    
    // The method to add to PATH depends on the operating system
    #[cfg(windows)]
    {
        // On Windows, we need to modify the registry
        // This requires administrative privileges, so we'll just print instructions
        println!("To add tlstuc to your PATH, add the following directory to your PATH environment variable:");
        println!("{}", install_dir.display());
        println!("You can do this by opening the Control Panel, going to System and Security > System > Advanced System Settings > Environment Variables, and editing the PATH variable.");
    }
    
    #[cfg(unix)]
    {
        // On Unix, we can add to .bashrc or .zshrc
        let home_dir = std::env::var("HOME")
            .context("Failed to get HOME directory")?;
        
        let shell = std::env::var("SHELL")
            .unwrap_or_else(|_| "/bin/bash".to_string());
        
        let rc_file = if shell.contains("zsh") {
            format!("{}/{}/.zshrc", home_dir)
        } else {
            format!("{}/{}/.bashrc", home_dir)
        };
        
        println!("To add tlstuc to your PATH, add the following line to {}:", rc_file);
        println!("export PATH=\"{}:$PATH\"", install_dir.display());
    }
    
    Ok(())
}