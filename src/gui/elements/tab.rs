use iced::{
    Alignment, Element, Length, widget::{button, container, row, text},
};

use crate::Message;

fn project_name_button(name: &String, id: &usize) -> Element<'static, Message> {
    button(text(name.to_owned()))
        .on_press(Message::ChangeProjects(*id))
        .style(|_theme, _status| {
            button::Style {
                background: None,
                ..Default::default()
            }
        })
        .into()
}

fn project_close_button() -> Element<'static, Message> {
    button(text("×"))
        .on_press(Message::CloseTab)
        .style(|_theme, _status| {
            button::Style {
                background: None,
                ..Default::default()
            }
        })
        .into()
}

pub fn new(name: &String, id: &usize) -> Element<'static, Message> {
    container(
        row![
            project_name_button(name, id),
            project_close_button(),
        ]
        .spacing(4)
        .height(Length::Fill)
        .align_y(Alignment::Center),
    )
    .width(Length::Shrink)
    .height(Length::Fill)
    .style(|_theme| {
        container::Style {
            background: Some(
                iced::Background::Color(
                    iced::Color::from_rgb(0.3, 0.3, 0.3),
                ),
            ),
            ..Default::default()
        }
    })
    .into()
}