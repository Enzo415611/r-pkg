mod alpm;
mod update;
mod view;

use iced::{
    Task,
    widget::pane_grid::{self, Pane},
};

use crate::{
    alpm::{AlpmPkg, AlpmState},
    view::view::UiState,
};

#[derive(Debug)]
struct AppState {
    ui_state: UiState,
    alpm_state: AlpmState,
}

#[derive(Debug, Clone)]
enum Message {
    Init,
    PkgSelected(AlpmPkg),
    ResizePane(pane_grid::ResizeEvent),
    ClonePane(Option<Pane>),
    SearchInputChanged(String),
    SearchInputSubmit,
}

fn main() -> iced::Result {
    iced::application(AppState::new, AppState::update, AppState::view).run()
}

impl AppState {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                alpm_state: AlpmState::default(),
                ui_state: UiState::default(),
            },
            Task::batch(vec![Task::done(Message::Init)]),
        )
    }
}
