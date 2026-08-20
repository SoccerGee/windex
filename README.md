# Windex

A macOS window manager with grid-based snapping and smooth animations, written in Rust.

Windex lives in your menu bar and snaps the focused window to halves, thirds, and
quarters of the screen — or shoves it to the next monitor — with global keyboard
shortcuts. Movements are animated with a short easing curve so windows glide into
place instead of jumping.

## Install

```sh
brew install --cask soccergee/tap/windex
xattr -dr com.apple.quarantine /Applications/Windex.app
```

Or download `Windex-<version>.dmg` from
[Releases](https://github.com/SoccerGee/windex/releases) and drag Windex to
Applications.

**That second line matters.** Windex isn't notarized (that needs a paid Apple
Developer account), and macOS quarantines anything you download — Homebrew
included, since it no longer offers a `--no-quarantine` option. Without
clearing the flag you'll get *"Apple could not verify Windex is free of
malware."*

If you hit that dialog, click **Done** (not Move to Trash), then either run the
`xattr` command above, or go to **System Settings → Privacy & Security →**
scroll to **Security → Open Anyway**, authenticate, and confirm. Either way it's
a one-time step.

On macOS 15 and later the old right-click → **Open** trick no longer works.

### First launch

macOS will ask for **Accessibility** access — Windex needs it to move other
apps' windows. Grant it in System Settings → Privacy & Security → Accessibility;
Windex restarts itself as soon as you flip the switch.

Then click the menu bar icon and turn on **Launch at Login** if you want it
starting with your Mac.

Requires macOS 11 or later. Runs natively on both Apple Silicon and Intel.

## Default shortcuts

All shortcuts use <kbd>⌃</kbd><kbd>⌥</kbd> (Ctrl+Alt) as the modifier. The menu
bar item's **Keyboard Shortcuts…** entry shows your live bindings, including any
you've remapped.

| Action | Shortcut |
| --- | --- |
| Left half | <kbd>⌃⌥</kbd> <kbd>←</kbd> |
| Right half | <kbd>⌃⌥</kbd> <kbd>→</kbd> |
| Top half | <kbd>⌃⌥</kbd> <kbd>↑</kbd> |
| Bottom half | <kbd>⌃⌥</kbd> <kbd>↓</kbd> |
| Left third | <kbd>⌃⌥</kbd> <kbd>D</kbd> |
| Center third | <kbd>⌃⌥</kbd> <kbd>F</kbd> |
| Right third | <kbd>⌃⌥</kbd> <kbd>G</kbd> |
| Left two-thirds | <kbd>⌃⌥</kbd> <kbd>E</kbd> |
| Right two-thirds | <kbd>⌃⌥</kbd> <kbd>T</kbd> |
| Top-left corner | <kbd>⌃⌥</kbd> <kbd>U</kbd> |
| Top-right corner | <kbd>⌃⌥</kbd> <kbd>I</kbd> |
| Bottom-left corner | <kbd>⌃⌥</kbd> <kbd>J</kbd> |
| Bottom-right corner | <kbd>⌃⌥</kbd> <kbd>K</kbd> |
| Maximize | <kbd>⌃⌥</kbd> <kbd>Enter</kbd> |
| Center | <kbd>⌃⌥</kbd> <kbd>C</kbd> |
| Move to next monitor | <kbd>⌃⌥</kbd> <kbd>N</kbd> |
| Move to previous monitor | <kbd>⌃⌥</kbd> <kbd>P</kbd> |

## Configuration

**Edit Config…** in the menu bar opens
`~/Library/Application Support/windex/config.toml`, written with defaults on
first run. Restart Windex to pick up changes.

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

Defaults are only written on first run — an existing config.toml is never
overwritten, so delete it if you want to pick up new defaults after an upgrade.
Remove a hotkey line to unbind that action. `launch_at_login` mirrors the menu
bar toggle — either one manages the LaunchAgent at
`~/Library/LaunchAgents/com.granttuttle.windex.plist`.

## Uninstall

```sh
brew uninstall --cask windex && brew uninstall --zap --cask windex
```

Or, for a manual install: `./install.sh uninstall`. Either way you may also want
to delete the Windex entry under System Settings → Privacy & Security →
Accessibility.

## What Windex can see

Windex needs two things that are worth understanding before you install it:

- **Accessibility access**, so it can move and resize other apps' windows.
- **A keyboard listener**, so global shortcuts work anywhere. It is registered
  in listen-only mode — Windex can observe keystrokes but cannot alter or
  swallow them — and it only ever acts on the combinations in your config.

Windex makes no network connections of any kind, and nothing it reads leaves
your Mac. Its log (`~/Library/Logs/windex.log`) records window geometry and
which shortcut fired, never key contents. The source is here if you'd rather
check than take my word for it.

## Building from source

Needs the [Rust toolchain](https://rustup.rs/).

```sh
make install      # build Windex.app and install it to /Applications
make app          # just build dist/Windex.app (universal binary)
make dmg          # package dist/Windex-<version>.dmg
make run          # run in the foreground with debug logging
```

`scripts/build-app.sh --native` skips the Intel slice for faster iteration.
Builds are ad-hoc signed by default; set `CODESIGN_IDENTITY` to a Developer ID
to sign properly. Note that ad-hoc signatures change on every build, so macOS
re-asks for Accessibility permission after each install from source.

## Releasing

```sh
./scripts/release.sh 0.2.0
```

That bumps the version, tags, and pushes. GitHub Actions builds the universal
app, attaches the DMG to the release, and — once `HOMEBREW_TAP_TOKEN` is set as
a repo secret — updates the Homebrew cask. `scripts/setup-tap.sh` creates the
tap the first time.

## License

[MIT](LICENSE) © Grant Tuttle
