use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn main() {
    web_sys::console::log_1(&"Hello world!".into());
}
