use wasm_bindgen::prelude::*;

mod komuna;
mod sekreto;
mod servajxoj;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {name}!"));
}

#[wasm_bindgen]
pub fn embed(cover: &str, embed: &str, stego: &str) {
    komuna::ensxipigxi(cover, embed, stego, None).unwrap();
}

#[wasm_bindgen]
pub fn extract(stego: &str) {
    komuna::ekstrakti(stego, None, None).unwrap();
}
