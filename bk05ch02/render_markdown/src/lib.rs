use pulldown_cmark::{Parser, html};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// Render a Markdown string to HTML
#[wasm_bindgen]
pub fn render_markdown(markdown_input: &str) -> String {
    // 1. Create a parser for the markdown
    let parser = Parser::new(markdown_input);

    // 2. Create a buffer to hold the HTML output
    let mut html_output = String::new();

    // 3. Push the parsed HTML into the buffer
    html::push_html(&mut html_output, parser);

    // 4. Return the HTML string
    html_output
}

/// Fetch Markdown from a URL and render it to HTML
#[wasm_bindgen]
pub async fn fetch_and_render(url: &str) -> Result<String, JsValue> {
    // 1. Set up the fetch request with CORS mode
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    // 2. Create the request object
    let request = Request::new_with_str_and_init(url, &opts)?;

    // 3. Get the global window object and call fetch
    let window = web_sys::window().ok_or("No window object")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // 4. Convert the response to a Response object
    let resp: Response = resp_value.dyn_into()?;

    // 5. Check if the response was successful
    if !resp.ok() {
        return Err(JsValue::from_str(&format!(
            "HTTP error: {} {}",
            resp.status(),
            resp.status_text()
        )));
    }

    // 6. Get the response body as text
    let text = JsFuture::from(resp.text()?).await?;
    let markdown = text.as_string().ok_or("Response was not valid text")?;

    // 7. Render the Markdown to HTML and return
    Ok(render_markdown(&markdown))
}
