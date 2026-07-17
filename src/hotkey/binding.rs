use crate::layout::LayoutAction;
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HotkeyParseError {
    #[error("Invalid hotkey format: {0}")]
    InvalidFormat(String),
    #[error("Unknown modifier: {0}")]
    UnknownModifier(String),
    #[error("Unknown key: {0}")]
    UnknownKey(String),
}

/// A binding between a hotkey string and a layout action
#[derive(Debug, Clone)]
pub struct HotkeyBinding {
    pub hotkey: HotKey,
    pub action: LayoutAction,
}

impl HotkeyBinding {
    /// Create a new hotkey binding from a string and action
    pub fn new(hotkey_str: &str, action: LayoutAction) -> Result<Self, HotkeyParseError> {
        let hotkey = parse_hotkey(hotkey_str)?;
        Ok(Self { hotkey, action })
    }
}

/// Parse a hotkey string like "ctrl+alt+left" into a HotKey
pub fn parse_hotkey(s: &str) -> Result<HotKey, HotkeyParseError> {
    let lowered = s.to_lowercase();
    let parts: Vec<&str> = lowered.split('+').collect();
    if parts.is_empty() {
        return Err(HotkeyParseError::InvalidFormat(s.to_string()));
    }

    let mut modifiers = Modifiers::empty();
    let key_part = parts.last().ok_or_else(|| HotkeyParseError::InvalidFormat(s.to_string()))?;

    // Parse modifiers (all parts except the last)
    for part in &parts[..parts.len() - 1] {
        match *part {
            "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
            "alt" | "option" | "opt" => modifiers |= Modifiers::ALT,
            "shift" => modifiers |= Modifiers::SHIFT,
            "cmd" | "command" | "meta" | "super" => modifiers |= Modifiers::META,
            other => return Err(HotkeyParseError::UnknownModifier(other.to_string())),
        }
    }

    // Parse the key
    let code = parse_key_code(key_part)?;

    Ok(HotKey::new(Some(modifiers), code))
}

/// Parse a key name into a Code
fn parse_key_code(key: &str) -> Result<Code, HotkeyParseError> {
    match key {
        // Letters
        "a" => Ok(Code::KeyA),
        "b" => Ok(Code::KeyB),
        "c" => Ok(Code::KeyC),
        "d" => Ok(Code::KeyD),
        "e" => Ok(Code::KeyE),
        "f" => Ok(Code::KeyF),
        "g" => Ok(Code::KeyG),
        "h" => Ok(Code::KeyH),
        "i" => Ok(Code::KeyI),
        "j" => Ok(Code::KeyJ),
        "k" => Ok(Code::KeyK),
        "l" => Ok(Code::KeyL),
        "m" => Ok(Code::KeyM),
        "n" => Ok(Code::KeyN),
        "o" => Ok(Code::KeyO),
        "p" => Ok(Code::KeyP),
        "q" => Ok(Code::KeyQ),
        "r" => Ok(Code::KeyR),
        "s" => Ok(Code::KeyS),
        "t" => Ok(Code::KeyT),
        "u" => Ok(Code::KeyU),
        "v" => Ok(Code::KeyV),
        "w" => Ok(Code::KeyW),
        "x" => Ok(Code::KeyX),
        "y" => Ok(Code::KeyY),
        "z" => Ok(Code::KeyZ),

        // Numbers
        "0" => Ok(Code::Digit0),
        "1" => Ok(Code::Digit1),
        "2" => Ok(Code::Digit2),
        "3" => Ok(Code::Digit3),
        "4" => Ok(Code::Digit4),
        "5" => Ok(Code::Digit5),
        "6" => Ok(Code::Digit6),
        "7" => Ok(Code::Digit7),
        "8" => Ok(Code::Digit8),
        "9" => Ok(Code::Digit9),

        // Arrow keys
        "left" | "arrowleft" => Ok(Code::ArrowLeft),
        "right" | "arrowright" => Ok(Code::ArrowRight),
        "up" | "arrowup" => Ok(Code::ArrowUp),
        "down" | "arrowdown" => Ok(Code::ArrowDown),

        // Special keys
        "enter" | "return" => Ok(Code::Enter),
        "space" => Ok(Code::Space),
        "backspace" => Ok(Code::Backspace),
        "tab" => Ok(Code::Tab),
        "escape" | "esc" => Ok(Code::Escape),
        "delete" => Ok(Code::Delete),

        // Function keys
        "f1" => Ok(Code::F1),
        "f2" => Ok(Code::F2),
        "f3" => Ok(Code::F3),
        "f4" => Ok(Code::F4),
        "f5" => Ok(Code::F5),
        "f6" => Ok(Code::F6),
        "f7" => Ok(Code::F7),
        "f8" => Ok(Code::F8),
        "f9" => Ok(Code::F9),
        "f10" => Ok(Code::F10),
        "f11" => Ok(Code::F11),
        "f12" => Ok(Code::F12),

        other => Err(HotkeyParseError::UnknownKey(other.to_string())),
    }
}
