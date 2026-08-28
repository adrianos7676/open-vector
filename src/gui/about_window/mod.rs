use iced::widget::{column, container, text};
use iced::{Element, Length, window};

use crate::{Message, State};

pub fn view(state: &State) -> Element<'_, Message> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    container(
        column![
            text(state.locale.about["program_name"].clone()).size(24),
            text(format!(
                "{} {}",
                state.locale.about["version"].clone(),
                VERSION
            )),
            text("© 2026 Open Creative Suite"),
            text(state.locale.about["licensed_under"].clone()),
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

pub fn window_opened(state: &mut State, id: window::Id) {
    state.about_window = Some(id);
}