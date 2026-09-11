use iced::{
    Alignment::{self, Center},
    Background,
    Element,
    Length,
    Task,
    window,
    widget::{
        button,
        column,
        container,
        mouse_area,
        row,
        svg,
        text,
    },
};
#[cfg(target_os = "linux")]
use iced::{Theme, Renderer};
#[cfg(target_os = "linux")]
use iced_aw::{Menu, MenuBar, menu::Item};

use crate::{
    Message, ShapeType, State, ToolType, gui::elements,
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

pub fn open() -> Task<Message> {
    let icon = image::load_from_memory(include_bytes!("../../../assets/icon.png"))
        .expect("Failed to load icon")
        .into_rgba8();

    let icon = window::icon::from_rgba(
        icon.as_raw().to_vec(),
        icon.width(),
        icon.height(),
    )
    .ok();

    let (_, task) = window::open(window::Settings {
        icon,
        size: iced::Size::new(1280.0, 800.0),
        resizable: true,
        ..Default::default()
    });
    
    return task.map(Message::MainWindowOpened);
}

pub fn opened(state: &mut State, id: window::Id) {
    state.main_window = Some(id);
    #[cfg(target_os = "macos")]
    state.main_window_menu_bar.menu.init_for_nsapp();
}

pub fn view(state: &State) -> Element<'_, Message> {
    #[cfg(target_os = "linux")]
    let file_menu = Menu::new(vec![
        Item::new(menu_button(&state.locale.menu.new_file_menu_button, Message::NewFile)),
        Item::new(menu_button(&state.locale.menu.open_projects_file_menu_button, Message::OpenFile)),
        Item::new(menu_button(&state.locale.menu.save_file_menu_button, Message::SaveFile)),
        Item::new(menu_button(&state.locale.menu.save_as_file_menu_button, Message::SaveAs)),
    ]);
    #[cfg(target_os = "linux")]
    let misc_menu = Menu::new(vec![
        Item::new(menu_button(&state.locale.menu.settings_menu_button, Message::SettingsWindowOpen)),
        Item::new(menu_button(&state.locale.menu.about_software_menu_button, Message::AboutWindowOpen)),
    ]);
    #[cfg(target_os = "linux")]
    let top_bar = container(
        MenuBar::new(vec![
            menu_item(&state.locale.menu.file_menu_dropdown, file_menu),
            menu_item(&state.locale.menu.misc_menu_dropdown, misc_menu),
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

    let add_icon = svg::Handle::from_memory(
        include_bytes!("../../../assets/icons/add.svg").to_vec()
    );
    let select_icon = svg::Handle::from_memory(
        include_bytes!("../../../assets/icons/select.svg").to_vec()
    );

    let tool_bar = container(column![
        row![
            button(
                svg(add_icon)
                .width(Length::Fill)
                .height(Length::Fill)
            )
            .on_press(Message::ChangeTool(ToolType::Add))
            .width(32)
            .height(32),
            button(
                svg(select_icon)
                .width(Length::Fill)
                .height(Length::Fill)
            )
            .on_press(Message::ChangeTool(ToolType::Select))
            .width(32)
            .height(32)
        ].spacing(5)
    ].spacing(5))
    .width(Length::Shrink)
    .height(Length::Fill);
    
    let rectangle_icon = svg::Handle::from_memory(
        include_bytes!("../../../assets/icons/select.svg").to_vec()
    );

    let circle_icon = svg::Handle::from_memory(
        include_bytes!("../../../assets/icons/select.svg").to_vec()
    );

    let tool_menu = container(row![
        button(column![
            svg(rectangle_icon)
            .height(Length::FillPortion(7))
            .width(Length::Fill),
            text("rectangle")
            .width(Length::Fill)
            .height(Length::FillPortion(3))
            .size(12)
            .align_x(Center)
            .align_y(Center)
        ])
        .width(64)
        .height(Length::Fill)
        .on_press(Message::AddShapeToCanvas(ShapeType::Rectangle)),
        button(column![
            svg(circle_icon)
            .height(Length::FillPortion(7))
            .width(Length::Fill),
            text("circle")
            .width(Length::Fill)
            .height(Length::FillPortion(3))
            .size(12)
            .align_x(Center)
            .align_y(Center)
        ])
        .width(64)
        .height(Length::Fill)
        .on_press(Message::AddShapeToCanvas(ShapeType::Circle)),
    ])
    .width(Length::Fill)
    .height(64);

    let tabs = row(
        state.open_projects
            .iter()
            .map(|project| elements::tab::new(&project.document.name, &project.document.id))
    )
    .spacing(2)
    .width(Length::Fill)
    .height(32);

   let canvas_area = canvas::draw(state);

    column(vec![
        #[cfg(target_os = "linux")]
        top_bar.into(),
        tool_menu.into(),
        tabs.into(),
        row(vec![
            tool_bar.into(),
            canvas_area.into(),
            side_bar.into(),
        ]).into(),
    ])
    .into()
}