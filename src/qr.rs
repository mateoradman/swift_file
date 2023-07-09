use std::net::SocketAddr;

use qrcode::render::unicode;
use qrcode::QrCode;

pub async fn generate_qr_code(server_addr: &SocketAddr, route: &str) {
    let complete_url = format!("http://{server_addr}{route}");
    let code = QrCode::new(&complete_url).unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("Swift File server is warming up...");
    println!("Scan the QR code to get started. Shut down the server by pressing \"Ctrl + c\"");
    println!("{complete_url}\n");
    println!("{}", image);
}
