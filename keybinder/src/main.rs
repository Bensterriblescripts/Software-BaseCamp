use powershell_script;
use std::process::Command;
use winapi::um::winuser::GetAsyncKeyState;

use winit::event_loop::EventLoop;

fn main() {

    // Winit
    let event_loop = EventLoop::new


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
        }
        else if alt != 0 && w != 0 {
            run_application(edge, edge_profile_work);
            println!("Running: Edge-Work");
        }
        else if alt != 0 && a != 0 {
            open_folder(local_repo);
            println!("Opened: Local Folder");
        }
        else if alt != 0 && c != 0 && n != 0 && m != 0 && enter != 0 {
            println!("Running: Windows Configuration Script");
            run_powershell(script_cleanup)
        }
        
        // Lets not overload the CPU
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

}


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