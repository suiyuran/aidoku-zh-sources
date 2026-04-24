#!/bin/bash
set -e

VERSION="${AIDOKU_CLI_VERSION:-v0.5.9}"
INSTALL_DIR="${AIDOKU_CLI_INSTALL_DIR:-$HOME/.local/bin}"

detect_arch() {
    local arch=$(uname -m)
    case "$arch" in
        x86_64|amd64)
            echo "x86_64"
            ;;
        aarch64|arm64)
            echo "arm64"
            ;;
        i386|i686)
            echo "i386"
            ;;
        *)
            echo "Unsupported architecture: $arch" >&2
            exit 1
            ;;
    esac
}

detect_os() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    case "$os" in
        linux)
            echo "linux"
            ;;
        darwin)
            echo "macos"
            ;;
        *)
            echo "Unsupported OS: $os" >&2
            exit 1
            ;;
    esac
}

main() {
    local os=$(detect_os)
    local arch=$(detect_arch)

    echo "Detected OS: $os, Architecture: $arch"
    echo "Installing aidoku-cli $VERSION..."

    local download_url="https://github.com/Aidoku/aidoku-cli/releases/download/${VERSION}/aidoku-cli_${VERSION#v}_${os}_${arch}.tar.gz"

    echo "Downloading from: $download_url"

    # Create temporary directory
    local tmp_dir=$(mktemp -d)
    trap "rm -rf $tmp_dir" EXIT

    # Download and extract
    curl -L -o "$tmp_dir/aidoku-cli.tar.gz" "$download_url"
    tar -xzf "$tmp_dir/aidoku-cli.tar.gz" -C "$tmp_dir"

    # Install binary
    mkdir -p "$INSTALL_DIR"
    cp "$tmp_dir/aidoku" "$INSTALL_DIR/aidoku"
    chmod +x "$INSTALL_DIR/aidoku"

    echo "aidoku-cli installed successfully to $INSTALL_DIR/aidoku"
    echo "Make sure $INSTALL_DIR is in your PATH"

    # Verify installation
    if "$INSTALL_DIR/aidoku" --version >/dev/null 2>&1; then
        echo "Verification successful!"
        "$INSTALL_DIR/aidoku" --version
    else
        echo "Warning: Installation may have failed. Unable to verify binary." >&2
        exit 1
    fi
}

main "$@"