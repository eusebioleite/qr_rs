use iced::alignment::Horizontal;
use iced::widget::{Space, button, column, container, row, space, text, text_input};
use iced::{Alignment, Background, Color, Element, Length, Task, Theme, window};
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
    CloseRequested,
    MinimizeRequested
}

pub fn main() -> iced::Result {
    let window_settings = window::Settings {
        size: iced::Size::new(320.0, 180.0), // Tamanho real da janela
        resizable: false,                    // Opcional: trava o redimensionamento
        decorations: false,
        exit_on_close_request: true,
        closeable: true,
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
    .theme(|_state: &State| Theme::CatppuccinLatte)
    .centered()
    .run()
}
fn update(state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        Message::GenerateQrCode => {
                match generate_qr_code(&state.content) {
                Ok(()) => println!("Success!"),
                Err(_) => println!("Error..."),
            }
            Task::none()
        },
        Message::ContentChanged(content) => {
            state.content = content;
            Task::none()
        },
        Message::CloseRequested => {
            iced::exit()
        },
        Message::MinimizeRequested => {
            iced::exit()
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {// 1. A BARRA (Preta, no topo)
    let title_bar = container(
        row![
            Space::new().width(Length::Fill),
            button(text("—").size(12))
            //background: Some(Color::from_rgb8(243, 156, 18).into()), // Laranja/Warning
            .style(|_theme, status| {
                let base_color = Color::from_rgb8(233, 146, 9); // Amarelo base
                let hover_color = Color::from_rgb8(243, 156, 18); // Amarelo mais claro ao passar o mouse

                button::Style {
                    background: match status {
                        iced::widget::button::Status::Hovered => Some(hover_color.into()),
                        iced::widget::button::Status::Pressed => Some(Color::BLACK.into()), // Feedback de clique
                        _ => Some(base_color.into()),
                    },
                    text_color: Color::WHITE,
                    border: iced::Border {
                        radius: 0.0.into(), // Quadrado absoluto
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
            .padding(3)
            .on_press(Message::MinimizeRequested),
            button(text("✕").size(12))
               .style(|_theme, status| {
                    let base_color = Color::from_rgb8(231, 76, 60); // Vermelho base
                    let hover_color = Color::from_rgb8(255, 100, 100); // Vermelho mais claro ao passar o mouse

                    button::Style {
                        background: match status {
                            iced::widget::button::Status::Hovered => Some(hover_color.into()),
                            iced::widget::button::Status::Pressed => Some(Color::BLACK.into()), // Feedback de clique
                            _ => Some(base_color.into()),
                        },
                        text_color: Color::WHITE,
                        border: iced::Border {
                            radius: 0.0.into(), // Quadrado absoluto
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
                .padding(3)
                .on_press(Message::CloseRequested)
        ]
    )
    .width(Length::Fill)
    .height(20)
    .style(|_| container::Style {
        background: Some(Color::from_rgb8(1, 120, 255).into()),
        ..Default::default()
    });

    // 2. O CONTEÚDO (Centralizado no espaço restante)
    let main_body = container(
        column![
            text_input("Sua URL aqui...", &state.content)
                .on_input(Message::ContentChanged)
                .padding(12),
            button("Gerar")
                .padding(10)
                .on_press(Message::GenerateQrCode)
        ]
        .spacing(20)
        .align_x(Horizontal::Center)
        .max_width(300)
    )
    .width(Length::Fill)
    .height(Length::Fill) // Isso faz o container ocupar todo o resto da tela
    .center_x(Length::Fill);

    // 3. O RETORNO (A coluna mestre que empilha os dois)
    column![
        title_bar,
        main_body
    ]
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
