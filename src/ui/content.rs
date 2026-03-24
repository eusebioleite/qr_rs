use iced::widget::{text, text_input, column, button, container};
use iced::{Alignment, Element, Length};

use crate::core::message::Message;
use crate::core::state::State;

pub fn render<'a>(state: &State) -> Element<'a, Message> {
    container(
        column![
            text("Gerador de QR Code").size(20),
            text_input("Digite a URL...", &state.url).on_input(Message::UrlChanged),
            button(text("Gerar")).on_press(Message::GenerateQrCode)
        ].padding(10).spacing(10).align_x(Alignment::Center)
    )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .into()
}