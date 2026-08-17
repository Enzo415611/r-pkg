use iced::{Task, widget::pane_grid};

use crate::{
    AppState, Message,
    view::view::{Pages, Panes},
};

impl AppState {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Init => {
                if let Ok(pkgs) = self.sync_alpm_dbs() {
                    self.alpm_state.start_pkg_list = pkgs;
                    self.alpm_state.pkg_list = self.alpm_state.start_pkg_list.to_owned();
                }
                Task::none()
            }
            Message::CurrentPage(p) => {
                self.ui_state.current_page = p;
                Task::none()
            }
            Message::PkgSelected(mut pkg) => {
                let is_installed = self.pkg_is_installed(&pkg.name);
                pkg.is_installed = is_installed;

                self.alpm_state.pkg_selected = pkg;
                if self.ui_state.pkg_selected_pane.is_none() {
                    if let Some((pane, _)) = self.ui_state.pane_grid_state.split(
                        pane_grid::Axis::Vertical,
                        self.ui_state.pkg_list_pane,
                        Panes::PkgInstall,
                    ) {
                        self.ui_state.pkg_selected_pane = Some(pane)
                    }
                }

                Task::none()
            }
            Message::ResizePane(r) => {
                self.ui_state.pane_grid_state.resize(r.split, r.ratio);
                Task::none()
            }
            Message::ClonePane(pane) => {
                if let Some(pane) = pane {
                    self.ui_state.pane_grid_state.close(pane);
                    self.ui_state.pkg_selected_pane = None;
                }
                Task::none()
            }
            Message::SearchInputChanged(text) => {
                self.ui_state.search_content = text;
                Task::none()
            }
            Message::SearchInputSubmit => {
                if self.ui_state.search_content.is_empty() {
                    self.alpm_state.pkg_list = self.alpm_state.start_pkg_list.to_owned();
                } else {
                    let pkgs = self.search_pkg_by_name();
                    self.alpm_state.pkg_list = pkgs;
                }

                Task::none()
            }
            Message::Terminal(event) => {
                match event {
                    iced_term::Event::BackendCall(_, cmd) => {
                        match self
                            .terminal
                            .term
                            .handle(iced_term::Command::ProxyToBackend(cmd.clone()))
                        {
                            _ => match cmd {
                                iced_term::BackendCommand::ProcessAlacrittyEvent(event) => {
                                    match event {
                                        iced_term::AlacrittyEvent::PtyWrite(_) => {
                                            let is = self.pkg_is_installed(
                                                &self.alpm_state.pkg_selected.name,
                                            );
                                            if is != self.alpm_state.pkg_selected.is_installed {
                                                self.terminal
                                                    .term
                                                    .handle(iced_term::Command::ProxyToBackend(
                                                        iced_term::BackendCommand::ProcessAlacrittyEvent(
                                                            iced_term::AlacrittyEvent::PtyWrite("clear\n".to_string()),
                                                        ),
                                                    ));
                                                self.ui_state.current_page = Pages::Home;
                                            }
                                            self.alpm_state.pkg_selected.is_installed = is;
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            },
                        }
                    }
                    _ => {}
                }
                Task::none()
            }
            Message::InstallPkg => {
                self.ui_state.current_page = Pages::InstallPkg;
                let command = iced_term::Command::ProxyToBackend(iced_term::BackendCommand::Write(
                    format!("sudo pacman -S {}\n", &self.alpm_state.pkg_selected.name)
                        .as_bytes()
                        .to_vec(),
                ));
                self.terminal.term.handle(command);
                Task::none()
            }
            Message::Uninstall => {
                self.ui_state.current_page = Pages::InstallPkg;
                let command = iced_term::Command::ProxyToBackend(iced_term::BackendCommand::Write(
                    format!("sudo pacman -R {}\n", &self.alpm_state.pkg_selected.name)
                        .as_bytes()
                        .to_vec(),
                ));
                self.terminal.term.handle(command);
                Task::none()
            }
            Message::CancelPkg => {
                self.terminal
                    .term
                    .handle(iced_term::Command::ProxyToBackend(
                        iced_term::BackendCommand::Write("\x04\n".as_bytes().to_vec()),
                    ));

                self.terminal
                    .term
                    .handle(iced_term::Command::ProxyToBackend(
                        iced_term::BackendCommand::ProcessAlacrittyEvent(
                            iced_term::AlacrittyEvent::PtyWrite("clear\n".to_string()),
                        ),
                    ));
                self.ui_state.current_page = Pages::Home;
                Task::none()
            }
        }
    }
}
