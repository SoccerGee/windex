# windex

A macOS window manager with grid-based snapping and smooth animations, written in Rust.

windex runs quietly in your menu bar and lets you snap the focused window to
halves, thirds, and quarters of the screen — or shove it to the next monitor —
with global keyboard shortcuts. Movements are animated with a short easing
curve so windows glide into place instead of jumping.

## Features

- **Halves** — snap left / right / top / bottom
- **Thirds** — left, center, right, plus left/right two-thirds
- **Quarters** — snap to any corner
- **Maximize** and **center** the focused window
- **Multi-monitor** — move a window to the next or previous display
- **Smooth animations** — configurable duration and easing
- **TOML config** — remap every hotkey
- **Menu bar tray icon** — lightweight, no dock icon
- **Launch at login** — optional LaunchAgent installer

## Requirements

- macOS
- [Rust toolchain](https://rustup.rs/) (to build from source)
- Accessibility permission (macOS will prompt on first launch — windex needs it
  to move and resize other apps' windows)

## Install

```sh
git clone https://github.com/SoccerGee/windex.git
cd windex
./install.sh
```

`install.sh` builds the release binary, installs it to `~/bin/windex`, and
registers a LaunchAgent so windex starts automatically at login. On first run
macOS will ask you to grant Accessibility permission — the installer opens the
right settings pane and walks you through it.

To remove it:

```sh
./install.sh uninstall
```

Or just build and run manually:

```sh
cargo build --release
./target/release/windex
```

## Default keybindings

All shortcuts use <kbd>Ctrl</kbd>+<kbd>Alt</kbd> (⌃⌥) as the modifier.

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

windex reads a TOML config file on launch (created with defaults on first run).
Every hotkey is remappable, and animation timing is adjustable:

```toml
[general]
launch_at_login = false

[hotkeys]
snap_left_half = "ctrl+alt+left"
snap_right_half = "ctrl+alt+right"
# ... see all keys below

[animation]
duration_ms = 100
easing = "ease-out-cubic"
```

Set any hotkey to remove it, or change the binding string to remap it. The full
list of hotkey keys matches the actions in the table above.

## Building

```sh
cargo build --release
```

The release profile enables LTO and `opt-level = 3` for a small, fast binary.

## License

[MIT](LICENSE) © Grant Tuttle
