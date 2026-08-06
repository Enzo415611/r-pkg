mod alpm;
mod update;
mod view;

use std::fmt;

use iced::{
    Subscription, Task,
    widget::pane_grid::{self, Pane},
};

use crate::{
    alpm::{AlpmPkg, AlpmState},
    view::{
        terminal::{Terminal, TerminalEvent},
        view::UiState,
    },
};

struct AppState {
    ui_state: UiState,
    alpm_state: AlpmState,
    terminal: Terminal,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("ui_state", &self.ui_state)
            .field("alpm_state", &self.alpm_state)
            .finish()
    }
}

#[derive(Debug, Clone)]
enum Message {
    Init,
    PkgSelected(AlpmPkg),
    ResizePane(pane_grid::ResizeEvent),
    ClonePane(Option<Pane>),
    SearchInputChanged(String),
    SearchInputSubmit,
    Terminal(iced_term::Event),
}

fn main() -> iced::Result {
    iced::application(AppState::new, AppState::update, AppState::view)
        .subscription(AppState::subscription)
        .run()
}

impl AppState {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                alpm_state: AlpmState::default(),
                ui_state: UiState::default(),
                terminal: Terminal::new(),
            },
            Task::batch(vec![Task::done(Message::Init)]),
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        let term = self.terminal.term.subscription().map(Message::Terminal);
        Subscription::batch([term])
    }
}
