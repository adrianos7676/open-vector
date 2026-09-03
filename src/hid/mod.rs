use iced::{Point, Task};

use crate::{InputKey, Message, State};

fn input_key_matches(a: &InputKey, b: &InputKey) -> bool {
    if let (Some(a), Some(b)) = (&a.keyboard_key, &b.keyboard_key) {
        return a == b;
    }

    if let (Some(a), Some(b)) = (&a.mouse_key, &b.mouse_key) {
        return a == b;
    }

    false
}

pub fn key_pressed(state: &mut State, key: InputKey) -> Task<Message> {
    println!("Key pressed: {:?}", key);

    if input_key_matches(&key, &state.settings.x_axis_scroll_button) {
        state.x_scroll_button_pressed = true;
        println!("x_scroll_button_pressed is now true");
    }

    if input_key_matches(&key, &state.settings.y_axis_scroll_button) {
        state.y_scroll_button_pressed = true;
        println!("y_scroll_button_pressed is now true");
    }

    if state.sellecting_keybind.is_some() {
        Task::done(Message::SellectedKeybind(key))
    } else {
        Task::none()
    }
}

pub fn key_released(state: &mut State, key: InputKey) {
    println!("Key released: {:?}", key);

    if input_key_matches(&key, &state.settings.x_axis_scroll_button) {
        state.x_scroll_button_pressed = false;
        println!("x_scroll_button_pressed is now false");
    }

    if input_key_matches(&key, &state.settings.y_axis_scroll_button) {
        state.y_scroll_button_pressed = false;
        println!("y_scroll_button_pressed is now false");
    }
}

pub fn mouse_moved(state: &mut State, position: Point) {
    if state.resizing_sidebar {
        state.sidebar_width =
            (state.window_size.width - position.x).clamp(150.0, 800.0);
    }
}
