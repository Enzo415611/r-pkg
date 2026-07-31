use iced::{
    Element,
    widget::{column, container, row},
};

use crate::{AppState, Message};

impl AppState {
    pub fn view(&self) -> Element<'_, Message> {
        container(column![row!["search"], self.pkg_list_view()]).into()
    }
}
