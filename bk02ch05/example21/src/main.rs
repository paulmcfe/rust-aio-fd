fn main() {
    let strings_you_bow = "Bass\tCello\tErhu\tViola\tViolin";
    let strings: Vec<&str> = strings_you_bow.split_whitespace().collect();
    println!("{:#?}", strings);
}
