use std::process::Command;

fn main() {

    // Executables
    let edge_personal = "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe";
    // Folders
    let local_repo = "C:\\Local\\repo";
    // Arguments
    let edge_profile_personal = "--profile-directory=Default";
    let edge_profile_work = "--profile-directory=Profile 1";

    // Set the var
    let mut app = edge_personal;
    let mut folder = local_repo;
    let mut arg = edge_profile_personal;

    let _app_opened = open_application(app, arg);
    let _folder_opened = open_folder(folder);
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
