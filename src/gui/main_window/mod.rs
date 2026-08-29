use iced::{
    Alignment, Background, Element, Length, Task, widget::{column, container, mouse_area, row, text}, window,
};
#[cfg(target_os = "linux")]
use iced::{widget::button, Theme, Renderer};
#[cfg(target_os = "linux")]
use iced_aw::{Menu, MenuBar, menu::Item};

use crate::{
    Message, State, gui::elements,
};
#[cfg(not(target_os = "linux"))]
pub mod menu_bar;
pub(crate) mod canvas;
#[cfg(target_os = "linux")]
pub fn menu_button(label: &str, message: Message) -> Element<'_, Message> {
    button(text(label))
        .on_press(message)
        .width(Length::Fill)
        .into()
}
#[cfg(target_os = "linux")]
fn menu_item<'a>(
    label: &'a str,
    menu: Menu<'a, Message, Theme, Renderer>,
) -> Item<'a, Message, Theme, Renderer> {
    Item::with_menu(
        button(text(label)),
        menu,
    )
}

pub fn open(state: &mut State) -> Task<Message> {
    let icon = image::load_from_memory(include_bytes!("../../../assets/icon.png"))
        .expect("Failed to load icon")
        .into_rgba8();

    let icon = window::icon::from_rgba(
        icon.as_raw().to_vec(),
        icon.width(),
        icon.height(),
    )
    .ok();

    let (window_id, task) = window::open(window::Settings {
        icon,
        size: iced::Size::new(1280.0, 800.0),
        resizable: true,
        ..Default::default()
    });

    state.main_window = Some(window_id);

    #[cfg(target_os = "macos")]
    state.main_window_menu_bar.menu.init_for_nsapp();

    #[cfg(target_os = "windows")]
    {
        use raw_window_handle::{HasWindowHandle, RawWindowHandle};

        let menu = state.main_window_menu_bar.menu.clone();

        return task.chain(
            window::run_with_handle(window_id, move |handle| {
                if let Ok(handle) = handle.window_handle() {
                    if let RawWindowHandle::Win32(win32) = handle.as_raw() {
                        let _ = menu.init_for_hwnd(win32.hwnd.get() as isize);
                    }
                }

                Message::NoOp
            })
        );
    }

    task.map(|_| Message::NoOp)
}


pub fn view(state: &State) -> Element<'_, Message> {
    #[cfg(target_os = "linux")]
    let file_menu = Menu::new(vec![
        Item::new(menu_button(&state.locale.menu["new_file_menu_button"], Message::NewFile)),
        Item::new(menu_button(&state.locale.menu["open_projects_file_menu_button"], Message::OpenFile)),
        Item::new(menu_button(&state.locale.menu["save_file_menu_button"], Message::SaveFile)),
        Item::new(menu_button(&state.locale.menu["save_as_file_menu_button"], Message::SaveAs)),
    ]);
    #[cfg(target_os = "linux")]
    let misc_menu = Menu::new(vec![
        Item::new(menu_button(&state.locale.menu["settings_menu_button"], Message::SettingsWindowOpen)),
        Item::new(menu_button(&state.locale.menu["about_software_menu_button"], Message::AboutWindowOpen)),
    ]);
    #[cfg(target_os = "linux")]
    let top_bar = container(
        MenuBar::new(vec![
            menu_item(&state.locale.menu["file_menu_dropdown"], file_menu),
            menu_item(&state.locale.menu["misc_menu_dropdown"], misc_menu),
        ])
        .spacing(8),
    )
    .width(Length::Fill)
    .padding(4);
    

    let side_bar = container(
        row![
        mouse_area(
            container("")
            .width(4)
            .height(Length::Fill),
        )
        .on_press(Message::StartSidebarResize),
        column![
        container(
            text("Title")
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
        )
        .width(Length::Fill)
        .height(32)
        .padding(8)
        .style(|_theme| {
            container::Style {
                background: Some(
                    Background::Color(
                        iced::Color::from_rgb(0.15, 0.15, 0.15)
                    )
                ),
                ..Default::default()
            }
        }),
        ]
        ]

    )
    .width(Length::Fixed(state.sidebar_width));


    let tabs = row(
        state.open_projects
            .iter()
            .map(|document| elements::tab::new(&document.name, &document.id))
    )
    .spacing(2)
    .width(Length::Fill)
    .height(32);

   let canvas_area = canvas::draw(state);

    column(vec![
        #[cfg(target_os = "linux")]
        top_bar.into(),
        tabs.into(),
        row(vec![
            canvas_area.into(),
            side_bar.into(),
        ]).into(),
    ])
    .into()
}