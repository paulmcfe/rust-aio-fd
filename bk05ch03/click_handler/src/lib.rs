use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document");
    
    // Get the button element
    let button = document.get_element_by_id(button_id)
        .expect("button not found");

    // Set up a listner on the button
    setup_click_listener(button);

}

pub fn setup_click_listener(button_id: &str) -> Result<(), JsValue> {

    // --- The Tricky Part ---
    // We wrap our Rust code in a Closure.
    // The `Box::new` puts it on the heap so it has a stable address.
    let handler = Closure::wrap(Box::new(move || {
        console::log_1(&"Button was clicked!".into());
    }) as Box<dyn FnMut()>);

    // Attach the event listener.
    // We have to convert our safe Closure into a generic Javascript reference.
    button.add_event_listener_with_callback(
        "click", 
        handler.as_ref().unchecked_ref()
    )?;

    // IMPORTANT: We must tell Rust to *forget* about this memory.
    // If we don't, Rust will drop the `handler` at the end of this function,
    // and the click listener will stop working (or crash) instantly.
    handler.forget();

    Ok(())
}