use iced::{
    Background, Element, Length, widget::{column, container, mouse_area, row, text},
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

pub fn view(state: &State) -> Element<'_, Message> {
    #[cfg(target_os = "linux")]
    {
        let file_menu = Menu::new(vec![
            Item::new(menu_button(&state.locale.menu["new_file_menu_button"], Message::NewFile)),
            Item::new(menu_button(&state.locale.menu["open_projects_file_menu_button"], Message::OpenFile)),
            Item::new(menu_button(&state.locale.menu["save_file_menu_button"], Message::SaveFile)),
            Item::new(menu_button(&state.locale.menu["save_as_file_menu_button"], Message::SaveAs)),
        ]);

        let misc_menu = Menu::new(vec![
            Item::new(menu_button(&state.locale.menu["settings_menu_button"], Message::Settings)),
            Item::new(menu_button(&state.locale.menu["about_software_menu_button"], Message::AboutSoftware)),
        ]);

        let top_bar = container(
            MenuBar::new(vec![
                menu_item(&state.locale.menu["file_menu_dropdown"], file_menu),
                menu_item(&state.locale.menu["misc_menu_dropdown"], misc_menu),
            ])
            .spacing(8),
        )
        .width(Length::Fill)
        .padding(4);
    }
    

    let side_bar = container(
        row![
        mouse_area(
            container("")
            .width(4)
            .height(Length::Fill)
            .style(|_theme| {
            container::Style {
                background: Some(
                    Background::Color(
                        iced::Color::from_rgb(1.0, 0.0, 0.0)
                    )
                ),
                ..Default::default()
            }
        }),
        )
        .on_press(Message::StartSidebarResize),
        column![
        container(text("Title"))
        .width(Length::Fill)
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
            .keys()
            .map(|name| elements::tab::new(name.clone()))
    )
    .spacing(2)
    .width(Length::Fill)
    .height(32);

    let canvas_area = container(
        text("canvas goes here"),
    )
    .width(Length::Fill)
    .height(Length::Fill);

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