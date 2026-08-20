use core_graphics::display::{CGDisplay, CGMainDisplayID};
use log::warn;
use core_graphics::geometry::{CGPoint, CGRect, CGSize};

/// Represents a physical display/monitor
#[derive(Debug, Clone)]
pub struct Monitor {
    /// The Core Graphics display ID
    pub id: u32,
    /// The full frame of the display in global coordinates
    pub frame: CGRect,
    /// The visible frame (excluding menu bar and dock)
    pub visible_frame: CGRect,
    /// Whether this is the primary display
    pub is_primary: bool,
}

impl Monitor {
    /// Get all connected monitors
    pub fn all() -> Vec<Monitor> {
        let max_displays = 16u32;
        let mut display_ids = vec![0u32; max_displays as usize];
        let mut display_count = 0u32;

        let err = unsafe {
            core_graphics::display::CGGetActiveDisplayList(
                max_displays,
                display_ids.as_mut_ptr(),
                &mut display_count,
            )
        };

        if err != 0 {
            warn!("CGGetActiveDisplayList failed (error {err}); no displays enumerated");
            display_count = 0;
        }

        // Never trust the reported count past the buffer we handed over.
        display_ids.truncate((display_count as usize).min(max_displays as usize));
        let main_display_id = unsafe { CGMainDisplayID() };

        display_ids
            .into_iter()
            .map(|id| {
                let display = CGDisplay::new(id);
                let frame = display.bounds();

                // Calculate visible frame (approximation - we'll improve this later)
                let visible_frame = Self::calculate_visible_frame(id, frame);

                Monitor {
                    id,
                    frame,
                    visible_frame,
                    is_primary: id == main_display_id,
                }
            })
            .collect()
    }

    #[allow(dead_code)] // part of the type's API; not called yet
    /// Get the primary monitor
    pub fn primary() -> Option<Monitor> {
        Self::all().into_iter().find(|m| m.is_primary)
    }

    #[allow(dead_code)] // part of the type's API; not called yet
    /// Find the monitor containing a given point
    pub fn containing_point(point: CGPoint) -> Option<Monitor> {
        Self::all().into_iter().find(|m| {
            point.x >= m.frame.origin.x
                && point.x < m.frame.origin.x + m.frame.size.width
                && point.y >= m.frame.origin.y
                && point.y < m.frame.origin.y + m.frame.size.height
        })
    }

    /// Calculate the visible frame for a display
    /// This excludes the menu bar and dock
    fn calculate_visible_frame(display_id: u32, frame: CGRect) -> CGRect {
        // Use NSScreen to get the actual visible frame
        // For now, we'll approximate by subtracting menu bar height
        // TODO: Use NSScreen.visibleFrame for accurate measurements

        let main_id = unsafe { CGMainDisplayID() };
        let is_primary = display_id == main_id;

        // Menu bar is only on the primary display (typically ~25 pixels)
        // Dock can be on any edge, but we'll assume bottom for now
        let menu_bar_height = if is_primary { 25.0 } else { 0.0 };

        CGRect::new(
            &CGPoint::new(frame.origin.x, frame.origin.y + menu_bar_height),
            &CGSize::new(frame.size.width, frame.size.height - menu_bar_height),
        )
    }
}
