use local_ip_address::local_ip;
use qrcode::render::unicode;
use qrcode::QrCode;

pub fn generate_qr_code(port: &u16, route: &str) {
    let machine_ip = local_ip().unwrap().to_string();
    let complete_url = format!("http://{machine_ip}:{port}{route}");
    println!("URL: {complete_url}");
    let code = QrCode::new(complete_url).unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", image);
}
