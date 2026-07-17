#!/usr/bin/env bash
#
# windex installer — builds the release binary, installs it to a stable path,
# and registers a LaunchAgent so it starts automatically at login.
#
# Usage:
#   ./install.sh            # build + install + load the login agent
#   ./install.sh uninstall  # stop the agent and remove installed files
#
set -euo pipefail

LABEL="com.granttuttle.windex"
REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_SRC="$REPO_DIR/target/release/windex"
BIN_DST="$HOME/bin/windex"
PLIST="$HOME/Library/LaunchAgents/$LABEL.plist"
LOG="$HOME/Library/Logs/windex.log"
GUI="gui/$(id -u)"

info() { printf '\033[1;34m==>\033[0m %s\n' "$1"; }
warn() { printf '\033[1;33mwarning:\033[0m %s\n' "$1"; }

unload_agent() {
  if launchctl print "$GUI/$LABEL" >/dev/null 2>&1; then
    info "Stopping existing agent"
    launchctl bootout "$GUI/$LABEL" 2>/dev/null || true
  fi
  # Kill any stray instances (e.g. ones launched manually from the build dir)
  pkill -f 'windex/target/release/windex' 2>/dev/null || true
  pkill -f "$BIN_DST" 2>/dev/null || true
}

uninstall() {
  unload_agent
  info "Removing installed files"
  rm -f "$PLIST" "$BIN_DST"
  info "Uninstalled. (You may also remove the Accessibility entry for windex in System Settings.)"
  exit 0
}

[ "${1:-}" = "uninstall" ] && uninstall

# 1. Build the release binary
info "Building release binary (cargo build --release)"
( cd "$REPO_DIR" && cargo build --release )

[ -f "$BIN_SRC" ] || { warn "Build did not produce $BIN_SRC"; exit 1; }

# 2. Stop any running instance before replacing the binary
unload_agent

# 3. Install to a stable path (survives `cargo clean` / rebuilds)
info "Installing binary to $BIN_DST"
mkdir -p "$HOME/bin"
cp "$BIN_SRC" "$BIN_DST"

# 4. Write the LaunchAgent plist
info "Writing LaunchAgent to $PLIST"
mkdir -p "$HOME/Library/LaunchAgents"
cat > "$PLIST" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>$LABEL</string>
    <key>ProgramArguments</key>
    <array>
        <string>$BIN_DST</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
    <key>ProcessType</key>
    <string>Interactive</string>
    <key>StandardOutPath</key>
    <string>$LOG</string>
    <key>StandardErrorPath</key>
    <string>$LOG</string>
</dict>
</plist>
EOF

plutil -lint "$PLIST" >/dev/null

# 5. Load (and start) the agent
info "Loading login agent"
launchctl bootstrap "$GUI" "$PLIST"
launchctl kickstart -k "$GUI/$LABEL"

sleep 1
info "Done. windex is installed and will start at login."

# 6. Accessibility permission (can't be automated — guide the user)
if grep -q "Accessibility permission not granted" "$LOG" 2>/dev/null; then
  warn "windex needs Accessibility permission for the binary at:"
  printf '       %s\n' "$BIN_DST"
  info "Opening System Settings → Privacy & Security → Accessibility…"
  open "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility" || true
  cat <<MSG

  Add and enable $BIN_DST there:
    1. Click the + button
    2. Press Cmd+Shift+G and paste:  $BIN_DST
    3. Toggle it on
  Then restart it:  launchctl kickstart -k $GUI/$LABEL
MSG
fi

cat <<MSG

Manage it later:
  Restart:    launchctl kickstart -k $GUI/$LABEL
  Stop:       launchctl bootout $GUI/$LABEL
  Logs:       tail -f $LOG
  Uninstall:  $0 uninstall
MSG
