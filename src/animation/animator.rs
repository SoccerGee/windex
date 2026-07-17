use super::transition::{TransitionState, WindowTransition};
use crate::accessibility::Window;
use core_graphics::geometry::CGRect;
use log::warn;
use std::time::Duration;

/// Coordinates window animations
pub struct Animator {
    /// Currently running transitions
    active_transitions: Vec<WindowTransition>,
    /// Animation duration
    duration: Duration,
}

impl Animator {
    /// Create a new animator with the specified animation duration
    pub fn new(duration_ms: u64) -> Self {
        Self {
            active_transitions: Vec::new(),
            duration: Duration::from_millis(duration_ms),
        }
    }

    /// Start animating a window to a target frame
    pub fn animate(&mut self, window: Window, start_frame: CGRect, target_frame: CGRect) {
        let transition = WindowTransition::new(window, start_frame, target_frame, self.duration);
        self.active_transitions.push(transition);
    }

    /// Process one frame of animation, returns true if there are still active animations
    pub fn tick(&mut self) -> bool {
        self.active_transitions.retain_mut(|transition| {
            match transition.tick() {
                TransitionState::InProgress(frame) => {
                    if let Err(e) = transition.window.set_frame(frame) {
                        warn!("Failed to set window frame during animation: {}", e);
                    }
                    true // Keep animating
                }
                TransitionState::Complete(frame) => {
                    if let Err(e) = transition.window.set_frame(frame) {
                        warn!("Failed to set final window frame: {}", e);
                    }
                    false // Remove from active
                }
            }
        });

        !self.active_transitions.is_empty()
    }

    /// Check if there are any active animations
    pub fn is_animating(&self) -> bool {
        !self.active_transitions.is_empty()
    }

    /// Immediately complete all animations (used for cleanup)
    pub fn complete_all(&mut self) {
        for transition in self.active_transitions.drain(..) {
            if let Err(e) = transition.window.set_frame(transition.end_frame) {
                warn!("Failed to set final window frame: {}", e);
            }
        }
    }
}

impl Default for Animator {
    fn default() -> Self {
        Self::new(100) // 100ms default
    }
}
