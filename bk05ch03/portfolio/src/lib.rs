use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct PortfolioData {
    cards: Vec<Card>,
}

#[derive(Deserialize)]
struct Card {
    image: String,
    title: String,
    description: String,
}

// Get the JSON file contents
const PORTFOLIO_JSON: &str = include_str!("portfolio.json");

// Launch at initialization
#[wasm_bindgen(start)]
pub fn render_portfolio() -> Result<(), JsValue> {
    // Get access to the DOM
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no global window exists"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("should have document on window"))?;

    // Parse the JSON data
    let data: PortfolioData = serde_json::from_str(PORTFOLIO_JSON)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse portfolio.json: {}", e)))?;

    // Get the portfolio container
    let portfolio = document
        .get_element_by_id("portfolio")
        .ok_or_else(|| JsValue::from_str("should have portfolio element"))?;

    // Create portfolio cards from the data
    for card in data.cards {
        // Create card div
        let card_div = document.create_element("div")?;
        card_div.set_class_name("card");

        // Create card image
        let img = document.create_element("img")?;
        img.set_attribute("src", &card.image)?;
        img.set_attribute("alt", &format!("{} book cover", card.title))?;

        // Create card heading
        let h2 = document.create_element("h2")?;
        h2.set_text_content(Some(&card.title));

        // Create card paragraph
        let p = document.create_element("p")?;
        p.set_text_content(Some(&card.description));

        // Append the elements to the card
        card_div.append_child(&img)?;
        card_div.append_child(&h2)?;
        card_div.append_child(&p)?;

        // Append the card to the portfolio
        portfolio.append_child(&card_div)?;
    }

    // Update the heading from "Loading..." to "My Latest Book Designs"
    let heading = document
        .get_element_by_id("main-heading")
        .ok_or_else(|| JsValue::from_str("should have main-heading element"))?;
    heading.set_text_content(Some("My Latest Book Designs"));

    Ok(())
}
