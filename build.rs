use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Check if we need to install LLVM
    if cfg!(feature = "llvm-static") {
        // When using static LLVM, we need to build it from source
        // This is a complex process and depends on the platform
        println!("cargo:warning=Building with static LLVM is not yet supported");
    } else {
        // When using dynamic LLVM, we need to find the installed version
        // The inkwell crate will handle most of this for us
        println!("cargo:warning=Using system LLVM installation");
        
        // Check if LLVM is installed
        let llvm_config = if cfg!(windows) {
            which::which("llvm-config")
                .or_else(|_| which::which("llvm-config-15"))
                .or_else(|_| which::which("llvm-config-15.0"))
        } else {
            which::which("llvm-config-15")
                .or_else(|_| which::which("llvm-config-15.0"))
                .or_else(|_| which::which("llvm-config"))
        };
        
        match llvm_config {
            Ok(path) => {
                println!("cargo:warning=Found LLVM config at {}", path.display());
                
                // Get LLVM version
                let output = Command::new(&path)
                    .arg("--version")
                    .output()
                    .expect("Failed to execute llvm-config");
                
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!("cargo:warning=LLVM version: {}", version);
                
                // Get LLVM library directory
                let output = Command::new(&path)
                    .arg("--libdir")
                    .output()
                    .expect("Failed to execute llvm-config");
                
                let libdir = String::from_utf8_lossy(&output.stdout).trim().to_string();
                println!("cargo:rustc-link-search=native={}", libdir);
            },
            Err(_) => {
                println!("cargo:warning=LLVM config not found, relying on system paths");
            }
        }
    }
    
    // Set up environment variables for inkwell
    if cfg!(windows) {
        // On Windows, we need to set LLVM_SYS_150_PREFIX to the LLVM installation directory
        // This is typically C:\Program Files\LLVM
        let llvm_dir = env::var("LLVM_SYS_150_PREFIX")
            .unwrap_or_else(|_| "C:\\Program Files\\LLVM".to_string());
        
        println!("cargo:warning=Using LLVM installation at {}", llvm_dir);
        println!("cargo:rustc-env=LLVM_SYS_150_PREFIX={}", llvm_dir);
    } else {
        // On Unix, we can use llvm-config to find the installation
        // The inkwell crate will handle this for us
    }
}