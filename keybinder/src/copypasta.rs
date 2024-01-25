
use std::thread;
use std::time::Duration;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};

fn main() {
    // Set terminal to raw mode
    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");

    // Create a channel for communication between threads
    let (tx, rx) = std::sync::mpsc::channel();

    // Spawn a thread to constantly check for keyboard input
    thread::spawn(move || {
        loop {
            // Check for keyboard events
            if event::poll(Duration::from_millis(100)).expect("Failed to poll events") {
                if let event::Event::Key(KeyEvent { code, modifiers, state }) = event::read().expect("Failed to read event") {
                    // Send the key event to the main thread
                    tx.send((code, modifiers, state)).expect("Failed to send key event");
                }
            }
        }
    });

    // Main thread
    loop {
        // Check if there is a key event
        if let Ok((code, modifiers, state)) = rx.try_recv() {
            // Handle the key event
            println!("Key: {:?}, Modifiers: {:?}, State: {:?}", code, modifiers, state);

            // Example: Exit the loop if the 'Esc' key is pressed
            if code == KeyCode::Esc {
                break;
            }
        }

        // Perform other tasks in the main thread
        // ...

        // Sleep for a short duration to avoid high CPU usage
        thread::sleep(Duration::from_millis(10));
    }

    // Disable raw mode before exiting
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
}
