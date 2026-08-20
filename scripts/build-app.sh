#!/usr/bin/env bash
#
# Build Windex.app.
#
# Produces dist/Windex.app containing a universal (Apple Silicon + Intel)
# binary, the generated .icns, and an Info.plist stamped with the version from
# Cargo.toml. The bundle is ad-hoc signed so macOS treats it as a stable app
# identity rather than a loose executable.
#
# Usage:
#   ./scripts/build-app.sh            # universal binary (both architectures)
#   ./scripts/build-app.sh --native   # this machine's architecture only (faster)
#
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST="$REPO_DIR/dist"
APP="$DIST/Windex.app"
NATIVE_ONLY=0

[ "${1:-}" = "--native" ] && NATIVE_ONLY=1

info() { printf '\033[1;34m==>\033[0m %s\n' "$1"; }
die() { printf '\033[1;31merror:\033[0m %s\n' "$1" >&2; exit 1; }

command -v cargo >/dev/null || die "cargo not found — install Rust from https://rustup.rs"

VERSION="$(awk -F'"' '/^version = /{print $2; exit}' "$REPO_DIR/Cargo.toml")"
[ -n "$VERSION" ] || die "could not read version from Cargo.toml"
info "Building Windex $VERSION"

# 1. Compile the binary (universal unless --native).
cd "$REPO_DIR"
if [ "$NATIVE_ONLY" = 1 ]; then
  cargo build --release --bin windex
  BIN="$REPO_DIR/target/release/windex"
else
  for target in aarch64-apple-darwin x86_64-apple-darwin; do
    rustup target list --installed 2>/dev/null | grep -qx "$target" \
      || { info "Adding Rust target $target"; rustup target add "$target"; }
    info "Building for $target"
    cargo build --release --bin windex --target "$target"
  done
  BIN="$REPO_DIR/target/universal-windex"
  info "Merging architectures with lipo"
  lipo -create -output "$BIN" \
    "$REPO_DIR/target/aarch64-apple-darwin/release/windex" \
    "$REPO_DIR/target/x86_64-apple-darwin/release/windex"
fi

# 2. Generate the app icon (.icns) from the rendered master PNG.
info "Generating app icon"
ICONSET="$DIST/Windex.iconset"
rm -rf "$ICONSET"
mkdir -p "$ICONSET"
MASTER="$DIST/icon-master.png"
python3 "$REPO_DIR/packaging/make_icons.py" "$MASTER"
for spec in "16 icon_16x16" "32 icon_16x16@2x" "32 icon_32x32" "64 icon_32x32@2x" \
            "128 icon_128x128" "256 icon_128x128@2x" "256 icon_256x256" \
            "512 icon_256x256@2x" "512 icon_512x512" "1024 icon_512x512@2x"; do
  set -- $spec
  sips -z "$1" "$1" "$MASTER" --out "$ICONSET/$2.png" >/dev/null
done
iconutil -c icns "$ICONSET" -o "$DIST/Windex.icns"

# 3. Assemble the bundle.
info "Assembling $APP"
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp "$BIN" "$APP/Contents/MacOS/windex"
chmod +x "$APP/Contents/MacOS/windex"
cp "$DIST/Windex.icns" "$APP/Contents/Resources/Windex.icns"
sed "s/@VERSION@/$VERSION/g" "$REPO_DIR/packaging/Info.plist.in" > "$APP/Contents/Info.plist"
printf 'APPL????' > "$APP/Contents/PkgInfo"
plutil -lint "$APP/Contents/Info.plist" >/dev/null

# 4. Sign. A Developer ID identity is used when CODESIGN_IDENTITY is set;
#    otherwise the bundle is ad-hoc signed (works, but Gatekeeper warns on
#    first launch and each rebuild resets the Accessibility grant).
IDENTITY="${CODESIGN_IDENTITY:--}"
if [ "$IDENTITY" = "-" ]; then
  info "Ad-hoc signing (set CODESIGN_IDENTITY to sign with a Developer ID)"
  codesign --force --sign - --timestamp=none "$APP"
else
  info "Signing with $IDENTITY"
  codesign --force --options runtime --timestamp --sign "$IDENTITY" "$APP"
fi
codesign --verify --strict "$APP"

rm -rf "$ICONSET" "$MASTER"
info "Built $APP"
lipo -archs "$APP/Contents/MacOS/windex" | sed 's/^/    architectures: /'
