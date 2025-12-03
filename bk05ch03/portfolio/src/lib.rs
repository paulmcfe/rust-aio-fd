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

const PORTFOLIO_JSON: &str = include_str!("portfolio.json");

#[wasm_bindgen(start)]
pub fn render_portfolio() {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have document on window");

    // Parse the JSON data
    let data: PortfolioData =
        serde_json::from_str(PORTFOLIO_JSON).expect("Failed to parse portfolio.json");

    // Get the portfolio container
    let portfolio = document
        .get_element_by_id("portfolio")
        .expect("should have portfolio element");

    // Create cards from the data
    for card in data.cards {
        // Create card div
        let card_div = document.create_element("div").expect("should create div");
        card_div.set_class_name("card");

        // Create image
        let img = document.create_element("img").expect("should create img");
        img.set_attribute("src", &card.image).unwrap();
        img.set_attribute("alt", &format!("{} book cover", card.title))
            .unwrap();

        // Create heading
        let h2 = document.create_element("h2").expect("should create h2");
        h2.set_text_content(Some(&card.title));

        // Create paragraph
        let p = document.create_element("p").expect("should create p");
        p.set_text_content(Some(&card.description));

        // Append elements to card
        card_div.append_child(&img).unwrap();
        card_div.append_child(&h2).unwrap();
        card_div.append_child(&p).unwrap();

        // Append card to portfolio
        portfolio.append_child(&card_div).unwrap();
    }

    // Update the heading from "Loading..." to "My Latest Book Designs"
    let heading = document
        .get_element_by_id("main-heading")
        .expect("should have main-heading element");
    heading.set_text_content(Some("My Latest Book Designs"));
}
