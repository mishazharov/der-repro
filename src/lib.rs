use wasm_bindgen::prelude::*;
use der;
use der::Length;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let mut result = [0u8; 24];
    let mut enc = der::Encoder::new(&mut result);
    enc.sequence(Length::ZERO, |_| { Ok(()) }).unwrap();
    alert(&format!("Hello, {}!", name));
}
