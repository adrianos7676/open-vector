use iced::{
    Subscription,
    futures::{SinkExt, channel::mpsc as iced_mpsc},
};
use muda::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use std::sync::mpsc as std_mpsc;

use crate::{Locale, Message, State};

pub struct AppMenu {
    pub menu: Menu,
    new_file: MenuItem,
    open_file: MenuItem,
    save_file: MenuItem,
    save_as: MenuItem,
    settings: MenuItem,
    about: MenuItem,
}

impl AppMenu {
    pub fn new(locale: &Locale) -> Self {
        let new_file = MenuItem::new(
            &locale.menu["new_file_menu_button"],
            true,
            None,
        );

        let open_file = MenuItem::new(
            &locale.menu["open_projects_file_menu_button"],
            true,
            None,
        );

        let save_file = MenuItem::new(
            &locale.menu["save_file_menu_button"],
            true,
            None,
        );

        let save_as = MenuItem::new(
            &locale.menu["save_as_file_menu_button"],
            true,
            None,
        );

        let settings = MenuItem::new(
            &locale.menu["settings_menu_button"],
            true,
            None,
        );

        let about = MenuItem::new(
            &locale.menu["about_software_menu_button"],
            true,
            None,
        );

        let file = Submenu::with_items(
            &locale.menu["file_menu_dropdown"],
            true,
            &[
                &new_file,
                &open_file,
                &PredefinedMenuItem::separator(),
                &save_file,
                &save_as,
            ],
        )
        .unwrap();

        let misc = Submenu::with_items(
            &locale.menu["misc_menu_dropdown"],
            true,
            &[
                &settings,
                &about,
            ],
        )
        .unwrap();
        #[cfg(not(target_os = "macos"))]
        let menu = Menu::with_items(&[
            &file,
            &misc,
        ])
        .unwrap();
        // misc must be first on macos
        #[cfg(target_os = "macos")]
        let menu = Menu::with_items(&[
            &misc,
            &file,
        ])
        .unwrap();

        Self {
            menu,
            new_file,
            open_file,
            save_file,
            save_as,
            settings,
            about,
        }
    }

    pub fn set_locale(&self, locale: &Locale) {
        self.new_file.set_text(
            &locale.menu["new_file_menu_button"]
        );

        self.open_file.set_text(
            &locale.menu["open_projects_file_menu_button"]
        );

        self.save_file.set_text(
            &locale.menu["save_file_menu_button"]
        );

        self.save_as.set_text(
            &locale.menu["save_as_file_menu_button"]
        );

        self.settings.set_text(
            &locale.menu["settings_menu_button"]
        );

        self.about.set_text(
            &locale.menu["about_software_menu_button"]
        );
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let ids = (
            self.new_file.id().clone(),
            self.open_file.id().clone(),
            self.save_file.id().clone(),
            self.save_as.id().clone(),
            self.settings.id().clone(),
            self.about.id().clone(),
        );

        Subscription::run_with(ids, |ids: &(muda::MenuId, muda::MenuId, muda::MenuId, muda::MenuId, muda::MenuId, muda::MenuId)| {
            let ids = ids.clone();
            iced::stream::channel(100, move |mut output: iced_mpsc::Sender<Message>| async move {
                use iced::futures::StreamExt;
                
                let (event_sender, event_receiver) = std_mpsc::channel::<muda::MenuEvent>();

                muda::MenuEvent::set_event_handler(Some(move |event| {
                    let _ = event_sender.send(event);
                }));

                
                let (mut async_tx, mut async_rx) = iced_mpsc::channel::<muda::MenuEvent>(100);

                std::thread::spawn(move || {
                    while let Ok(event) = event_receiver.recv() {
                        if iced::futures::executor::block_on(async_tx.send(event)).is_err() {
                            break;
                        }
                    }
                });

                while let Some(event) = async_rx.next().await {
                    let message = if event.id == ids.0 {
                        Message::NewFile
                    } else if event.id == ids.1 {
                        Message::OpenFile
                    } else if event.id == ids.2 {
                        Message::SaveFile
                    } else if event.id == ids.3 {
                        Message::SaveAs
                    } else if event.id == ids.4 {
                        Message::SettingsWindowOpen
                    } else if event.id == ids.5 {
                        Message::AboutWindowOpen
                    } else {
                        continue;
                    };

                    if output.send(message).await.is_err() {
                        break;
                    }
                }
            })
        })
    }
}

pub fn menu_subscription(state: &State) -> Subscription<Message> {
    state.main_window_menu_bar.subscription()
}