use std::path::PathBuf;

/// Program resources are in the crate_root/assets directory by default:
pub fn get_assets_directory() -> PathBuf {
    let mut assets_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assets_dir.push("assets");
    assets_dir
}
