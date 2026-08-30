use iced::widget::{column, container, text};
use iced::window::Level;
use iced::{Element, Length, Task, window};

use crate::{Message, State};

pub fn view(state: &State) -> Element<'_, Message> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    container(
        column![
            text(state.locale.about.program_name.clone()).size(24),
            text(format!(
                "{} {}",
                state.locale.about.version.clone(),
                VERSION
            )),
            text("© 2026 Open Creative Suite"),
            text(state.locale.about.licensed_under.clone()),
        ]
        .spacing(8)
        .padding(20),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

pub fn open(state: &mut State) -> Task<Message> {
    match state.about_window {
        None => {
            let (_, task) = window::open(window::Settings {
                size: iced::Size::new(400.0, 300.0),
                resizable: false,
                level: Level::AlwaysOnTop,
                ..Default::default()
            });
            return task.map(Message::AboutWindowOpened)
        }
        Some(id) => {
            let _ = window::gain_focus::<Message>(id);
            return Task::none()
        },
    }
}

pub fn opened(state: &mut State, window_id: window::Id) {
    state.about_window = Some(window_id);
}