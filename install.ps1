# Git File Monitor (gfm) - Windows Installation Script
# This script downloads and installs the latest version of gfm

param(
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\gfm",
    [string]$Version = "latest"
)

$ErrorActionPreference = "Stop"

Write-Host "Installing Git File Monitor (gfm)..." -ForegroundColor Cyan

# Create installation directory if it doesn't exist
if (-not (Test-Path $InstallDir)) {
    Write-Host "Creating installation directory: $InstallDir" -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Determine the download URL
if ($Version -eq "latest") {
    $releaseUrl = "https://api.github.com/repos/KhaiStimpson/git-changes-monitor/releases/latest"
    Write-Host "Fetching latest release information..." -ForegroundColor Yellow
    
    try {
        $release = Invoke-RestMethod -Uri $releaseUrl -Headers @{
            "User-Agent" = "gfm-installer"
        }
        $downloadUrl = $release.assets | Where-Object { $_.name -eq "gfm-windows-x86_64.exe" } | Select-Object -ExpandProperty browser_download_url
        $Version = $release.tag_name
    } catch {
        Write-Host "Error fetching release information: $_" -ForegroundColor Red
        Write-Host "Using fallback version v0.1.0" -ForegroundColor Yellow
        $downloadUrl = "https://github.com/KhaiStimpson/git-changes-monitor/releases/download/v0.1.0/gfm-windows-x86_64.exe"
        $Version = "v0.1.0"
    }
} else {
    $downloadUrl = "https://github.com/KhaiStimpson/git-changes-monitor/releases/download/$Version/gfm-windows-x86_64.exe"
}

$destinationFile = Join-Path $InstallDir "gfm.exe"

# Download the binary
Write-Host "Downloading gfm $Version..." -ForegroundColor Yellow
Write-Host "From: $downloadUrl" -ForegroundColor Gray

try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $destinationFile -UseBasicParsing
    Write-Host "Downloaded successfully!" -ForegroundColor Green
} catch {
    Write-Host "Error downloading gfm: $_" -ForegroundColor Red
    exit 1
}

# Verify the file was downloaded
if (-not (Test-Path $destinationFile)) {
    Write-Host "Installation failed: File not found at $destinationFile" -ForegroundColor Red
    exit 1
}

# Add to PATH if not already present
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$InstallDir*") {
    Write-Host "Adding $InstallDir to user PATH..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$userPath;$InstallDir",
        "User"
    )
    Write-Host "PATH updated! You may need to restart your terminal for changes to take effect." -ForegroundColor Yellow
} else {
    Write-Host "Installation directory already in PATH." -ForegroundColor Green
}

Write-Host ""
Write-Host "Installation complete!" -ForegroundColor Green
Write-Host "Version: $Version" -ForegroundColor Cyan
Write-Host "Installed to: $destinationFile" -ForegroundColor Cyan
Write-Host ""
Write-Host "To use gfm, open a new terminal and run:" -ForegroundColor Yellow
Write-Host "  gfm" -ForegroundColor White
Write-Host ""
Write-Host "For more information, visit:" -ForegroundColor Yellow
Write-Host "  https://github.com/KhaiStimpson/git-changes-monitor" -ForegroundColor White
