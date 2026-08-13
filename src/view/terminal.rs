use iced::{Element, Length, widget::container};

use crate::{AppState, Message};

pub struct Terminal {
    pub term: iced_term::Terminal,
}

impl Terminal {
    pub fn new() -> Self {
        #[cfg(not(windows))]
        let system_shell = std::env::var("SHELL")
            .expect("SHELL variable is not defined")
            .to_string();

        #[cfg(windows)]
        let system_shell = "cmd.exe".to_string();

        let term_id = 0;
        let term_settings = iced_term::settings::Settings {
            font: iced_term::settings::FontSettings {
                size: 15.0,
                ..Default::default()
            },
            backend: iced_term::settings::BackendSettings {
                program: system_shell,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut term = iced_term::Terminal::new(term_id, term_settings)
            .expect("failed to create the new terminal instance");

        // clear terminal
        term.handle(iced_term::Command::ProxyToBackend(
            iced_term::BackendCommand::Write("clear\n".as_bytes().to_vec()),
        ));

        Self { term }
    }
}

#[derive(Debug, Clone)]
pub enum TerminalEvent {
    BackendCall(u64, iced_term::BackendCommand),
}

impl AppState {
    pub fn terminal_view(&self) -> Element<'_, Message> {
        container(iced_term::TerminalView::show(&self.terminal.term).map(Message::Terminal))
            .width(400.0)
            .height(Length::Fill)
            .into()
    }
}
