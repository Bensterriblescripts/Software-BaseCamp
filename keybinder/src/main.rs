use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers, KeyEventKind};


fn main() {

    // Executables
    let edge = "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe";
    // Folders
    let local_repo = "C:\\Local\\repo";
    // Arguments
    let edge_profile_personal = "Default";
    let edge_profile_work = "Profile 1";
    // Keybinds
    let bind_edge_personal = KeyCode::Char('q');
    let bind_edge_work = KeyCode::Char('w');
    let bind_folder_local = KeyCode::Char('a');

    // Crossterm
    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");
    let (tx, rx) = mpsc::channel();
    // Keyboard input Thread
    thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(100)).expect("Failed to poll events") {
                if let event::Event::Key(KeyEvent { code, modifiers, state, kind }) = event::read().expect("Failed to read event") {
                    tx.send((code, modifiers, state, kind)).expect("Failed to send key event");
                }
            }
        }
    });
    // Main thread
    loop {
        if let Ok((code, modifiers, state, kind)) = rx.try_recv() {
            println!("Key: {:?}, Modifiers: {:?}, State: {:?}, Kind: {:?}", code, modifiers, state, kind);
            if code == KeyCode::Esc {
                break;
            }
            // Edge - Work
            else if code == bind_edge_personal && modifiers.contains (KeyModifiers::ALT) && kind == KeyEventKind::Press {
                let app = edge;
                let arg = &format!("--profile-directory={}", edge_profile_personal);
                open_application(app, arg);
                thread::sleep(Duration::from_millis(100));
            }
            // Edge - Personal
            else if code == bind_edge_work && modifiers.contains (KeyModifiers::ALT) && kind == KeyEventKind::Press {
                let app = edge;
                let arg = &format!("--profile-directory={}", edge_profile_work);
                open_application(app, arg);
                thread::sleep(Duration::from_millis(100));
            }
            // Folder - Local
            else if code == bind_folder_local && modifiers.contains (KeyModifiers::ALT) && kind == KeyEventKind::Press {
                let folder = local_repo;
                open_folder(folder);
                thread::sleep(Duration::from_millis(100));
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

fn open_application<'a>(app: &'a str, arg: &str) -> &'a str<> {
    let output = Command::new(app)
    .arg(arg)
    .output();

    return app;
}
fn open_folder(folder: &str) -> &str {
    let output = Command::new("explorer")
        .arg(folder)
        .output();

    return folder;
}
