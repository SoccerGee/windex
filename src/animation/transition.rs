use crate::accessibility::Window;
use core_graphics::geometry::{CGPoint, CGRect, CGSize};
use std::time::{Duration, Instant};

/// Represents an in-progress window animation
pub struct WindowTransition {
    pub window: Window,
    pub start_frame: CGRect,
    pub end_frame: CGRect,
    pub start_time: Instant,
    pub duration: Duration,
}

/// The state of a window transition
pub enum TransitionState {
    /// Animation is still in progress, with the current interpolated frame
    InProgress(CGRect),
    /// Animation is complete, with the final frame
    Complete(CGRect),
}

impl WindowTransition {
    /// Create a new window transition
    pub fn new(window: Window, start_frame: CGRect, end_frame: CGRect, duration: Duration) -> Self {
        Self {
            window,
            start_frame,
            end_frame,
            start_time: Instant::now(),
            duration,
        }
    }

    /// Calculate the current state of the transition
    pub fn tick(&self) -> TransitionState {
        let elapsed = self.start_time.elapsed();

        if elapsed >= self.duration {
            return TransitionState::Complete(self.end_frame);
        }

        let t = elapsed.as_secs_f64() / self.duration.as_secs_f64();
        let eased_t = ease_out_cubic(t);

        let frame = interpolate_rect(self.start_frame, self.end_frame, eased_t);
        TransitionState::InProgress(frame)
    }
}

/// Ease-out cubic easing function for natural deceleration
fn ease_out_cubic(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

/// Linearly interpolate between two rectangles
fn interpolate_rect(start: CGRect, end: CGRect, t: f64) -> CGRect {
    CGRect::new(
        &CGPoint::new(
            lerp(start.origin.x, end.origin.x, t),
            lerp(start.origin.y, end.origin.y, t),
        ),
        &CGSize::new(
            lerp(start.size.width, end.size.width, t),
            lerp(start.size.height, end.size.height, t),
        ),
    )
}

/// Linear interpolation between two values
fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}
