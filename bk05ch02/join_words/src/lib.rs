use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn join_words(arr: &Array, separator: &str) -> String {
    let mut pieces = Vec::new();

    for value in arr.iter() {
        if let Some(s) = value.as_string() {
            pieces.push(s);
        }
    }

    pieces.join(separator)
}
