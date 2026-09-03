use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::InputKey;

#[derive(Serialize,Deserialize)]
pub struct Settings {
    pub zoom_speed: f32,
    pub x_axis_scroll_button: InputKey,
    pub y_axis_scroll_button: InputKey,
}

pub fn save_settings(state: &Settings) {
    let config = config_dir()
        .expect("Nie można znaleźć katalogu konfiguracji");

    let open_vector_dir = config.join("open-vector");

    fs::create_dir_all(&open_vector_dir)
        .expect("Could not create open-vector folder");

    let settings_file = open_vector_dir.join("settings.yaml");

    let yaml = serde_yaml::to_string(state)
        .expect("Could not serialize settings");

    fs::write(&settings_file, yaml)
        .expect("Could not write settings.yaml");
}

pub fn load_settings() -> Option<Settings> {
    let config = config_dir()?;
    let settings_file = config.join("open-vector").join("settings.yaml");

    let yaml = fs::read_to_string(settings_file).ok()?;

    serde_yaml::from_str(&yaml).ok()
}
