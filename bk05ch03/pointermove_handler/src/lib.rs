use wasm_bindgen::prelude::*;
use web_sys::PointerEvent;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no global window")?;

    let handler = Closure::wrap(Box::new(move |event: PointerEvent| {
        let x = event.client_x();
        let y = event.client_y();
        web_sys::console::log_1(&format!("Pointer at coordinates: ({}, {})", x, y).into());
    }) as Box<dyn FnMut(PointerEvent)>);

    window.add_event_listener_with_callback("pointermove", handler.as_ref().unchecked_ref())?;

    handler.forget();

    Ok(())
}
