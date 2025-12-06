use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlButtonElement, HtmlElement};

// Here's the application state
#[derive(Debug)]
struct AppState {
    count: i32,
    step: i32,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Get access to the DOM
    let window = web_sys::window().ok_or("no global window")?;
    let document = window.document().ok_or("no document")?;

    // Get the Increment button and downcast it to HtmlButtonElement
    let increment_button = document
        .get_element_by_id("increment_btn")
        .ok_or("no `increment_btn` button")?
        .dyn_into::<HtmlButtonElement>()?;

    // Get the Step button and downcast it to HtmlButtonElement
    let step_button = document
        .get_element_by_id("step_btn")
        .ok_or("no `step_btn` button")?
        .dyn_into::<HtmlButtonElement>()?;

    // Get the Display element and downcast it to HtmlElement
    let display = document
        .get_element_by_id("display")
        .ok_or("no `display` element")?
        .dyn_into::<HtmlElement>()?;

    // Initialize the application state using Rc<RefCell<AppState>>
    let state = Rc::new(RefCell::new(AppState { count: 0, step: 1 }));

    update_display(&display, &state.borrow());

    // Click the Increment button: add `step` to `count`.
    {
        // Get a smart pointer to the state
        let state = Rc::clone(&state);
        let display = display.clone();

        let handler = Closure::wrap(Box::new(move |_event: Event| {
            // Borrow the state mutably
            let mut s = state.borrow_mut();

            // Update the count property
            s.count += s.step;
            update_display(&display, &s);
        }) as Box<dyn FnMut(_)>);

        increment_button
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;
        handler.forget();
    }

    // Click the Step button: increase the increment size.
    {
        // Get a smart pointer to the state
        let state = Rc::clone(&state);
        let display = display.clone();

        let handler = Closure::wrap(Box::new(move |_event: Event| {
            // Borrow the state mutably
            let mut s = state.borrow_mut();

            // Update the count property
            s.step += 1;
            update_display(&display, &s);
        }) as Box<dyn FnMut(_)>);

        step_button.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;
        handler.forget();
    }

    Ok(())
}

// Use the application state to update the display
fn update_display(display: &HtmlElement, state: &AppState) {
    display.set_inner_text(&format!("Count: {} (step = {})", state.count, state.step));
}
