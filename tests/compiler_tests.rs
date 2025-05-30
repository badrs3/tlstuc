use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_compile_and_run_hello_world() {
    // Create a temporary directory for the test
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let file_path = temp_dir.path().join("hello.c");
    
    // Write a simple C program to the file
    let program = r#"#include <stdio.h>

int main() {
    printf("Hello, World!\n");
    return 0;
}
"#;
    
    std::fs::write(&file_path, program).expect("Failed to write test file");
    
    // Build the tc binary if running as a test
    let tc_path = if cfg!(debug_assertions) {
        // Running as a test, use the debug build
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("target/debug/tc");
        if cfg!(windows) {
            path.set_extension("exe");
        }
        path
    } else {
        // Running as a binary, use the current executable
        std::env::current_exe().expect("Failed to get current executable path")
    };
    
    // Run the tc command on the file
    let output = Command::new(&tc_path)
        .arg(&file_path)
        .output()
        .expect("Failed to execute tc command");
    
    // Check that the command succeeded
    assert!(output.status.success(), "tc command failed: {:?}", output);
    
    // Check that the output contains "Hello, World!"
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello, World!"), "Output does not contain 'Hello, World!': {}", stdout);
}

#[test]
fn test_compile_and_run_with_args() {
    // Create a temporary directory for the test
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let file_path = temp_dir.path().join("args.c");
    
    // Write a C program that prints its arguments
    let program = r#"#include <stdio.h>

int main(int argc, char** argv) {
    printf("Number of arguments: %d\n", argc);
    for (int i = 0; i < argc; i++) {
        printf("Argument %d: %s\n", i, argv[i]);
    }
    return 0;
}
"#;
    
    std::fs::write(&file_path, program).expect("Failed to write test file");
    
    // Build the tc binary if running as a test
    let tc_path = if cfg!(debug_assertions) {
        // Running as a test, use the debug build
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("target/debug/tc");
        if cfg!(windows) {
            path.set_extension("exe");
        }
        path
    } else {
        // Running as a binary, use the current executable
        std::env::current_exe().expect("Failed to get current executable path")
    };
    
    // Compile the program
    let output = Command::new(&tc_path)
        .arg(&file_path)
        .output()
        .expect("Failed to execute tc command");
    
    // Check that the command succeeded
    assert!(output.status.success(), "tc command failed: {:?}", output);
    
    // Check that the output contains "Number of arguments"
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Number of arguments"), "Output does not contain 'Number of arguments': {}", stdout);
}

#[test]
fn test_init_command() {
    // Create a temporary directory for the test
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    
    // Build the tc binary if running as a test
    let tc_path = if cfg!(debug_assertions) {
        // Running as a test, use the debug build
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("target/debug/tc");
        if cfg!(windows) {
            path.set_extension("exe");
        }
        path
    } else {
        // Running as a binary, use the current executable
        std::env::current_exe().expect("Failed to get current executable path")
    };
    
    // Run the tc init command
    let output = Command::new(&tc_path)
        .arg("init")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute tc init command");
    
    // Check that the command succeeded
    assert!(output.status.success(), "tc init command failed: {:?}", output);
    
    // Check that the tc.c file was created
    let tc_file_path = temp_dir.path().join("tc.c");
    assert!(tc_file_path.exists(), "tc.c file was not created");
    
    // Check that the file contains the expected content
    let content = std::fs::read_to_string(&tc_file_path).expect("Failed to read tc.c file");
    assert!(content.contains("Hello from tlstuc"), "tc.c file does not contain expected content");
}