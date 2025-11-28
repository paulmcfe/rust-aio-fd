use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn split_words(text: &str) -> Array {
    let arr = Array::new();

    for word in text.split_whitespace() {
        // Push the slice to the Array as a JsValue
        arr.push(&JsValue::from(word));
    }

    arr
}
