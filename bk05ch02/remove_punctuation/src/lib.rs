use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn remove_punctuation(text: &str) -> String {
    text.chars().filter(|c| !c.is_ascii_punctuation()).collect()
}
