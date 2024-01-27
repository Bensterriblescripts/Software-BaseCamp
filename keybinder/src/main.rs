use std::process::Command;
use winapi::um::winuser::GetAsyncKeyState;

fn main() {

    // Applications
    let edge = "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe";
    // Folders
    let local_repo = "C:\\Local";
    // Arguments
    let edge_profile_personal = "--profile-directory=Default";
    let edge_profile_work = "--profile-directory=Profile 4";
    // Keybinds
    let _edge_personal = ["alt", "q"];
    let _edge_work = ["alt", "q"];

    loop {

        let alt = unsafe { GetAsyncKeyState(0x12) };
        let q = unsafe { GetAsyncKeyState(0x51) };
        let w = unsafe { GetAsyncKeyState(0x57) };
        // let e = unsafe { GetAsyncKeyState(0x45) };
        // let r = unsafe { GetAsyncKeyState(0x52) };
        let a = unsafe { GetAsyncKeyState(0x41) };

        if alt != 0 && q != 0 {
            open_application(edge, edge_profile_personal);
            println!("Opened: Edge-Personal");
        }
        if alt != 0 && w != 0 {
            open_application(edge, edge_profile_work);
            println!("Opened: Edge-Work");
        }
        if alt != 0 && a != 0 {
            open_folder(local_repo);
            println!("Opened: Local Folder");
        }

        
        
        // Lets not overload the CPU
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

}


fn open_application<'a>(app: &'a str, arg: &str) -> &'a str<> {
    let _output = Command::new(app)
    .arg(arg)
    .output();

    return app;
}
fn open_folder(folder: &str) -> &str {
    let _output = Command::new("explorer")
        .arg(folder)
        .output();

    return folder;
}