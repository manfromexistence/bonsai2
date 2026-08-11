$ErrorActionPreference = "Continue"

$ROOT = "G:\Dx"
$BIN  = "$ROOT\bin"
$LOGS = "$ROOT\build-logs"

New-Item -ItemType Directory -Force -Path $BIN  | Out-Null
New-Item -ItemType Directory -Force -Path $LOGS | Out-Null

$projects = @(
    @{ dir = "check";      cargo_bin = "dx-check";               final_bin = "dx-check";               pkg = "";             jobs = 12 ; skip_arm64_msvc = $true }
    @{ dir = "dcp";        cargo_bin = "dcp";                    final_bin = "dx-dcp";                 pkg = "";             jobs = 12 ; skip_arm64_msvc = $true }
    @{ dir = "i18n";       cargo_bin = "dx-i18n";                final_bin = "dx-i18n";                pkg = "";             jobs = 12 ; bindgen_fix = $true ; whisper_fix = $true ; skip_arm64_msvc = $true }
    #@{ dir = "providers";  cargo_bin = "dx-providers";           final_bin = "dx-providers";           pkg = "";             jobs = 12 }
    @{ dir = "driven";     cargo_bin = "driven";                 final_bin = "dx-driven";              pkg = "";             jobs = 12 ; skip_arm64_msvc = $true }
    #@{ dir = "flow";       cargo_bin = "dx-flow";                final_bin = "dx-flow";                pkg = "";             jobs = 8  ; windows_only = $true }
    @{ dir = "forge";      cargo_bin = "dx-forge";               final_bin = "dx-forge";               pkg = "";             jobs = 12 ; cflags_fix = $true ; skip_arm64_msvc = $true }
    @{ dir = "icon";       cargo_bin = "icon";                   final_bin = "dx-icon";                pkg = "";             jobs = 12 ; skip_arm64_msvc = $true }
    @{ dir = "media";      cargo_bin = "media";                  final_bin = "dx-media";               pkg = "";             jobs = 12 ; skip_arm64_msvc = $true }
    @{ dir = "metasearch"; cargo_bin = "metasearch";             final_bin = "dx-metasearch";          pkg = "";             jobs = 12 ; cflags_fix = $true ; skip_arm64_msvc = $true }
    @{ dir = "serializer"; cargo_bin = "dx-serializer";           final_bin = "dx-serializer";           pkg = "";             jobs = 12 ; skip_arm64_msvc = $true }
    @{ dir = "style";      cargo_bin = "dx-style";               final_bin = "dx-style";               pkg = "";             jobs = 12 ; skip_arm64_msvc = $true }
    @{ dir = "script";     cargo_bin = "dx-script";            final_bin = "dx-script";            pkg = "-p dx-script";  jobs = 12 ; skip_arm64_msvc = $true }
    #@{ dir = "build";      cargo_bin = "dx-build";               final_bin = "dx-build";               pkg = "-p rolldown";  jobs = 6  }
    @{ dir = "cli";        cargo_bin = "dx-cli";                 final_bin = "dx-cli";                 pkg = "-p dx-cli";    jobs = 12 ; skip_arm64_msvc = $true }
    #@{ dir = "native";     cargo_bin = "native";                 final_bin = "dx-native";              pkg = "-p tauri-cli"; jobs = 12 }
    #@{ dir = "py";         cargo_bin = "dx-py-package-manager";  final_bin = "dx-py-package-manager";  pkg = "-p dx-py-package-manager";  jobs = 12 ; subdir = "package-manager"; unix_features = "uv-distribution/static" }
    @{ dir = "www";        cargo_bin = "dx-www";                 final_bin = "dx-www";                 pkg = "-p dx-www";    jobs = 12 ; skip_gnullvm = $true ; skip_arm64_msvc = $true }
    #@{ dir = "agent";      cargo_bin = "dx-agent";               final_bin = "dx-agent";               pkg = "";             jobs = 12 ; bindgen_fix = $true ; windows_only = $true ; skip_gnullvm = $true }
    #@{ dir = "js";         cargo_bin = "dx-js";                  final_bin = "dx-js";                  pkg = "-p bun_bin";   jobs = 4  ; windows_only = $true }
    @{ dir = "token";      cargo_bin = "dx-token";              final_bin = "dx-token";              pkg = "-p dx-token";  jobs = 12 }
    @{ dir = "tree";       cargo_bin = "dx-tree";               final_bin = "dx-tree";               pkg = "-p rust_tree"; jobs = 12 ; skip_arm64_msvc = $true }
    @{ dir = "os";         cargo_bin = "dx-os";                 final_bin = "dx-os";                 pkg = "";             jobs = 12 ; native = $true }
    @{ dir = "hexxed\terminal\dx-video-player"; final_bin = "dx-video-player";   jobs = 2  ; meson = $true; meson_bin = "dx-video-player.exe" }
)

$targets = @(
    @{ triple = "x86_64-pc-windows-msvc";         ext = ".exe" }
    @{ triple = "aarch64-pc-windows-msvc";        ext = ".exe" }
    @{ triple = "x86_64-pc-windows-gnullvm";       ext = ".exe" }
    @{ triple = "aarch64-pc-windows-gnullvm";      ext = ".exe" }
    @{ triple = "x86_64-unknown-linux-gnu";        ext = ""     }
    @{ triple = "x86_64-unknown-linux-musl";        ext = ""     }
    @{ triple = "aarch64-unknown-linux-gnu";       ext = ""     }
    @{ triple = "armv7-unknown-linux-gnueabihf";   ext = ""     }
    @{ triple = "aarch64-linux-android";            ext = ""     }
    @{ triple = "x86_64-apple-darwin";             ext = ""     }
    @{ triple = "aarch64-apple-darwin";            ext = ""     }
)

# macOS SDK path for framework linking (osxcross)
$MACOS_SDK = "G:\osxcross\target\SDK\MacOSX11.3.sdk"

