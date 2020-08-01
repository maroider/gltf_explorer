use std::sync::Arc;

use iced::{
    executor,
    widget::{
        button::{self, Button},
        Container, Row, Text,
    },
    Align, Application, Command, Element, Length, Subscription,
};
use log::warn;

use super::Args;

mod subscriptions;
mod widgets;

pub(crate) struct Explorer {
    open_file_btn: button::State,
    state: State,
}

impl Application for Explorer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Args;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let state = flags
            .file
            .map(|file| {
                State::ExploringDocument(
                    subscriptions::import_gltf::Document::import(file).unwrap(),
                    Exploration::default(),
                )
            })
            .unwrap_or(State::Initial);
        (
            Self {
                open_file_btn: button::State::new(),
                state,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let app_name = "glTF Explorer";

        match &self.state {
            State::Initial | State::ChoosingInitialDocument => String::from(app_name),
            State::ExploringDocument(document, _) | State::ChoosingNewDocument(document, _) => {
                let document_name = document
                    .path
                    .file_name()
                    .unwrap_or("<unnamed file>".as_ref())
                    .to_string_lossy();

                format!("{} - {}", document_name, app_name)
            }
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::OpenFileDialog => match &self.state {
                State::Initial => self.state = State::ChoosingInitialDocument,
                State::ExploringDocument(document, exploration) => {
                    self.state = State::ChoosingNewDocument(document.clone(), exploration.clone())
                }
                State::ChoosingInitialDocument | State::ChoosingNewDocument(_, _) => {}
            },
            Message::DocumentOpenProgress(progress) => match self.state {
                State::Initial | State::ExploringDocument(_, _) => {}
                State::ChoosingInitialDocument | State::ChoosingNewDocument(_, _) => {
                    use subscriptions::import_gltf::PickAndImport;
                    match progress {
                        PickAndImport::NoFileSelected => match &self.state {
                            State::Initial | State::ExploringDocument(_, _) => {}
                            State::ChoosingInitialDocument => self.state = State::Initial,
                            State::ChoosingNewDocument(document, exploration) => {
                                self.state =
                                    State::ExploringDocument(document.clone(), exploration.clone())
                            }
                        },
                        PickAndImport::Finished(document) => {
                            self.state =
                                State::ExploringDocument(document.clone(), Exploration::default())
                        }
                        PickAndImport::Errored(error) => {
                            warn!("Could not open glTF document: {}", error);
                            match &self.state {
                                State::Initial | State::ExploringDocument(_, _) => {}
                                State::ChoosingInitialDocument => self.state = State::Initial,
                                State::ChoosingNewDocument(document, exploration) => {
                                    self.state = State::ExploringDocument(
                                        document.clone(),
                                        exploration.clone(),
                                    )
                                }
                            }
                        }
                    }
                }
            },
        }

        Command::none()
    }

    fn view<'a>(&'a mut self) -> Element<'a, Self::Message> {
        let mut open_document_button =
            Button::new(&mut self.open_file_btn, Text::new("Open glTF File"));
        if matches!(self.state, State::Initial | State::ExploringDocument(_, _)) {
            open_document_button = open_document_button
                .on_press(Message::OpenFileDialog)
                .into();
        }

        if let State::ExploringDocument(document, exploration)
        | State::ChoosingNewDocument(document, exploration) = &mut self.state
        {
            Row::new()
                .push(
                    Row::new()
                        .push(widgets::document_statistics::stats(
                            &document.document,
                            &mut exploration.document_statistics,
                        ))
                        .push(widgets::gltf_node_tree::tree(
                            &document.document,
                            &mut exploration.gltf_node_tree,
                        )),
                )
                .into()
        } else {
            Container::new(open_document_button)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Align::Center)
                .align_y(Align::Center)
                .into()
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match self.state {
            State::Initial | State::ExploringDocument(_, _) => Subscription::none(),
            State::ChoosingInitialDocument | State::ChoosingNewDocument(_, _) => {
                subscriptions::import_gltf::pick_and_import().map(Message::DocumentOpenProgress)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    OpenFileDialog,
    DocumentOpenProgress(subscriptions::import_gltf::PickAndImport),
}

enum State {
    Initial,
    ChoosingInitialDocument,
    ExploringDocument(Arc<subscriptions::import_gltf::Document>, Exploration),
    ChoosingNewDocument(Arc<subscriptions::import_gltf::Document>, Exploration),
}

#[derive(Clone, Default)]
struct Exploration {
    document_statistics: widgets::document_statistics::State,
    gltf_node_tree: widgets::gltf_node_tree::State,
}
