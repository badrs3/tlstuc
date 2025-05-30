# Contributing to tlstuc

Thank you for your interest in contributing to tlstuc! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Please be respectful and considerate of others when contributing to this project. We aim to foster an inclusive and welcoming community.

## Getting Started

### Prerequisites

- Rust (1.70 or later)
- LLVM 15.0
- Git

### Setting Up the Development Environment

1. Clone the repository:

```bash
git clone https://github.com/tlstuc/tlstuc.git
cd tlstuc
```

2. Build the project:

```bash
cargo build
```

3. Run the tests:

```bash
cargo test
```

## Development Workflow

1. Create a new branch for your changes:

```bash
git checkout -b feature/your-feature-name
```

2. Make your changes and commit them with clear, descriptive commit messages:

```bash
git commit -m "Add feature X to improve Y"
```

3. Push your branch to GitHub:

```bash
git push origin feature/your-feature-name
```

4. Create a pull request on GitHub.

## Pull Request Guidelines

- Provide a clear description of the changes in your pull request
- Include any relevant issue numbers in the pull request description
- Make sure all tests pass
- Add new tests for new functionality
- Follow the code style of the project

## Code Style

We follow the Rust standard code style. You can use `rustfmt` to format your code:

```bash
cargo fmt
```

And `clippy` to check for common issues:

```bash
cargo clippy
```

## Testing

All new features should include tests. We use the standard Rust testing framework. You can run the tests with:

```bash
cargo test
```

## Documentation

Please document your code using standard Rust documentation comments. You can generate the documentation with:

```bash
cargo doc
```

## Project Structure

- `src/main.rs`: Entry point for the `tc` command-line tool
- `src/compiler/`: C compiler implementation
- `src/runtime/`: Runtime support for C programs
- `src/update/`: Update mechanism
- `src/utils/`: Utility functions and helpers
- `tests/`: Integration tests
- `docs/`: Documentation

## Areas for Contribution

Here are some areas where contributions are particularly welcome:

- Improving the C parser to support more C language features
- Enhancing the code generator to produce more efficient code
- Adding support for more standard library functions
- Improving error messages and diagnostics
- Adding more tests and examples
- Improving documentation
- Fixing bugs

## Reporting Bugs

If you find a bug, please report it by creating an issue on GitHub. Please include:

- A clear description of the bug
- Steps to reproduce the bug
- Expected behavior
- Actual behavior
- Any relevant logs or error messages
- Your operating system and version
- Your Rust version
- Your LLVM version

## Feature Requests

If you have an idea for a new feature, please create an issue on GitHub. Please include:

- A clear description of the feature
- Why you think it would be valuable
- Any relevant examples or use cases

## Communication

- GitHub Issues: For bug reports, feature requests, and general discussion
- Pull Requests: For code contributions

## License

By contributing to this project, you agree that your contributions will be licensed under the project's MIT License.