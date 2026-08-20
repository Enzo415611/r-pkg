use iced::{
    Border, Color,
    widget::{button, container},
};

pub fn button_style(s: button::Status) -> button::Style {
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

pub fn pkg_list_button_style(s: button::Status) -> button::Style {
    match s {
        button::Status::Hovered => button::Style {
            text_color: Color::WHITE,
            border: iced::Border {
                color: Color::from_rgb8(65, 69, 89),
                width: 1.0,
                radius: iced::border::Radius::new(0),
            },
            background: Some(iced::Background::Color(Color::WHITE.scale_alpha(0.3))),
            ..Default::default()
        },
        _ => button::Style {
            text_color: Color::WHITE,
            border: iced::Border {
                color: Color::from_rgb8(65, 69, 89),
                width: 1.0,
                radius: iced::border::Radius::new(0),
            },
            ..Default::default()
        },
    }
}

pub fn container_style() -> container::Style {
    container::Style {
        border: Border {
            color: Color::from_rgba8(83, 83, 83, 0.8),
            width: 1.0,
            ..Default::default()
        },
        ..Default::default()
    }
}