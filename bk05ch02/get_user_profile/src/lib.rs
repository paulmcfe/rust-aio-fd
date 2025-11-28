use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct UserProfile {
    username: String,
    age: u8,
    is_admin: bool,
}

#[wasm_bindgen]
pub fn get_user_profile() -> JsValue {
    let profile = UserProfile {
        username: "Alice".to_string(),
        age: 42,
        is_admin: true,
    };

    // Convert the struct into a JsValue using serde-wasm-bindgen
    serde_wasm_bindgen::to_value(&profile).unwrap()
}
