// Simple test to verify global hotkeys work on this system

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use std::time::Duration;

fn main() {
    println!("Testing global hotkeys...");
    println!("This test will listen for Ctrl+Alt+A for 30 seconds.");
    println!("Press Ctrl+Alt+A (Control + Option + A) to test.");
    println!();

    // Create hotkey manager
    let manager = GlobalHotKeyManager::new().expect("Failed to create hotkey manager");

    // Register a simple hotkey: Ctrl+Alt+A
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyA);
    let hotkey_id = hotkey.id();

    manager.register(hotkey).expect("Failed to register hotkey");
    println!("Registered hotkey Ctrl+Alt+A (id: {})", hotkey_id);

    // Get the receiver
    let receiver = GlobalHotKeyEvent::receiver();

    println!("Listening for hotkey events...");

    // Listen for 30 seconds
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(30);

    loop {
        if start.elapsed() > timeout {
            println!("Timeout reached. No hotkey events received.");
            break;
        }

        // Check for events with a short timeout
        match receiver.try_recv() {
            Ok(event) => {
                println!("*** HOTKEY EVENT RECEIVED! ***");
                println!("Event ID: {}", event.id);
                println!("State: {:?}", event.state);
                if event.id == hotkey_id {
                    println!("This is our registered hotkey!");
                }
            }
            Err(_) => {
                // No event, sleep briefly
                std::thread::sleep(Duration::from_millis(100));
            }
        }

        // Run the event loop briefly to process Carbon events
        unsafe {
            core_foundation::runloop::CFRunLoopRunInMode(
                core_foundation::runloop::kCFRunLoopDefaultMode,
                0.01,
                false as u8,
            );
        }
    }

    println!("Test complete.");
}
