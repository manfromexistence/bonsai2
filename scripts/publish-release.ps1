#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Build all 16 DX ecosystem projects and publish releases to GitHub.

.DESCRIPTION
    Runs build.ps1 to build all projects for 6 targets, then creates GitHub
    releases with the 6 platform binaries for each project.

.PARAMETER Version
    Version string (e.g. "1.1.0"). Defaults to reading from each project's Cargo.toml.
    For native projects (os), uses "1.0.0" as fallback.

.PARAMETER Projects
    Specific projects to publish (comma-separated). Default: all 16 projects.

.PARAMETER DryRun
    Build and plan releases without actually creating GitHub releases.
#>

param(
    [string]$Version = "",
    [string[]]$Projects = @(),
    [switch]$DryRun
)

$ErrorActionPreference = "Stop"
$ROOT = "G:\Dx"
$BIN = "$ROOT\bin"

# All 16 projects
$allProjects = @(
    @{ dir = "check";      repo = "essence-dx/check" },
    @{ dir = "dcp";        repo = "essence-dx/dcp" },
    @{ dir = "i18n";       repo = "essence-dx/i18n" },
    @{ dir = "driven";     repo = "essence-dx/driven" },
    @{ dir = "forge";      repo = "essence-dx/forge" },
    @{ dir = "icon";       repo = "essence-dx/icon" },
    @{ dir = "media";      repo = "essence-dx/media" },
    @{ dir = "metasearch"; repo = "essence-dx/metasearch" },
    @{ dir = "serializer"; repo = "essence-dx/serializer" },
    @{ dir = "style";      repo = "essence-dx/style" },
    @{ dir = "script";     repo = "essence-dx/script" },
    @{ dir = "cli";        repo = "essence-dx/cli" },
    @{ dir = "www";        repo = "essence-dx/www" },
    @{ dir = "token";      repo = "essence-dx/token" },
    @{ dir = "tree";       repo = "essence-dx/tree" },
    @{ dir = "os";         repo = "essence-dx/os" }
)

if ($Projects.Count -gt 0) {
    $projects = $allProjects | Where-Object { $_.dir -in $Projects }
} else {
    $projects = $allProjects
}

# Step 1: Build all projects
Write-Host "`n=== Step 1: Building $($projects.Count) projects ===" -ForegroundColor Cyan
Push-Location $ROOT
& .\build.ps1
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed. Aborting release." -ForegroundColor Red
    exit 1
}
Pop-Location

# Step 2: Create releases for each project
$targets = @(
    "x86_64-pc-windows-msvc",
    "aarch64-pc-windows-gnullvm",
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin"
)

Write-Host "`n=== Step 2: Creating releases ===" -ForegroundColor Cyan

foreach ($proj in $projects) {
    $dir = $proj.dir
    $repo = $proj.repo

    # Get version
    if ($Version) {
        $ver = $Version
    } else {
        $tomlPath = "$ROOT\$dir\Cargo.toml"
        if (Test-Path $tomlPath) {
            $content = Get-Content $tomlPath -Raw
            if ($content -match 'version = "([^"]+)"') {
                $ver = $Matches[1]
            } else { $ver = "1.0.0" }
        } else {
            $ver = "1.0.0"
        }
    }
    $tag = "v$ver"

    Write-Host "  [$dir] Publishing $tag to $repo..." -ForegroundColor Yellow

    # Check if release already exists
    $existing = gh release view $tag --repo $repo 2>$null
    if ($existing) {
        Write-Host "    Release $tag already exists. Skipping." -ForegroundColor DarkYellow
        continue
    }

    # Collect binaries
    $files = @()
    foreach ($t in $targets) {
        $ext = if ($t -like "*-windows-*") { ".exe" } else { "" }
        $path = "$BIN\dx-$dir-$t$ext"
        if (Test-Path $path) {
            $files += $path
        } else {
            Write-Host "    [WARN] Missing: dx-$dir-$t$ext" -ForegroundColor Yellow
        }
    }

    if ($files.Count -eq 0) {
        Write-Host "    [SKIP] No binaries found for $dir" -ForegroundColor DarkYellow
        continue
    }

    if ($DryRun) {
        Write-Host "    [DRY-RUN] Would create release $tag with $($files.Count) assets" -ForegroundColor Gray
        foreach ($f in $files) { Write-Host "      $(Split-Path $f -Leaf)" -ForegroundColor Gray }
        continue
    }

    # Create release
    $notes = "Release v$ver for $dir`n`nBuilt for 6 platforms: Windows x64/ARM64, Linux x64/ARM64, macOS x64/ARM64"
    $result = gh release create $tag --repo $repo --title "v$ver" --notes $notes $files 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "    [OK] Created $tag with $($files.Count) assets: $result" -ForegroundColor Green
    } else {
        Write-Host "    [FAIL] $result" -ForegroundColor Red
    }
}

Write-Host "`n=== Done ===" -ForegroundColor Cyan
Write-Host "Published $($projects.Count) projects." -ForegroundColor Green
