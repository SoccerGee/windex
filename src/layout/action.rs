use serde::{Deserialize, Serialize};

/// Layout actions that can be triggered by hotkeys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutAction {
    // Halves
    SnapLeftHalf,
    SnapRightHalf,
    SnapTopHalf,
    SnapBottomHalf,

    // Thirds
    SnapLeftThird,
    SnapCenterThird,
    SnapRightThird,
    SnapLeftTwoThirds,
    SnapRightTwoThirds,

    // Quarters (corners)
    SnapTopLeft,
    SnapTopRight,
    SnapBottomLeft,
    SnapBottomRight,

    // Special actions
    Maximize,
    Center,

    // Multi-monitor
    MoveToNextMonitor,
    MoveToPreviousMonitor,
}

impl LayoutAction {
    /// Returns all available layout actions
    pub fn all() -> &'static [LayoutAction] {
        use LayoutAction::*;
        &[
            SnapLeftHalf,
            SnapRightHalf,
            SnapTopHalf,
            SnapBottomHalf,
            SnapLeftThird,
            SnapCenterThird,
            SnapRightThird,
            SnapLeftTwoThirds,
            SnapRightTwoThirds,
            SnapTopLeft,
            SnapTopRight,
            SnapBottomLeft,
            SnapBottomRight,
            Maximize,
            Center,
            MoveToNextMonitor,
            MoveToPreviousMonitor,
        ]
    }

    /// Get the config key name for this action
    pub fn config_key(&self) -> &'static str {
        match self {
            LayoutAction::SnapLeftHalf => "snap_left_half",
            LayoutAction::SnapRightHalf => "snap_right_half",
            LayoutAction::SnapTopHalf => "snap_top_half",
            LayoutAction::SnapBottomHalf => "snap_bottom_half",
            LayoutAction::SnapLeftThird => "snap_left_third",
            LayoutAction::SnapCenterThird => "snap_center_third",
            LayoutAction::SnapRightThird => "snap_right_third",
            LayoutAction::SnapLeftTwoThirds => "snap_left_two_thirds",
            LayoutAction::SnapRightTwoThirds => "snap_right_two_thirds",
            LayoutAction::SnapTopLeft => "snap_top_left",
            LayoutAction::SnapTopRight => "snap_top_right",
            LayoutAction::SnapBottomLeft => "snap_bottom_left",
            LayoutAction::SnapBottomRight => "snap_bottom_right",
            LayoutAction::Maximize => "maximize",
            LayoutAction::Center => "center",
            LayoutAction::MoveToNextMonitor => "move_to_next_monitor",
            LayoutAction::MoveToPreviousMonitor => "move_to_previous_monitor",
        }
    }
}
