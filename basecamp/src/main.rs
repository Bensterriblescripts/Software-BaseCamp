mod conf;
mod execute;
mod vars;

use std::collections::HashMap;

use winapi::um::winuser::GetAsyncKeyState;

fn main() {

    // Core Variables
    let mut launch = false;
    let mut keystates: HashMap<&str, i16> = HashMap::new();

    // Binds
    let mut keybinds: HashMap<(&str, &str, &str), (&str, &str, &str, &str, u32)> = HashMap::new();
    let collected_keybinds = vec![
        (("alt",    "q",    ""),    (vars::EDGE, vars::ARG_EDGE_PERSONAL, "", "Application", 0)),
        (("alt",    "w",    ""),    (vars::EDGE, vars::ARG_EDGE_WORK, "", "Application", 1)),
        (("alt",    "e",    ""),    (vars::EDGE, vars::ARG_EDGE_PERSONAL, vars::ARG_EDGE_PRIVATE, "Application", 2)),
        (("alt",    "r",    ""),    (vars::VSCODE, "", "", "Application", 3)),
        (("alt",    "s",    ""),    (vars::STEAM, "", "", "Application", 4)),
        (("alt",    "d",    ""),    (vars::DISCORD, "", "", "Application", 5)),
        (("alt",    "a",    ""),    (vars::FOLDER_LOCAL, "", "", "Folder", 6)),
        (("lctrl",  "alt",  "f"),   ("Keybinds", "Toggle", "", "Setting", 7)),
    ];
    keybinds.extend(collected_keybinds);

    // Processes
    let mut processes: HashMap<u32, u32> = HashMap::new();

    // Settings
    let mut settings: HashMap<&str, i32> = HashMap::new();
    let collected_settings = vec![
        ("Keybinds", 1),
    ];
    settings.extend(collected_settings);

    // Setup edge profiles
    let edge_profile_paths = conf::get_edgeprofiles();
    let _edge_profile_metadata = conf::get_edgeprofile_data(edge_profile_paths);

    // Auto-Open Work Browser Links
    let _edge_work_sharepoint = "https://sparknz.sharepoint.com/";
    let _edge_work_outlook = "https://outlook.office.com/mail/";

    // Core Keylogger
    loop {

        // Get all key states again
        let keystates_collected = vec![
            ("alt",         unsafe { GetAsyncKeyState(0x12) } ),
            ("enter",       unsafe { GetAsyncKeyState(0x0D) }),
            ("lctrl",       unsafe { GetAsyncKeyState(0x11) }),
            ("a",           unsafe { GetAsyncKeyState(0x41) }),
            ("c",           unsafe { GetAsyncKeyState(0x43) }),
            ("d",           unsafe { GetAsyncKeyState(0x44) }),
            ("e",           unsafe { GetAsyncKeyState(0x45) }),
            ("f",           unsafe { GetAsyncKeyState(0x46) }),
            ("m",           unsafe { GetAsyncKeyState(0x4D) }),
            ("n",           unsafe { GetAsyncKeyState(0x4E) }),
            ("q",           unsafe { GetAsyncKeyState(0x51) }),
            ("r",           unsafe { GetAsyncKeyState(0x52) }),
            ("s",           unsafe { GetAsyncKeyState(0x53) }),
            ("w",           unsafe { GetAsyncKeyState(0x57) }),
        ];
        keystates.extend(keystates_collected);

        // Setting - Check if keybinds are enabled
        if let Some(keybind_setting) = settings.get("Keybinds") {
            if keybind_setting == &1 {
                // Actions by key
                for (keypress, value) in keystates.iter() {
                    // If the key has been pressed
                    if *value != 0 {
                        // Locate the key in our keybinds hashmap
                        for ((first_key, second_key, third_key), (target, arg1, arg2, launchtype, process)) in &keybinds {
                            // Single key binds
                            if first_key == keypress && second_key.is_empty() && third_key.is_empty() {
                                launch = true;
                            }

                            // Two key binds
                            else if first_key == keypress && !second_key.is_empty() && third_key.is_empty() {
                                // If the second key is also pressed
                                if let Some(&second_keypress) = keystates.get(second_key) {
                                    if second_keypress != 0 {
                                        launch = true;
                                    }
                                }
                            }

                            // Three key binds
                            else if first_key == keypress && !second_key.is_empty() && !third_key.is_empty() {
                                // If the second key is also pressed
                                if let Some(&second_keypress) = keystates.get(second_key) {
                                    if second_keypress != 0 {
                                        // If the third key is also pressed
                                        if let Some(&third_keypress) = keystates.get(third_key) {
                                            if third_keypress != 0 {
                                                launch = true;
                                            }
                                        }
                                    }
                                }
                            }

                            // Launch
                            if launch {
                                if launchtype == &"Application" {
                                    // Check for the existing application in the local hashmap
                                    if !processes.contains_key(process) {
                                        execute::run_application(target, arg1, arg2);
                                    }
                                    // Need to set it as main window here
                                    else {
                                        println!("Application is already open.");
                                    }
                                }
                                else if launchtype == &"Folder" {
                                    execute::open_folder(target);
                                }
                                else if launchtype == &"Script" {
                                    execute::run_powershell(target);
                                }
                                else if launchtype == &"Setting" {
                                    if let Some(value) = settings.get_mut(target) {
                                        if arg1 == &"Toggle" && value == &0 {
                                            println!("Enabled: {}", target);
                                            *value = 1;
                                        }
                                        else if arg1 == &"Toggle" && value == &1 {
                                            println!("Disabled: {}", target);
                                            *value = 0;
                                        }
                                    }
                                }
                                launch = false;
                                std::thread::sleep(std::time::Duration::from_millis(150));
                            }
                        }
                    }
                }
            }
            // Fallback to re-enable keybinds
            else if keybind_setting == &0 {
                for ((bind1, bind2, bind3), (target, _arg1, _arg2, _launchtype, _process)) in keybinds.iter() {
                    if target == &"Keybinds" {
                        // Single bind
                        if bind2.is_empty() && bind3.is_empty() {
                            if let Some(first_keypress) = keystates.get_mut(bind1) {
                                if first_keypress != &0 {
                                    if let Some(value) = settings.get_mut(target) {
                                        *value = 1;
                                        println!("Re-enabled keybinds");
                                    }
                                }
                            }
                        }
                        // Dual bind
                        if bind3.is_empty() {
                            if let Some(first_keypress) = keystates.get_mut(bind1) {
                                if first_keypress != &0 {
                                    if let Some(second_keypress) = keystates.get_mut(bind2) {
                                        if second_keypress != &0 {
                                            if let Some(value) = settings.get_mut(target) { 
                                                *value = 1;
                                                println!("Re-enabled keybinds");
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        // Triple bind
                        if let Some(first_keypress) = keystates.get(bind1) {
                            if first_keypress != &0 {
                                if let Some(second_keypress) = keystates.get(bind2) {
                                    if second_keypress != &0 {
                                        if let Some(third_keypress) = keystates.get(bind3) {
                                            if third_keypress != &0 {
                                                if let Some(value) = settings.get_mut(target) { 
                                                    *value = 1;
                                                    println!("Re-enabled keybinds");
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        std::thread::sleep(std::time::Duration::from_millis(150));
                    }
                }
            }
        }
        else {
        }

        // Lets not overload the CPU
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
