use iced::widget::{button, column, container, row, text, MouseArea, Space};
use iced::{window, Color, Element, Length, Theme};

// MAIN LOOP

#[derive(Default)]
struct State {}

#[derive(Debug, Clone)]
enum Message {
    WindowDrag,
    CloseRequested,
    MinimizeRequested,
}

pub fn main() -> iced::Result {
    let window_settings = window::Settings {
        size: iced::Size::new(320.0, 180.0),
        resizable: false,
        decorations: false,
        exit_on_close_request: true,
        closeable: true,
        ..Default::default()
    };
    iced::application(|| State {}, update, view)
        .title(|_state: &State| "Template".to_string())
        .window(window_settings)
        .theme(|_state: &State| Theme::CatppuccinLatte)
        .centered()
        .run()
}
fn update(_state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        Message::WindowDrag => window::latest().and_then(|id| window::drag(id)),
        Message::CloseRequested => iced::exit(),
        Message::MinimizeRequested => window::latest().and_then(|id| window::minimize(id, true)),
    }
}

fn view(_state: &State) -> Element<'_, Message> {
    // TOOLBAR
    let toolbar = MouseArea::new(
        container(row![
            Space::new().width(Length::Fill),
            button(text("—").size(12))
                .style(|_theme, status| {
                    let base_color = Color::from_rgb8(233, 146, 9);
                    let hover_color = Color::from_rgb8(243, 156, 18);
                    let pressed_color = Color::from_rgb8(243, 156, 18);
                    button::Style {
                        background: match status {
                            iced::widget::button::Status::Hovered => Some(hover_color.into()),
                            iced::widget::button::Status::Pressed => Some(pressed_color.into()),
                            _ => Some(base_color.into()),
                        },
                        text_color: Color::WHITE,
                        border: iced::Border {
                            radius: (0.0).into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
                .padding(3)
                .on_press(Message::MinimizeRequested),
            button(text("✕").size(12))
                .style(|_theme, status| {
                    let base_color = Color::from_rgb8(231, 76, 60);
                    let hover_color = Color::from_rgb8(255, 100, 100);
                    let pressed_color = Color::from_rgb8(255, 100, 100);
                    button::Style {
                        background: match status {
                            iced::widget::button::Status::Hovered => Some(hover_color.into()),
                            iced::widget::button::Status::Pressed => Some(pressed_color.into()),
                            _ => Some(base_color.into()),
                        },
                        text_color: Color::WHITE,
                        border: iced::Border {
                            radius: (0.0).into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
                .padding(3)
                .on_press(Message::CloseRequested)
        ])
        .width(Length::Fill)
        .height(20)
        .style(|_| container::Style {
            background: Some(Color::from_rgb8(1, 120, 255).into()),
            ..Default::default()
        }),
    )
    .on_press(Message::WindowDrag);

    // CONTENT
    let content = container(Space::new())
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill);

    // OUTPUT: TOOLBAR + CONTENT
    column![toolbar, Space::new().height(10), content].into()
}
