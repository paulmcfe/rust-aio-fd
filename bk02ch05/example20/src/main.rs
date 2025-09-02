fn main() {
    let strings_you_bow = "Bass\tCello\tErhu\tViola\tViolin";
    for instrument in strings_you_bow.split_whitespace() {
        println!("{instrument}");
    }
}
