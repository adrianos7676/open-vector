use std::{collections::hash_map::HashMap, fs};
use serde::Deserialize;

use iced::{Element, Point, Subscription, Task, keyboard::{self, Key}, mouse, window};

use crate::gui::{main_window, settings_window::AvailableLocale};
#[cfg(not(target_os = "linux"))]
use crate::gui::main_window::menu_bar::{self, AppMenu};
mod gui;
mod file;
mod hid;

fn main() -> iced::Result {
    iced::daemon(boot, update, view)
        .title(title)
        .subscription(subscription)
        .run()
}

fn load_locale(language: &str) -> Locale {
    let path = format!("locales/{}.yaml", language);

    let content = fs::read_to_string(path)
        .expect("Nie można odczytać pliku lokalizacji");

    serde_yaml::from_str(&content)
        .expect("Nie można sparsować pliku lokalizacji")
}

#[derive(Debug, Deserialize)]
struct Locale {
    lang: String,
    menu: HashMap<String, String>,
    settings: HashMap<String, String>,
    about: HashMap<String, String>,
}

impl Default for State {
    fn default() -> Self {
        let locale = load_locale("en");
        Self {
            #[cfg(not(target_os = "linux"))]
            main_window_menu_bar: AppMenu::new(&locale),
            locale: locale,
            main_window: None,
            about_window: None,
            settings_window: None,
            selected_project: None,
            open_projects: HashMap::new(),
            open_projects_state: HashMap::new(),
            resizing_sidebar: false,
            sidebar_width: 240.0,
            window_size: iced::Size { width: 1280.0, height: 720.0 },
            shift_pressed: false,
            control_pressed: false,
        }
    }
}

struct ProjectState {
    zoom: f32,
    offset: iced::Vector,
}

struct State {
    locale: Locale,
    main_window: Option<window::Id>,
    #[cfg(not(target_os = "linux"))]
    main_window_menu_bar: menu_bar::AppMenu,
    about_window: Option<window::Id>,
    settings_window: Option<window::Id>,
    selected_project: Option<String>,
    open_projects: HashMap<String, String>,
    open_projects_state: HashMap<String, ProjectState>,
    resizing_sidebar: bool,
    sidebar_width: f32,
    window_size: iced::Size,
    shift_pressed: bool,
    control_pressed: bool,
}

#[derive(Debug, Clone)]
enum Message {
    AboutWindowOpen,
    AboutWindowOpened(window::Id),
    SettingsWindowOpen,
    SettingsWindowOpened(window::Id),
    WindowClosed(window::Id),
    ButtonPressed(Key),
    ButtonReleased(Key),
    NewFile,
    OpenFile,
    SaveFile,
    SaveAs,
    ChangeProjects(String),
    CloseTab,
    FileSelected(Option<std::path::PathBuf>),
    MouseMoved(Point),
    StartSidebarResize,
    StopSidebarResize,
    WindowResize(iced::Size),
    CanvasScrolled(f32, Point),
    LocaleChange(AvailableLocale),
    NoOp,
}

fn boot() -> (State, Task<Message>) {
    let mut state = State::default();
    let task = main_window::open(&mut state);

    (state, task)
}

fn subscription(
    #[cfg(target_os = "linux")] _state: &State,
    #[cfg(not(target_os = "linux"))] state: &State,
) -> Subscription<Message> {
    Subscription::batch([
        #[cfg(not(target_os = "linux"))]
        menu_bar::menu_subscription(state),
        window::close_events().map(Message::WindowClosed),
        iced::event::listen_with(|event, _status, _winodw_id| {
            match event {
                iced::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    Some(Message::MouseMoved(position))
                },
                iced::Event::Mouse(mouse::Event::ButtonReleased(iced::mouse::Button::Left)) => {
                    Some(Message::StopSidebarResize)
                },
                iced::Event::Keyboard(keyboard::Event::KeyPressed { key, modified_key: _, physical_key: _, location: _, modifiers: _, text: _, repeat: _ }) => {
                    Some(Message::ButtonPressed(key))
                },
                iced::Event::Keyboard(keyboard::Event::KeyReleased { key, modified_key: _, physical_key: _, location: _, modifiers: _}) => {
                    Some(Message::ButtonReleased(key))
                },
                iced::Event::Window(window::Event::Resized(size)) => {
                    Some(Message::WindowResize(size))
                },
                _ => None,
            }
        }),
    ])
}

fn title(state: &State, window_id: window::Id) -> String {
    if state.settings_window == Some(window_id) {
        state.locale.settings["window_title"].to_string()
    } else if state.about_window == Some(window_id) {
        state.locale.about["window_title"].to_string()
    } else {
        state.locale.menu["window_title"].to_string()
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ButtonPressed(key) => hid::key_pressed(state, key),

        Message::ButtonReleased(key) => hid::key_released(state, key),

        Message::AboutWindowOpen => return gui::about_window::open(state),
        Message::AboutWindowOpened(window_id) => gui::about_window::opened(state, window_id),

        Message::SettingsWindowOpen => return gui::settings_window::open(state),
        Message::SettingsWindowOpened(window_id) => gui::settings_window::opened(state, window_id),

        Message::WindowClosed(id) => return gui::window_closed(state, id),

        Message::NewFile => return file::message_new_file(),

        Message::OpenFile => return file::message_open_file(),

        Message::SaveFile => {
            println!("Save");
        }

        Message::SaveAs => {
            println!("Save As...");
        }

        Message::ChangeProjects(project_name) => state.selected_project = Some(project_name),
        Message::CloseTab => {
            println!("CloseTab");
        },
        Message::FileSelected(path) => return file::message_file_selected(path, state),
        Message::MouseMoved(position) => hid::mouse_moved(state, position),
        Message::StartSidebarResize => state.resizing_sidebar = true,
        Message::StopSidebarResize => state.resizing_sidebar = false,
        Message::WindowResize(size) => {
            state.window_size = size;
        },
        Message::LocaleChange(locale) => {
            state.locale = load_locale(&locale.lang);
            #[cfg(not(target_os = "linux"))]
            state.main_window_menu_bar.set_locale(&state.locale);
        },
        Message::CanvasScrolled(delta, point) => main_window::canvas::scrolled(state, delta, point),
        Message::NoOp => {},
    }

    Task::none()
}

fn view(state: &State, window_id: window::Id) -> Element<'_, Message> {
    if state.about_window == Some(window_id) {
        return gui::about_window::view(state);
    } else if state.settings_window == Some(window_id) {
        return gui::settings_window::view(state);
    }

    gui::main_window::view(state)
}