
use arboard::{Clipboard, ImageData};
use image::{Luma};
use qrcode::QrCode;
use std::borrow::Cow;

pub fn generate_qr_code(url: &str) {
    let code = QrCode::new(url.as_bytes()).expect("[ERR: QR_GEN_FAILED]");

    let image_luma = code.render::<Luma<u8>>().build();
    let (width, height) = image_luma.dimensions();

    let mut rgba_buffer = Vec::with_capacity((width * height * 4) as usize);
    for pixel in image_luma.pixels() {
        let luma = pixel.0[0];
        rgba_buffer.extend_from_slice(&[luma, luma, luma, 255]); // R, G, B, Alpha Full
    }

    let mut clipboard = Clipboard::new().expect("[ERR: CLIPBOARD_ACCESS_DENIED]");

    let data = ImageData {
        width: width as usize,
        height: height as usize,
        bytes: Cow::from(rgba_buffer),
    };

    clipboard.set_image(data).expect("[ERR: CLIPBOARD_INJECTION_FAILED]");
    
}