# Prebuilt FFmpeg pkg-config dirs per target (BtbN shared builds; relocatable .pc files)
$ffmpegPkgConfig = @{
    "x86_64-pc-windows-msvc"    = "G:\ffmpeg-prebuilt\win64\ffmpeg-master-latest-win64-gpl-shared\lib\pkgconfig"
    "aarch64-pc-windows-msvc"   = "G:\ffmpeg-prebuilt\ffmpeg-master-latest-winarm64-gpl-shared\lib\pkgconfig"
    "x86_64-unknown-linux-gnu"  = "G:\ffmpeg-prebuilt\ffmpeg-master-latest-linux64-gpl-shared\lib\pkgconfig"
    "aarch64-unknown-linux-gnu" = "G:\ffmpeg-prebuilt\ffmpeg-master-latest-linuxarm64-gpl-shared\lib\pkgconfig"
}

# MSVC + Windows SDK include/lib paths for clang-cl cross builds (must be in shell env, not just cross file)
$MSVC_ROOT = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207"
$SDK_ROOT  = "C:\Program Files (x86)\Windows Kits\10"
$SDK_VER   = "10.0.26100.0"
$msvcInclude = "$MSVC_ROOT\include;$SDK_ROOT\Include\$SDK_VER\ucrt;$SDK_ROOT\Include\$SDK_VER\um;$SDK_ROOT\Include\$SDK_VER\shared"
$msvcLibX64   = "$MSVC_ROOT\lib\x64;$MSVC_ROOT\lib\x64\store;$SDK_ROOT\Lib\$SDK_VER\ucrt\x64;$SDK_ROOT\Lib\$SDK_VER\um\x64"
$msvcLibArm64 = "$MSVC_ROOT\lib\arm64;$MSVC_ROOT\lib\arm64\store;$SDK_ROOT\Lib\$SDK_VER\ucrt\arm64;$SDK_ROOT\Lib\$SDK_VER\um\arm64"

$totalProjects = $projects.Count
$totalTargets  = $targets.Count
$allOk         = $true
$builtCount    = 0

# Ensure MSVC link.exe is found before Git's Unix link.exe (which doesn't understand MSVC flags)
# Git's link.exe was renamed to link.exe.bak to prevent shadowing.
$cliTools = "G:\Dx\cli\tools\bin"
$msvcBin  = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64"
$msvcArm  = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207\bin\Hostx64\arm64"
$cargoBin = "D:\DevCaches\cargo\bin"
$zigBin   = "G:\Dev\Tools\Scoop\shims"
$llvmBin  = "C:\Program Files\LLVM\bin"
$cmakeBin = "C:\Program Files\CMake\bin"
$gitBin   = "C:\Program Files\Git\cmd"
$mingwBin = "G:\Dev\Tools\Scoop\apps\mingw\current\bin"
$ndkBin   = "G:\android-ndk-r27\toolchains\llvm\prebuilt\windows-x86_64\bin"
$mesonBin = "C:\Users\Computer\AppData\Roaming\Python\Python312\Scripts"
$nasmBin  = "C:\Program Files\NASM"
$env:PATH = "$cliTools;$msvcBin;$msvcArm;$cargoBin;$zigBin;$mingwBin;$llvmBin;$cmakeBin;$gitBin;$ndkBin;$mesonBin;$nasmBin;" + ($env:PATH -split ';' | Where-Object {
    $_ -notmatch 'Git\\bin|Git\\usr|Git\\mingw' -and $_ -ne $cliTools -and $_ -ne $msvcBin -and $_ -ne $msvcArm -and $_ -ne $cargoBin -and $_ -ne $zigBin -and $_ -ne $mingwBin -and $_ -ne $llvmBin -and $_ -ne $cmakeBin -and $_ -ne $gitBin -and $_ -ne $ndkBin
}) -join ';'

# Release profile optimizations for all Rust builds (applies to every cargo build --release)
$env:CARGO_PROFILE_RELEASE_LTO = "true"           # Link-time optimization: smaller + faster binaries
$env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "1"    # Max inlining across all codegen units
$env:CARGO_PROFILE_RELEASE_STRIP = "symbols"      # Strip debug symbols from release builds
$env:CARGO_PROFILE_RELEASE_PANIC = "abort"        # Abort on panic (no unwinding tables, smaller binary)
$env:CARGO_PROFILE_RELEASE_DEBUG = "0"            # No debug info in release builds

# Redirect zig's caches off C: (C: filled up during the darwin source builds)
$env:ZIG_GLOBAL_CACHE_DIR = "G:\Dev\zig-cache"
$env:ZIG_LOCAL_CACHE_DIR  = "G:\Dev\zig-cache\local"

# Install any missing Rust targets automatically
$installedTargets = rustup target list --installed
foreach ($tgt in $targets) {
    if ($installedTargets -notcontains $tgt.triple) {
        Write-Host "  [INFO] Installing missing target: $($tgt.triple)" -ForegroundColor Cyan
        rustup target add $tgt.triple 2>&1 | Out-Null
    }
}

