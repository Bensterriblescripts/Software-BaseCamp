use std::process::Command;
use powershell_script;

// Applications
pub fn run_application<'a>(app: &'a str, arg: &str, arg2: &str) -> &'a str<> {

    // Run with two arguments
    if !arg2.is_empty() {
        let output = Command::new(app)
        .arg(arg)
        .arg(arg2)
        .spawn();

        println!("Process ID: {}", output.unwrap().id())
    }
    // Run with a single argument
    else if !arg.is_empty() {
        let output = Command::new(app)
        .arg(arg)
        .spawn();

        println!("Process ID: {}", output.unwrap().id())
    }
    // Run with no arguments
    else {
        let output = Command::new(app)
        .spawn();

        println!("Process ID: {}", output.unwrap().id())
    }

    return app;
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