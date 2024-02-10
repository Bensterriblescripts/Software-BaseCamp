use std::env;
use std::fs;

// Edge
pub fn get_edgeprofiles() -> Vec<String> {
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
pub fn get_edgeprofile_data(profile_paths: Vec<String>) -> Vec<String> {

    let profile_data: Vec<String> = Vec::new();

    for profile in profile_paths {
        println!("{:?}", profile);
    }

    return profile_data;
}