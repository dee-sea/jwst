#[cfg(target_os = "windows")]
pub const IMAGE_PATH: &str = "jwsp_wallpapers";

#[cfg(not(target_os = "windows"))]
pub const IMAGE_PATH: &str = ".jwsp_wallpapers";

pub fn get_path() -> String {
    let mut path = dirs::home_dir().unwrap();
    path.push(IMAGE_PATH);
    path.to_str().unwrap().to_string()
}
