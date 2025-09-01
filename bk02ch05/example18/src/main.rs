fn main() {
    let email = "cellos@orchestra.org";
    let url = "https://orchestra.org";

    // Check if a string contains a substring
    if email.contains("@") {
        println!("{email} looks like a legit address. Nice.");
    }

    // Check the start of a string
    if url.starts_with("https://") {
        println!("{url} is a secure site. Awesome.")
    }

    // Check the end of a string
    if url.ends_with(".org") {
        println!("{url} is a non-profit. Cool.")
    }

    // Find the position of a substring
    if let Some(at_position) = email.find("@") {
        let username = &email[..at_position];
        let domain = &email[at_position + 1..];
        println!("Username: {username}. Domain: {domain}. Boom!");
    }
}
