use std::fs;

use iced::{
    Element, Length, widget::{column, container, pick_list, row, scrollable, space::horizontal, text}, window,
};

use serde::Deserialize;

use crate::{Message, State};


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
        text(state.locale.settings["locale_label"].clone()),
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

pub fn view(state: &State) -> Element<'_, Message> {
    scrollable(column![
        locale_select(state),
    ])
    .into()
}

pub fn window_opened(state: &mut State, id: window::Id) {
    state.settings_window = Some(id);
}