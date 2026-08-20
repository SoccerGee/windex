use crate::accessibility::Window;
use crate::animation::Animator;
use crate::config::Config;
use crate::display::{DisplayManager, Monitor};
use crate::hotkey::HotkeyManager;
use crate::layout::{calculate_frame, LayoutAction};
use crate::menu::{self, MenuAction};
use anyhow::Result;
use core_graphics::geometry::CGRect;
use log::{error, info, warn};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSEventMask};
use objc2_foundation::{MainThreadMarker, NSDate, NSDefaultRunLoopMode};
use std::sync::mpsc::Receiver;

/// Run the main application event loop
pub fn run(config: Config) -> Result<()> {
    info!("Starting event loop");

    // We must be on the main thread for NSApplication
    let mtm = MainThreadMarker::new().expect("Must run on main thread");

    // Initialize NSApplication (required for tray icon)
    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);

    unsafe {
        app.finishLaunching();
    }

    // Create display manager
    let display_manager = DisplayManager::new();

    // Create animator
    let mut animator = Animator::new(config.animation.duration_ms);

    // Start hotkey listener (returns a receiver for actions)
    let hotkey_rx = HotkeyManager::start_listener(&config.hotkeys)?;

    // Build tray icon
    let tray = menu::build_tray(crate::startup::is_enabled())?;
    info!("Tray icon created");

    // Get menu event receiver
    let menu_rx = menu::tray::menu_receiver();

    info!("Entering main event loop");

    let mut should_quit = false;

    // Main loop
    while !should_quit {
        // Pump the NSApplication event queue so the tray icon renders AND its
        // mouse/menu events are dispatched. The first call blocks for up to
        // ~16ms waiting for an event (this also paces the loop); we then drain
        // any remaining queued events without blocking.
        unsafe {
            let deadline = NSDate::dateWithTimeIntervalSinceNow(0.016);
            let mut next: Option<&NSDate> = Some(&deadline);
            while let Some(event) = app.nextEventMatchingMask_untilDate_inMode_dequeue(
                NSEventMask::Any,
                next,
                NSDefaultRunLoopMode,
                true,
            ) {
                app.sendEvent(&event);
                // After the first (blocking) wait, drain the rest immediately.
                next = None;
            }
        }

        // Process hotkey events from rdev listener thread
        while let Ok(action) = hotkey_rx.try_recv() {
            info!("Hotkey action received: {:?}", action);
            execute_action(&display_manager, &mut animator, action);
        }

        // Process menu events
        while let Ok(event) = menu_rx.try_recv() {
            let action = menu::tray::event_to_action(&event);
            match action {
                MenuAction::Settings => {
                    info!("Settings requested");
                    open_config_file();
                }
                MenuAction::LaunchAtLogin => {
                    // The checkbox has already flipped itself; apply the new
                    // state, and put it back if registration failed.
                    let desired = tray.launch_at_login_checked();
                    info!("Launch at login toggled to {}", desired);
                    if let Err(e) = crate::startup::set_enabled(desired) {
                        error!("Could not change launch-at-login: {}", e);
                        tray.set_launch_at_login(!desired);
                    }
                }
                MenuAction::About => {
                    info!("Shortcuts requested");
                    menu::shortcuts::show(&config.hotkeys);
                }
                MenuAction::Quit => {
                    info!("Quit requested");
                    should_quit = true;
                }
                MenuAction::Unknown => {}
            }
        }

        // Process animations
        if animator.is_animating() {
            animator.tick();
        }
    }

    info!("Shutting down");
    animator.complete_all();

    Ok(())
}

/// Execute a layout action
fn execute_action(display_manager: &DisplayManager, animator: &mut Animator, action: LayoutAction) {
    info!("Executing action: {:?}", action);

    // Get the focused window
    let window = match Window::focused() {
        Ok(w) => {
            info!("Got focused window");
            w
        }
        Err(e) => {
            warn!("Could not get focused window: {}", e);
            return;
        }
    };

    // Get the current window frame
    let current_frame = match window.frame() {
        Ok(f) => {
            info!("Current frame: {:?}", f);
            f
        }
        Err(e) => {
            warn!("Could not get window frame: {}", e);
            return;
        }
    };

    // Handle multi-monitor actions specially
    match action {
        LayoutAction::MoveToNextMonitor => {
            move_to_next_monitor(display_manager, animator, window, current_frame);
            return;
        }
        LayoutAction::MoveToPreviousMonitor => {
            move_to_previous_monitor(display_manager, animator, window, current_frame);
            return;
        }
        _ => {}
    }

    // Find the monitor containing the window
    let monitor = display_manager
        .monitor_at(current_frame.origin)
        .or_else(|| display_manager.primary());

    let screen = match monitor {
        Some(m) => {
            info!("Found monitor, visible frame: {:?}", m.visible_frame);
            m.visible_frame
        }
        None => {
            warn!("Could not find monitor for window");
            return;
        }
    };

    // Calculate the target frame
    let target_frame = calculate_frame(action, screen, current_frame);
    info!("Target frame: {:?}", target_frame);

    // Animate to the target
    animator.animate(window, current_frame, target_frame);
}

/// Move window to the next monitor
fn move_to_next_monitor(
    display_manager: &DisplayManager,
    animator: &mut Animator,
    window: Window,
    current_frame: CGRect,
) {
    let current_monitor = display_manager.monitor_at(current_frame.origin);
    let current_id = current_monitor.map(|m| m.id).unwrap_or(0);

    if let Some(next_monitor) = display_manager.next_monitor(current_id) {
        let target_frame = calculate_frame_for_monitor(current_frame, next_monitor);
        animator.animate(window, current_frame, target_frame);
    }
}

/// Move window to the previous monitor
fn move_to_previous_monitor(
    display_manager: &DisplayManager,
    animator: &mut Animator,
    window: Window,
    current_frame: CGRect,
) {
    let current_monitor = display_manager.monitor_at(current_frame.origin);
    let current_id = current_monitor.map(|m| m.id).unwrap_or(0);

    if let Some(prev_monitor) = display_manager.previous_monitor(current_id) {
        let target_frame = calculate_frame_for_monitor(current_frame, prev_monitor);
        animator.animate(window, current_frame, target_frame);
    }
}

/// Calculate window frame for a target monitor
fn calculate_frame_for_monitor(current_frame: CGRect, target_monitor: &Monitor) -> CGRect {
    let target = target_monitor.visible_frame;

    let x = target.origin.x + (target.size.width - current_frame.size.width) / 2.0;
    let y = target.origin.y + (target.size.height - current_frame.size.height) / 2.0;

    CGRect::new(
        &core_graphics::geometry::CGPoint::new(x, y),
        &current_frame.size,
    )
}

/// Open the configuration file in the default editor
fn open_config_file() {
    let path = crate::config::config_path();
    if let Err(e) = std::process::Command::new("open").arg(&path).spawn() {
        error!("Failed to open config file: {}", e);
    }
}
