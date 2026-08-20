#!/usr/bin/env bash
#
# One-time setup of the Homebrew tap that friends install from.
#
# Creates (or reuses) github.com/SoccerGee/homebrew-tap and commits a cask
# pointing at an existing GitHub release, so `brew install --cask
# soccergee/tap/windex` works. After this, the release workflow keeps the cask
# up to date on its own — see .github/workflows/release.yml.
#
# Usage: ./scripts/setup-tap.sh [version]     # defaults to Cargo.toml's version
#
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OWNER="SoccerGee"
TAP_REPO="homebrew-tap"

info() { printf '\033[1;34m==>\033[0m %s\n' "$1"; }
die() { printf '\033[1;31merror:\033[0m %s\n' "$1" >&2; exit 1; }

command -v gh >/dev/null || die "GitHub CLI not found — brew install gh && gh auth login"
gh auth status >/dev/null 2>&1 || die "not logged in — run: gh auth login"

VERSION="${1:-$(awk -F'"' '/^version = /{print $2; exit}' "$REPO_DIR/Cargo.toml")}"
DMG_URL="https://github.com/$OWNER/windex/releases/download/v$VERSION/Windex-$VERSION.dmg"

info "Checking release v$VERSION"
# Checked unauthenticated on purpose: this is exactly what Homebrew and your
# friends will do, so a private repo has to fail here rather than at their end.
if ! curl -fsSLI "$DMG_URL" >/dev/null 2>&1; then
  if gh release view "v$VERSION" --repo "$OWNER/windex" >/dev/null 2>&1; then
    die "release v$VERSION exists but $DMG_URL is not publicly readable.
       $OWNER/windex is private, so Homebrew cannot download the DMG. Make it
       public with:  gh repo edit $OWNER/windex --visibility public"
  fi
  die "no published DMG at $DMG_URL — run ./scripts/release.sh $VERSION first"
fi

info "Computing checksum"
SHA="$(curl -fsSL "$DMG_URL" | shasum -a 256 | awk '{print $1}')"

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

if gh repo view "$OWNER/$TAP_REPO" >/dev/null 2>&1; then
  info "Using existing $OWNER/$TAP_REPO"
  gh repo clone "$OWNER/$TAP_REPO" "$WORK/tap" -- --quiet
else
  info "Creating $OWNER/$TAP_REPO"
  gh repo create "$OWNER/$TAP_REPO" --public \
    --description "Homebrew tap for Grant's apps" --clone=false
  git init -q "$WORK/tap"
  git -C "$WORK/tap" remote add origin "https://github.com/$OWNER/$TAP_REPO.git"
  cat > "$WORK/tap/README.md" <<'TAPREADME'
# homebrew-tap

    brew install --cask soccergee/tap/windex
    xattr -dr com.apple.quarantine /Applications/Windex.app

Windex is not notarized and Homebrew quarantines every cask, so the second
line is what lets macOS open it. See https://github.com/SoccerGee/windex
TAPREADME
fi

mkdir -p "$WORK/tap/Casks"
sed -e "s/@VERSION@/$VERSION/g" -e "s/@SHA256@/$SHA/g" \
  "$REPO_DIR/packaging/homebrew/windex.rb.in" > "$WORK/tap/Casks/windex.rb"

cd "$WORK/tap"
git add -A
git commit -qm "windex $VERSION" || { info "Cask already current"; exit 0; }
git branch -M main
git push -q -u origin main

cat <<MSG

Tap published. Friends install with:

  brew install --cask soccergee/tap/windex

To let releases update the cask automatically, add a repo secret named
HOMEBREW_TAP_TOKEN to SoccerGee/windex — a fine-grained PAT with
"Contents: read and write" on $OWNER/$TAP_REPO:

  gh secret set HOMEBREW_TAP_TOKEN --repo $OWNER/windex
MSG
