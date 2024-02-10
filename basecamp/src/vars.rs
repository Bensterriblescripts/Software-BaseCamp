// Applications
pub const EDGE: &str = "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe";
pub const STEAM: &str = "C:\\Program Files (x86)\\Steam\\steam.exe";
pub const DISCORD: &str = "C:\\Users\\coffe\\AppData\\Local\\Discord\\app-1.0.9032\\Discord.exe";

// Edge Profiles
pub const ARG_EDGE_PERSONAL: &str = "--profile-directory=Default";
pub const ARG_EDGE_WORK: &str = "--profile-directory=Profile 4";
pub const ARG_EDGE_PRIVATE: &str = "--inprivate";

// Scripts
pub const _FULL_CONFIGURATION: &str = include_str!("scripts/configure_windows.ps1");
pub const _ENABLE_FULL_CONTEXT_MENU: &str = include_str!("scripts/configure_contextmenu.ps1");
pub const _DISABLE_HIBERNATION: &str = include_str!("scripts/configure_hibernate.ps1");
pub const _DISABLE_ONLINESEARCH: &str = include_str!("scripts/configure_searchbar.ps1");
pub const CONFIGURE_TASKBAR: &str = include_str!("scripts/configure_taskbar.ps1");

// Folders
pub const FOLDER_LOCAL: &str = "C:\\Local";

// Settings
pub const KEYBINDS_TOGGLE: bool =  true;