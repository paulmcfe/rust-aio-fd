use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn show_warning(value: i32) {
    if value > 100 {
        alert("Warning: Value exceeds maximum!");
    }
}
