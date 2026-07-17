use crate::animation::Animator;
use crate::config::Config;
use crate::display::DisplayManager;
use crate::hotkey::HotkeyManager;
use anyhow::Result;

/// Application state container
pub struct AppState {
    pub config: Config,
    pub display_manager: DisplayManager,
    pub hotkey_manager: HotkeyManager,
    pub animator: Animator,
    pub should_quit: bool,
}

impl AppState {
    /// Create new application state from configuration
    pub fn new(config: Config) -> Result<Self> {
        let display_manager = DisplayManager::new();
        let mut hotkey_manager = HotkeyManager::new()?;
        hotkey_manager.register_from_config(&config.hotkeys)?;

        let animator = Animator::new(config.animation.duration_ms);

        Ok(Self {
            config,
            display_manager,
            hotkey_manager,
            animator,
            should_quit: false,
        })
    }

    /// Signal that the application should quit
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
