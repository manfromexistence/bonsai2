#!/bin/sh
# DX CLI Installer — macOS & Linux
# Usage: curl -fsSL https://dx.vercel.app/install.sh | sh
# Or:    wget -qO- https://dx.vercel.app/install.sh | sh

set -eu

# ── Detect platform ──────────────────────────────────────────────
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)  OS="unknown-linux-gnu" ;;
    Darwin) OS="apple-darwin" ;;
    *)      echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
    x86_64|amd64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *)          echo "Unsupported arch: $ARCH"; exit 1 ;;
esac

PLATFORM="${ARCH}-${OS}"
EXT=""
[ "$OS" != "apple-darwin" ] || EXT=""

# ── Resolve install directory ────────────────────────────────────
: "${DX_HOME:=${HOME}/.dx}"
BIN_DIR="${DX_HOME}/bin"
DX_BIN="${BIN_DIR}/dx${EXT}"

# ── Display plan ─────────────────────────────────────────────────
echo ""
echo "  ╭─ DX Install ─────────────────────────────────╮"
echo "  │  Platform:      ${PLATFORM}"
echo "  │  Install dir:   ${BIN_DIR}"
echo "  │  Binary:        dx-cli (latest release)"
echo "  │  Tools:         All 15 companion tools"
echo "  ╰────────────────────────────────────────────────╯"
echo ""

# ── Create install directory ─────────────────────────────────────
mkdir -p "${BIN_DIR}"

# ── Download dx-cli ──────────────────────────────────────────────
RELEASES_URL="https://api.github.com/repos/essence-dx/cli/releases/latest"
ASSET_NAME="dx-cli-${PLATFORM}${EXT}"

echo "  Fetching latest release info..."
DOWNLOAD_URL=$(curl -fsSL -H "User-Agent: dx-installer/1.0" "${RELEASES_URL}" \
    | grep -o "https://github.com/essence-dx/cli/releases/download/[^\"]*/${ASSET_NAME}" \
    | head -1)

if [ -z "$DOWNLOAD_URL" ]; then
    echo "  [ERROR] No asset found for ${ASSET_NAME}"
    echo "  Available assets for latest release:"
    curl -fsSL -H "User-Agent: dx-installer/1.0" "${RELEASES_URL}" \
        | grep -o '"name":"[^"]*"' | head -20
    exit 1
fi

echo "  Downloading dx-cli for ${PLATFORM}..."
if command -v curl >/dev/null 2>&1; then
    curl -fsSL -o "${DX_BIN}" "${DOWNLOAD_URL}"
elif command -v wget >/dev/null 2>&1; then
    wget -qO "${DX_BIN}" "${DOWNLOAD_URL}"
else
    echo "  [ERROR] Neither curl nor wget found"
    exit 1
fi
chmod +x "${DX_BIN}"
echo "  [OK] Downloaded to ${DX_BIN}"

# ── Add to PATH ──────────────────────────────────────────────────
case ":${PATH}:" in
    *:"${BIN_DIR}":*)
        echo "  [OK] ${BIN_DIR} already in PATH"
        ;;
    *)
        SHELL_PROFILE=""
        case "${SHELL:-}" in
            */zsh) SHELL_PROFILE="${HOME}/.zshrc" ;;
            */bash) SHELL_PROFILE="${HOME}/.bashrc" ;;
        esac
        if [ -n "${SHELL_PROFILE}" ]; then
            echo "export PATH=\"${BIN_DIR}:\$PATH\"" >> "${SHELL_PROFILE}"
            echo "  [OK] Added ${BIN_DIR} to PATH in ${SHELL_PROFILE}"
        else
            echo "  [HINT] Add 'export PATH=\"${BIN_DIR}:\$PATH\"' to your shell profile"
        fi
        export PATH="${BIN_DIR}:${PATH}"
        ;;
esac

# ── Install all tools ────────────────────────────────────────────
echo ""
echo "  Installing all 15 companion tools..."
if "${DX_BIN}" update apply --install-all --json; then
    echo "  [OK] All tools installed successfully"
else
    echo "  [WARN] Some tools may not have installed."
    echo "  Run 'dx update apply --install-all' to retry."
fi

# ── Verify installation ──────────────────────────────────────────
echo ""
echo "  Verifying installation..."
FOUND=0
for TOOL in check dcp driven forge i18n icon media metasearch serializer style script token tree os www; do
    TOOL_PATH="${BIN_DIR}/dx-${TOOL}${EXT}"
    if [ -f "${TOOL_PATH}" ]; then
        echo "  [OK] dx-${TOOL}"
        FOUND=$((FOUND + 1))
    else
        echo "  [--] dx-${TOOL} (not found)"
    fi
done

echo ""
echo "  ╭────────────────────────────────────────────────────╮"
echo "  │  DX CLI installed successfully!                    │"
echo "  │  ${FOUND}/15 tools available                           │"
echo "  │                                                    │"
echo "  │  Run 'dx --help' to get started                    │"
echo "  │  Run 'dx update apply --install-all' to retry      │"
echo "  ╰────────────────────────────────────────────────────╯"
