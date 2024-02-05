use std::env;
use std::fs;
use std::process::Command;

use powershell_script;
use serde::Deserialize;
use winapi::um::winuser::GetAsyncKeyState;

// use winit::event_loop::EventLoop;

fn main() {

<<<<<<< Updated upstream
    // Winit
    // let event_loop = EventLoop::new

    let edge_profile_paths = get_edgeprofiles();
    let edge_profile_metadata = get_edgeprofile_data(edge_profile_paths);


=======
>>>>>>> Stashed changes
    // Applications
    let edge = "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe";
    // Folders
    let local_repo = "C:\\Local";
    // Script
    let script_cleanup = include_str!("configure_windows.ps1");
    // Arguments
    let edge_profile_personal = "--profile-directory=Default";
    let edge_profile_work = "--profile-directory=Profile 4";
    // Keybinds
    let _edge_personal = ["alt", "q"];
    let _edge_work = ["alt", "q"];

    // Core Keylogger
    loop {
        let enter = unsafe { GetAsyncKeyState(0x0D) };
        let alt = unsafe { GetAsyncKeyState(0x12) };
        let q = unsafe { GetAsyncKeyState(0x51) };
        let w = unsafe { GetAsyncKeyState(0x57) };
        let _e = unsafe { GetAsyncKeyState(0x45) };
        let _r = unsafe { GetAsyncKeyState(0x52) };
        let a = unsafe { GetAsyncKeyState(0x41) };
        let c = unsafe { GetAsyncKeyState(0x43) };
        let n = unsafe { GetAsyncKeyState(0x4E) };
        let m = unsafe { GetAsyncKeyState(0x4D) };

        if alt != 0 && q != 0 {
            run_application(edge, edge_profile_personal);
            println!("Running: Edge-Personal");
            std::thread::sleep(std::time::Duration::from_millis(150));
        }
        else if alt != 0 && w != 0 {
            run_application(edge, edge_profile_work);
            println!("Running: Edge-Work");
            std::thread::sleep(std::time::Duration::from_millis(150));
        }
        else if alt != 0 && a != 0 {
            open_folder(local_repo);
            println!("Opened: Local Folder");
            std::thread::sleep(std::time::Duration::from_millis(150));
        }
        else if alt != 0 && c != 0 && n != 0 && m != 0 && enter != 0 {
            println!("Running: Windows Configuration Script");
            run_powershell(script_cleanup);
            std::thread::sleep(std::time::Duration::from_millis(150));
        }
        
        // Lets not overload the CPU
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

}

// Executing/Launching
fn run_application<'a>(app: &'a str, arg: &str) -> &'a str<> {
    let _output = Command::new(app)
    .arg(arg)
    .output();

    return app;
}
fn run_powershell(script: &str) {
    match powershell_script::run(script) {
        Ok(output) => {
            println!("{}", output);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
fn open_folder(folder: &str) -> &str {
    let _output = Command::new("explorer")
        .arg(folder)
        .output();

    return folder;
}

// Edge
fn get_edgeprofiles() -> Vec<String> {

    let local_appdata = env::var("LocalAppData").unwrap();
    let edge_path = format!("{}\\Microsoft\\Edge\\User Data", local_appdata);
    let mut profiles: Vec<String> = Vec::new();
    let mut profile_paths: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(edge_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    if let Some(folder_name) = entry.file_name().to_str() {
                        let folder_name_string = folder_name.to_string();
                        if folder_name_string.contains("Profile") {
                            profiles.push(folder_name_string);
                        }

                    }
                }
            }
        }
    }
    else {
        println!("Error reading directory.");
    }
    for profile in profiles {
        println!("Found new profile: {}", profile);
        let profile_path = format!("{}\\Microsoft\\Edge\\User Data\\{}", local_appdata, profile);
        profile_paths.push(profile_path);
    }
    return profile_paths;
}
fn get_edgeprofile_data(profile_paths: Vec<String>) -> Vec<String> {

    let profile_data: Vec<String> = Vec::new();

    for profile in profile_paths {
        
    }

    return profile_data;
}