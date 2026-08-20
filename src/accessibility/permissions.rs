use anyhow::Result;
use log::{info, warn};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

/// Check if the application has accessibility permission
pub fn check_accessibility_permission() -> bool {
    let trusted = macos_accessibility_client::accessibility::application_is_trusted();
    if trusted {
        info!("Accessibility permission granted");
    } else {
        warn!("Accessibility permission not granted");
    }
    trusted
}

/// Ask for accessibility permission without blocking startup.
///
/// Shows the system prompt, then watches for the grant in the background. The
/// keyboard listener can only attach once permission exists, so Windex relaunches
/// itself the moment the user flips the switch — otherwise they would have to
/// quit and reopen it by hand.
pub fn request_accessibility_permission() -> Result<()> {
    info!("Requesting accessibility permission…");
    macos_accessibility_client::accessibility::application_is_trusted_with_prompt();

    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(2));
            if macos_accessibility_client::accessibility::application_is_trusted() {
                info!("Accessibility permission granted — relaunching");
                relaunch();
                return;
            }
        }
    });

    Ok(())
}

/// Restart the process so it picks up the newly granted permission.
fn relaunch() {
    let spawned = match app_bundle_path() {
        // `open -n` gives the new instance a fresh app context rather than
        // reusing this process's (still unprivileged) session.
        Some(bundle) => std::process::Command::new("open")
            .arg("-n")
            .arg(&bundle)
            .spawn()
            .is_ok(),
        None => std::env::current_exe()
            .ok()
            .and_then(|exe| std::process::Command::new(exe).spawn().ok())
            .is_some(),
    };

    if !spawned {
        warn!("Could not relaunch automatically — please quit and reopen Windex");
        return;
    }

    std::process::exit(0);
}

/// The enclosing `.app` bundle, when running from one.
fn app_bundle_path() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    // …/Windex.app/Contents/MacOS/windex
    let bundle = exe.parent()?.parent()?.parent()?;
    (bundle.extension()? == "app").then(|| bundle.to_path_buf())
}
