use core_graphics::display::{CGDisplay, CGMainDisplayID};
use log::warn;
use core_graphics::geometry::{CGPoint, CGRect, CGSize};
use objc2::runtime::{AnyObject, NSObject, NSObjectProtocol};
use objc2_app_kit::NSScreen;
use objc2_foundation::{MainThreadMarker, NSNumber, NSString};

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
        let mtm = MainThreadMarker::new();

        display_ids
            .into_iter()
            .map(|id| {
                let display = CGDisplay::new(id);
                let frame = display.bounds();

                let visible_frame = mtm
                    .and_then(|mtm| Self::visible_frame_from_nsscreen(id, mtm))
                    .unwrap_or_else(|| Self::approximate_visible_frame(id, frame));

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

    /// Get the display's usable area from its NSScreen — `visibleFrame`
    /// excludes the menu bar (including the notch cutout) and the Dock,
    /// per display, wherever the Dock is pinned.
    fn visible_frame_from_nsscreen(display_id: u32, mtm: MainThreadMarker) -> Option<CGRect> {
        let key = NSString::from_str("NSScreenNumber");
        for screen in NSScreen::screens(mtm).iter() {
            let desc = screen.deviceDescription();
            let Some(value) = desc.get(&key) else { continue };
            // Documented to be an NSNumber holding the CGDirectDisplayID.
            let value_obj = unsafe { &*(value as *const AnyObject as *const NSObject) };
            if !value_obj.is_kind_of::<NSNumber>() {
                continue;
            }
            let number = unsafe { &*(value as *const AnyObject as *const NSNumber) };
            if number.as_u32() != display_id {
                continue;
            }

            // Cocoa rects are y-up from the bottom-left of the primary
            // display; Core Graphics (and the Accessibility API) are y-down
            // from its top-left. Flip around the primary display's height.
            let visible = screen.visibleFrame();
            let primary_height = CGDisplay::main().bounds().size.height;
            return Some(CGRect::new(
                &CGPoint::new(
                    visible.origin.x,
                    primary_height - (visible.origin.y + visible.size.height),
                ),
                &CGSize::new(visible.size.width, visible.size.height),
            ));
        }
        None
    }

    /// Fallback when the NSScreen lookup isn't possible (not on the main
    /// thread, or no screen matched the display ID): subtract a nominal
    /// menu bar from the primary display and ignore the Dock.
    fn approximate_visible_frame(display_id: u32, frame: CGRect) -> CGRect {
        let main_id = unsafe { CGMainDisplayID() };
        let is_primary = display_id == main_id;
        let menu_bar_height = if is_primary { 25.0 } else { 0.0 };

        CGRect::new(
            &CGPoint::new(frame.origin.x, frame.origin.y + menu_bar_height),
            &CGSize::new(frame.size.width, frame.size.height - menu_bar_height),
        )
    }
}
