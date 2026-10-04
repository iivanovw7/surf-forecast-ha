use axum::extract::State;
use tera::Context;

use crate::types::config::AppState;

use std::fs;
use std::path::PathBuf;

use crate::types::config::CssManifest;

pub fn get_app_root() -> PathBuf {
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if current_dir.join("templates").exists() && current_dir.join("assets").exists() {
        return current_dir;
    }

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            if exe_dir.join("templates").exists() && exe_dir.join("assets").exists() {
                return exe_dir.to_path_buf();
            }
            if let Some(target_dir) = exe_dir.parent() {
                if let Some(project_root) = target_dir.parent() {
                    if project_root.join("templates").exists() && project_root.join("assets").exists() {
                        return project_root.to_path_buf();
                    }
                }
            }
        }
    }

    current_dir
}

fn load_css_assets_manifest() -> String {
    let app_root = get_app_root();
    let manifest_path = app_root.join("assets/css/manifest.json");
    let manifest_content = fs::read_to_string(&manifest_path)
        .unwrap_or_else(|err| panic!("Failed to read CSS manifest at {:?}: {}", manifest_path, err));
    let manifest: CssManifest = serde_json::from_str(&manifest_content).unwrap();

    manifest.entries["main.scss"].clone()
}

pub fn create_context(_state: &State<AppState>) -> Context {
    let mut context = Context::new();

    context.insert("css_file", &load_css_assets_manifest());

    context
}
