use iced::{Point, Task, keyboard::Key};

use crate::{Message, State};

pub fn key_pressed(state: &mut State, key: Key) -> Task<Message> {
    println!("Key pressed: {:?}", key);

    match &key {
        val if val == &state.settings.x_axis_scroll_button => {
            state.x_scroll_button_pressed = true;
        }
        val if val == &state.settings.y_axis_scroll_button => {
            state.y_scroll_button_pressed = true;
        }
        _ => {}
    }

    if state.sellecting_keybind.is_some() {
        Task::done(Message::SellectedKeybind(key))
    } else {
        Task::none()
    }
}

pub fn key_released(state: &mut State, key: Key) {
    println!("Key released: {:?}", key);
    match key {
        val if val == state.settings.x_axis_scroll_button => state.x_scroll_button_pressed = false,
        val if val == state.settings.y_axis_scroll_button => state.y_scroll_button_pressed = false,
        _ => {},
    }
}

pub fn mouse_moved(state: &mut State, position: Point) {
    if state.resizing_sidebar {
        state.sidebar_width = (state.window_size.width - position.x).clamp(150.0, 800.0);
    }
}