use wasm_bindgen::prelude::*;
//use web_sys::{Document, Element, HtmlElement, Node, Window};

// Define the function that follows as the default startup function
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // 1. Get the Window
    let window = web_sys::window().ok_or("No global `window` found")?;

    // 2. Get the Document
    let document = window.document().ok_or("No document on window")?;

    // 3. Get the Element
    let status = document
        .get_element_by_id("status")
        .ok_or("No element with id `status`")?;

    // 4. Set the HTML element's text content
    status.set_text_content(Some("Rust says “Hello, DOM!” via WebAssembly!"));

    Ok(())
}
