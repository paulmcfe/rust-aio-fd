use wasm_bindgen::prelude::*;
use web_sys::{Element, console};

// Run this function upon initialization
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Get access to the DOM
    let window = web_sys::window().ok_or("no global window")?;
    let document = window.document().ok_or("no document")?;

    // Get the button element
    let button = document
        .get_element_by_id("click-me")
        .ok_or("button not found")?;

    // Set up a listener on the button
    set_up_click_listener(&button)?;

    Ok(())
}

pub fn set_up_click_listener(button: &Element) -> Result<(), JsValue> {
    // 1. Wrap the Rust event handler code in a Closure
    let handler = Closure::wrap(Box::new(move || {
        console::log_1(&"Button was clicked!".into());
    }) as Box<dyn FnMut()>);

    // 2. Attach the event listener
    button.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;

    // 3. Tell Rust to forget about this memory
    handler.forget();

    Ok(())
}
