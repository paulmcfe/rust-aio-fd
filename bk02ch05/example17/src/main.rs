fn main() {
    let messy_input = "  \t  Pull some strings  \t  ";

    // Remove whitespace from both ends
    let cleaned_input = messy_input.trim();
    println!("'{messy_input}' -> '{cleaned_input}'");

    // Remove whitespace from the start
    let cleaned_start = messy_input.trim_start();
    println!("'{messy_input}' -> '{cleaned_start}'");

    // Remove whitespace from the end
    let cleaned_end = messy_input.trim_end();
    println!("'{messy_input}' -> '{cleaned_end}'");
}
