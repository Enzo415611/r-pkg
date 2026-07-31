use iced::{Element, widget::{column, container, text}};

use crate::{AppState, Message};

impl AppState {
    fn selected_pkg_view(&self) -> Element<'_, Message> {
        let column = column![
            
        ];
        container(column).into()
    }
}
