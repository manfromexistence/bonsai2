#!/usr/bin/env pwsh
param(
    [Parameter(Mandatory)][string]$Project,
    [Parameter(Mandatory)][string]$Version,
    [string]$PrivateKeyHex = "",
    [string]$KeyId = "dx-release-root-2026-01",
    [string]$OutputDir = "G:\Dx\releases\signed"
)

$ErrorActionPreference = "Stop"
if (-not $PrivateKeyHex) { $PrivateKeyHex = [Environment]::GetEnvironmentVariable("DX_RELEASE_PRIVATE_KEY") }
if (-not $PrivateKeyHex) { $PrivateKeyHex = Read-Host "Enter Ed25519 private key (hex)" }

$targets = @("x86_64-pc-windows-msvc","aarch64-pc-windows-gnullvm","x86_64-unknown-linux-gnu","aarch64-unknown-linux-gnu","x86_64-apple-darwin","aarch64-apple-darwin")
$releaseId = "$Project-v$Version-$(Get-Date -Format 'yyyyMMdd')"
$artifacts = @()

foreach ($t in $targets) {
    $ext = if ($t -like "*-windows-*") { ".exe" } else { "" }
    $binName = "dx-$Project"
    $assetName = "$binName-$t$ext"
    $path = "G:\Dx\bin\$assetName"
    if (-not (Test-Path $path)) { Write-Host "  [WARN] Missing: $assetName" -ForegroundColor Yellow; continue }
    $file = Get-Item $path
    $sha = (Get-FileHash $path -Algorithm SHA256).Hash.ToLower()
    $url = "https://github.com/essence-dx/$Project/releases/download/v$Version/$assetName"
    $artifacts += @{ id="$binName-$t"; platform=$t; kind="binary"; install_path="bin/$binName$ext"; url=$url; size_bytes=$file.Length; sha256=$sha }
}

$manifest = @{ schema_version="dx.update.release_manifest.v1"; release_id=$releaseId; channel="stable"; version=$Version; published_at=(Get-Date -Format "o"); artifacts=$artifacts }
$manifestJson = $manifest | ConvertTo-Json -Depth 10 -Compress

# SHA-256 of payload
$sha256 = [Security.Cryptography.SHA256]::Create()
$payloadBytes = [Text.Encoding]::UTF8.GetBytes($manifestJson)
$payloadB64 = [Convert]::ToBase64String($payloadBytes).Replace('+','-').Replace('/','_').Replace('=','')
$payloadHash = $sha256.ComputeHash($payloadBytes)
$payloadHashHex = [BitConverter]::ToString($payloadHash).Replace('-','').ToLower()

# Build envelope (without actual Ed25519 signing — placeholder)
$envelope = @{
    schema_version="dx.update.release_envelope.v1"
    payload_encoding="base64url"
    payload_b64=$payloadB64
    payload_sha256=$payloadHashHex
    signature=@{ algorithm="ed25519"; key_id=$KeyId; value_base64="PLACEHOLDER_SIGNATURE_replace_with_real_signing" }
}

New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
$outPath = "$OutputDir\$Project-v$Version-envelope.json"
$envelope | ConvertTo-Json -Depth 10 | Set-Content $outPath -Encoding UTF8
Write-Host "[OK] Envelope written to $outPath" -ForegroundColor Green
Write-Host "Payload SHA-256: $payloadHashHex" -ForegroundColor Gray
