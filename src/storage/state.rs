use crate::data::types::PersistedState;
use std::path::PathBuf;

fn state_path() -> PathBuf {
    let config = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config.join("ffmpuzzle").join("state.json")
}

pub fn load_state() -> Option<PersistedState> {
    let path = state_path();
    let data = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

pub fn save_state(state: &PersistedState) -> std::io::Result<()> {
    let path = state_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(state)?;
    std::fs::write(path, json)
}

pub fn generate_ref_code() -> String {
    use rand::Rng;
    let chars = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let mut rng = rand::thread_rng();
    (0..4)
        .map(|_| chars[rng.gen_range(0..chars.len())] as char)
        .collect()
}
