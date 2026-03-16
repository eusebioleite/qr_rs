use iced::widget::{button, column, text, text_input};
use iced::Element;
use image::Luma;
use qrcode::QrCode;

pub fn main() -> iced::Result {
    iced::run(update, view)
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::GenerateQrCode(url) => match generate_qr_code(&url) {
            Ok(()) => println!("Success!"),
            Err(_) => println!("Error..."),
        },
        Message::ContentChanged(content) => {
            state.content = content;
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    column![
        text("Digite a url"),
        text_input("Type something here...", &state.content).on_input(Message::ContentChanged),
        button("Criar QR Code").on_press(Message::GenerateQrCode(String::from(&state.content)))
    ]
    .spacing(10)
    .into()
}
fn generate_qr_code(url: &String) -> Result<(), Box<dyn std::error::Error>> {
    // Geração dos bits do QR
    let code = QrCode::new(url.as_bytes())?;

    // Renderização para imagem (Luma = Tons de cinza/Preto e Branco)
    let image = code.render::<Luma<u8>>().build();

    // Persistência no storage D: ou C: conforme sua configuração de SSD
    image.save("qrcode_output.png")?;
    Ok(())
}

#[derive(Default)]
struct State {
    content: String,
}

#[derive(Debug, Clone)]
enum Message {
    GenerateQrCode(String),
    ContentChanged(String),
}
