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

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting Windex window manager");

    // Check accessibility permissions first
    if !accessibility::permissions::check_accessibility_permission() {
        accessibility::permissions::request_accessibility_permission()?;
    }

    // Load configuration
    let config = config::load_config()?;
    info!("Configuration loaded");

    // Run the application
    app::run(config)?;

    Ok(())
}
