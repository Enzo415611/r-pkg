use iced::{
    Alignment, Border, Color, Element,
    Length::Fill,
    widget::{button, column, container, row, text},
};

use crate::{AppState, Message};

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

        let install_pkg_button = button(is_installed).on_press_with(|| {
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
                text(*db),
                text(self.alpm_state.pkg_selected.size),
                text(*desc),
                install_pkg_button,
                self.terminal_view()
            ]
            .width(Fill)
            .align_x(Alignment::Center),
        ]
        .padding(5.0);
        container(column).style(|_| container_style()).into()
    }
}

fn container_style() -> container::Style {
    container::Style {
        border: Border {
            color: Color::from_rgba8(83, 83, 83, 0.8),
            width: 1.0,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn button_style(s: button::Status) -> button::Style {
    match s {
        button::Status::Hovered => button::Style {
            text_color: Color::WHITE,
            border: iced::Border {
                color: Color::from_rgb8(65, 69, 89),
                width: 1.0,
                radius: iced::border::Radius::new(15.0),
            },
            background: Some(iced::Background::Color(Color::WHITE.scale_alpha(0.3))),
            ..Default::default()
        },
        _ => button::Style {
            text_color: Color::WHITE,
            border: iced::Border {
                color: Color::from_rgb8(65, 69, 89),
                width: 1.0,
                radius: iced::border::Radius::new(15.0),
            },
            ..Default::default()
        },
    }
}
