use iced::{
    Alignment, Border, Color, Element,
    Length::{Fill, Shrink},
    widget::{
        Scrollable, button, column, container, row,
        scrollable::{Direction, Scrollbar},
        space, text,
    },
};

use crate::{AppState, Message, view::style::style::pkg_list_button_style};

impl AppState {
    pub fn pkg_list_view(&self) -> Element<'_, Message> {
        let column = column![
            row![
                column![row![text("Packages")]]
                    .extend(self.alpm_state.pkg_list.iter().map(|pkg| {
                        row![
                            button(text(&pkg.name))
                                .on_press(Message::PkgSelected(pkg.to_owned()))
                                .style(|_, s| pkg_list_button_style(s))
                                .width(Shrink)
                        ]
                        .into()
                    }))
                    .spacing(3.0),
                column![row![text("Repositorys"), space()]]
                    .align_x(Alignment::Center)
                    .extend(self.alpm_state.pkg_list.iter().map(|pkg| {
                        row![
                            button(text(pkg.db.as_deref().unwrap_or_default()))
                                .style(|_, s| pkg_list_button_style(s))
                                .on_press(Message::PkgSelected(pkg.to_owned()))
                        ]
                        .into()
                    }))
                    .spacing(3.0),
                column![row![text("Description"), space()]]
                    .extend(self.alpm_state.pkg_list.iter().map(|pkg| {
                        row![
                            button(text(pkg.desc.as_deref().unwrap_or_default()))
                                .style(|_, s| pkg_list_button_style(s))
                                .on_press(Message::PkgSelected(pkg.to_owned()))
                        ]
                        .into()
                    }))
                    .spacing(3.0)
            ]
            .spacing(10.0)
            .height(Fill)
        ]
        .padding(5.0);

        container(
            Scrollable::new(column)
                .direction(Direction::Both {
                    vertical: Scrollbar::new(),
                    horizontal: Scrollbar::new(),
                })
                .width(Fill)
                .height(Fill),
        )
        .width(Fill)
        .height(Fill)
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
