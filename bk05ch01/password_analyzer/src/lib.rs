use wasm_bindgen::prelude::*;

/// Analyzes a password and returns a short report.
///
/// This function is compiled to WebAssembly and called from JavaScript.
#[wasm_bindgen]
pub fn analyze_password(pw: &str) -> String {
    // Count Unicode scalar values
    // This is safer than using pw.len(), which returns the number of bytes
    let length = pw.chars().count();

    // Initialize the character categories
    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_digit = false;
    let mut has_symbol = false;

    for c in pw.chars() {
        // Check for a lowercase letter
        if c.is_ascii_lowercase() {
            has_lower = true;
        // Check for an uppercase letter
        } else if c.is_ascii_uppercase() {
            has_upper = true;
        // Check for a digit
        } else if c.is_ascii_digit() {
            has_digit = true;
        // Otherwise, it's a symbol
        } else {
            has_symbol = true;
        }
    }

    // Add up the character categories
    let mut char_categories = 0u32;
    if has_lower {
        char_categories += 1;
    }
    if has_upper {
        char_categories += 1;
    }
    if has_digit {
        char_categories += 1;
    }
    if has_symbol {
        char_categories += 1;
    }

    // Calculate the score as follows:
    // - up to 10 characters, 5 points each (max 50 points)
    // - each character category adds 10 points (max 40 points)
    // - bonus for long + diverse passwords (10 points)
    let mut score = 0u32;
    score += (length as u32).min(10) * 5;
    score += char_categories * 10;
    if length > 10 && char_categories >= 3 {
        score += 10;
    }
    if score > 100 {
        score = 100;
    }

    // Rate the password based on the score
    let rating = if score < 30 {
        "Very weak"
    } else if score < 50 {
        "Weak"
    } else if score < 70 {
        "Okay"
    } else if score < 90 {
        "Strong"
    } else {
        "Very strong"
    };

    // Return a report
    format!(
        "Length: {length} characters\n\
         Character types: {char_categories}\n\
         Score: {score}/100 ({rating})"
    )
}
