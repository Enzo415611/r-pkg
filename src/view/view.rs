use iced::{
    Element,
    widget::{column, container, pane_grid, pane_grid::Pane, row},
};

use crate::{AppState, Message};

#[derive(Debug)]
pub struct UiState {
    pub pane_grid_state: pane_grid::State<Panes>,
    pub pkg_list_pane: Pane,
    pub pkg_selected_pane: Option<Pane>,
}

impl UiState {
    pub fn default() -> Self {
        let (panes_state, pkg_list_pane) = pane_grid::State::new(Panes::Pkg);
        Self {
            pane_grid_state: panes_state,
            pkg_list_pane,
            pkg_selected_pane: None,
        }
    }
}

#[derive(Debug)]
pub enum Panes {
    Pkg,
    PkgInstall,
}

impl AppState {
    pub fn view(&self) -> Element<'_, Message> {
        let pane_grid_pkg = pane_grid(&self.ui_state.pane_grid_state, |_, s, _| {
            pane_grid::Content::new(match s {
                Panes::Pkg => self.pkg_list_view(),
                Panes::PkgInstall => self.pkg_selected_view(),
            })
        })
        .on_resize(30, Message::ResizePane);

        container(column![row!["search"], pane_grid_pkg]).into()
    }
}
