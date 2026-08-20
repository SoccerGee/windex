#!/usr/bin/env bash
#
# Package dist/Windex.app into a distributable disk image.
#
# Produces dist/Windex-<version>.dmg with an /Applications symlink so the
# window reads as the usual drag-to-install. Builds the app first if needed.
#
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST="$REPO_DIR/dist"
APP="$DIST/Windex.app"

info() { printf '\033[1;34m==>\033[0m %s\n' "$1"; }

VERSION="$(awk -F'"' '/^version = /{print $2; exit}' "$REPO_DIR/Cargo.toml")"
DMG="$DIST/Windex-$VERSION.dmg"

[ -d "$APP" ] || "$REPO_DIR/scripts/build-app.sh" "$@"

info "Staging disk image contents"
STAGE="$DIST/dmg-stage"
rm -rf "$STAGE"
mkdir -p "$STAGE"
cp -R "$APP" "$STAGE/"
ln -s /Applications "$STAGE/Applications"

info "Creating $DMG"
rm -f "$DMG"
hdiutil create \
  -volname "Windex $VERSION" \
  -srcfolder "$STAGE" \
  -ov -format UDZO \
  "$DMG" >/dev/null

rm -rf "$STAGE"

shasum -a 256 "$DMG" | tee "$DMG.sha256"
info "Built $DMG"
