// Test keyboard listening using rdev crate

use rdev::{listen, Event, EventType, Key};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Testing keyboard capture with rdev...");
    println!("Press any key to test. Press Ctrl+C to exit.");
    println!("Specifically try: Ctrl+Alt+A");
    println!();

    // Track modifier state
    let mut ctrl_pressed = false;
    let mut alt_pressed = false;

    // Listen for keyboard events
    if let Err(error) = listen(move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                println!("Key pressed: {:?}", key);

                match key {
                    Key::ControlLeft | Key::ControlRight => ctrl_pressed = true,
                    Key::Alt | Key::AltGr => alt_pressed = true,
                    Key::KeyA => {
                        if ctrl_pressed && alt_pressed {
                            println!("*** CTRL+ALT+A DETECTED! ***");
                        }
                    }
                    _ => {}
                }
            }
            EventType::KeyRelease(key) => {
                match key {
                    Key::ControlLeft | Key::ControlRight => ctrl_pressed = false,
                    Key::Alt | Key::AltGr => alt_pressed = false,
                    _ => {}
                }
            }
            _ => {}
        }
    }) {
        println!("Error listening: {:?}", error);
    }
}
