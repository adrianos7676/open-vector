use iced::{self, window, Task};

use crate::{Message, State};

pub mod about_window;
pub mod main_window;
pub mod settings_window;
pub mod elements;

pub fn window_closed(state: &mut State, id: window::Id) -> Task<Message> {
    if state.main_window == Some(id) {
        state.main_window = None;

        return iced::exit();
    } else if state.about_window == Some(id) {
        state.about_window = None;
    } else if state.settings_window == Some(id) {
        state.settings_window = None;
    }

    Task::none()
}
