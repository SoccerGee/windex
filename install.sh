#!/usr/bin/env bash
#
# Build Windex from source and install it into /Applications.
#
# Most people should install the release build instead:
#   brew install --cask --no-quarantine soccergee/tap/windex
#
# Usage:
#   ./install.sh              # build, install to /Applications, launch
#   ./install.sh --native     # skip the Intel slice (faster on Apple Silicon)
#   ./install.sh uninstall    # quit, remove the app and the login item
#
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LABEL="com.granttuttle.windex"
APP_DST="/Applications/Windex.app"
PLIST="$HOME/Library/LaunchAgents/$LABEL.plist"
GUI="gui/$(id -u)"

info() { printf '\033[1;34m==>\033[0m %s\n' "$1"; }

stop_running() {
  launchctl bootout "$GUI/$LABEL" 2>/dev/null || true
  # Exact process name only: -f would also match, say, an editor that happens
  # to have the windex path on its command line.
  pkill -x windex 2>/dev/null || true
}

if [ "${1:-}" = "uninstall" ]; then
  info "Stopping Windex"
  stop_running
  info "Removing $APP_DST and the login item"
  rm -rf "$APP_DST"
  rm -f "$PLIST"
  info "Uninstalled. You can also remove Windex from System Settings → Privacy & Security → Accessibility."
  exit 0
fi

"$REPO_DIR/scripts/build-app.sh" "$@"

info "Stopping any running copy"
stop_running

info "Installing to $APP_DST"
rm -rf "$APP_DST"
cp -R "$REPO_DIR/dist/Windex.app" "$APP_DST"

info "Launching Windex"
open "$APP_DST"

cat <<MSG

Windex is in your menu bar.

  • Grant Accessibility access when macOS asks (System Settings →
    Privacy & Security → Accessibility). Windex restarts itself once you do.
  • Turn on "Launch at Login" from the menu bar icon to start it automatically.
  • Logs: tail -f ~/Library/Logs/windex.log
  • Uninstall: ./install.sh uninstall
MSG
