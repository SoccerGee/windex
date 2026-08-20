//! Launch-at-login support.
//!
//! Manages a per-user LaunchAgent pointing at the running executable, so the
//! toggle keeps working whether Windex was installed to /Applications or is
//! being run straight out of the build directory.

use anyhow::{Context, Result};
use log::{info, warn};
use std::path::PathBuf;
use std::process::Command;

const LABEL: &str = "com.granttuttle.windex";

/// Path of the LaunchAgent plist for the current user.
pub fn plist_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join("Library/LaunchAgents")
        .join(format!("{}.plist", LABEL))
}

/// Whether Windex is currently registered to launch at login.
pub fn is_enabled() -> bool {
    plist_path().exists()
}

/// Register or unregister the login item.
pub fn set_enabled(enabled: bool) -> Result<()> {
    if enabled {
        enable()
    } else {
        disable()
    }
}

fn service_target() -> String {
    // SAFETY: getuid() is always safe to call.
    format!("gui/{}", unsafe { libc::getuid() })
}

fn enable() -> Result<()> {
    let exe = std::env::current_exe().context("could not determine executable path")?;
    let path = plist_path();

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let plist = plist_body(&exe.to_string_lossy(), &log_path().to_string_lossy());

    std::fs::write(&path, plist).with_context(|| format!("writing {}", path.display()))?;

    // Never hand launchd a plist we can't parse ourselves — if escaping ever
    // regressed, a malformed file is safer deleted than registered.
    if !is_valid_plist(&path) {
        let _ = std::fs::remove_file(&path);
        anyhow::bail!(
            "generated LaunchAgent was not a valid plist (executable path: {})",
            exe.display()
        );
    }

    // Register it now so a fresh install doesn't need a logout/login cycle.
    // The agent is already running (we are it), so bootstrap failing because
    // the service exists is expected and harmless.
    let _ = Command::new("launchctl")
        .args(["bootstrap", &service_target()])
        .arg(&path)
        .output();

    info!("Launch at login enabled ({})", path.display());
    Ok(())
}

/// Whether `plutil` can parse the file we just wrote.
fn is_valid_plist(path: &std::path::Path) -> bool {
    Command::new("plutil")
        .arg("-lint")
        .arg(path)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Render the LaunchAgent plist for a given executable and log path.
fn plist_body(exe: &str, log: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{label}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{exe}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
    <key>ProcessType</key>
    <string>Interactive</string>
    <key>StandardOutPath</key>
    <string>{log}</string>
    <key>StandardErrorPath</key>
    <string>{log}</string>
</dict>
</plist>
"#,
        label = LABEL,
        exe = xml_escape(exe),
        log = xml_escape(log),
    )
}

fn disable() -> Result<()> {
    let path = plist_path();

    let _ = Command::new("launchctl")
        .args(["bootout", &format!("{}/{}", service_target(), LABEL)])
        .output();

    if path.exists() {
        std::fs::remove_file(&path).with_context(|| format!("removing {}", path.display()))?;
    }

    info!("Launch at login disabled");
    Ok(())
}

/// Escape text for inclusion in an XML element.
///
/// Path components may legally contain `<`, `>`, `&` and quotes. Interpolating
/// them raw lets a crafted directory name close the `<string>` element and
/// inject arbitrary `ProgramArguments` — i.e. run anything at login.
fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            // Control characters are illegal in XML 1.0 even when escaped.
            c if (c as u32) < 0x20 && c != '\t' && c != '\n' && c != '\r' => {}
            c => out.push(c),
        }
    }
    out
}

fn log_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join("Library/Logs/windex.log")
}

/// Bring the on-disk login item in line with the configured preference.
///
/// Runs at startup so `launch_at_login` in config.toml is authoritative, and
/// so editing the config file has the same effect as using the menu.
pub fn sync_with_config(desired: bool) {
    if is_enabled() == desired {
        return;
    }
    if let Err(e) = set_enabled(desired) {
        warn!("Could not update launch-at-login state: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The payload below produced a *valid* plist whose ProgramArguments were
    /// fully attacker-controlled, before paths were escaped.
    #[test]
    fn escapes_plist_breakout_attempt() {
        let evil = "x</string></array><key>ProgramArguments</key>\
                    <array><string>/bin/sh</string><string>-c</string>\
                    <string>touch /tmp/pwned</string><string>";
        let escaped = xml_escape(evil);
        assert!(!escaped.contains('<'), "unescaped '<' survived: {}", escaped);
        assert!(!escaped.contains('>'), "unescaped '>' survived: {}", escaped);
        assert!(escaped.contains("&lt;/string&gt;"));
        assert!(escaped.contains("&amp;") == evil.contains('&'));
    }

    /// End-to-end: the exploit path must produce a plist that macOS parses as
    /// exactly one program argument, with no injected keys.
    #[test]
    fn generated_plist_survives_a_hostile_executable_path() {
        let evil_dir = "x</string></array><key>ProgramArguments</key>\
                        <array><string>/bin/sh</string><string>-c</string>\
                        <string>touch /tmp/windex-pwned</string><string>";
        let exe = format!("/Users/tester/{}/Windex.app/Contents/MacOS/windex", evil_dir);
        let body = plist_body(&exe, "/Users/tester/Library/Logs/windex.log");

        let path = std::env::temp_dir().join("windex-startup-test.plist");
        std::fs::write(&path, body).unwrap();

        assert!(is_valid_plist(&path), "generated plist did not parse");

        let args = Command::new("plutil")
            .args(["-extract", "ProgramArguments", "json", "-o", "-"])
            .arg(&path)
            .output()
            .unwrap();
        // plutil's JSON escapes forward slashes as \/, so compare unescaped.
        let args = String::from_utf8_lossy(&args.stdout).replace("\\/", "/");
        let args = args.trim();

        // Exactly one argument, the whole hostile path taken literally — not
        // the four-element ["/bin/sh", "-c", …] array the injection produced.
        assert_eq!(
            args,
            format!("[\"{}\"]", exe),
            "ProgramArguments was not a single literal path"
        );

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn escapes_each_significant_character() {
        assert_eq!(xml_escape(r#"a&b<c>d"e'f"#), "a&amp;b&lt;c&gt;d&quot;e&apos;f");
    }

    #[test]
    fn strips_control_characters() {
        assert_eq!(xml_escape("ok\u{0}\u{1}here"), "okhere");
        assert_eq!(xml_escape("tab\there"), "tab\there");
    }

    #[test]
    fn leaves_ordinary_paths_untouched() {
        let path = "/Applications/Windex.app/Contents/MacOS/windex";
        assert_eq!(xml_escape(path), path);
    }
}
