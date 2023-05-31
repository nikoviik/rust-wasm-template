use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    web_sys::console::log_1(&"Hello world!".into());
}
