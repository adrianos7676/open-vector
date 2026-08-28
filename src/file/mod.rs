use std::path::PathBuf;

use iced::{Task};

use crate::{Message, State};

pub fn message_new_file() -> Task<Message> {
    Task::perform(
        async {
            rfd::AsyncFileDialog::new()
            .add_filter("Open Vector Project", &["ovp"])
            .set_file_name("Project.ovp")
            .save_file()
            .await
            .map(|file| file.path().to_owned())
        },
    Message::FileSelected)
}

pub fn message_open_file() -> Task<Message> {
    Task::perform(
        async {
            rfd::AsyncFileDialog::new()
                .add_filter("Open Vector Project", &["ovp"])
                .pick_file()
                .await
                .map(|file| file.path().to_owned())
        },
        Message::FileSelected,
    )
}

pub fn message_file_selected(path: Option<PathBuf>, state: &mut State) -> Task<Message> {
    if let Some(path) = path {
        if state.open_projects.values().any(|value| value == &path.to_string_lossy()) {
            return Task::none();
        }

        if !path.exists() {
            if let Err(error) = std::fs::File::create(&path) {
                return Task::perform(
                    async move {
                        rfd::AsyncMessageDialog::new()
                            .set_title("Error")
                            .set_description(format!("Error: {}", error))
                            .show()
                            .await;
                    },
                    |_| Message::NoOp,);

                }
            }

            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    state.open_projects.insert(
                    file_name.to_string_lossy().to_string(),
                    path.to_string_lossy().to_string(),
                );
            }
        }
    }
    Task::none()
}