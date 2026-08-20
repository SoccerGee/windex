<img src="docs/icon.png" alt="" width="96" align="left">

# Windex

**A macOS window manager with grid-based snapping and smooth animations.**
Written in Rust, lives in your menu bar, gets out of the way.

<br clear="left">

[![Release](https://github.com/SoccerGee/windex/actions/workflows/release.yml/badge.svg)](https://github.com/SoccerGee/windex/actions/workflows/release.yml)
[![Audit](https://github.com/SoccerGee/windex/actions/workflows/audit.yml/badge.svg)](https://github.com/SoccerGee/windex/actions/workflows/audit.yml)
[![Latest release](https://img.shields.io/github/v/release/SoccerGee/windex)](https://github.com/SoccerGee/windex/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

Snap the focused window to halves, thirds, and quarters of the screen — or shove
it to the next monitor — with global keyboard shortcuts. Movements are animated
with a short easing curve, so windows glide into place instead of jumping.

## Install

```sh
brew install --cask soccergee/tap/windex
xattr -dr com.apple.quarantine /Applications/Windex.app
```

Or grab `Windex-<version>.dmg` from
[Releases](https://github.com/SoccerGee/windex/releases/latest) and drag Windex
to Applications.

Requires macOS 11 or later. One universal build runs natively on both Apple
Silicon and Intel.

<details>
<summary><strong>Why that second command is needed</strong></summary>

Windex isn't notarized — that requires a paid Apple Developer account — and
macOS quarantines anything you download. Homebrew is no exception: it removed
its `--no-quarantine` option, so every cask arrives quarantined. Without
clearing the flag you'll get *"Apple could not verify Windex is free of
malware."*

If you hit that dialog, click **Done** (not *Move to Trash*), then either run
the `xattr` command above, or open **System Settings → Privacy & Security**,
scroll to **Security → Open Anyway**, and confirm. Either way it's once per
install.

On macOS 15 and later, the old right-click → **Open** trick no longer works.

</details>

### First launch

macOS asks for **Accessibility** access — Windex needs it to move other apps'
windows. Grant it under System Settings → Privacy & Security → Accessibility and
Windex restarts itself the moment you flip the switch. If the prompt doesn't
appear, Windex opens that settings pane for you, and the menu bar item's
**Accessibility Access…** entry reopens it any time.

Then turn on **Launch at Login** from the menu bar icon if you want it starting
with your Mac.

## Shortcuts

Everything uses <kbd>⌃</kbd><kbd>⌥</kbd> (Control+Option). **Keyboard
Shortcuts…** in the menu bar shows your live bindings, including any you've
remapped.

| Action | | Action | |
| --- | --- | --- | --- |
| Left half | <kbd>⌃⌥</kbd> <kbd>←</kbd> | Top-left corner | <kbd>⌃⌥</kbd> <kbd>U</kbd> |
| Right half | <kbd>⌃⌥</kbd> <kbd>→</kbd> | Top-right corner | <kbd>⌃⌥</kbd> <kbd>I</kbd> |
| Top half | <kbd>⌃⌥</kbd> <kbd>↑</kbd> | Bottom-left corner | <kbd>⌃⌥</kbd> <kbd>J</kbd> |
| Bottom half | <kbd>⌃⌥</kbd> <kbd>↓</kbd> | Bottom-right corner | <kbd>⌃⌥</kbd> <kbd>K</kbd> |
| Left third | <kbd>⌃⌥</kbd> <kbd>D</kbd> | Maximize | <kbd>⌃⌥</kbd> <kbd>Enter</kbd> |
| Center third | <kbd>⌃⌥</kbd> <kbd>F</kbd> | Center | <kbd>⌃⌥</kbd> <kbd>C</kbd> |
| Right third | <kbd>⌃⌥</kbd> <kbd>G</kbd> | Next monitor | <kbd>⌃⌥</kbd> <kbd>N</kbd> |
| Left two-thirds | <kbd>⌃⌥</kbd> <kbd>E</kbd> | Previous monitor | <kbd>⌃⌥</kbd> <kbd>P</kbd> |
| Right two-thirds | <kbd>⌃⌥</kbd> <kbd>T</kbd> | | |

## Configuration

**Edit Config…** opens `~/Library/Application Support/windex/config.toml`,
written with defaults on first run. Restart Windex to pick up changes.

```toml
[general]
launch_at_login = false

[hotkeys]
snap_left_half = "ctrl+alt+left"
snap_right_half = "ctrl+alt+right"
# ...one key per action in the table above

[animation]
duration_ms = 100
easing = "ease-out-cubic"
```

Bindings are `modifier+modifier+key`, using `ctrl`, `alt`, `shift`, and `cmd`.
Delete a line to unbind that action. `launch_at_login` mirrors the menu bar
toggle — either one manages the LaunchAgent at
`~/Library/LaunchAgents/com.granttuttle.windex.plist`.

An existing `config.toml` is never overwritten, so delete it if you want to pick
up new defaults after an upgrade.

**One caveat when remapping.** Windex watches the keyboard in listen-only mode,
which means it can't swallow a keystroke. If you bind something an app already
uses, both fire — <kbd>⌥⌘←</kbd> would snap the window *and* switch browser
tabs. The <kbd>⌃⌥</kbd> defaults were chosen because few apps claim them.

## Permissions and privacy

Windex asks for a lot of trust, so here's exactly what it uses and why:

- **Accessibility access** — the only way to move and resize other apps'
  windows on macOS.
- **A keyboard listener** — so shortcuts work in any app. It's registered
  listen-only: Windex can observe keystrokes but cannot alter, block, or inject
  them. It acts solely on the combinations in your config.

Windex makes **no network connections of any kind**. Nothing it reads leaves
your Mac. The log at `~/Library/Logs/windex.log` records window geometry and
which shortcut fired — never key contents. Dependencies are checked against the
[RustSec advisory database](https://rustsec.org) on every dependency change and
weekly; see the Audit badge above.

## Uninstall

```sh
brew uninstall --zap --cask windex
```

`--zap` also removes the config file, LaunchAgent, and log. For a manual
install, use `./install.sh uninstall`. Either way, you may want to delete the
leftover Windex entry under System Settings → Privacy & Security →
Accessibility.

## Building from source

Needs the [Rust toolchain](https://rustup.rs/).

```sh
make install    # build Windex.app and install it to /Applications
make app        # build dist/Windex.app (universal)
make dmg        # package dist/Windex-<version>.dmg
make run        # run in the foreground with debug logging
cargo test      # unit tests
```

`./scripts/build-app.sh --native` skips the Intel slice for faster iteration.
Builds are ad-hoc signed; set `CODESIGN_IDENTITY` to a Developer ID to sign
properly. Ad-hoc signatures change on every build, so macOS re-asks for
Accessibility permission after each install from source — release builds don't
have that problem.

```
src/accessibility/  Accessibility API wrapper — find and move windows
src/animation/      Easing and per-frame interpolation
src/app/            NSApplication event loop
src/config/         TOML config, written with defaults on first run
src/display/        Monitor enumeration and multi-display logic
src/hotkey/         Keyboard listener and binding parser
src/layout/         Grid math — where a window lands for each action
src/menu/           Menu bar item and the shortcuts cheat sheet
src/startup.rs      Launch-at-login LaunchAgent management
packaging/          Info.plist, icon renderer, cask template
scripts/            Build, package, release, and tap tooling
examples/           Keyboard probes: cargo run --example probe_rdev
```

## Releasing

```sh
./scripts/release.sh 0.2.0   # bump, tag, push — Actions builds and publishes
./scripts/setup-tap.sh       # point the Homebrew cask at the new release
```

The first command tags and pushes; GitHub Actions builds the universal app,
packages the DMG, and attaches it to the release. The second updates the cask in
[SoccerGee/homebrew-tap](https://github.com/SoccerGee/homebrew-tap), pinning the
new build's SHA256 so Homebrew verifies every download.

The release workflow can update the cask itself if a `HOMEBREW_TAP_TOKEN` secret
is set — a fine-grained PAT with *Contents: read and write* on the tap. It isn't
configured, deliberately: running `setup-tap.sh` takes seconds and avoids
keeping a long-lived credential around.

## License

[MIT](LICENSE) © Grant Tuttle
