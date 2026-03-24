
use iced::window;
use crate::core::message::Message;
use crate::core::state::State;
use crate::core::utils::generate_qr_code;

pub fn handle(state: &mut State, message: Message) -> iced::Task<Message> {
    match message {

        // Notification
        Message::ClearNotification => {
            state.show_success = false;
            iced::Task::none()
        }

        // Button
        Message::GenerateQrCode => {
            generate_qr_code(&state.url);
            state.show_success = true;
            iced::Task::perform(
                async { std::thread::sleep(std::time::Duration::from_secs(2)); },
                |_| Message::ClearNotification
            )
        }

        // Text Input
        Message::UrlChanged(url) => {
            state.url = url;
            iced::Task::none()
        }
        // Default
        Message::WindowDrag => window::oldest().and_then(|id| window::drag(id)),
        Message::CloseRequested => iced::exit(),
        Message::MinimizeRequested => window::oldest().and_then(|id| window::minimize(id, true)),
    }
}