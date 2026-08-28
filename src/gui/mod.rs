use iced::{Element, Length, Renderer, Theme};
use iced::widget::{button, text};
use iced_aw::{Menu, menu::Item};

use crate::Message;

pub mod about_window;
pub mod main_window;
pub mod settings_window;
pub mod elements;

pub fn menu_button(label: &str, message: Message) -> Element<'_, Message> {
    button(text(label))
        .on_press(message)
        .width(Length::Fill)
        .into()
}

fn menu_item<'a>(
    label: &'a str,
    menu: Menu<'a, Message, Theme, Renderer>,
) -> Item<'a, Message, Theme, Renderer> {
    Item::with_menu(
        button(text(label)),
        menu,
    )
}
