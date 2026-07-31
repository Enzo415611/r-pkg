use iced::{
    Alignment, Color, Element,
    Length::Fill,
    widget::{button, column, container, row, text},
};

use crate::{AppState, Message};

impl AppState {
    pub fn pkg_selected_view(&self) -> Element<'_, Message> {
        let db = self
            .alpm_state
            .pkg_selected
            .db
            .as_deref()
            .unwrap_or_default();

        let desc = self
            .alpm_state
            .pkg_selected
            .desc
            .as_deref()
            .unwrap_or_default();

        let column = column![
            row![button("X").on_press(Message::ClonePane(self.ui_state.pkg_selected_pane))],
            column![
                text(&self.alpm_state.pkg_selected.name).size(25.0),
                text(db),
                text(self.alpm_state.pkg_selected.size),
                text(desc)
            ]
            .width(Fill)
            .align_x(Alignment::Center),
        ];
        container(column).into()
    }
}

fn button_style(s: button::Status) -> button::Style {
    match s {
        button::Status::Hovered => button::Style {
            text_color: Color::WHITE,
            background: Some(iced::Background::Color(Color::WHITE.scale_alpha(0.3))),
            ..Default::default()
        },
        _ => button::Style {
            text_color: Color::WHITE,
            ..Default::default()
        },
    }
}
