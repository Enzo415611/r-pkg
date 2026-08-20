use iced::{
    Alignment, Element,
    Length::Fill,
    widget::{button, column, container, row, text},
};

use crate::{
    AppState, Message,
    view::style::style::{button_style, container_style},
};

impl AppState {
    pub fn pkg_selected_view(&self) -> Element<'_, Message> {
        let pkg_selected = &self.alpm_state.pkg_selected;

        let db = &pkg_selected.db.as_deref().unwrap_or_default();

        let desc = &pkg_selected.desc.as_deref().unwrap_or_default();

        let is_installed = if pkg_selected.is_installed {
            "Uninstall"
        } else {
            "Install"
        };

        let install_pkg_button = button(is_installed)
            .style(|_, s| button_style(s))
            .on_press_with(|| {
                if pkg_selected.is_installed {
                    return Message::Uninstall;
                }
                return Message::InstallPkg;
            });

        let column = column![
            row![
                button("X")
                    .style(|_, s| button_style(s))
                    .on_press(Message::ClonePane(self.ui_state.pkg_selected_pane))
            ],
            column![
                text(&self.alpm_state.pkg_selected.name).size(25.0),
                text(format!("repository: {}", *db)),
                text(format!("size: {}", self.alpm_state.pkg_selected.size)),
                text(*desc),
                install_pkg_button
            ]
            .width(Fill)
            .align_x(Alignment::Center),
        ]
        .padding(5.0);
        container(column).style(|_| container_style()).into()
    }
}
