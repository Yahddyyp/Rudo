use crate::app::state::Appstate;
use std::fs;
use std::path::PathBuf;

fn get_data_path() -> Option<PathBuf> {
    let mut path = dirs::config_dir()?;
    path.push("rudo");
    fs::create_dir_all(&path).ok()?;
    path.push("appdata.json");
    Some(path)
}

pub fn save_state(app_state: &Appstate) -> Result<(), std::io::Error> {
    if let Some(path) = get_data_path() {
        let data = serde_json::to_string(app_state)?;
        fs::write(path, data)?;
    }
    Ok(())
}

pub fn load_state() -> Option<Appstate> {
    get_data_path().and_then(|path| {
        fs::read_to_string(path)
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
    })
}
