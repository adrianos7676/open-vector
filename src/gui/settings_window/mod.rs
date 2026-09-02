use std::fs;

use iced::{
    Element, Length, Task, keyboard::Key, widget::{button, column, container, pick_list, row, scrollable, space::horizontal, text}, window::{self, Level},
};

use serde::Deserialize;

use crate::{InputKey, Message, State};


#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AvailableLocale {
    pub lang: String,
    pub lang_full: String,
}

impl std::fmt::Display for AvailableLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lang_full)
    }
}

fn read_available_locales() -> Vec<AvailableLocale> {
    fs::read_dir("locales")
        .expect("Nie można odczytać katalogu locales")
        .filter_map(|entry| {
            let path = entry.ok()?.path();

            if path.extension()?.to_str()? != "yaml" {
                return None;
            }

            let content = fs::read_to_string(path).ok()?;

            serde_yaml::from_str::<AvailableLocale>(&content).ok()
        })
        .collect()
}

fn locale_select(state: &State) -> Element<'static, Message> {
    let locales = read_available_locales();

    let selected = locales
        .iter()
        .find(|locale| locale.lang == state.locale.lang)
        .cloned();
    container(
    row![
        text(state.locale.settings.locale_label.clone()),
        horizontal(),
        pick_list(
            locales,
            selected,
            |locale| Message::LocaleChange(locale),
        ),
    ])
    .width(Length::Fill)
    .into()
}

fn key_to_string(key: &InputKey) -> String {
    if let Some(mouse_key) = &key.mouse_key {
        return format!("{mouse_key:?}");
    }

    if let Some(keyboard_key) = &key.keyboard_key {
        return match keyboard_key {
            Key::Named(named) => format!("{named:?}"),
            Key::Character(c) => c.to_string(),
            Key::Unidentified => "Unidentified".to_string(),
        };
    }

    "Unassigned".to_string()
}

fn x_scroll_keybind_select(state: &State) -> Element<'static, Message> {
    let key = if state.sellecting_keybind == Some(crate::Keybind::XaxisScrollButton) {
        "Press a key".to_string()
    } else {
        key_to_string(&state.settings.x_axis_scroll_button)
    };

    container(row![
        text(state.locale.settings.x_scroll_keybind_label.clone()),
        horizontal(),
        button(text(key))
        .on_press(Message::SellectKeybind(crate::Keybind::XaxisScrollButton))
    ])
    .width(Length::Fill)
    .into()
}

fn y_scroll_keybind_select(state: &State) -> Element<'static, Message> {
    let key = if state.sellecting_keybind == Some(crate::Keybind::YaxisScrollButton) {
        "Press a key".to_string()
    } else {
        key_to_string(&state.settings.y_axis_scroll_button)
    };


    container(row![
        text(state.locale.settings.y_scroll_keybind_label.clone()),
        horizontal(),
        button(text(key))
        .on_press(Message::SellectKeybind(crate::Keybind::YaxisScrollButton))
    ])
    .width(Length::Fill)
    .into()
}

fn zoom_speed(state: &State) -> Element<'static, Message> {
    let value = state.settings.zoom_speed;

    container(row![
        text(state.locale.settings.zoom_speed_label.clone()),
        horizontal(),
        button("-").on_press(Message::DecreseZoomSpeed),
        text(format!("{}", value)),
        button("+").on_press(Message::IncreseZoomSpeed),
    ])
    .width(Length::Fill)
    .into()
}

pub fn view(state: &State) -> Element<'_, Message> {
    scrollable(column![
        locale_select(state),
        zoom_speed(state),
        x_scroll_keybind_select(state),
        y_scroll_keybind_select(state),
    ])
    .into()
}

pub fn open(state: &mut State) -> Task<Message> {
    match state.settings_window {
        None => {
            let (_, task) = window::open(window::Settings {
                size: iced::Size::new(400.0, 300.0),
                resizable: false,
                level: Level::AlwaysOnTop,
                ..Default::default()
            });
            return task.map(Message::SettingsWindowOpened)
        }
        Some(id) => {
            let _ = window::gain_focus::<Message>(id);
            return Task::none()
        },
    }
}

pub fn opened(state: &mut State, window_id: window::Id) {
    state.settings_window = Some(window_id);
}