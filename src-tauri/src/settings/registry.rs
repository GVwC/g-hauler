// src-tauri/src/settings/registry.rs
use once_cell::sync::Lazy;
use serde_json::json;

use super::models::*;

pub static SETTINGS_REGISTRY: Lazy<Vec<Setting>> = Lazy::new(|| {
    vec![
        Setting {
            key: "autostart".into(),
            label: "Launch on Startup".into(),
            description: Some("Start the app automatically when you sign in".into()),
            category: SettingCategory::General,
            default_value: json!(false),
            setting_type: SettingType::Toggle,
            requires_restart: false,
            system_managed: true, // handled via adapter
        },
        Setting {
            key: "lghub_data_path".into(),
            label: "G HUB data location".into(),
            description: Some("Folder that contains G HUB data (e.g. ProgramData/LGHUB)".into()),
            category: SettingCategory::Paths,
            default_value: json!(r"C:\ProgramData\LGHUB"),
            setting_type: SettingType::Path {
                directory: true,
                extensions: None, // not used for directories
            },
            requires_restart: false,
            system_managed: false, // no OS side-effect, just a stored path
        },
        // …add more settings here later
    ]
});

pub fn all() -> &'static [Setting] { &SETTINGS_REGISTRY }
pub fn find(key: &str) -> Option<&'static Setting> { SETTINGS_REGISTRY.iter().find(|s| s.key == key) }
