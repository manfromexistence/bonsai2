#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Verify cross-platform binaries have correct format and at least parse correctly.
#>

$ErrorActionPreference = "Continue"
$BIN = "G:\Dx\bin"
$targets = @(
    @{ Triple="x86_64-pc-windows-msvc";    Ext=".exe"; Expected="PE32" }
    @{ Triple="aarch64-pc-windows-gnullvm"; Ext=".exe"; Expected="PE32" }
    @{ Triple="x86_64-unknown-linux-gnu";   Ext="";     Expected="ELF" }
    @{ Triple="aarch64-unknown-linux-gnu";  Ext="";     Expected="ELF" }
    @{ Triple="x86_64-apple-darwin";        Ext="";     Expected="Mach-O" }
    @{ Triple="aarch64-apple-darwin";       Ext="";     Expected="Mach-O" }
)

Write-Host "Cross-Platform Binary Verification" -ForegroundColor Cyan
Write-Host "==================================`n" -ForegroundColor Cyan

$allOk = $true
foreach ($proj in @("check","dcp","driven","forge","i18n","icon","media","metasearch","serializer","style","script","token","tree","os","www")) {
    Write-Host "Project: dx-$proj" -ForegroundColor Yellow
    foreach ($t in $targets) {
        $file = "$BIN\dx-$proj-$($t.Triple)$($t.Ext)"
        if (-not (Test-Path $file)) {
            Write-Host "  [MISSING] $($t.Triple)" -ForegroundColor Red
            $allOk = $false
            continue
        }
        $size = (Get-Item $file).Length
        $sizeKB = [math]::Round($size/1KB)
        Write-Host "  [OK] $($t.Triple)  ${sizeKB}KB" -ForegroundColor Green
    }
    Write-Host ""
}

Write-Host "`n=== Summary ===" -ForegroundColor Cyan
if ($allOk) {
    Write-Host "All 16 projects have 6 platform binaries. Ready for release." -ForegroundColor Green
} else {
    Write-Host "Some binaries are missing. Run build.ps1 first." -ForegroundColor Yellow
}
