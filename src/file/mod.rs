use std::path::PathBuf;

use iced::{Task, Vector};

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
        if state.open_projects.iter().any(|value| value.name == path.to_string_lossy()) {
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
                    let id = state.open_projects.len();
                    state.open_projects.insert(id, crate::Document{ id: id, name: file_name.to_string_lossy().to_string(), zoom: 1.0, offset: Vector { x: 0.0, y: 0.0 } });
                    state.open_project = Some(id);
            }
        }
    }
    Task::none()
}