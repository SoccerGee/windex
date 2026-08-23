use super::monitor::Monitor;
use core_graphics::geometry::CGPoint;

/// Manages display/monitor operations
pub struct DisplayManager {
    monitors: Vec<Monitor>,
}

impl DisplayManager {
    /// Create a new display manager
    pub fn new() -> Self {
        Self {
            monitors: Monitor::all(),
        }
    }

    /// Refresh the list of monitors (call when displays change)
    pub fn refresh(&mut self) {
        self.monitors = Monitor::all();
    }

    #[allow(dead_code)] // part of the type's API; not called yet
    /// Get all monitors
    pub fn monitors(&self) -> &[Monitor] {
        &self.monitors
    }

    /// Get the primary monitor
    pub fn primary(&self) -> Option<&Monitor> {
        self.monitors.iter().find(|m| m.is_primary)
    }

    /// Find the monitor containing a given point
    pub fn monitor_at(&self, point: CGPoint) -> Option<&Monitor> {
        self.monitors.iter().find(|m| {
            point.x >= m.frame.origin.x
                && point.x < m.frame.origin.x + m.frame.size.width
                && point.y >= m.frame.origin.y
                && point.y < m.frame.origin.y + m.frame.size.height
        })
    }

    /// Get the next monitor (wraps around)
    ///
    /// Returns `None` when no displays are active — enumeration can legitimately
    /// come back empty, and the wrap-around arithmetic would otherwise panic.
    pub fn next_monitor(&self, current_id: u32) -> Option<&Monitor> {
        let len = self.monitors.len();
        if len == 0 {
            return None;
        }
        let idx = self
            .monitors
            .iter()
            .position(|m| m.id == current_id)
            .unwrap_or(0);
        self.monitors.get((idx + 1) % len)
    }

    /// Get the previous monitor (wraps around)
    ///
    /// Returns `None` when no displays are active. See `next_monitor`.
    pub fn previous_monitor(&self, current_id: u32) -> Option<&Monitor> {
        let len = self.monitors.len();
        if len == 0 {
            return None;
        }
        let idx = self
            .monitors
            .iter()
            .position(|m| m.id == current_id)
            .unwrap_or(0);
        let prev_idx = if idx == 0 { len - 1 } else { idx - 1 };
        self.monitors.get(prev_idx)
    }
}

impl Default for DisplayManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both of these panicked before the length guards: a divide-by-zero in
    /// `next_monitor` and a subtract-overflow in `previous_monitor`.
    #[test]
    fn wrap_around_handles_no_displays() {
        let manager = DisplayManager {
            monitors: Vec::new(),
        };
        assert!(manager.next_monitor(0).is_none());
        assert!(manager.previous_monitor(0).is_none());
        assert!(manager.primary().is_none());
    }
}
