use iced::{
    Border, Color, Element,
    widget::{
        column, container,
        pane_grid::{self, Pane},
        row, text_input,
    },
};

use crate::{AppState, Message};

#[derive(Debug)]
pub struct UiState {
    pub current_page: Pages,
    pub pane_grid_state: pane_grid::State<Panes>,
    pub pkg_list_pane: Pane,
    pub pkg_selected_pane: Option<Pane>,
    pub search_content: String,
    pub install_mode: bool
}

impl UiState {
    pub fn default() -> Self {
        let (panes_state, pkg_list_pane) = pane_grid::State::new(Panes::Pkg);
        Self {
            pane_grid_state: panes_state,
            pkg_list_pane,
            pkg_selected_pane: None,
            search_content: String::new(),
            current_page: Pages::Home,
            install_mode: false
        }
    }
}

#[derive(Debug, Clone)]
pub enum Pages {
    Home,
    InstallPkg
}

#[derive(Debug)]
pub enum Panes {
    Pkg,
    PkgInstall,
}

impl AppState {
    pub fn view(&self) -> Element<'_, Message> {
        let search_row = row![
            text_input("Package Name", &self.ui_state.search_content)
                .style(|_, s| search_style(s))
                .on_submit(Message::SearchInputSubmit)
                .on_input(Message::SearchInputChanged)
        ];

        let pane_grid_pkg = pane_grid::PaneGrid::new(&self.ui_state.pane_grid_state, |_, s, _| {
            pane_grid::Content::new(match s {
                Panes::Pkg => self.pkg_list_view(),
                Panes::PkgInstall => self.pkg_selected_view(),
            })
        })
        .min_size(200.0)
        .spacing(15.0)
        .on_resize(10, Message::ResizePane);

        match self.ui_state.current_page {
            Pages::Home => {
                container(column![search_row, pane_grid_pkg].spacing(5.0).padding(4.0)).into()
            }
            Pages::InstallPkg => {
                container(self.install_pkg_page()).into()
            }
        }
    }
}

fn search_style(_: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: iced::Background::Color(iced::Color::TRANSPARENT),
        icon: Color::WHITE,
        border: Border::default()
            .color(Color::from_rgb8(65, 69, 89))
            .width(1.0)
            .rounded(10.0),
        placeholder: Color::WHITE.scale_alpha(0.1),
        selection: Color::from_rgb8(0, 100, 255),
        value: Color::WHITE,
    }
}
