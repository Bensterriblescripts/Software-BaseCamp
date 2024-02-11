use std::process::{ Command, exit };
use powershell_script;

// Applications
pub fn run_application<'a>(app: &'a str, arg: &str, arg2: &str) -> u32 {

    // Run with two arguments
    if !arg2.is_empty() {
        let output = Command::new(app)
        .arg(arg)
        .arg(arg2)
        .spawn();

        match output {
            Ok(mut child) => {
                println!("Process ID: {}", child.id());
                let status = child.wait();
                match status {
                    Ok(exit_status) => {
                        if exit_status.success() {
                            return child.id();
                        }
                        else {
                            exit(exit_status.code().unwrap_or(1));
                        }
                    }
                    Err(e) => {
                        eprintln!("Error waiting for child process: {}", e);
                        return 0;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error spawning new process: {}", e);
                return 0;
            }
        }
    }
    // Run with a single argument
    else if !arg.is_empty() {
        
        let output = Command::new(app)
        .arg(arg)
        .spawn();

        match output {
            Ok(mut child) => {
                println!("Process ID: {}", child.id());
                let status = child.wait();
                match status {
                    Ok(exit_status) => {
                        if exit_status.success() {
                            return child.id();
                        }
                        else {
                            exit(exit_status.code().unwrap_or(1));
                        }
                    }
                    Err(e) => {
                        eprintln!("Error waiting for child process: {}", e);
                        return 0;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error spawning new process: {}", e);
                return 0;
            }
        }
    }
    // Run with no arguments
    else {
        let output = Command::new(app)
        .spawn();

        match output {
            Ok(mut child) => {
                println!("Process ID: {}", child.id());
                let status = child.wait();
                match status {
                    Ok(exit_status) => {
                        if exit_status.success() {
                            return child.id();
                        }
                        else {
                            exit(exit_status.code().unwrap_or(1));
                        }
                    }
                    Err(e) => {
                        eprintln!("Error waiting for child process: {}", e);
                        return 0;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error spawning new process: {}", e);
                return 0;
            }
        }
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