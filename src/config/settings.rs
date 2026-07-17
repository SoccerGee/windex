use serde::{Deserialize, Serialize};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub hotkeys: HotkeyConfig,
    #[serde(default)]
    pub animation: AnimationConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            hotkeys: HotkeyConfig::default(),
            animation: AnimationConfig::default(),
        }
    }
}

/// General application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_true")]
    pub launch_at_login: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            launch_at_login: false,
        }
    }
}

/// Hotkey bindings configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    // Halves
    pub snap_left_half: Option<String>,
    pub snap_right_half: Option<String>,
    pub snap_top_half: Option<String>,
    pub snap_bottom_half: Option<String>,

    // Thirds
    pub snap_left_third: Option<String>,
    pub snap_center_third: Option<String>,
    pub snap_right_third: Option<String>,
    pub snap_left_two_thirds: Option<String>,
    pub snap_right_two_thirds: Option<String>,

    // Quarters
    pub snap_top_left: Option<String>,
    pub snap_top_right: Option<String>,
    pub snap_bottom_left: Option<String>,
    pub snap_bottom_right: Option<String>,

    // Special
    pub maximize: Option<String>,
    pub center: Option<String>,

    // Multi-monitor
    pub move_to_next_monitor: Option<String>,
    pub move_to_previous_monitor: Option<String>,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            // Halves - arrow keys
            snap_left_half: Some("ctrl+alt+left".to_string()),
            snap_right_half: Some("ctrl+alt+right".to_string()),
            snap_top_half: Some("ctrl+alt+up".to_string()),
            snap_bottom_half: Some("ctrl+alt+down".to_string()),

            // Thirds
            snap_left_third: Some("ctrl+alt+d".to_string()),
            snap_center_third: Some("ctrl+alt+f".to_string()),
            snap_right_third: Some("ctrl+alt+g".to_string()),
            snap_left_two_thirds: Some("ctrl+alt+e".to_string()),
            snap_right_two_thirds: Some("ctrl+alt+t".to_string()),

            // Quarters
            snap_top_left: Some("ctrl+alt+u".to_string()),
            snap_top_right: Some("ctrl+alt+i".to_string()),
            snap_bottom_left: Some("ctrl+alt+j".to_string()),
            snap_bottom_right: Some("ctrl+alt+k".to_string()),

            // Special
            maximize: Some("ctrl+alt+enter".to_string()),
            center: Some("ctrl+alt+c".to_string()),

            // Multi-monitor
            move_to_next_monitor: Some("ctrl+alt+n".to_string()),
            move_to_previous_monitor: Some("ctrl+alt+p".to_string()),
        }
    }
}

/// Animation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    #[serde(default = "default_duration")]
    pub duration_ms: u64,
    #[serde(default = "default_easing")]
    pub easing: String,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration_ms: default_duration(),
            easing: default_easing(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_duration() -> u64 {
    100
}

fn default_easing() -> String {
    "ease-out-cubic".to_string()
}
