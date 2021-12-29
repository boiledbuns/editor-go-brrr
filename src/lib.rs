use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    // deref coercing -> converts String into &str
    alert(&format!("Hello, {}!", s));
}
