#[derive(Debug, Clone)]
pub enum Message {

    // Success
    ClearNotification,
    
    // Button
    GenerateQrCode,

    // Text Input
    UrlChanged(String),

    // Default
    WindowDrag,
    CloseRequested,
    MinimizeRequested,
}