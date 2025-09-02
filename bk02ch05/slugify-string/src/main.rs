fn slugify(input: &str) -> String {
    fn push_hyphen(slug: &mut String, prev_hyphen: &mut bool) {
        if !*prev_hyphen && !slug.is_empty() {
            slug.push('-');
        }
        *prev_hyphen = true;
    }

    fn transliterate(ch: char) -> Option<&'static str> {
        match ch {
            // Single-letter replacements
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' => Some("a"),
            'ç' | 'ć' | 'ĉ' | 'ċ' | 'č' => Some("c"),
            'ď' | 'đ' => Some("d"),
            'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ĕ' | 'ė' | 'ę' | 'ě' => Some("e"),
            'ƒ' => Some("f"),
            'ĝ' | 'ğ' | 'ġ' | 'ģ' => Some("g"),
            'ĥ' | 'ħ' => Some("h"),
            'ì' | 'í' | 'î' | 'ï' | 'ī' | 'ĭ' | 'į' | 'ı' => Some("i"),
            'ĵ' => Some("j"),
            'ķ' => Some("k"),
            'ĺ' | 'ļ' | 'ľ' | 'ł' => Some("l"),
            'ñ' | 'ń' | 'ņ' | 'ň' | 'ŋ' => Some("n"),
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ŏ' | 'ő' => Some("o"),
            'ŕ' | 'ŗ' | 'ř' => Some("r"),
            'ś' | 'ŝ' | 'ş' | 'š' | 'ș' => Some("s"),
            'ţ' | 'ť' | 'ŧ' | 'ț' => Some("t"),
            'ù' | 'ú' | 'û' | 'ü' | 'ū' | 'ŭ' | 'ů' | 'ű' | 'ų' => Some("u"),
            'ẞ' | 'ß' => Some("ss"),
            'ý' | 'ÿ' | 'ŷ' => Some("y"),
            'ź' | 'ż' | 'ž' => Some("z"),
            // Common ligatures
            'æ' => Some("ae"),
            'œ' => Some("oe"),
            _ => None,
        }
    }

    let mut slug = String::with_capacity(input.len());
    let mut prev_hyphen = false;

    let trimmed = input.trim();

    for trimmed_char in trimmed.chars() {
        for ch in trimmed_char.to_lowercase() {
            // Is the character an ASCII character?
            // If so, push it as is to the slug
            // and skip over the rest of the loop
            if ch.is_ascii_alphanumeric() {
                slug.push(ch);
                prev_hyphen = false;
                continue;
            }

            // Is the character a whitespace character?
            // If so, push it as a hyphen
            // and skip over the rest of the loop
            if ch.is_whitespace() {
                push_hyphen(&mut slug, &mut prev_hyphen);
                continue;
            }

            // Is the character a separator character?
            // If so, push it as a hyphen
            // and skip over the rest of the loop
            match ch {
                '-' | '_' | '–' | '—' | '‐' | '/' | '\\' | '·' | '•' | '|' => {
                    push_hyphen(&mut slug, &mut prev_hyphen);
                    continue;
                }
                _ => {}
            }

            // Is the character a symbol that represents a word?
            // If so, push the word to the slug
            // and skip over the rest of the loop
            match ch {
                '&' => {
                    push_hyphen(&mut slug, &mut prev_hyphen);
                    slug.push_str("and");
                    prev_hyphen = false;
                    continue;
                }
                '+' => {
                    push_hyphen(&mut slug, &mut prev_hyphen);
                    slug.push_str("plus");
                    prev_hyphen = false;
                    continue;
                }
                '@' => {
                    push_hyphen(&mut slug, &mut prev_hyphen);
                    slug.push_str("at");
                    prev_hyphen = false;
                    continue;
                }
                _ => {}
            }

            // Is the character a punctuation mark?
            // If so, bypass it and skip over the rest of the loop
            match ch {
                '.' | ',' | ':' | ';' | '!' | '?' | '"' | '“' | '”' | '„' | '«' | '»' | '\''
                | '’' | '‘' | '(' | ')' | '[' | ']' | '{' | '}' | '#' | '$' | '%' | '^' | '='
                | '<' | '>' => {
                    continue;
                }
                _ => {}
            }

            // Now run the transliterate() function
            if let Some(transliterated_ch) = transliterate(ch) {
                slug.push_str(transliterated_ch);
                prev_hyphen = false;
            }

            // Any character not processed my a match or
            // by the transliterate() function is simply
            // discarded at this point. Bye!
        }
    }
    // Remove any trailing hyphens that might be hanging around
    while slug.ends_with('-') {
        slug.pop();
    }
    // Is our slug empty?
    if slug.is_empty() {
        // Return a default slug
        String::from("slug")
    } else {
        // Return the slug
        slug
    }
}

fn main() {
    let samples = [
        "  Rust For Dummies!  ",
        "Strings—Without—Tears",         // em dashes
        "Café-noir & crème brûlée",      // diacritics + ampersand
        "Rust: Tips, Tricks, & Gotchas", // punctuation
        "AC/DC + C++ @ 3am",             // slash, plus, at
        "“Smart” quotes — ‘everywhere’", // curly quotes + dash
        "Straße & Æsir vs. coöperate",   // ß, ligatures, diaeresis
        "路径/пример/δοκιμή",            // non-Latin drops
        "  -- already-sluggy --  ",      // leading/trailing separators
    ];

    for s in samples {
        println!("{:35} -> {}", s, slugify(s));
    }
}
