use crate::config::settings::HotkeyConfig;
use crate::layout::LayoutAction;
use anyhow::Result;
use log::{info, warn};
use rdev::{Event, EventType, Key};
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver};
use std::thread;

/// Manages keyboard listening and hotkey detection
pub struct HotkeyManager;

/// A combination of modifier keys and a main key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HotkeyCombo {
    ctrl: bool,
    alt: bool,
    shift: bool,
    cmd: bool,
    key: Key,
}

impl HotkeyManager {
    /// Register hotkeys from configuration and start the listener thread
    pub fn start_listener(config: &HotkeyConfig) -> Result<Receiver<LayoutAction>> {
        let (action_tx, action_rx) = mpsc::channel();

        // Build the hotkey bindings
        let mut bindings: HashMap<HotkeyCombo, LayoutAction> = HashMap::new();

        let hotkey_mappings = [
            (&config.snap_left_half, LayoutAction::SnapLeftHalf),
            (&config.snap_right_half, LayoutAction::SnapRightHalf),
            (&config.snap_top_half, LayoutAction::SnapTopHalf),
            (&config.snap_bottom_half, LayoutAction::SnapBottomHalf),
            (&config.snap_left_third, LayoutAction::SnapLeftThird),
            (&config.snap_center_third, LayoutAction::SnapCenterThird),
            (&config.snap_right_third, LayoutAction::SnapRightThird),
            (&config.snap_left_two_thirds, LayoutAction::SnapLeftTwoThirds),
            (&config.snap_right_two_thirds, LayoutAction::SnapRightTwoThirds),
            (&config.snap_top_left, LayoutAction::SnapTopLeft),
            (&config.snap_top_right, LayoutAction::SnapTopRight),
            (&config.snap_bottom_left, LayoutAction::SnapBottomLeft),
            (&config.snap_bottom_right, LayoutAction::SnapBottomRight),
            (&config.maximize, LayoutAction::Maximize),
            (&config.center, LayoutAction::Center),
            (&config.move_to_next_monitor, LayoutAction::MoveToNextMonitor),
            (
                &config.move_to_previous_monitor,
                LayoutAction::MoveToPreviousMonitor,
            ),
        ];

        for (hotkey_opt, action) in hotkey_mappings {
            if let Some(hotkey_str) = hotkey_opt {
                match parse_hotkey_combo(hotkey_str) {
                    Ok(combo) => {
                        info!("Registered hotkey '{}' for {:?}", hotkey_str, action);
                        bindings.insert(combo, action);
                    }
                    Err(e) => {
                        warn!("Failed to parse hotkey '{}': {}", hotkey_str, e);
                    }
                }
            }
        }

        // Spawn the listener thread
        info!("Starting keyboard listener thread...");
        thread::spawn(move || {
            info!("Keyboard listener thread started");
            let mut ctrl_pressed = false;
            let mut alt_pressed = false;
            let mut shift_pressed = false;
            let mut cmd_pressed = false;

            if let Err(e) = rdev::listen(move |event: Event| {
                match event.event_type {
                    EventType::KeyPress(key) => {
                        // Update modifier state
                        match key {
                            Key::ControlLeft | Key::ControlRight => ctrl_pressed = true,
                            Key::Alt | Key::AltGr => alt_pressed = true,
                            Key::ShiftLeft | Key::ShiftRight => shift_pressed = true,
                            Key::MetaLeft | Key::MetaRight => cmd_pressed = true,
                            _ => {
                                // Check if this key combo matches a hotkey
                                let combo = HotkeyCombo {
                                    ctrl: ctrl_pressed,
                                    alt: alt_pressed,
                                    shift: shift_pressed,
                                    cmd: cmd_pressed,
                                    key,
                                };

                                if let Some(&action) = bindings.get(&combo) {
                                    info!("Hotkey MATCH! Action: {:?}", action);
                                    let _ = action_tx.send(action);
                                }
                            }
                        }
                    }
                    EventType::KeyRelease(key) => {
                        // Update modifier state
                        match key {
                            Key::ControlLeft | Key::ControlRight => ctrl_pressed = false,
                            Key::Alt | Key::AltGr => alt_pressed = false,
                            Key::ShiftLeft | Key::ShiftRight => shift_pressed = false,
                            Key::MetaLeft | Key::MetaRight => cmd_pressed = false,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }) {
                warn!("Keyboard listener error: {:?}", e);
            }
            info!("Keyboard listener thread ended");
        });

        Ok(action_rx)
    }
}

/// Parse a hotkey string like "ctrl+alt+left" into a HotkeyCombo
fn parse_hotkey_combo(s: &str) -> Result<HotkeyCombo> {
    let lowered = s.to_lowercase();
    let parts: Vec<&str> = lowered.split('+').collect();

    if parts.is_empty() {
        anyhow::bail!("Empty hotkey string");
    }

    let mut ctrl = false;
    let mut alt = false;
    let mut shift = false;
    let mut cmd = false;

    // Parse modifiers (all parts except the last)
    for part in &parts[..parts.len() - 1] {
        match *part {
            "ctrl" | "control" => ctrl = true,
            "alt" | "option" | "opt" => alt = true,
            "shift" => shift = true,
            "cmd" | "command" | "meta" | "super" => cmd = true,
            other => anyhow::bail!("Unknown modifier: {}", other),
        }
    }

    // Parse the key (last part)
    let key_str = parts.last().unwrap();
    let key = parse_key(key_str)?;

    Ok(HotkeyCombo {
        ctrl,
        alt,
        shift,
        cmd,
        key,
    })
}

/// Parse a key name into an rdev Key
fn parse_key(key: &str) -> Result<Key> {
    Ok(match key {
        // Letters
        "a" => Key::KeyA,
        "b" => Key::KeyB,
        "c" => Key::KeyC,
        "d" => Key::KeyD,
        "e" => Key::KeyE,
        "f" => Key::KeyF,
        "g" => Key::KeyG,
        "h" => Key::KeyH,
        "i" => Key::KeyI,
        "j" => Key::KeyJ,
        "k" => Key::KeyK,
        "l" => Key::KeyL,
        "m" => Key::KeyM,
        "n" => Key::KeyN,
        "o" => Key::KeyO,
        "p" => Key::KeyP,
        "q" => Key::KeyQ,
        "r" => Key::KeyR,
        "s" => Key::KeyS,
        "t" => Key::KeyT,
        "u" => Key::KeyU,
        "v" => Key::KeyV,
        "w" => Key::KeyW,
        "x" => Key::KeyX,
        "y" => Key::KeyY,
        "z" => Key::KeyZ,

        // Numbers
        "0" => Key::Num0,
        "1" => Key::Num1,
        "2" => Key::Num2,
        "3" => Key::Num3,
        "4" => Key::Num4,
        "5" => Key::Num5,
        "6" => Key::Num6,
        "7" => Key::Num7,
        "8" => Key::Num8,
        "9" => Key::Num9,

        // Arrow keys
        "left" | "arrowleft" => Key::LeftArrow,
        "right" | "arrowright" => Key::RightArrow,
        "up" | "arrowup" => Key::UpArrow,
        "down" | "arrowdown" => Key::DownArrow,

        // Special keys
        "enter" | "return" => Key::Return,
        "space" => Key::Space,
        "backspace" => Key::Backspace,
        "tab" => Key::Tab,
        "escape" | "esc" => Key::Escape,
        "delete" => Key::Delete,

        // Function keys
        "f1" => Key::F1,
        "f2" => Key::F2,
        "f3" => Key::F3,
        "f4" => Key::F4,
        "f5" => Key::F5,
        "f6" => Key::F6,
        "f7" => Key::F7,
        "f8" => Key::F8,
        "f9" => Key::F9,
        "f10" => Key::F10,
        "f11" => Key::F11,
        "f12" => Key::F12,

        other => anyhow::bail!("Unknown key: {}", other),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::settings::HotkeyConfig;

    /// Every shipped default must parse — an unparseable binding is only a
    /// warning at runtime, so the shortcut would silently do nothing.
    #[test]
    fn every_default_binding_parses() {
        let config = HotkeyConfig::default();
        let defaults = [
            &config.snap_left_half,
            &config.snap_right_half,
            &config.snap_top_half,
            &config.snap_bottom_half,
            &config.snap_left_third,
            &config.snap_center_third,
            &config.snap_right_third,
            &config.snap_left_two_thirds,
            &config.snap_right_two_thirds,
            &config.snap_top_left,
            &config.snap_top_right,
            &config.snap_bottom_left,
            &config.snap_bottom_right,
            &config.maximize,
            &config.center,
            &config.move_to_next_monitor,
            &config.move_to_previous_monitor,
        ];

        let mut seen = Vec::new();
        for binding in defaults {
            let binding = binding.as_ref().expect("default bindings are all set");
            let combo = parse_hotkey_combo(binding)
                .unwrap_or_else(|e| panic!("default binding {binding:?} does not parse: {e}"));
            assert!(
                !seen.contains(&combo),
                "two defaults share the combo {binding:?}"
            );
            seen.push(combo);
        }
    }

    #[test]
    fn parses_a_full_modifier_stack() {
        let combo = parse_hotkey_combo("ctrl+shift+cmd+right").unwrap();
        assert!(combo.ctrl && combo.shift && combo.cmd && !combo.alt);
        assert_eq!(combo.key, Key::RightArrow);
    }
}
