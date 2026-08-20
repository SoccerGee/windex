use anyhow::Result;
use log::info;

mod accessibility;
mod animation;
mod app;
mod config;
mod display;
mod hotkey;
mod layout;
mod menu;
mod startup;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting Windex {}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = config::load_config()?;
    info!("Configuration loaded");

    // Keep the login item in step with the config file
    startup::sync_with_config(config.general.launch_at_login);

    // Ask for accessibility permission if we don't have it. This does not block:
    // the menu bar icon comes up either way, and Windex relaunches itself once
    // permission is granted.
    if !accessibility::permissions::check_accessibility_permission() {
        accessibility::permissions::request_accessibility_permission()?;
    }

    // Run the application
    app::run(config)?;

    Ok(())
}
