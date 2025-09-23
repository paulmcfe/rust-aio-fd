fn split_before<'a>(text: &'a str, marker: &str) -> &'a str {
    if let Some(index) = text.find(marker) {
        &text[..index]
    } else {
        text
    }
}

fn main() {
    let email = "user@example.com";
    let username = split_before(email, "@");
    println!("{username}");
}
