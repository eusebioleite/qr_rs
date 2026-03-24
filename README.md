# qr_rs

**Simple and minimalist QR Code generator in Rust** ✨

A lightweight GUI application fully developed in Rust and Iced that generates QR Codes from URLs and **automatically copies the image to the clipboard**. Perfect for anyone who wants to generate QR codes quickly and effortlessly.

<img width="398" height="225" alt="image" src="https://github.com/user-attachments/assets/4d71c0e9-6ac7-47cf-b469-e20db9f9952e" />

## 🛠 Technologies

- **Rust** (edition 2024)
- [Iced](https://iced.rs) — framework GUI
- [qrcode](https://crates.io/crates/qrcode) — QR generation
- [image](https://crates.io/crates/image) — image processing
- [arboard](https://crates.io/crates/arboard) — clipboard

## 🚀 How to Use

### 1. Clone the repository
```bash
git clone https://github.com/eusebioleite/qr_rs.git
cd qr_rs
```

### 2. Execute
```bash
cargo run
```

## 📦 Build
```bash
cargo build --release
```

The executable will be in `target/release/qr_rs`
