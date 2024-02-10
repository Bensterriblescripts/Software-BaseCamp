use std::env;
use std::fs;
use std::process::Command;

use powershell_script;
use winapi::um::winuser::GetAsyncKeyState;

// use winit::event_loop::EventLoop;

fn main() {

    let edge_profile_paths = get_edgeprofiles();
    let _edge_profile_metadata = get_edgeprofile_data(edge_profile_paths);

    // Core Variables
    let mut keys_active = true;

    // Applications
    let edge = "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe";
    let steam = "C:\\Program Files (x86)\\Steam\\steam.exe";
    let discord = "C:\\Users\\coffe\\AppData\\Local\\Discord\\app-1.0.9032\\Discord.exe";

    // Folders
    let local_repo = "C:\\Local";

    // Script
    let script_cleanup = include_str!("configure_windows.ps1");

    // Arguments
    let edge_profile_personal = "--profile-directory=Default";
    let edge_personal_arg2 = "";
    let mut edge_personal_open = false;

    let edge_profile_work = "--profile-directory=Profile 4";
    let edge_work_arg2 = env::var("TenantLoginPage").unwrap().as_str().to_owned();
    let mut edge_work_open = false;

    // Auto-Open Work Browser Links
    let _edge_work_sharepoint = "https://sparknz.sharepoint.com/";
    let _edge_work_outlook = "https://outlook.office.com/mail/";
    let _edge_work_citrix = env::var("Citrix").unwrap();

    // Core Keylogger
    loop {
        let enter = unsafe { GetAsyncKeyState(0x0D) };
        let alt = unsafe { GetAsyncKeyState(0x12) };
        let left_ctrl = unsafe { GetAsyncKeyState(0x11) };
        let a = unsafe { GetAsyncKeyState(0x41) };
        let c = unsafe { GetAsyncKeyState(0x43) };
        let d = unsafe { GetAsyncKeyState(0x44) };
        let e = unsafe { GetAsyncKeyState(0x45) };
        let f = unsafe { GetAsyncKeyState(0x46) };
        let m = unsafe { GetAsyncKeyState(0x4D) };
        let n = unsafe { GetAsyncKeyState(0x4E) };
        let q = unsafe { GetAsyncKeyState(0x51) };
        let _r = unsafe { GetAsyncKeyState(0x52) };
        let s = unsafe { GetAsyncKeyState(0x53) };
        let w = unsafe { GetAsyncKeyState(0x57) };

        // Keybinds
        if keys_active {

            // Edge - Personal
            if alt != 0 && q != 0 && !edge_personal_open {
                run_application(edge, edge_profile_personal, &edge_personal_arg2);
                edge_personal_open = true;
                println!("Running: Edge-Personal");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }
            else if alt != 0 && q != 0 && edge_personal_open {
                println!("Personal profile is already open.");
            }
            else if alt != 0 && e != 0 {
                let edge_personal_arg2 = "--inprivate";
                run_application(edge, edge_profile_personal, &edge_personal_arg2);
                edge_personal_open = true;
                println!("Running: Edge-Personal-Private");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }

            // Edge - Work
            if alt != 0 && w != 0 && !edge_work_open {
                run_application(edge, edge_profile_work, &edge_work_arg2);
                edge_work_open = true;
                println!("Running: Edge-Work");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }
            else if alt != 0 && w != 0 && edge_work_open {
                println!("Work profile is already open.")
            }

            // Steam
            if alt != 0 && s != 0 {
                run_application(steam, "", "");
                println!("Opened: Steam");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }

            // Discord
            if alt != 0 && d != 0 {
                run_application(discord, "", "");
                println!("Opened: Discord");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }

            // Folder
            if alt != 0 && a != 0 {
                open_folder(local_repo);
                println!("Opened: Local Folder");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }

            // Scripts
            if alt != 0 && c != 0 && n != 0 && m != 0 && enter != 0 {
                println!("Running: Windows Configuration Script");
                run_powershell(script_cleanup);
                std::thread::sleep(std::time::Duration::from_millis(150));
            }
        }
        
        // Start/End Keybinds
        if alt != 0 && left_ctrl != 0 && f != 0 {
            if keys_active {
                keys_active = false;
                println!("Keybinds Disabled");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }
            else {
                keys_active = true;
                println!("Keybinds Enabled");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }
        }

        // Lets not overload the CPU
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

}

// Executing/Launching
fn run_application<'a>(app: &'a str, arg: &str, arg2: &str) -> &'a str<> {

    // Run with extra argument
    if !arg2.is_empty() {
        let _output = Command::new(app)
        .arg(arg)
        .arg(arg2)
        .output();
    }
    // Run the application
    else if !arg.is_empty() {
        let _output = Command::new(app)
        .arg(arg)
        .output();
    }
    else {
        let _output = Command::new(app)
        .output();
    }

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
        println!("{:?}", profile);
    }

    return profile_data;
}