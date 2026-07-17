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
    pub fn next_monitor(&self, current_id: u32) -> Option<&Monitor> {
        let idx = self
            .monitors
            .iter()
            .position(|m| m.id == current_id)
            .unwrap_or(0);
        let next_idx = (idx + 1) % self.monitors.len();
        self.monitors.get(next_idx)
    }

    /// Get the previous monitor (wraps around)
    pub fn previous_monitor(&self, current_id: u32) -> Option<&Monitor> {
        let idx = self
            .monitors
            .iter()
            .position(|m| m.id == current_id)
            .unwrap_or(0);
        let prev_idx = if idx == 0 {
            self.monitors.len() - 1
        } else {
            idx - 1
        };
        self.monitors.get(prev_idx)
    }
}

impl Default for DisplayManager {
    fn default() -> Self {
        Self::new()
    }
}
