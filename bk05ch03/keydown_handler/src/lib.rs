use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no global window")?;
    let document = window.document().ok_or("no document")?;

    let handler = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        let key = event.key();

        match key.as_str() {
            "ArrowUp" | "w" | "W" => {
                web_sys::console::log_1(&"Moving up!".into());
            }
            "ArrowDown" | "s" | "S" => {
                web_sys::console::log_1(&"Moving down!".into());
            }
            "ArrowLeft" | "a" | "A" => {
                web_sys::console::log_1(&"Moving left!".into());
            }
            "ArrowRight" | "d" | "D" => {
                web_sys::console::log_1(&"Moving right!".into());
            }
            " " => {
                web_sys::console::log_1(&"Spacebar pressed!".into());
            }
            _ => {}
        }
    }) as Box<dyn FnMut(KeyboardEvent)>);

    document.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}
