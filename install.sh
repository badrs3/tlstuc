#!/bin/bash
# tlstuc Installation Script for Unix-based systems (Linux and macOS)

set -e

# Configuration
INSTALL_DIR="/usr/local/tlstuc"
BIN_DIR="/usr/local/bin"
REPO_URL="https://github.com/badrs3/tlstuc"
RELEASE_URL="https://github.com/badrs3/tlstuc/releases/latest"

# Check if running as root
if [ "$(id -u)" -ne 0 ]; then
    echo "Please run this script as root or with sudo!"
    exit 1
fi

# Create installation directory
echo "Creating installation directory..."
mkdir -p "$INSTALL_DIR"

# Check if tlstuc is already installed
if [ -f "$BIN_DIR/tc" ]; then
    echo "tlstuc is already installed. Checking for updates..."
    
    # Run the update command
    "$BIN_DIR/tc" update
    
    if [ $? -eq 0 ]; then
        echo "tlstuc is up to date."
        exit 0
    fi
fi

# Detect OS
OS="unknown"
if [ "$(uname)" == "Darwin" ]; then
    OS="macos"
elif [ "$(uname)" == "Linux" ]; then
    OS="linux"
fi

if [ "$OS" == "unknown" ]; then
    echo "Unsupported operating system!"
    exit 1
fi

# Detect architecture
ARCH="$(uname -m)"
if [ "$ARCH" == "x86_64" ]; then
    ARCH="amd64"
elif [ "$ARCH" == "aarch64" ] || [ "$ARCH" == "arm64" ]; then
    ARCH="arm64"
fi

# Download the latest release
echo "Downloading the latest release for $OS-$ARCH..."

# Create a temporary directory
TEMP_DIR="$(mktemp -d)"
trap "rm -rf $TEMP_DIR" EXIT

# Get the latest release URL
RELEASE_INFO="$(curl -s https://api.github.com/repos/badrs3/tlstuc/releases/latest)"
ASSET_URL="$(echo "$RELEASE_INFO" | grep -o "https://github.com/badrs3/tlstuc/releases/download/[^\"]*$OS-$ARCH[^\"]*.tar.gz" | head -n 1)"

if [ -z "$ASSET_URL" ]; then
    echo "Could not find release asset for $OS-$ARCH!"
    exit 1
fi

# Download the asset
TAR_PATH="$TEMP_DIR/tlstuc.tar.gz"
curl -L -o "$TAR_PATH" "$ASSET_URL"

# Extract the tarball
echo "Extracting files..."
tar -xzf "$TAR_PATH" -C "$TEMP_DIR"

# Copy the files to the installation directory
echo "Installing files..."
cp "$TEMP_DIR/tc" "$INSTALL_DIR/"

# Create a symbolic link
ln -sf "$INSTALL_DIR/tc" "$BIN_DIR/tc"

# Make the binary executable
chmod +x "$INSTALL_DIR/tc"
chmod +x "$BIN_DIR/tc"

echo "tlstuc has been installed successfully!"
echo "You can now use the 'tc' command to compile and run C files."
echo "Example: tc init"
echo "Example: tc hello.c"