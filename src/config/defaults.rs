/// Default configuration values as a TOML string
pub const DEFAULT_CONFIG_TOML: &str = r#"
[general]
launch_at_login = false

[hotkeys]
# Halves - use arrow keys
snap_left_half = "ctrl+alt+left"
snap_right_half = "ctrl+alt+right"
snap_top_half = "ctrl+alt+up"
snap_bottom_half = "ctrl+alt+down"

# Thirds
snap_left_third = "ctrl+alt+d"
snap_center_third = "ctrl+alt+f"
snap_right_third = "ctrl+alt+g"
snap_left_two_thirds = "ctrl+alt+e"
snap_right_two_thirds = "ctrl+alt+t"

# Quarters (corners)
snap_top_left = "ctrl+alt+u"
snap_top_right = "ctrl+alt+i"
snap_bottom_left = "ctrl+alt+j"
snap_bottom_right = "ctrl+alt+k"

# Special actions
maximize = "ctrl+alt+enter"
center = "ctrl+alt+c"

# Multi-monitor
move_to_next_monitor = "ctrl+alt+n"
move_to_previous_monitor = "ctrl+alt+p"

[animation]
duration_ms = 100
easing = "ease-out-cubic"
"#;
