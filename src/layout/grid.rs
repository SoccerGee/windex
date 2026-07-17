use super::action::LayoutAction;
use core_graphics::geometry::{CGPoint, CGRect, CGSize};

/// Calculate the target frame for a window based on the layout action and screen bounds
///
/// # Arguments
/// * `action` - The layout action to perform
/// * `screen` - The visible screen bounds (excluding menu bar and dock)
/// * `current_frame` - The current window frame (used for Center action to preserve size)
pub fn calculate_frame(action: LayoutAction, screen: CGRect, current_frame: CGRect) -> CGRect {
    match action {
        // Halves
        LayoutAction::SnapLeftHalf => left_half(screen),
        LayoutAction::SnapRightHalf => right_half(screen),
        LayoutAction::SnapTopHalf => top_half(screen),
        LayoutAction::SnapBottomHalf => bottom_half(screen),

        // Thirds
        LayoutAction::SnapLeftThird => left_third(screen),
        LayoutAction::SnapCenterThird => center_third(screen),
        LayoutAction::SnapRightThird => right_third(screen),
        LayoutAction::SnapLeftTwoThirds => left_two_thirds(screen),
        LayoutAction::SnapRightTwoThirds => right_two_thirds(screen),

        // Quarters
        LayoutAction::SnapTopLeft => top_left_quarter(screen),
        LayoutAction::SnapTopRight => top_right_quarter(screen),
        LayoutAction::SnapBottomLeft => bottom_left_quarter(screen),
        LayoutAction::SnapBottomRight => bottom_right_quarter(screen),

        // Special
        LayoutAction::Maximize => maximize(screen),
        LayoutAction::Center => center(screen, current_frame.size),

        // Monitor movement handled elsewhere
        LayoutAction::MoveToNextMonitor | LayoutAction::MoveToPreviousMonitor => current_frame,
    }
}

// Halves

fn left_half(screen: CGRect) -> CGRect {
    CGRect::new(
        &screen.origin,
        &CGSize::new(screen.size.width / 2.0, screen.size.height),
    )
}

fn right_half(screen: CGRect) -> CGRect {
    CGRect::new(
        &CGPoint::new(
            screen.origin.x + screen.size.width / 2.0,
            screen.origin.y,
        ),
        &CGSize::new(screen.size.width / 2.0, screen.size.height),
    )
}

fn top_half(screen: CGRect) -> CGRect {
    CGRect::new(
        &screen.origin,
        &CGSize::new(screen.size.width, screen.size.height / 2.0),
    )
}

fn bottom_half(screen: CGRect) -> CGRect {
    CGRect::new(
        &CGPoint::new(
            screen.origin.x,
            screen.origin.y + screen.size.height / 2.0,
        ),
        &CGSize::new(screen.size.width, screen.size.height / 2.0),
    )
}

// Thirds

fn left_third(screen: CGRect) -> CGRect {
    CGRect::new(
        &screen.origin,
        &CGSize::new(screen.size.width / 3.0, screen.size.height),
    )
}

fn center_third(screen: CGRect) -> CGRect {
    CGRect::new(
        &CGPoint::new(
            screen.origin.x + screen.size.width / 3.0,
            screen.origin.y,
        ),
        &CGSize::new(screen.size.width / 3.0, screen.size.height),
    )
}

fn right_third(screen: CGRect) -> CGRect {
    CGRect::new(
        &CGPoint::new(
            screen.origin.x + screen.size.width * 2.0 / 3.0,
            screen.origin.y,
        ),
        &CGSize::new(screen.size.width / 3.0, screen.size.height),
    )
}

fn left_two_thirds(screen: CGRect) -> CGRect {
    CGRect::new(
        &screen.origin,
        &CGSize::new(screen.size.width * 2.0 / 3.0, screen.size.height),
    )
}

fn right_two_thirds(screen: CGRect) -> CGRect {
    CGRect::new(
        &CGPoint::new(
            screen.origin.x + screen.size.width / 3.0,
            screen.origin.y,
        ),
        &CGSize::new(screen.size.width * 2.0 / 3.0, screen.size.height),
    )
}

// Quarters (corners)

