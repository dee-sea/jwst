#[cfg(target_os = "windows")]
pub const IMAGE_PATH: &str = "jwsp_wallpapers";

#[cfg(not(target_os = "windows"))]
pub const IMAGE_PATH: &str = ".jwsp_wallpapers";

/// This function takes no arguments and returns a path to store the images based
/// on the current operating system.
pub fn get_path() -> String {
    let mut path = dirs::home_dir().unwrap();
    path.push(IMAGE_PATH);
    path.to_str().unwrap().to_string()
}
