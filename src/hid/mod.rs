use iced::{Point, keyboard::{Key, key::Named}};

use crate::State;

pub fn key_pressed(state: &mut State, key: Key) {
    println!("Key pressed: {:?}", key);
    match key {
        Key::Named(Named::Shift) => state.shift_pressed = true,
        Key::Named(Named::Control) => state.control_pressed = true,
        _ => {},
    }
}

pub fn key_released(state: &mut State, key: Key) {
    println!("Key Released: {:?}", key);
    match key {
        Key::Named(Named::Shift) => state.shift_pressed = false,
        Key::Named(Named::Control) => state.control_pressed = false,
        _ => {},
    }
}

pub fn mouse_moved(state: &mut State, position: Point) {
    if state.resizing_sidebar {
        state.sidebar_width = (state.window_size.width - position.x).clamp(150.0, 800.0);
    }
}