fn top_left_quarter(screen: CGRect) -> CGRect {
    CGRect::new(
        &screen.origin,
        &CGSize::new(screen.size.width / 2.0, screen.size.height / 2.0),
    )
}

fn top_right_quarter(screen: CGRect) -> CGRect {
    CGRect::new(
        &CGPoint::new(
            screen.origin.x + screen.size.width / 2.0,
            screen.origin.y,
        ),
        &CGSize::new(screen.size.width / 2.0, screen.size.height / 2.0),
    )
}

fn bottom_left_quarter(screen: CGRect) -> CGRect {
    CGRect::new(
        &CGPoint::new(
            screen.origin.x,
            screen.origin.y + screen.size.height / 2.0,
        ),
        &CGSize::new(screen.size.width / 2.0, screen.size.height / 2.0),
    )
}

fn bottom_right_quarter(screen: CGRect) -> CGRect {
    CGRect::new(
        &CGPoint::new(
            screen.origin.x + screen.size.width / 2.0,
            screen.origin.y + screen.size.height / 2.0,
        ),
        &CGSize::new(screen.size.width / 2.0, screen.size.height / 2.0),
    )
}

// Special actions

fn maximize(screen: CGRect) -> CGRect {
    screen
}

fn center(screen: CGRect, size: CGSize) -> CGRect {
    CGRect::new(
        &CGPoint::new(
            screen.origin.x + (screen.size.width - size.width) / 2.0,
            screen.origin.y + (screen.size.height - size.height) / 2.0,
        ),
        &size,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_screen() -> CGRect {
        CGRect::new(&CGPoint::new(0.0, 0.0), &CGSize::new(1920.0, 1080.0))
    }

    fn test_window() -> CGRect {
        CGRect::new(&CGPoint::new(100.0, 100.0), &CGSize::new(800.0, 600.0))
    }

    #[test]
    fn test_left_half() {
        let result = calculate_frame(LayoutAction::SnapLeftHalf, test_screen(), test_window());
        assert_eq!(result.origin.x, 0.0);
        assert_eq!(result.origin.y, 0.0);
        assert_eq!(result.size.width, 960.0);
        assert_eq!(result.size.height, 1080.0);
    }

    #[test]
    fn test_right_half() {
        let result = calculate_frame(LayoutAction::SnapRightHalf, test_screen(), test_window());
        assert_eq!(result.origin.x, 960.0);
        assert_eq!(result.origin.y, 0.0);
        assert_eq!(result.size.width, 960.0);
        assert_eq!(result.size.height, 1080.0);
    }

    #[test]
    fn test_left_third() {
        let result = calculate_frame(LayoutAction::SnapLeftThird, test_screen(), test_window());
        assert_eq!(result.origin.x, 0.0);
        assert_eq!(result.size.width, 640.0);
    }

    #[test]
    fn test_center_third() {
        let result = calculate_frame(LayoutAction::SnapCenterThird, test_screen(), test_window());
        assert_eq!(result.origin.x, 640.0);
        assert_eq!(result.size.width, 640.0);
    }

    #[test]
    fn test_top_left_quarter() {
        let result = calculate_frame(LayoutAction::SnapTopLeft, test_screen(), test_window());
        assert_eq!(result.origin.x, 0.0);
        assert_eq!(result.origin.y, 0.0);
        assert_eq!(result.size.width, 960.0);
        assert_eq!(result.size.height, 540.0);
    }

    #[test]
    fn test_maximize() {
        let result = calculate_frame(LayoutAction::Maximize, test_screen(), test_window());
        assert_eq!(result.origin.x, 0.0);
        assert_eq!(result.origin.y, 0.0);
        assert_eq!(result.size.width, 1920.0);
        assert_eq!(result.size.height, 1080.0);
    }

    #[test]
    fn test_center() {
        let result = calculate_frame(LayoutAction::Center, test_screen(), test_window());
        // Window is 800x600, screen is 1920x1080
        // Center x: (1920 - 800) / 2 = 560
        // Center y: (1080 - 600) / 2 = 240
        assert_eq!(result.origin.x, 560.0);
        assert_eq!(result.origin.y, 240.0);
        assert_eq!(result.size.width, 800.0);
        assert_eq!(result.size.height, 600.0);
    }
}
