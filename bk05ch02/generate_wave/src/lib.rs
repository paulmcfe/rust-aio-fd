use js_sys::Float32Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_wave(samples: u32) -> Float32Array {
    let mut data = Vec::with_capacity(samples as usize);
    let frequency = 440.0; // A4 note

    for i in 0..samples {
        let t = i as f32 / samples as f32;
        // Calculate sine values in radians (note: TAU = 2π)
        let value = (t * frequency * std::f32::consts::TAU).sin();
        data.push(value);
    }

    // Copy the Vec<f32> data from WASM memory into a JavaScript Float32Array
    // This allows JS to access the samples in its own typed array format
    Float32Array::from(data.as_slice())
}
