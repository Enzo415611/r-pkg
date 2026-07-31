use iced::{
    Border, Color, Element,
    Length::{Fill, Shrink},
    Padding,
    widget::{Scrollable, button, column, container, text},
};

use crate::{AppState, Message};

impl AppState {
    pub fn pkg_list_view(&self) -> Element<'_, Message> {
        let column = column![].extend(self.alpm_state.start_pkgs.iter().map(|pkg| {
            button(text(&pkg.name))
                .on_press(Message::PkgSelected(pkg.to_owned()))
                .style(|_, s| button_style(s))
                .width(Shrink)
                .into()
        }));

        container(Scrollable::new(column).width(Fill).height(Fill))
            .width(Fill)
            .height(Fill)
            .padding(Padding::default().right(14.0))
            .style(|_| container_style())
            .into()
    }
}

// style
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
            background: Some(iced::Background::Color(Color::WHITE.scale_alpha(0.2))),
            ..Default::default()
        },
        _ => button::Style {
            text_color: Color::WHITE,
            background: Some(iced::Background::Color(Color::WHITE.scale_alpha(0.1))),
            ..Default::default()
        },
    }
}
