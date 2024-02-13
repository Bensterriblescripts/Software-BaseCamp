use std::process::Command;
use powershell_script;

// Applications
pub fn run_application<'a>(app: &'a str, arg: &str, arg2: &str) -> u32 {

    // Arguments
    let mut output = Command::new(app);
    if !arg2.is_empty() {
        output.arg(arg).arg(arg2);
    }
    else if !arg.is_empty() {
        output.arg(arg);
    }

    // New process
    if let Ok(child) = output.spawn() {
        println!("Process ID: {}", child.id());
        return child.id();
    }
    else {
        println!("Failed to spawn the process");
        return 0;
    }
}

// Folders
pub fn open_folder(folder: &str) -> &str {
    let _output = Command::new("explorer")
        .arg(folder)
        .output();

    return folder;
}

// Scripts
pub fn run_powershell(script: &str) {
    match powershell_script::run(script) {
        Ok(output) => {
            println!("{}", output);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}