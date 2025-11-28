use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    theme: String,
    show_tips: bool,
    items_per_page: u32,
}

#[wasm_bindgen]
pub fn apply_settings(settings_js: &JsValue) -> String {
    // Try to deserialize the incoming JS object into the Rust struct
    let settings: AppSettings = match serde_wasm_bindgen::from_value(settings_js.clone()) {
        Ok(s) => s,
        Err(err) => {
            return format!("Failed to parse settings: {err}");
        }
    };

    // Here you'd normally store the settings, update global state, etc.
    // For the demo, just return a summary string
    format!(
        "Applied settings: theme = {}, show_tips = {}, items_per_page = {}",
        settings.theme, settings.show_tips, settings.items_per_page
    )
}
