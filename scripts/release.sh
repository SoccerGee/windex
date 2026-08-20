#!/usr/bin/env bash
#
# Cut a release: bump the version, tag it, and push. GitHub Actions builds the
# universal app, attaches the .dmg to the release, and updates the Homebrew
# cask (if HOMEBREW_TAP_TOKEN is configured).
#
# Usage: ./scripts/release.sh 0.2.0
#
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_DIR"

info() { printf '\033[1;34m==>\033[0m %s\n' "$1"; }
die() { printf '\033[1;31merror:\033[0m %s\n' "$1" >&2; exit 1; }

VERSION="${1:-}"
[ -n "$VERSION" ] || die "usage: $0 <version>   e.g. $0 0.2.0"
[[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "version must look like 1.2.3"

[ -z "$(git status --porcelain)" ] || die "working tree is dirty — commit or stash first"
git rev-parse "v$VERSION" >/dev/null 2>&1 && die "tag v$VERSION already exists"

info "Setting version to $VERSION"
# Only the [package] version — the first `version = ` line in the file.
awk -v v="$VERSION" '!done && /^version = /{sub(/"[^"]*"/, "\"" v "\""); done=1} {print}' \
  Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml

# Refresh Cargo.lock's own record of the package version.
cargo update --workspace --offline >/dev/null 2>&1 || cargo update --workspace >/dev/null

git add Cargo.toml Cargo.lock
if git diff --cached --quiet; then
  # Cargo.toml already carried this version — tag the commit as it stands.
  info "Version is already $VERSION; tagging the current commit"
else
  git commit -m "Release $VERSION"
fi
git tag -a "v$VERSION" -m "Windex $VERSION"

info "Pushing tag v$VERSION"
git push origin HEAD "v$VERSION"

cat <<MSG

Release started. Watch it here:
  https://github.com/SoccerGee/windex/actions

When it finishes, friends can install with:
  brew install --cask --no-quarantine soccergee/tap/windex
MSG