foreach ($proj in $projects) {
    $dir     = $proj.dir
    $cBin    = $proj.cargo_bin
    $fBin    = $proj.final_bin
    $pkg     = $proj.pkg
    $jobs    = $proj.jobs
    $subdir  = if ($proj.ContainsKey("subdir")) { $proj.subdir } else { $null }

    $projPath = Join-Path $ROOT $dir
    if (-not (Test-Path $projPath)) {
        Write-Host "[SKIP] $dir - directory not found" -ForegroundColor DarkYellow
        continue
    }

    $workDir = $projPath
    if ($subdir) {
        $workDir = Join-Path $projPath $subdir
        if (-not (Test-Path $workDir)) {
            Write-Host "[SKIP] $dir/$subdir - not found" -ForegroundColor DarkYellow
            continue
        }
    }

    $hasUnixFeatures = $proj.ContainsKey("unix_features") -and -not [string]::IsNullOrEmpty($proj.unix_features)
    $windowsOnly = $proj.ContainsKey("windows_only") -and $proj.windows_only

    # Skip if all target binaries already exist in bin/
    $allExist = $true
    foreach ($tgt in $targets) {
        $checkName = "$($proj.final_bin)-$($tgt.triple)$($tgt.ext)"
        if (-not (Test-Path (Join-Path $BIN $checkName))) { $allExist = $false; break }
    }
    if ($allExist) {
        Write-Host "[SKIP] $dir - all $($targets.Count) targets already in bin\" -ForegroundColor DarkYellow
        continue
    }

    Write-Host "`n================================================================" -ForegroundColor Cyan
    $sizeLabel = if ($jobs -ge 12) { "small" } elseif ($jobs -ge 8) { "medium" } elseif ($jobs -ge 6) { "large" } else { "very large" }
    $featLabel = if ($hasUnixFeatures) { " (unix: --no-default-features --features $($proj.unix_features))" } else { "" }
    Write-Host "  [$sizeLabel | $jobs jobs] $dir  ->  $fBin$featLabel" -ForegroundColor Cyan
    Write-Host "================================================================  " -ForegroundColor Cyan

    $projOk = $true

    foreach ($tgt in $targets) {
        $triple    = $tgt.triple
        $ext       = $tgt.ext
        $isWinTgt = ($triple -like "*-pc-windows-*")

        $isMacTgt = ($triple -like "*-apple-darwin")
        if ($windowsOnly -and -not $isWinTgt) {
            Write-Host "    [SKIP] $dir / $triple (windows-only project)" -ForegroundColor DarkYellow
            continue
        }

        $isNative = $proj.ContainsKey("native") -and $proj.native
        if ($isNative) {
            $dstName = "$fBin-$triple$ext"
            $dstPath = Join-Path $BIN $dstName
            # Convert Rust target triple to zig-compatible format
            $zigTarget = $triple -replace '-unknown-', '-'
            if ($triple -like "armv7-*") { $zigTarget = $triple -replace 'armv7-unknown-linux-gnueabihf', 'arm-linux-gnueabihf' }
            if ($triple -like "*-apple-darwin") { $zigTarget = $triple -replace '-apple-darwin', '-macos-none' }
            $logFile = Join-Path $LOGS "$dir-$( $triple.Replace('-','_') ).log"
            $nativeOk = $true

            $gitCommit = $(& "C:\Program Files\Git\cmd\git.exe" rev-parse --short HEAD 2>$null)
            if (-not $gitCommit) { $gitCommit = "unknown" }

            if ($isWinTgt) {
                Write-Host "    [$triple] zig c++ (windows)..."
                $winDir = Join-Path $workDir "windows"
                $zigDir = Join-Path $winDir "build_zig"
                New-Item -ItemType Directory -Force -Path $zigDir | Out-Null

                Push-Location $winDir
@"
// SPDX-License-Identifier: Apache-2.0
#pragma once
#include <string_view>
constexpr std::string_view GIT_COMMIT        = "$gitCommit";
constexpr std::string_view COMPILER          = "zig c++";
constexpr std::string_view COMPILER_VERSION  = "0.16.0";
constexpr std::string_view CONFIGURE_COMMAND = "build.ps1";
"@ | Set-Content -Path (Join-Path $zigDir "config.h") -Encoding ASCII

                $srcFiles = @(
                    "src/dx.cpp","src/dx_collect.cpp","src/dx_config.cpp",
                    "src/dx_draw.cpp","src/dx_input.cpp","src/dx_menu.cpp",
                    "src/dx_theme.cpp","src/dx_tools.cpp","src/comsupp_stub.cpp"
                )
                $cflags = @(
                    "-std=c++23","-O2","-DFMT_HEADER_ONLY","-D_WIN32_WINNT=0x0A00",
                    "-DUNICODE","-D_UNICODE",
                    "-Wall","-Wextra","-Wpedantic",
                    "-fexceptions","-fstack-protector",
                    "-isystem","include","-I","src","-I",$zigDir
                )
                $ldflags = @("-lpsapi","-loleaut32","-lole32","-lpdh","-lpowrprof","-liphlpapi","-lws2_32","-ladvapi32","-lwbemuuid","-lntdll")
                $zigWinTarget = if ($triple -like "aarch64*") { "aarch64-windows-gnu" } else { "x86_64-windows-gnu" }
                $objs = @()
                foreach ($sf in $srcFiles) {
                    $obj = Join-Path $zigDir "$($sf.Replace('\','_').Replace('/','_').Replace('.cpp','.o'))"
                    $objs += $obj
                    & zig c++ -target $zigWinTarget $cflags -c $sf -o $obj *>> $logFile
                    if ($LASTEXITCODE -ne 0) { $nativeOk = $false; break }
                }
                if ($nativeOk) {
                    $srcPath = Join-Path $zigDir $cBin
                    & zig c++ -target $zigWinTarget $cflags $ldflags $objs -o $srcPath *>> $logFile
                    if ($LASTEXITCODE -ne 0) { $nativeOk = $false }
                }
                Pop-Location

            } else {
                if ($isMacTgt) {
                    Write-Host "    [$triple] zig c++ (macos)..."
                    $posixDir = Join-Path $workDir "posix"
                $zigDir = Join-Path $posixDir "build_zig"
                New-Item -ItemType Directory -Force -Path $zigDir | Out-Null
                $MACOS_SDK = "G:\osxcross\target\SDK\MacOSX11.3.sdk"
                $cf = "$MACOS_SDK/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation_patched.tbd"
                $io = "$MACOS_SDK/System/Library/Frameworks/IOKit.framework/IOKit.tbd"
                $objc = "$MACOS_SDK/usr/lib/libobjc.A.tbd"
                # Patch CoreFoundation.tbd if not already patched
                if (-not (Test-Path $cf)) {
                    $content = Get-Content "$MACOS_SDK/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation.tbd"
                    $before = $content | Select-String "reexported-libraries" -SimpleMatch
                    if ($before) {
                        $idx = $before.LineNumber - 1; $lineCount = 0
                        for ($i = $idx; $i -lt $content.Count; $i++) { if ($content[$i] -match "^\S" -and $i -ne $idx) { break }; $lineCount++ }
                        ($content[0..($idx-1)] + $content[($idx+$lineCount)..($content.Count-1)]) | Set-Content $cf
                    }
                }
                Push-Location $posixDir
@"
// SPDX-License-Identifier: Apache-2.0
#pragma once
#include <string_view>
constexpr std::string_view GIT_COMMIT        = "$gitCommit";
constexpr std::string_view COMPILER          = "zig c++";
constexpr std::string_view COMPILER_VERSION  = "0.16.0";
constexpr std::string_view CONFIGURE_COMMAND = "build.ps1";
"@ | Set-Content -Path (Join-Path $zigDir "config.h") -Encoding ASCII

                $srcFiles = @(
                    "src/main.cpp","src/dx.cpp","src/dx_cli.cpp","src/dx_config.cpp",
                    "src/dx_draw.cpp","src/dx_input.cpp","src/dx_log.cpp","src/dx_menu.cpp",
                    "src/dx_shared.cpp","src/dx_theme.cpp","src/dx_tools.cpp",
                    "src/osx/dx_collect.cpp","src/osx/sensors.cpp","src/osx/smc.cpp"
                )
                $cflags = @(
                    "-std=c++23","-O2","-DFMT_HEADER_ONLY","-D_FILE_OFFSET_BITS=64",
                    "-D_GNU_SOURCE",
                    "-Wall","-Wextra","-Wpedantic","-Wno-elaborated-enum-base",
                    "-fexceptions","-fstack-protector",
                    "-isystem","include","-I","src","-I",$zigDir,
                    "-isysroot",$MACOS_SDK,"-F","$MACOS_SDK/System/Library/Frameworks",
                    "-idirafter","$MACOS_SDK/usr/include",
                    "-DkIOMainPortDefault=kIOMasterPortDefault"
                )
                $zigMacTarget = if ($triple -like "aarch64*") { "aarch64-macos-none" } else { "x86_64-macos-none" }
                $objs = @()
                foreach ($sf in $srcFiles) {
                    $obj = Join-Path $zigDir "$($sf.Replace('\','_').Replace('/','_').Replace('.cpp','.o'))"
                    $objs += $obj
                    & zig c++ -target $zigMacTarget $cflags -c $sf -o $obj *>> $logFile
                    if ($LASTEXITCODE -ne 0) { $nativeOk = $false; break }
                }
                if ($nativeOk) {
                    $srcPath = Join-Path $zigDir $cBin
                    & zig c++ -target $zigMacTarget $cflags $cf $io $objc $objs -o $srcPath *>> $logFile
                    if ($LASTEXITCODE -ne 0) { $nativeOk = $false }
                }
                Pop-Location

            } else {
                Write-Host "    [$triple] zig c++ (posix)..."
                $posixDir = Join-Path $workDir "posix"
                $zigDir = Join-Path $posixDir "build_zig"
                New-Item -ItemType Directory -Force -Path $zigDir | Out-Null
                Push-Location $posixDir
@"
// SPDX-License-Identifier: Apache-2.0
#pragma once
#include <string_view>
constexpr std::string_view GIT_COMMIT        = "$gitCommit";
constexpr std::string_view COMPILER          = "zig c++";
constexpr std::string_view COMPILER_VERSION  = "0.16.0";
constexpr std::string_view CONFIGURE_COMMAND = "build.ps1";
"@ | Set-Content -Path (Join-Path $zigDir "config.h") -Encoding ASCII

                $srcFiles = @(
                    "src/main.cpp","src/dx.cpp","src/dx_cli.cpp","src/dx_config.cpp",
                    "src/dx_draw.cpp","src/dx_input.cpp","src/dx_log.cpp","src/dx_menu.cpp",
                    "src/dx_shared.cpp","src/dx_theme.cpp","src/dx_tools.cpp",
                    "src/linux/dx_collect.cpp"
                )
                $cflags = @(
                    "-std=c++23","-O2","-DFMT_HEADER_ONLY","-D_FILE_OFFSET_BITS=64",
                    "-D_GNU_SOURCE","-D_POSIX_C_SOURCE=200809L",
                    "-Wall","-Wextra","-Wpedantic",
                    "-fexceptions","-fstack-protector",
                    "-isystem","include","-I","src","-I",$zigDir
                )
                # Android: use NDK's clang++ directly (zig 0.16.0 Android C++ support has mbstate_t issues)
                if ($triple -like "*-android") {
                    $ndkDir = "G:\android-ndk-r27\toolchains\llvm\prebuilt\windows-x86_64"
                    $ndkCxx = "$ndkDir\bin\aarch64-linux-android35-clang++.cmd"
                    $ndkCc = "$ndkDir\bin\aarch64-linux-android35-clang.cmd"
                    $ldflags = @("-lc++", "-lc++abi")
                } else {
                    $ldflags = @("-lpthread")
                }

                $objs = @()
                foreach ($sf in $srcFiles) {
                    $obj = Join-Path $zigDir "$($sf.Replace('\','_').Replace('/','_').Replace('.cpp','.o'))"
                    $objs += $obj
                    if ($triple -like "*-android") {
                        & "$ndkCxx" -std=c++23 -O2 -DFMT_HEADER_ONLY -D_FILE_OFFSET_BITS=64 -D_GNU_SOURCE -D_POSIX_C_SOURCE=200809L -Wall -Wextra -Wpedantic -fexceptions -fstack-protector -isystem include -I src -I $zigDir -c $sf -o $obj *>> $logFile
                    } else {
                        & zig c++ -target $zigTarget $cflags -c $sf -o $obj *>> $logFile
                    }
                    if ($LASTEXITCODE -ne 0) { $nativeOk = $false; break }
                }
                if ($nativeOk) {
                    $srcPath = Join-Path $zigDir $cBin
                    if ($triple -like "*-android") {
                        & "$ndkCxx" $objs -o $srcPath $ldflags *>> $logFile
                    } else {
                        & zig c++ -target $zigTarget $cflags $ldflags $objs -o $srcPath *>> $logFile
                    }
                    if ($LASTEXITCODE -ne 0) { $nativeOk = $false }
                }
                Pop-Location
            }
            }

            if (-not $nativeOk) {
                Write-Host "    [FAIL] $dir / $triple  (log: $logFile)" -ForegroundColor Red
                $projOk = $false
                $allOk  = $false
                continue
            }

            if (Test-Path $srcPath) {
                Copy-Item -Path $srcPath -Destination $dstPath -Force
                $size = (Get-Item $dstPath).Length
                $sizeKB = [math]::Round($size/1KB)
                Write-Host "    [ OK ] $dstName  (${sizeKB}KB)" -ForegroundColor Green
            } else {
                Write-Host "    [WARN] Binary not found at $srcPath" -ForegroundColor Yellow
                $projOk = $false
                $allOk  = $false
            }
            continue
        }

        # Meson projects (dx-video-player) — build via Meson + Ninja
        if ($proj.ContainsKey("meson") -and $proj.meson) {
            $logLabel = $dir.Replace('\', '_').Replace('/', '_')
            $logFile = Join-Path $LOGS "$logLabel-$( $triple.Replace('-','_') ).log"
            $buildDir = Join-Path $workDir "build-$triple"
            $crossFile = Join-Path $workDir "cross" "$triple.ini"
            $nativeFile = Join-Path $workDir "native-$triple.ini"
            $buildMachineFile = Join-Path $workDir "native-build-machine.ini"
            $mesonBin = if ($triple -like "*-pc-windows-*") { $proj.meson_bin } else { $proj.meson_bin -replace '\.exe$', '' }

            # Skip if the final binary already exists in bin (avoids rebuilding the slow
            # FFmpeg-source targets on every full run)
            $dstName = "$fBin-$triple$ext"
            $dstPath = Join-Path $BIN $dstName
            if (Test-Path $dstPath) {
                Write-Host "    [SKIP] $dstName already built" -ForegroundColor DarkYellow
                continue
            }

            # Disable features not supported across all targets
            $featArgs = @("-Ddefault_library=static", "-Dbuildtype=release", "-Dwarning_level=0",
                          "-Dbuild-date=false", "-Dtests=false", "-Dfuzzers=false",
                          "-Dlua=lua5.1", "--force-fallback-for=lua5.1",
                          "-Ddirect3d=disabled", "-Dd3d-hwaccel=disabled", "-Dd3d9-hwaccel=disabled",
                          "-Dgl-dxinterop-d3d9=disabled", "-Dvaapi=disabled",
                          "-Dcuda-hwaccel=disabled", "-Dcuda-interop=disabled",
                          "-Dlibass:require-system-font-provider=false")
            # Unix targets: disable Windows-only features
            if ($triple -notlike "*-pc-windows-*") {
                $featArgs += @("-Dd3d11=disabled", "-Dwasapi=disabled", "-Dshaderc=disabled",
                               "-Dspirv-cross=disabled", "-Dvulkan=disabled", "-Degl-angle=disabled",
                               "-Degl-angle-lib=disabled", "-Degl-angle-win32=disabled",
                               "-Dwin32-smtc=disabled")
            }
            # macOS targets: disable Swift/ObjC (can't cross-compile from Windows)
            if ($triple -like "*-apple-darwin") {
                $featArgs += @("-Dswift-build=disabled", "-Dcocoa=disabled", "-Dcoreaudio=disabled",
                               "-Daudiounit=disabled", "-Davfoundation=disabled",
                               "-Dvideotoolbox-gl=disabled", "-Dvideotoolbox-pl=disabled",
                               "-Dmacos-cocoa-cb=disabled", "-Dmacos-media-player=disabled")
            }
            # Windows targets: enable font provider (DirectWrite via win32)
            if ($triple -like "*-pc-windows-*") {
                $featArgs = $featArgs | Where-Object { $_ -ne "-Dlibass:require-system-font-provider=false" }
            }
            # Zig gnullvm does not need the GUI console wrapper, whose custom
            # CRT entry point conflicts with Zig's startup objects. Also keep
            # source-built FFmpeg targets from filling the disk with programs
            # and test executables that dx-video-player does not use.
    if ($triple -like "*-gnullvm") {
        $featArgs += @(
            "-Dwin32-subsystem=console", "-Ddrm=disabled", "-Dalsa=disabled", "-Djack=disabled",
            "-Dpipewire=disabled", "-Dpulse=disabled", "-Dsndio=disabled", "-Dcaca=disabled",
            "-Ddvdnav=disabled", "-Dlibarchive=disabled", "-Dvaapi-drm=disabled", "-Degl-drm=disabled",
            "-Dffmpeg:programs=disabled", "-Dffmpeg:tests=disabled"
        )
    }
            # Android: use NDK clang, disable non-Android features
            if ($triple -like "*-android") {
                $featArgs += @("-Dgl=disabled", "-Degl-android=disabled", "-Dandroid-media-ndk=disabled",
                               "-Daudiotrack=disabled", "-Daaudio=disabled", "-Dopensles=disabled",
                               "-Dplain-gl=disabled")
            }
            # MSVC targets: clang-cl needs INCLUDE/LIB + PKG_CONFIG_PATH in the shell env.
            # libass aarch64 hand-written asm has no MSVC-compatible host compiler.
            if ($triple -eq "x86_64-pc-windows-msvc" -or $triple -eq "aarch64-pc-windows-msvc") {
                $env:INCLUDE = $msvcInclude
                $env:LIB = if ($triple -like "aarch64*") { $msvcLibArm64 } else { $msvcLibX64 }
                if ($triple -like "aarch64*") { $featArgs += "-Dlibass:asm=disabled" }
            } else {
                Remove-Item Env:INCLUDE -ErrorAction SilentlyContinue
                Remove-Item Env:LIB -ErrorAction SilentlyContinue
            }
            if ($ffmpegPkgConfig.ContainsKey($triple)) {
                $sourcePkgConfigDir = $ffmpegPkgConfig[$triple]
                $env:PKG_CONFIG_PATH = $sourcePkgConfigDir

                # Meson turns absolute -L entries from pkg-config dependencies
                # into ELF RUNPATH entries. The Windows-hosted FFmpeg paths are
                # not valid on the target Linux system, so keep the absolute
                # path only as a global linker search path and remove it from
                # the dependency metadata that Meson scans for rpaths.
                if ($triple -eq "x86_64-unknown-linux-gnu" -or $triple -eq "aarch64-unknown-linux-gnu") {
                    $ffmpegLibDir = Split-Path -Parent $sourcePkgConfigDir
                    $ffmpegRoot = Split-Path -Parent $ffmpegLibDir
                    $sanitizedPkgConfigDir = Join-Path $ROOT "build-pkgconfig\$triple"
                    New-Item -ItemType Directory -Force -Path $sanitizedPkgConfigDir | Out-Null
                    $ffmpegPrefix = $ffmpegRoot -replace '\\', '/'

                    foreach ($pcFile in Get-ChildItem $sourcePkgConfigDir -Filter '*.pc') {
                        $pcText = Get-Content -Raw $pcFile.FullName
                        # Match only the prefix line's content so the existing
                        # CRLF separator before exec_prefix is preserved.
                        $pcText = $pcText -replace '(?m)^prefix=[^\r\n]*', "prefix=$ffmpegPrefix"
                        # Remove only linker -L tokens; do not match the
                        # lowercase "-l" inside directory names such as
                        # ffmpeg-master-latest.
                        $pcText = $pcText -replace '(?m)(^|[ \t])-L[^\s]+[ \t]*', '$1'
                        Set-Content -Path (Join-Path $sanitizedPkgConfigDir $pcFile.Name) -Value $pcText -Encoding ASCII
                    }

                    $env:PKG_CONFIG_PATH = $sanitizedPkgConfigDir
                    $ffmpegLinkDir = $ffmpegLibDir -replace '\\', '/'
                    $featArgs += @("-Dc_link_args=-L$ffmpegLinkDir", "-Dcpp_link_args=-L$ffmpegLinkDir")
                }
            } else {
                Remove-Item Env:PKG_CONFIG_PATH -ErrorAction SilentlyContinue
            }

            # Reconfigure existing build dirs in place so the already-built
            # FFmpeg source subprojects survive this relink-only rebuild.
            # Pin the build-machine compiler to mingw gcc so meson does not pick
            # MSVC cl.exe (which breaks when INCLUDE/LIB point at another arch).
            Push-Location $workDir
            $reconfigure = Test-Path (Join-Path $buildDir "meson-private\coredata.dat")
            if (Test-Path $crossFile) {
                $setupArgs = @("setup")
                if ($reconfigure) { $setupArgs += "--reconfigure" }
                $setupArgs += @($buildDir, "--cross-file", $crossFile, "--native-file", $buildMachineFile) + $featArgs
                Write-Host "    [$triple] meson cross-build..."
            } elseif (Test-Path $nativeFile) {
                $setupArgs = @("setup")
                if ($reconfigure) { $setupArgs += "--reconfigure" }
                $setupArgs += @($buildDir, "--native-file", $nativeFile) + $featArgs
                Write-Host "    [$triple] meson native-build..."
            } else {
                $setupArgs = @("setup")
                if ($reconfigure) { $setupArgs += "--reconfigure" }
                $setupArgs += @($buildDir, "--native-file", $buildMachineFile) + $featArgs
                Write-Host "    [$triple] meson build (zig cc)..."
            }

            & meson $setupArgs *>> $logFile
            $mesonExit = $LASTEXITCODE
            if ($mesonExit -ne 0 -and $reconfigure) {
                # A moved checkout can leave absolute native-file paths in
                # Meson's cached coredata. Retry once with a generated-build
                # wipe; source subprojects and package caches remain intact.
                Write-Host "    [$triple] reconfigure failed; retrying with --wipe..." -ForegroundColor Yellow
                $wipeArgs = @("setup", "--wipe") + @($setupArgs | Select-Object -Skip 2)
                & meson $wipeArgs *>> $logFile
                $mesonExit = $LASTEXITCODE
            }
            $exitCode = $mesonExit
            if ($mesonExit -eq 0) {
                & ninja -C $buildDir -j $proj.jobs *>> $logFile
                $exitCode = $LASTEXITCODE
            }
            if ($exitCode -ne 0) {
                Write-Host "    [FAIL] $dir / $triple  (log: $logFile)" -ForegroundColor Red
                $projOk = $false; $allOk = $false
                Pop-Location
                continue
            }

            $srcPath = Join-Path $buildDir $mesonBin
            $dstName = "$fBin-$triple$ext"
            $dstPath = Join-Path $BIN $dstName
            if (Test-Path $srcPath) {
                Copy-Item -Path $srcPath -Destination $dstPath -Force
                $sizeKB = [math]::Round((Get-Item $dstPath).Length/1KB)
                Write-Host "    [ OK ] $dstName  (${sizeKB}KB)" -ForegroundColor Green
                # MSVC targets link the prebuilt shared FFmpeg; bundle its DLLs next to the exe
                if ($ffmpegPkgConfig.ContainsKey($triple) -and $triple -like "*-pc-windows-msvc") {
                    $ffmpegRoot = Split-Path -Parent (Split-Path -Parent $ffmpegPkgConfig[$triple])
                    if (Test-Path (Join-Path $ffmpegRoot "bin")) {
                        Copy-Item -Path (Join-Path $ffmpegRoot "bin\*.dll") -Destination $BIN -Force
                    }
                }
            } else {
                Write-Host "    [WARN] Binary not found at $srcPath" -ForegroundColor Yellow
                $projOk = $false; $allOk = $false
            }
            Pop-Location
            continue
        }

        $srcPath = [System.IO.Path]::Combine($workDir, "target", $triple, "release", "$cBin$ext")
        $dstName = "$fBin-$triple$ext"
        $dstPath = Join-Path $BIN $dstName

        Write-Host "    [$triple] building..." -ForegroundColor Gray

        Push-Location $workDir
        $logFile = Join-Path $LOGS "$dir-$( $triple.Replace('-','_') ).log"

        $featureArgs = @()
        if ($hasUnixFeatures -and -not $isWinTgt) {
            $featureArgs = @("--no-default-features", "--features", $proj.unix_features)
        }

        if ($isMacTgt) {
            $env:SDKROOT = $MACOS_SDK
            $env:MACOSX_DEPLOYMENT_TARGET = "10.15"
            $env:WHISPER_NO_ACCELERATE = "ON"
        }

        # Bindgen fix for gnullvm targets (clang doesn't recognize the triple)
        if ($proj.ContainsKey("bindgen_fix") -and $proj.bindgen_fix -and $triple -like "*-gnullvm") {
            $env:BINDGEN_EXTRA_CLANG_ARGS = "--target=aarch64-pc-windows-msvc"
        } else {
            $env:BINDGEN_EXTRA_CLANG_ARGS = $null
        }

        # Whisper fix: disable x86 CPU feature flags on ARM64 targets
        if ($proj.ContainsKey("whisper_fix") -and $proj.whisper_fix -and $triple -like "aarch64*") {
            $env:WHISPER_NO_AVX = "ON"
            $env:WHISPER_NO_AVX2 = "ON"
            $env:WHISPER_NO_AVX512 = "ON"
            $env:WHISPER_NO_FMA = "ON"
            $env:WHISPER_NO_F16C = "ON"
        } else {
            $env:WHISPER_NO_AVX = $null
            $env:WHISPER_NO_AVX2 = $null
            $env:WHISPER_NO_AVX512 = $null
            $env:WHISPER_NO_FMA = $null
            $env:WHISPER_NO_F16C = $null
        }

        # MSVC environment setup
        $msvcDir = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207"
        $msvcVer = "14.44.35207"
        $sdkVer = "10.0.26100.0"
        $sdkRoot = "C:\Program Files (x86)\Windows Kits\10"
        $env:LIB = "$msvcDir\lib\x64;$msvcDir\lib\x64\store;$sdkRoot\Lib\$sdkVer\ucrt\x64;$sdkRoot\Lib\$sdkVer\um\x64"
        $env:INCLUDE = "$msvcDir\include;$sdkRoot\Include\$sdkVer\ucrt;$sdkRoot\Include\$sdkVer\um;$sdkRoot\Include\$sdkVer\shared"
        # These env vars help the cc crate (zstd-sys, ring, etc.) find cl.exe
        # and clang without needing VSWHERE or registry entries.
        $env:VCINSTALLDIR = $msvcDir
        $env:VCToolsVersion = $msvcVer
        $env:WindowsSdkDir = "$sdkRoot\"
        $env:WindowsSDKVersion = "$sdkVer\"
        # LLVM clang is needed by ring crate for aarch64-windows-msvc
        $env:PATH = "C:\Program Files\LLVM\bin;$env:PATH"
        if ($triple -eq "aarch64-pc-windows-msvc") {
            $env:CARGO_TARGET_AARCH64_PC_WINDOWS_MSVC_LINKER = "G:\Dx\cli\tools\bin\link-arm64-wrapper.cmd"
            # Don't set CC_aarch64_pc_windows_msvc — let the cc crate detect MSVC
            # via VCINSTALLDIR + PATH. Explicit CC breaks flag detection (MSVC vs GCC).
            $env:CC_aarch64_pc_windows_msvc = $null
            $env:CXX_aarch64_pc_windows_msvc = $null
            # cmake needs explicit generator for ARM64 cross-compilation
            $env:CMAKE_GENERATOR = "Visual Studio 17 2022"
            $env:CMAKE_GENERATOR_PLATFORM = "ARM64"
        } else {
            $env:CARGO_TARGET_AARCH64_PC_WINDOWS_MSVC_LINKER = $null
            $env:CC_aarch64_pc_windows_msvc = $null
            $env:CXX_aarch64_pc_windows_msvc = $null
            $env:CMAKE_GENERATOR = $null
            $env:CMAKE_GENERATOR_PLATFORM = $null
        }

        # Skip gnullvm targets for projects that can't cross-compile
        if ($proj.ContainsKey("skip_gnullvm") -and $proj.skip_gnullvm -and $triple -like "*-gnullvm") {
            Write-Host "    [SKIP] $triple (gnullvm not supported)" -ForegroundColor DarkYellow
            continue
        }

        # Skip aarch64-msvc for projects with incompatible C dependencies
        if ($proj.ContainsKey("skip_arm64_msvc") -and $proj.skip_arm64_msvc -and $triple -eq "aarch64-pc-windows-msvc") {
            Write-Host "    [SKIP] aarch64-pc-windows-msvc (C code compat issues)" -ForegroundColor DarkYellow
            continue
        }

        # Skip target if binary already exists (partial rebuild)
        if (Test-Path $dstPath) {
            Write-Host "    [SKIP] $triple (already in bin)" -ForegroundColor DarkYellow
            continue
        }

        $buildArgs = @("build", "--release") + @($pkg.Split(" ", [StringSplitOptions]::RemoveEmptyEntries)) + @("-j", "$jobs", "--target", $triple) + $featureArgs

        # Skip cmake compiler detection for zig cc targets (circumvents VARARGS header bug)
        if ($triple -like "*-gnullvm" -or $triple -like "*-musl" -or $triple -like "*-android" -or $triple -like "*-linux-*") {
            $env:CMAKE_C_COMPILER_WORKS = "TRUE"
            $env:CMAKE_CXX_COMPILER_WORKS = "TRUE"
        } else {
            $env:CMAKE_C_COMPILER_WORKS = $null
            $env:CMAKE_CXX_COMPILER_WORKS = $null
        }

        if ($isWinTgt -and $triple -notlike "*-gnullvm") {
            & cargo $buildArgs *> $logFile
        } elseif ($triple -like "*-android") {
            # Android builds need NDK clang instead of zig
            $ndkDir = "G:\android-ndk-r27\toolchains\llvm\prebuilt\windows-x86_64"
            $ndkRoot = "G:\android-ndk-r27"
            $ndkClang = "$ndkDir\bin\aarch64-linux-android35-clang.cmd"
            $ndkClangCxx = "$ndkDir\bin\aarch64-linux-android35-clang++.cmd"
            # Cargo config for Android target linker + archiver
            $cargoConfigDir = Join-Path $workDir ".cargo"
            New-Item -ItemType Directory -Force -Path $cargoConfigDir | Out-Null
            # Use forward slashes in TOML paths (TOML treats \ as escape char)
            $tomlLinker = $ndkClang -replace '\\', '/'
            $tomlAr = "$ndkDir\bin\llvm-ar.exe" -replace '\\', '/'
@"
[target.aarch64-linux-android]
linker = "$tomlLinker"
ar = "$tomlAr"
"@ | Set-Content -Path (Join-Path $cargoConfigDir "config.toml") -Encoding ASCII
            # cmake (whisper.cpp) Android NDK detection + ABI override (CMake 4.2 + NDK r27 gap: NDK_PROC_* not defined)
            $env:ANDROID_NDK_HOME = "$ndkRoot"
            $env:ANDROID_NDK_ROOT = "$ndkRoot"
            $env:CMAKE_ANDROID_ARCH_ABI = "arm64-v8a"
            # Use NDK's target-specific clang (GCC-compatible, not clang-cl) so cc crate uses GCC-mode flags
            $env:CC_aarch64_linux_android = "$ndkClang"
            $env:AR_aarch64_linux_android = "$ndkDir\bin\llvm-ar.exe"
            # cmake needs explicit generator for Android
            $env:CMAKE_GENERATOR = "Ninja"
            $env:CMAKE_MAKE_PROGRAM = (Get-Command ninja).Source
            $env:CMAKE_C_COMPILER = "$ndkDir\bin\aarch64-linux-android35-clang.cmd"
            $env:CMAKE_CXX_COMPILER = "$ndkDir\bin\aarch64-linux-android35-clang++.cmd"
            & cargo $buildArgs *> $logFile
        } else {
            # zigbuild doesn't want the 'build' subcommand; strip it
            $zbArgs = $buildArgs | Where-Object { $_ -ne "build" }
            & cargo zigbuild $zbArgs *> $logFile
        }
        $exitCode = $LASTEXITCODE
        Pop-Location

        if ($exitCode -ne 0) {
            Write-Host "    [FAIL] $dir / $triple  (log: $logFile)" -ForegroundColor Red
            $projOk = $false
            $allOk  = $false
            continue
        }

        if (Test-Path $srcPath) {
            Copy-Item -Path $srcPath -Destination $dstPath -Force
            $size = (Get-Item $dstPath).Length
            $sizeKB = [math]::Round($size/1KB)
            Write-Host "    [ OK ] $dstName  (${sizeKB}KB)" -ForegroundColor Green
        } else {
            Write-Host "    [WARN] Binary not found at $srcPath" -ForegroundColor Yellow
            $projOk = $false
            $allOk  = $false
        }
    }

    if ($projOk) {
        $builtCount++
        Write-Host "  [PASS] $dir - ALL $totalTargets TARGETS PASSED" -ForegroundColor Green
    } else {
        Write-Host "  [FAIL] $dir - SOME TARGETS FAILED (check logs in $LOGS)" -ForegroundColor Red
    }

    # Clean build artifacts (binaries already in G:\Dx\bin)
    if ($proj.ContainsKey("native") -and $proj.native) {
        Remove-Item -Recurse -Force (Join-Path $workDir "posix\build_zig") -ErrorAction SilentlyContinue | Out-Null
        Remove-Item -Recurse -Force (Join-Path $workDir "windows\Release") -ErrorAction SilentlyContinue | Out-Null
    } else {
        Push-Location $workDir
        cargo clean 2>&1 | Out-Null
        Pop-Location
    }
}

# Copy plain-named binaries for the native platform (production installs)
$nativeTriple = "x86_64-pc-windows-msvc"
$nativeExt = ".exe"
Write-Host "`nCopying plain-named binaries for $nativeTriple..." -ForegroundColor Cyan
Get-ChildItem "$BIN\*-$nativeTriple$nativeExt" -ErrorAction SilentlyContinue | ForEach-Object {
    $plainName = $_.Name -replace "-$nativeTriple$nativeExt", "$nativeExt"
    $plainPath = Join-Path $BIN $plainName
    Copy-Item $_.FullName $plainPath -Force
    Write-Host "  $($_.Name) → $plainName" -ForegroundColor Gray
}
Write-Host "  Done." -ForegroundColor Green

# Copy os binary: the native C++ project uses zig, so plain name is already set up
# For dx-cli: rename dx-cli.exe → dx.exe (CLI binary)
if (Test-Path "$BIN\dx-cli.exe") {
    Copy-Item "$BIN\dx-cli.exe" "$BIN\dx.exe" -Force
    Write-Host "  dx-cli.exe → dx.exe" -ForegroundColor Gray
}

Write-Host "`n================================================================" -ForegroundColor Cyan
if ($allOk) {
    Write-Host "  ALL $totalProjects projects x $totalTargets targets built successfully!" -ForegroundColor Green
} else {
    Write-Host "  $builtCount / $totalProjects projects fully passed." -ForegroundColor Yellow
    Write-Host "  Check logs in: $LOGS" -ForegroundColor Yellow
}
Write-Host "================================================================  " -ForegroundColor Cyan
