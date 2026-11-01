#!/usr/bin/env bash
set -euo pipefail
REPO="https://github.com/avelone123123/slopguard.git"
if command -v cargo >/dev/null 2>&1; then
  echo "Installing SlopGuard via Cargo Git..."
  cargo install --git "$REPO" --bin slopguard
  exit 0
fi
BIN_DIR="${HOME}/.cargo/bin"
mkdir -p "$BIN_DIR"
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
TARGET="slopguard-linux-x64"
if [ "$OS" = "darwin" ]; then TARGET="slopguard-macos-universal"; fi
URL="https://github.com/avelone123123/slopguard/releases/latest/download/${TARGET}"
echo "Downloading SlopGuard (${TARGET}) from GitHub Releases..."
curl -fsSL "$URL" -o "${BIN_DIR}/slopguard"
chmod +x "${BIN_DIR}/slopguard"
echo "SlopGuard installed into ${BIN_DIR}/slopguard"