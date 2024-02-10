mod conf;
mod execute;
mod vars;

use std::env;

use winapi::um::winuser::GetAsyncKeyState;

fn main() {

    // Core Variables
    let mut keys_active = true;

    // Setup edge profiles
    let edge_profile_paths = conf::get_edgeprofiles();
    let _edge_profile_metadata = conf::get_edgeprofile_data(edge_profile_paths);

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

        // Keybinds
        if keys_active {

            // Edge - Personal
            if alt != 0 && q != 0 && !edge_personal_open {
                execute::run_application(vars::EDGE, edge_profile_personal, &edge_personal_arg2);
                edge_personal_open = true;
                println!("Running: Edge-Personal");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }
            else if alt != 0 && q != 0 && edge_personal_open {
                println!("Personal profile is already open.");
            }
            else if alt != 0 && e != 0 {
                let edge_personal_arg2 = "--inprivate";
                execute::run_application(vars::EDGE, edge_profile_personal, &edge_personal_arg2);
                edge_personal_open = true;
                println!("Running: Edge-Personal-Private");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }

            // Edge - Work
            if alt != 0 && w != 0 && !edge_work_open {
                execute::run_application(vars::EDGE, edge_profile_work, &edge_work_arg2);
                edge_work_open = true;
                println!("Running: Edge-Work");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }
            else if alt != 0 && w != 0 && edge_work_open {
                println!("Work profile is already open.")
            }

            // Steam
            if alt != 0 && s != 0 {
                execute::run_application(vars::STEAM, "", "");
                println!("Opened: Steam");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }

            // Discord
            if alt != 0 && d != 0 {
                execute::run_application(vars::DISCORD, "", "");
                println!("Opened: Discord");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }

            // Folder
            if alt != 0 && a != 0 {
                execute::open_folder(vars::FOLDER_LOCAL);
                println!("Opened: Local Folder");
                std::thread::sleep(std::time::Duration::from_millis(150));
            }

            // Scripts
            if alt != 0 && c != 0 && n != 0 && m != 0 && enter != 0 { // Full configuration script
                println!("Running: Windows Configuration Script");
                execute::run_powershell(vars::FULL_CONFIGURATION);
                std::thread::sleep(std::time::Duration::from_millis(150));
            }
        }

        // Lets not overload the CPU
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}