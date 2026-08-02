use iced::{Element, widget::container};

use crate::{AppState, Message};

impl AppState {
    pub fn terminal_view(&self) -> Element<'_, Message> {
        container("terminal").into()
    }
}
