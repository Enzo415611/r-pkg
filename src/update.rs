use iced::{Task, widget::pane_grid};

use crate::{AppState, Message, view::view::Panes};

impl AppState {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Init => {
                if let Ok(pkgs) = self.sync_alpm_dbs() {
                    self.alpm_state.start_pkgs = pkgs;
                }
                Task::none()
            }
            Message::PkgSelected(pkg_name) => {
                self.alpm_state.pkg_selected = pkg_name;
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
        }
    }
}
