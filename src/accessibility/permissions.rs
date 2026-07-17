use anyhow::{anyhow, Result};
use log::{info, warn};
use std::thread;
use std::time::{Duration, Instant};

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

/// Request accessibility permission from the user
/// This will show a system dialog prompting the user to grant permission
pub fn request_accessibility_permission() -> Result<()> {
    info!("Requesting accessibility permission...");

    // Trigger the system permission prompt
    macos_accessibility_client::accessibility::application_is_trusted_with_prompt();

    // Poll for permission (user needs to go to System Settings)
    let start = Instant::now();
    let timeout = Duration::from_secs(120); // 2 minute timeout
    let poll_interval = Duration::from_secs(1);

    info!("Please grant accessibility permission in System Settings...");
    info!("Waiting for permission (timeout: {:?})...", timeout);

    while start.elapsed() < timeout {
        if macos_accessibility_client::accessibility::application_is_trusted() {
            info!("Accessibility permission granted!");
            return Ok(());
        }
        thread::sleep(poll_interval);
    }

    Err(anyhow!(
        "Accessibility permission not granted within timeout. \
         Please grant permission in System Settings > Privacy & Security > Accessibility"
    ))
}
