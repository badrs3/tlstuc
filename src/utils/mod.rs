use anyhow::{Context, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Project configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub compiler_options: CompilerOptions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerOptions {
    pub optimization_level: OptimizationLevel,
    pub warnings_as_errors: bool,
    pub include_paths: Vec<PathBuf>,
    pub library_paths: Vec<PathBuf>,
    pub libraries: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OptimizationLevel {
    None,
    Less,
    Default,
    Aggressive,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "tlstuc_project".to_string(),
            version: "0.1.0".to_string(),
            author: None,
            compiler_options: CompilerOptions {
                optimization_level: OptimizationLevel::Default,
                warnings_as_errors: false,
                include_paths: vec![],
                library_paths: vec![],
                libraries: vec![],
            },
        }
    }
}

/// Load project configuration from tc.toml
pub fn load_config() -> Result<Config> {
    let config_path = std::env::current_dir()?.join("tc.toml");
    
    if !config_path.exists() {
        debug!("No tc.toml found, using default configuration");
        return Ok(Config::default());
    }
    
    debug!("Loading configuration from {}", config_path.display());
    
    let config_str = std::fs::read_to_string(&config_path)
        .context("Failed to read tc.toml")?;
    
    let config: Config = toml::from_str(&config_str)
        .context("Failed to parse tc.toml")?;
    
    Ok(config)
}

/// Save project configuration to tc.toml
pub fn save_config(config: &Config) -> Result<()> {
    let config_path = std::env::current_dir()?.join("tc.toml");
    
    debug!("Saving configuration to {}", config_path.display());
    
    let config_str = toml::to_string_pretty(config)
        .context("Failed to serialize configuration")?;
    
    std::fs::write(&config_path, config_str)
        .context("Failed to write tc.toml")?;
    
    Ok(())
}

/// Find all C files in a directory
pub fn find_c_files(dir: &Path) -> Result<Vec<PathBuf>> {
    debug!("Finding C files in {}", dir.display());
    
    let mut c_files = Vec::new();
    
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("c") {
            c_files.push(path);
        } else if path.is_dir() {
            // Skip hidden directories
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if !file_name_str.starts_with(".") {
                        c_files.append(&mut find_c_files(&path)?);
                    }
                }
            }
        }
    }
    
    Ok(c_files)
}

/// Create a formatted error message
pub fn format_error(message: &str, file: Option<&Path>, line: Option<usize>, column: Option<usize>) -> String {
    let location = if let Some(file) = file {
        let file_name = file.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("<unknown>");
        
        if let Some(line) = line {
            if let Some(column) = column {
                format!("{} ({}:{})", file_name, line, column)
            } else {
                format!("{} (line {})", file_name, line)
            }
        } else {
            file_name.to_string()
        }
    } else {
        "".to_string()
    };
    
    if location.is_empty() {
        format!("error: {}", message)
    } else {
        format!("error: {} in {}", message, location)
    }
}

/// Get the absolute path for a file
pub fn get_absolute_path(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        let current_dir = std::env::current_dir()?;
        Ok(current_dir.join(path))
    }
}