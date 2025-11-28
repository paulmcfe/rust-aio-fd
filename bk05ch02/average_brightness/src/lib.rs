use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// Receive the JS data as a slice: &[u8]
pub fn average_brightness(pixels: &[u8]) -> f32 {
    if pixels.is_empty() {
        return 0.0;
    }

    let sum: u32 = pixels.iter().map(|&b| b as u32).sum();
    sum as f32 / pixels.len() as f32
}
