//! The "Keyboard Shortcuts…" cheat sheet.
//!
//! Rendered from the live config rather than hard-coded, so a friend who has
//! remapped anything sees their own bindings.

use crate::config::settings::HotkeyConfig;
use anyhow::Result;
use log::error;
use std::fmt::Write as _;

/// Write the cheat sheet to a temp file and open it in the default browser.
pub fn show(hotkeys: &HotkeyConfig) {
    match write_and_open(hotkeys) {
        Ok(()) => {}
        Err(e) => error!("Could not show shortcuts: {}", e),
    }
}

fn write_and_open(hotkeys: &HotkeyConfig) -> Result<()> {
    // Written next to the config rather than in a shared temp dir: a fixed
    // filename under a world-writable /tmp (which is what temp_dir() falls back
    // to when TMPDIR is unset) can be pre-planted as a symlink to any file this
    // process may write.
    let path = cheatsheet_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, render(hotkeys))?;
    std::process::Command::new("open").arg(&path).spawn()?;
    Ok(())
}

/// Where the generated cheat sheet lives — a directory only this user owns.
fn cheatsheet_path() -> Result<std::path::PathBuf> {
    let dir = crate::config::config_path()
        .parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| anyhow::anyhow!("could not locate the config directory"))?;
    Ok(dir.join("shortcuts.html"))
}

/// Escape text interpolated into the generated page.
///
/// Hotkey strings come from config.toml, which a user may well have been handed
/// by someone else; without escaping, a binding like `ctrl+alt+<img onerror=…>`
/// executes script when the cheat sheet opens.
fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

/// Rows in the order they read best, not the order they are declared.
fn rows(h: &HotkeyConfig) -> Vec<(&'static str, &'static str, &Option<String>)> {
    vec![
        ("Halves", "Left half", &h.snap_left_half),
        ("Halves", "Right half", &h.snap_right_half),
        ("Halves", "Top half", &h.snap_top_half),
        ("Halves", "Bottom half", &h.snap_bottom_half),
        ("Thirds", "Left third", &h.snap_left_third),
        ("Thirds", "Center third", &h.snap_center_third),
        ("Thirds", "Right third", &h.snap_right_third),
        ("Thirds", "Left two-thirds", &h.snap_left_two_thirds),
        ("Thirds", "Right two-thirds", &h.snap_right_two_thirds),
        ("Corners", "Top left", &h.snap_top_left),
        ("Corners", "Top right", &h.snap_top_right),
        ("Corners", "Bottom left", &h.snap_bottom_left),
        ("Corners", "Bottom right", &h.snap_bottom_right),
        ("Window", "Maximize", &h.maximize),
        ("Window", "Center", &h.center),
        ("Displays", "Next monitor", &h.move_to_next_monitor),
        ("Displays", "Previous monitor", &h.move_to_previous_monitor),
    ]
}

/// Turn `"ctrl+alt+left"` into `⌃ ⌥ ←` key caps.
fn key_caps(binding: &str) -> String {
    binding
        .split('+')
        .map(|part| match part.trim().to_ascii_lowercase().as_str() {
            "ctrl" | "control" => "⌃".to_string(),
            "alt" | "option" => "⌥".to_string(),
            "shift" => "⇧".to_string(),
            "cmd" | "command" | "meta" | "super" => "⌘".to_string(),
            "left" => "←".to_string(),
            "right" => "→".to_string(),
            "up" => "↑".to_string(),
            "down" => "↓".to_string(),
            "enter" | "return" => "⏎".to_string(),
            "space" => "Space".to_string(),
            other => other.to_uppercase(),
        })
        .map(|k| format!("<kbd>{}</kbd>", html_escape(&k)))
        .collect::<Vec<_>>()
        .join(" ")
}

