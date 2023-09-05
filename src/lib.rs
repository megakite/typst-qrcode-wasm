use wasm_minimal_protocol::*;
use qrcode::{QrCode, render::svg};

initiate_protocol!();

#[wasm_func]
pub fn generate(arg: &[u8]) -> Vec<u8> {
    let code = QrCode::new(arg).unwrap();
    let svg_xml = code.render::<svg::Color>().build();

    svg_xml.into_bytes()
}
