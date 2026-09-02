use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MenuLocale {
    pub window_title: String,
    pub new_file_menu_button: String,
    pub open_projects_file_menu_button: String,
    pub save_file_menu_button: String,
    pub save_as_file_menu_button: String,
    pub settings_menu_button: String,
    pub about_software_menu_button: String,
    pub file_menu_dropdown: String,
    pub misc_menu_dropdown: String,
}

#[derive(Debug, Deserialize)]
pub struct SettingsLocale {
    pub window_title: String,
    pub locale_label: String,
    pub x_scroll_keybind_label: String,
    pub y_scroll_keybind_label: String,
    pub zoom_speed_label: String,
}

#[derive(Debug, Deserialize)]
pub struct AboutLocale {
    pub window_title: String,
    pub program_name: String,
    pub version: String,
    pub licensed_under: String,
}

#[derive(Debug, Deserialize)]
pub struct Locale {
    pub lang: String,
    pub menu: MenuLocale,
    pub settings: SettingsLocale,
    pub about: AboutLocale,
}