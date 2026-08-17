use iced::{
    Alignment, Element, Length,
    widget::{button, column, container, row},
};

use crate::{AppState, Message};

impl AppState {
    pub fn install_pkg_page(&self) -> Element<'_, Message> {
        let button_cancel = row![button("X").on_press(Message::CancelPkg)]
            .width(Length::Fill)
            .align_y(Alignment::Center);

        let terminal = self.terminal_view();

        container(column![button_cancel, terminal].align_x(Alignment::Center)).into()
    }
}
