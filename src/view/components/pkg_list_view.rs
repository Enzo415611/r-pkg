use iced::{
    Alignment, Color, Element,
    Length::{self, Fill, Shrink},
    widget::{
        Scrollable, button, column, container, mouse_area, row,
        scrollable::{Direction, Scrollbar},
        space, text,
    },
};

use crate::{
    AppState, Message,
    view::style::style::{container_style, pkg_list_button_style},
};

impl AppState {
    pub fn pkg_list_view(&self) -> Element<'_, Message> {
        let column = column![
            row![
                self.packages_column(),
                self.repository_column(),
                self.description_column()
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

    fn packages_column(&self) -> Element<'_, Message> {
        column![row![text("Packages")]]
            .extend(self.alpm_state.pkg_list.iter().map(|pkg| {
                let selected = self.alpm_state.pkg_selected.name == pkg.name;

                let background_color: Color;

                if selected {
                    if self.pkg_is_installed(&pkg.name) {
                        background_color = Color::from_rgb8(0, 255, 20);
                    } else {
                        background_color = Color::from_rgb8(255, 0, 0);
                    }
                } else {
                    if pkg.is_installed {
                        background_color = Color::from_rgb8(0, 255, 20);
                    } else {
                        background_color = Color::from_rgb8(255, 0, 0);
                    }
                };

                row![
                    mouse_area(
                        container("")
                            .width(20)
                            .height(Length::Fill)
                            .style(move |_| {
                                container::Style {
                                    background: Some(iced::Background::Color(background_color)),
                                    ..Default::default()
                                }
                            })
                    )
                    .on_press(Message::PkgSelected(pkg.clone())),
                    button(text(&pkg.name))
                        .on_press(Message::PkgSelected(pkg.to_owned()))
                        .style(|_, s| pkg_list_button_style(s))
                        .width(Shrink)
                ]
                .into()
            }))
            .spacing(3.0)
            .into()
    }

    fn repository_column(&self) -> Element<'_, Message> {
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
            .spacing(3.0)
            .into()
    }

    fn description_column(&self) -> Element<'_, Message> {
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
            .into()
    }
}
