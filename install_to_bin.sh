#!/usr/bin/env bash
# Build the portal in release mode and install it into the shared bin directory.
#
# Usage: ./install_to_bin.sh
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEST_DIR="/SNS/VENUS/shared/software/bin"

CARGO="$(command -v cargo || true)"
[[ -z "$CARGO" && -x "$HOME/.cargo/bin/cargo" ]] && CARGO="$HOME/.cargo/bin/cargo"
if [[ -z "$CARGO" ]]; then
    echo "Error: cargo not found." >&2
    exit 1
fi

echo "Building portal_to_all_rust_applications (release)..."
(cd "$REPO_DIR" && "$CARGO" build --release)

echo "Installing into $DEST_DIR..."
install -m 755 "$REPO_DIR/target/release/portal_to_all_rust_applications" \
    "$DEST_DIR/portal_to_all_rust_applications"
install -m 755 "$REPO_DIR/deploy/launch_portal.sh" "$DEST_DIR/launch_portal.sh"

echo "Done."
