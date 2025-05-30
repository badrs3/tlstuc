# tlstuc Architecture

This document describes the architecture of the tlstuc runtime and compiler system.

## Overview

tlstuc is designed as a modern C language runtime and compiler built from scratch using Rust. It aims to provide a simple, user-friendly experience for C development without relying on traditional compilers like GCC or Clang.

The system consists of several key components:

1. **Command-Line Interface (CLI)**: The entry point for users to interact with the system
2. **Compiler**: Parses C code and generates executable binaries
3. **Runtime**: Provides the execution environment for compiled C programs
4. **Update System**: Handles checking for and applying updates
5. **Utilities**: Common functionality used throughout the system

## Component Details

### Command-Line Interface (CLI)

The CLI is implemented in `src/main.rs` and provides the following commands:

- `tc init`: Initialize a new C project with a template file
- `tc <file.c>`: Compile and run a C file
- `tc update`: Check for updates to tlstuc
- `tc version`: Show version information

The CLI is built using the `clap` crate for argument parsing and command handling.

### Compiler

The compiler is implemented in the `src/compiler/` directory and consists of several subcomponents:

- **Parser**: Parses C code into an Abstract Syntax Tree (AST)
- **Code Generator**: Generates LLVM IR from the AST
- **Optimizer**: Applies optimizations to the generated code
- **Linker**: Links the generated code with libraries to create an executable

The compiler uses the LLVM framework via the `inkwell` crate to generate machine code. This allows tlstuc to leverage LLVM's powerful optimization and code generation capabilities while providing a clean, Rust-based interface.

#### Parsing Process

1. **Lexical Analysis**: Converts the source code into a stream of tokens
2. **Syntax Analysis**: Builds an AST from the token stream
3. **Semantic Analysis**: Performs type checking and other semantic validations

#### Code Generation Process

1. **LLVM IR Generation**: Converts the AST into LLVM Intermediate Representation (IR)
2. **Optimization**: Applies LLVM optimization passes to the IR
3. **Machine Code Generation**: Converts the optimized IR into machine code
4. **Linking**: Links the machine code with libraries to create an executable

### Runtime

The runtime is implemented in the `src/runtime/` directory and provides the execution environment for compiled C programs. It includes:

- **Standard Library Support**: Implementations or bindings to standard C library functions
- **Memory Management**: Handles memory allocation and deallocation
- **Program Execution**: Manages the execution of compiled programs

### Update System

The update system is implemented in the `src/update/` directory and handles checking for and applying updates to tlstuc. It uses the `self_update` crate to interact with GitHub releases and download updates.

### Utilities

The utilities module is implemented in the `src/utils/` directory and provides common functionality used throughout the system, such as:

- **Configuration Management**: Handles loading and saving project configuration
- **File Operations**: Provides utilities for file handling
- **Error Handling**: Provides error formatting and reporting utilities

## Data Flow

1. User invokes the `tc` command with a C file
2. The CLI parses the command and calls the appropriate function
3. The compiler reads the C file and parses it into an AST
4. The compiler generates LLVM IR from the AST
5. The compiler optimizes the IR and generates machine code
6. The compiler links the machine code with libraries to create an executable
7. The runtime executes the compiled program

## Future Enhancements

- **Improved Error Reporting**: Provide more detailed and helpful error messages
- **Debugger Integration**: Add support for debugging C programs
- **Package Management**: Add support for managing dependencies
- **IDE Integration**: Provide integration with popular IDEs and text editors
- **Language Extensions**: Add modern features to the C language while maintaining compatibility

## Design Decisions

### Why Rust?

Rust was chosen as the implementation language for tlstuc because:

- **Memory Safety**: Rust's ownership model prevents memory-related bugs
- **Performance**: Rust provides performance comparable to C/C++
- **Modern Tooling**: Rust has excellent tooling and package management
- **LLVM Integration**: Rust has good support for integrating with LLVM

### Why LLVM?

LLVM was chosen as the backend for the compiler because:

- **Mature Codebase**: LLVM is a mature, well-tested codebase
- **Optimization**: LLVM provides powerful optimization capabilities
- **Cross-Platform**: LLVM supports multiple target architectures and platforms
- **Community**: LLVM has a large, active community

### Standalone vs. Wrapper

tlstuc is designed as a standalone compiler and runtime rather than a wrapper around existing compilers like GCC or Clang. This decision was made to:

- **Simplify the User Experience**: Provide a consistent, user-friendly experience
- **Control the Toolchain**: Have full control over the compilation process
- **Enable Innovation**: Allow for future language extensions and improvements
- **Reduce Dependencies**: Eliminate dependencies on external compilers