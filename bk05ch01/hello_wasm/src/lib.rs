use wasm_bindgen::prelude::*;

/// A tiny greeting maker just to prove Rust can run in your browser
#[wasm_bindgen]
pub fn hello(name: &str) -> String {
    format!("Greetings and salutations from Rust, {name}!")
}
