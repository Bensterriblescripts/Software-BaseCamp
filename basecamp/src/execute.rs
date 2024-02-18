use std::process::Command;
use powershell_script;

// Applications
pub fn run_application<'a>(app: &'a str, arg: &str, arg2: &str) {

    // Arguments
    let mut output = Command::new(app);
    if !arg2.is_empty() {
        output.arg(arg).arg(arg2);
    }
    else if !arg.is_empty() {
        output.arg(arg);
    }

    // New process
    if let Ok(_) = output.output() {

    }
    else {
        println!("Error launching exdecutable");
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