# tlstuc - A Modern C Language Runtime

tlstuc is a modern, user-friendly runtime and compiler for the C programming language. It aims to make C development simple, fast, and accessible again — without relying on traditional compilers like GCC or Clang.

## Features

- **Complete Compiler and Runtime**: Built from scratch using Rust, independent of GCC or Clang
- **Simple Developer Workflow**: Easy-to-use commands for initializing, compiling, and running C code
- **Cross-Platform Support**: Works on Windows, macOS, and Linux
- **Smart Updates**: Automatically checks for and applies updates

## Installation

### Windows

1. Download the latest release from [GitHub Releases](https://github.com/badrs3/tlstuc/releases)
2. Run the installer or extract the ZIP file
3. Add the installation directory to your PATH (the installer should do this automatically)

Alternatively, you can use PowerShell to install tlstuc:

```powershell
Invoke-WebRequest -Uri https://raw.githubusercontent.com/badrs3/tlstuc/main/install.ps1 -OutFile install.ps1
PowerShell -ExecutionPolicy Bypass -File .\install.ps1
```

### macOS and Linux

```bash
curl -sSL https://raw.githubusercontent.com/badrs3/tlstuc/main/install.sh | sudo bash
```

## Usage

### Initialize a New Project

```bash
tc init
```

This creates a starter `tc.c` file in the current directory.

### Compile and Run a C File

```bash
tc hello.c
```

This compiles and runs the specified C file.

### Check for Updates

```bash
tc update
```

This checks for updates to tlstuc and installs them if available.

## Example

```c
// hello.c
#include <stdio.h>

int main() {
    printf("Hello, tlstuc!\n");
    return 0;
}
```

```bash
tc hello.c
# Output: Hello, tlstuc!
```

## Building from Source

### Prerequisites

- Rust (1.70 or later)
- LLVM 15.0

### Build Steps

1. Clone the repository:

```bash
git clone https://github.com/badrs3/tlstuc.git
cd tlstuc
```

2. Build the project:

```bash
cargo build --release
```

3. Install the binary:

```bash
cargo install --path .
```

## Project Structure

- `src/main.rs`: Entry point for the `tc` command-line tool
- `src/compiler/`: C compiler implementation
- `src/runtime/`: Runtime support for C programs
- `src/update/`: Update mechanism
- `src/utils/`: Utility functions and helpers

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.