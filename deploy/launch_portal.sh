#!/usr/bin/env bash
# Launch the Neutron Imaging Application Portal.
#
# Runs the pre-built portal binary installed next to this script.
# The sources live in /SNS/VENUS/shared/software/git/portal_to_all_rust_applications;
# after changing them, rebuild (cargo build --release) and copy the new binary here
# (or run install_to_bin.sh from the repo).
#
# Usage: ./launch_portal.sh
set -euo pipefail

BIN_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY="$BIN_DIR/portal_to_all_rust_applications"

# GUI apps need a display (e.g. a ThinLinc session).
if [[ -z "${DISPLAY:-}" && -z "${WAYLAND_DISPLAY:-}" ]]; then
    echo "Error: no display found (DISPLAY/WAYLAND_DISPLAY unset)." >&2
    echo "Run this from a graphical session such as ThinLinc." >&2
    exit 1
fi

if [[ ! -x "$BINARY" ]]; then
    echo "Error: portal binary not found at $BINARY" >&2
    exit 1
fi

exec "$BINARY" "$@"
