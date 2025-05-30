# Release Files

This directory contains the release files for the tlstuc project.

## Contents

- `tc.exe` - Windows executable
- `tc` - Linux/macOS executable
- `README.md` - This file

## How to Create a Release

1. Build the project using `cargo build --release`
2. Copy the built executable from `target/release/tc.exe` (Windows) or `target/release/tc` (Linux/macOS) to this directory
3. Create a ZIP file (Windows) or tarball (Linux/macOS) containing the executable
4. Upload the archive to GitHub Releases

## Release Naming Convention

Release files should follow this naming convention:

- Windows: `tlstuc-v{VERSION}-windows-{ARCH}.zip`
- Linux: `tlstuc-v{VERSION}-linux-{ARCH}.tar.gz`
- macOS: `tlstuc-v{VERSION}-macos-{ARCH}.tar.gz`

Where:
- `{VERSION}` is the version number (e.g., `0.1.0`)
- `{ARCH}` is the architecture (e.g., `amd64`, `arm64`)