fn render(h: &HotkeyConfig) -> String {
    let mut body = String::new();
    let mut current_group = "";

    for (group, label, binding) in rows(h) {
        let Some(binding) = binding else { continue };
        if group != current_group {
            let _ = write!(body, "<tr class=\"group\"><th colspan=\"2\">{}</th></tr>", group);
            current_group = group;
        }
        let _ = write!(
            body,
            "<tr><td>{}</td><td class=\"keys\">{}</td></tr>",
            label,
            key_caps(binding)
        );
    }

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Windex Shortcuts</title>
<style>
  :root {{ color-scheme: light dark; }}
  body {{
    font: 15px/1.5 -apple-system, BlinkMacSystemFont, "SF Pro Text", sans-serif;
    margin: 0; padding: 40px 24px; display: flex; justify-content: center;
    background: Canvas; color: CanvasText;
  }}
  main {{ width: 100%; max-width: 460px; }}
  h1 {{ font-size: 20px; margin: 0 0 4px; }}
  p.sub {{ margin: 0 0 24px; opacity: .6; font-size: 13px; }}
  table {{ width: 100%; border-collapse: collapse; }}
  th.group {{
    text-align: left; font-size: 11px; letter-spacing: .08em;
    text-transform: uppercase; opacity: .5; padding: 20px 0 6px;
  }}
  tr.group:first-child th {{ padding-top: 0; }}
  td {{ padding: 6px 0; border-top: 1px solid color-mix(in srgb, CanvasText 12%, transparent); }}
  td.keys {{ text-align: right; white-space: nowrap; }}
  kbd {{
    display: inline-block; min-width: 1.6em; padding: 3px 7px; font: inherit;
    font-size: 13px; text-align: center;
    background: color-mix(in srgb, CanvasText 8%, transparent);
    border: 1px solid color-mix(in srgb, CanvasText 18%, transparent);
    border-radius: 6px;
  }}
  footer {{ margin-top: 28px; font-size: 13px; opacity: .6; }}
  code {{ font-size: 12px; }}
</style>
</head>
<body>
<main>
  <h1>Windex {version}</h1>
  <p class="sub">Snap the focused window. Edit these in the menu bar under “Edit Config…”.</p>
  <table>{body}</table>
  <footer>Config file: <code>{config}</code></footer>
</main>
</body>
</html>
"#,
        version = env!("CARGO_PKG_VERSION"),
        body = body,
        config = html_escape(&crate::config::config_path().to_string_lossy()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_key_caps_as_symbols() {
        assert_eq!(
            key_caps("ctrl+alt+left"),
            "<kbd>⌃</kbd> <kbd>⌥</kbd> <kbd>←</kbd>"
        );
        assert_eq!(key_caps("ctrl+alt+d"), "<kbd>⌃</kbd> <kbd>⌥</kbd> <kbd>D</kbd>");
    }

    #[test]
    fn escapes_markup_from_config() {
        let mut hotkeys = HotkeyConfig::default();
        hotkeys.maximize = Some("ctrl+alt+<img src=x onerror=alert(1)>".to_string());
        let html = render(&hotkeys);
        assert!(
            !html.to_lowercase().contains("<img"),
            "config markup reached the page unescaped"
        );
        assert!(html.contains("&lt;IMG SRC=X ONERROR=ALERT(1)&gt;"));
    }

    #[test]
    fn escapes_ampersands_and_quotes() {
        assert_eq!(html_escape(r#"a&b<c>"d""#), "a&amp;b&lt;c&gt;&quot;d&quot;");
    }

    #[test]
    fn cheatsheet_lives_beside_the_config() {
        let path = cheatsheet_path().unwrap();
        assert_eq!(path.parent(), crate::config::config_path().parent());
        assert!(!path.starts_with("/tmp"));
    }

    #[test]
    fn omits_unbound_actions() {
        let mut hotkeys = HotkeyConfig::default();
        hotkeys.center = None;
        let html = render(&hotkeys);
        assert!(html.contains("Maximize"));
        assert!(!html.contains(">Center<"));
    }
}
