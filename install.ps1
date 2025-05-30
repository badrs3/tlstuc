# tlstuc Installation Script for Windows

# Ensure we're running as administrator
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Warning "Please run this script as Administrator!"
    exit 1
}

# Configuration
$InstallDir = "$env:ProgramFiles\tlstuc"
$BinDir = "$InstallDir\bin"
$RepoUrl = "https://github.com/badrs3/tlstuc"
$ReleaseUrl = "https://github.com/badrs3/tlstuc/releases/latest"

# Create installation directory
Write-Host "Creating installation directory..."
if (!(Test-Path -Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

if (!(Test-Path -Path $BinDir)) {
    New-Item -ItemType Directory -Path $BinDir | Out-Null
}

# Check if tlstuc is already installed
$TcPath = "$BinDir\tc.exe"
if (Test-Path -Path $TcPath) {
    Write-Host "tlstuc is already installed. Checking for updates..."
    
    # Run the update command
    & $TcPath update
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "tlstuc is up to date."
        exit 0
    }
}

# Download the latest release
Write-Host "Downloading the latest release..."

# Create a temporary directory
$TempDir = [System.IO.Path]::GetTempPath() + [System.Guid]::NewGuid().ToString()
New-Item -ItemType Directory -Path $TempDir | Out-Null

# Download the latest release information
$LatestReleaseInfo = Invoke-RestMethod -Uri "https://api.github.com/repos/badrs3/tlstuc/releases/latest"
$WindowsAsset = $LatestReleaseInfo.assets | Where-Object { $_.name -like "*windows*.zip" } | Select-Object -First 1

if ($null -eq $WindowsAsset) {
    Write-Error "Could not find Windows release asset!"
    exit 1
}

# Download the asset
$ZipPath = "$TempDir\tlstuc.zip"
Invoke-WebRequest -Uri $WindowsAsset.browser_download_url -OutFile $ZipPath

# Extract the ZIP file
Write-Host "Extracting files..."
Expand-Archive -Path $ZipPath -DestinationPath $TempDir -Force

# Copy the files to the installation directory
Write-Host "Installing files..."
Copy-Item -Path "$TempDir\tc.exe" -Destination $BinDir -Force

# Add to PATH if not already there
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::Machine)
if ($CurrentPath -notlike "*$BinDir*") {
    Write-Host "Adding tlstuc to PATH..."
    [Environment]::SetEnvironmentVariable(
        "Path",
        $CurrentPath + ";$BinDir",
        [EnvironmentVariableTarget]::Machine
    )
}

# Clean up
Remove-Item -Path $TempDir -Recurse -Force

Write-Host "tlstuc has been installed successfully!"
Write-Host "You can now use the 'tc' command to compile and run C files."
Write-Host "Example: tc init"
Write-Host "Example: tc hello.c"