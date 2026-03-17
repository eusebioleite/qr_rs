use iced::alignment::Horizontal;
use iced::widget::{button, column, container, text_input};
use iced::{window, Element, Length, Task, Theme};
use image::Luma;
use qrcode::QrCode;

// MAIN LOOP

#[derive(Default)]
struct State {
    content: String,
}

#[derive(Debug, Clone)]
enum Message {
    GenerateQrCode,
    ContentChanged(String),
}

pub fn main() -> iced::Result {
    let window_settings = window::Settings {
        size: iced::Size::new(320.0, 180.0), // Tamanho real da janela
        resizable: false,                    // Opcional: trava o redimensionamento
        ..Default::default()
    };

    // Usamos o builder para rodar com as configurações
    iced::application(
        || State {
            content: String::new(),
        },
        update,
        view,
    )
    .title(|_state: &State| "QR Code Generator".to_string())
    .window(window_settings)
    .theme(|_state: &State| Theme::GruvboxLight)
    .centered()
    .run()
}
fn update(state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        Message::GenerateQrCode => match generate_qr_code(&state.content) {
            Ok(()) => println!("Success!"),
            Err(_) => println!("Error..."),
        },
        Message::ContentChanged(content) => {
            state.content = content;
        }
    }
    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {
    let content = column![
        text_input("Sua URL aqui...", &state.content)
            .on_input(Message::ContentChanged)
            .padding(12),
        button("Gerar")
            .padding(10)
            .on_press(Message::GenerateQrCode)
    ]
    .spacing(20)
    .align_x(Horizontal::Center)
    .max_width(300);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(20)
        .into()
}

// UTILITY FUNCTIONS
fn generate_qr_code(url: &String) -> Result<(), Box<dyn std::error::Error>> {
    let file_name: String = clean_filename(url);
    let code = QrCode::new(url.as_bytes())?;
    let image = code.render::<Luma<u8>>().build();
    image.save(format!("{}.png", file_name))?;
    Ok(())
}

fn clean_filename(input: &str) -> String {
    input
        .chars()
        .filter(|&c| {
            !matches!(
                c,
                '<' | '>'
                    | ':'
                    | '"'
                    | '/'
                    | '\\'
                    | '|'
                    | '?'
                    | '*'
                    | '\''
                    | ' '
                    | '!'
                    | '('
                    | ')'
                    | '~'
                    | '`'
                    | '@'
                    | '#'
                    | '$'
            ) && !c.is_control()
        })
        .collect()
}
